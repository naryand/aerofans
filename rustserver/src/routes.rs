use crate::{
    db::Db,
    model::{
        AuthUser, CreatePost, CreateReply, LoginResponse, LoginUser, Post, Reply, Token,
        UpdatePost, UpdateReply,
    },
    StdErr,
};

use std::pin::Pin;

use actix_web::{
    cookie::Cookie,
    dev::Payload,
    error::InternalError,
    http::StatusCode,
    web::{self, Data, Json, Path, ServiceConfig},
    FromRequest, HttpMessage, HttpRequest, HttpResponse,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use futures::{future, Future, FutureExt};

// Helper functions for returning status codes
fn to_internal_error(e: StdErr) -> InternalError<StdErr> {
    InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)
}

fn to_not_found(e: StdErr) -> InternalError<StdErr> {
    InternalError::new(e, StatusCode::NOT_FOUND)
}

fn to_ok(_: ()) -> HttpResponse {
    HttpResponse::new(StatusCode::OK)
}

//
// --------------------------- USERS ---------------------------
//

// POST /register
// Takes in JSON encoded LoginUser
// On success, returns 200 OK with JSON encoded LoginResponse
// On error, returns 500 Internal Server Error
async fn create_user(create_user: Json<LoginUser>, db: Data<Db>) -> Json<LoginResponse> {
    let hash = hash(create_user.0.password, DEFAULT_COST).unwrap();
    match db
        .create_user(LoginUser {
            username: create_user.0.username,
            password: hash,
        })
        .await
    {
        Ok(_) => Json(LoginResponse {
            status: true,
            message: String::from("registration successful"),
        }),
        Err(_) => Json(LoginResponse {
            status: false,
            message: String::from("username is taken"),
        }),
    }
}

// POST /login
// Takes in JSON encoded LoginUser
// On success, returns 200 OK with JSON encoded LoginResponse and cookie
// On error, returns 500 Internal Server Error
async fn login_user(login_user: Json<LoginUser>, db: Data<Db>) -> HttpResponse {
    let user = match db.read_user(login_user.0.username).await {
        Ok(u) => u,
        Err(_) => {
            return HttpResponse::build(StatusCode::OK).json(LoginResponse {
                status: false,
                message: String::from("username doesn't exist"),
            })
        }
    };

    if verify(login_user.0.password, &user.password).unwrap() {
        // Build response
        let mut response = HttpResponse::build(StatusCode::OK).json(LoginResponse {
            status: true,
            message: String::from("login successful"),
        });

        // Generate token id and add to response as cookie
        let mut buf = [0; 16];
        openssl::rand::rand_bytes(&mut buf).unwrap();
        let id = openssl::base64::encode_block(&buf);
        response.add_cookie(&Cookie::new("token", &id)).unwrap();

        // Set expiry
        let expiry = chrono::Utc::now() + chrono::Duration::hours(1);

        // Build
        let token = Token {
            id: String::clone(&id),
            user_id: user.id,
            expires_at: expiry,
        };

        // Put token in database
        match db.create_token(token).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                return HttpResponse::InternalServerError().finish();
            }
        }

        response
    } else {
        HttpResponse::build(StatusCode::OK).json(LoginResponse {
            status: false,
            message: String::from("incorrect login info"),
        })
    }
}

// Implements user authentication
// Takes token from cookie and checks against database
impl FromRequest for AuthUser {
    type Error = InternalError<&'static str>;
    type Config = ();

    type Future = futures::future::Either<
        future::Ready<Result<Self, Self::Error>>,
        Pin<Box<dyn Future<Output = Result<Self, Self::Error>> + 'static>>,
    >;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let cookie = match req.cookie("token") {
            Some(c) => c,
            None => {
                return future::err(InternalError::new(
                    "missing cookie",
                    StatusCode::BAD_REQUEST,
                ))
                .left_future()
            }
        };

        let id = cookie.value().to_owned();

        let db = match req.app_data::<Data<Db>>() {
            Some(db) => &**db,
            None => {
                return future::err(InternalError::new(
                    "no database",
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
                .left_future()
            }
        };

        let db = db.clone();

        async move {
            let token = match db.read_token(id).await {
                Ok(t) => t,
                Err(_) => {
                    return Err(InternalError::new(
                        "invalid token",
                        StatusCode::UNAUTHORIZED,
                    ))
                }
            };

            if token.expires_at < chrono::Utc::now() {
                return Err(InternalError::new(
                    "expired token",
                    StatusCode::UNAUTHORIZED,
                ));
            }

            Ok(AuthUser { id: token.user_id })
        }
        .boxed_local()
        .right_future()
    }
}

//
// --------------------------- POSTS ---------------------------
//

// POST /post
// Takes in JSON encoded CreatePost and user auth
// On success, returns 200 OK with JSON encoded Post
// On error, returns 500 Internal Server Error
async fn create_post(
    create_post: Json<CreatePost>,
    db: Data<Db>,
    author: AuthUser,
) -> Result<Json<Post>, InternalError<StdErr>> {
    db.create_post(create_post.0, author.id)
        .await
        .map(Json)
        .map_err(to_internal_error)
}

// GET /post/all
// On success, returns 200 OK with JSON encoded Posts
// On error, returns 500 Internal Server Error
async fn read_all_posts(db: Data<Db>) -> Result<Json<Vec<Post>>, InternalError<StdErr>> {
    db.read_all_posts()
        .await
        .map(Json)
        .map_err(to_internal_error)
}

// GET /post/{post_id}
// On success, returns 200 OK with JSON encoded Post
// If post_id does not exist, returns 404 Not Found
async fn read_post(
    Path(post_id): Path<i64>,
    db: Data<Db>,
) -> Result<Json<Post>, InternalError<StdErr>> {
    db.read_post(post_id).await.map(Json).map_err(to_not_found)
}

// PATCH /post/{post_id}
// Takes in JSON encoded UpdatePost and user auth
// On success, updates and returns 200 OK with JSON encoded Post
// If post_id does not exist, returns 404 Not Found
async fn update_post(
    update_post: Json<UpdatePost>,
    Path(post_id): Path<i64>,
    db: Data<Db>,
    author: AuthUser,
) -> Result<Json<Post>, InternalError<StdErr>> {
    let post = db.read_post(post_id).await.map_err(to_not_found)?;

    if post.author != author.id {
        return Err(InternalError::new(
            String::from("not real author").into(),
            StatusCode::UNAUTHORIZED,
        ));
    }

    db.update_post(post_id, update_post.0)
        .await
        .map(Json)
        .map_err(to_internal_error)
}

// DELETE /post/{post_id}
// Takes in user auth
// On success, deletes and returns 200 OK
// If post_id does not exist, returns 404 Not Found
async fn delete_post(
    Path(post_id): Path<i64>,
    db: Data<Db>,
    author: AuthUser,
) -> Result<HttpResponse, InternalError<StdErr>> {
    let post = db.read_post(post_id).await.map_err(to_not_found)?;

    if post.author != author.id {
        return Err(InternalError::new(
            String::from("not real author").into(),
            StatusCode::UNAUTHORIZED,
        ));
    }

    db.delete_post(post_id)
        .await
        .map(to_ok)
        .map_err(to_internal_error)
}

//
// --------------------------- REPLIES ---------------------------
//

// POST /post/{post_id}/reply
// Takes in JSON encoded CreateReply and user auth
// On success, returns 200 OK with JSON encoded Reply
// If post_id does not exist, returns 404 Not Found
async fn create_reply(
    create_reply: Json<CreateReply>,
    Path(post_id): Path<i64>,
    db: Data<Db>,
    author: AuthUser,
) -> Result<Json<Reply>, InternalError<StdErr>> {
    db.create_reply(create_reply.0, post_id, author.id)
        .await
        .map(Json)
        .map_err(to_not_found)
}

// GET /post/{post_id}/reply/all
// On success, returns 200 OK with JSON encoded Replies
// If post_id does not exist, returns 404 Not Found
async fn read_all_replies(
    Path(post_id): Path<i64>,
    db: Data<Db>,
) -> Result<Json<Vec<Reply>>, InternalError<StdErr>> {
    db.read_all_replies(post_id)
        .await
        .map(Json)
        .map_err(to_not_found)
}

// GET /post/{post_id}/reply/{reply_id}
// On success, returns 200 OK with JSON encoded Reply
// If post_id, reply_id does not exist, returns 404 Not Found
async fn read_reply(
    Path((post_id, reply_id)): Path<(i64, i64)>,
    db: Data<Db>,
) -> Result<Json<Reply>, InternalError<StdErr>> {
    db.read_reply(reply_id, post_id)
        .await
        .map(Json)
        .map_err(to_not_found)
}

// PATCH /post/{post_id}/reply/{reply_id}
// Takes in JSON encoded UpdateReply and user auth
// On success, updates and returns 200 OK with JSON encoded Reply
// If post_id, reply_id does not exist, returns 404 Not Found
async fn update_reply(
    update_reply: Json<UpdateReply>,
    Path((post_id, reply_id)): Path<(i64, i64)>,
    db: Data<Db>,
    author: AuthUser,
) -> Result<Json<Reply>, InternalError<StdErr>> {
    let reply = db
        .read_reply(reply_id, post_id)
        .await
        .map_err(to_not_found)?;

    if reply.author != author.id {
        return Err(InternalError::new(
            String::from("not real author").into(),
            StatusCode::UNAUTHORIZED,
        ));
    }

    db.update_reply(reply_id, post_id, update_reply.0)
        .await
        .map(Json)
        .map_err(to_internal_error)
}

// DELETE /post/{post_id}/reply/{reply_id}
// Takes in user auth
// On success, deletes and returns 200 OK
// If post_id, reply_id does not exist, returns 404 Not Found
async fn delete_reply(
    Path((post_id, reply_id)): Path<(i64, i64)>,
    db: Data<Db>,
    author: AuthUser,
) -> Result<HttpResponse, InternalError<StdErr>> {
    let reply = db
        .read_reply(reply_id, post_id)
        .await
        .map_err(to_not_found)?;

    if reply.author != author.id {
        return Err(InternalError::new(
            String::from("not real author").into(),
            StatusCode::UNAUTHORIZED,
        ));
    }

    db.delete_reply(reply_id, post_id)
        .await
        .map(to_ok)
        .map_err(to_internal_error)
}

// Configure API routes
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/post")
            .route("", web::post().to(create_post))
            .route("/all", web::get().to(read_all_posts))
            .service(
                web::scope("/{post_id}")
                    .route("", web::get().to(read_post))
                    .route("", web::patch().to(update_post))
                    .route("", web::delete().to(delete_post))
                    .service(
                        web::scope("/reply")
                            .route("", web::post().to(create_reply))
                            .route("/all", web::get().to(read_all_replies))
                            .service(
                                web::scope("/{reply_id}")
                                    .route("", web::get().to(read_reply))
                                    .route("", web::patch().to(update_reply))
                                    .route("", web::delete().to(delete_reply)),
                            ),
                    ),
            ),
    )
    .service(web::resource("/register").route(web::post().to(create_user)))
    .service(web::resource("/login").route(web::post().to(login_user)));
}
