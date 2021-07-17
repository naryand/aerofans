use crate::{
    db::Db,
    model::{CreatePost, CreateReply, Post, Reply, UpdatePost, UpdateReply},
    StdErr,
};

use actix_web::{
    dev::HttpServiceFactory,
    error::InternalError,
    http::StatusCode,
    web::{self, Data, Json, Path},
    HttpResponse,
};

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
// --------------------------- POSTS ---------------------------
//

// /post
// Takes in POST request with JSON encoded CreatePost in body
// On success, returns 200 OK with JSON encoded Post in body
// On error, returns 500 Internal Server Error
async fn create_post(
    create_post: Json<CreatePost>,
    db: Data<Db>,
) -> Result<Json<Post>, InternalError<StdErr>> {
    db.create_post(create_post.0)
        .await
        .map(Json)
        .map_err(to_internal_error)
}

// /post/all
// Takes in GET request
// On success, returns 200 OK with JSON encoded Posts in body
// On error, returns 500 Internal Server Error
async fn read_all_posts(db: Data<Db>) -> Result<Json<Vec<Post>>, InternalError<StdErr>> {
    db.read_all_posts()
        .await
        .map(Json)
        .map_err(to_internal_error)
}

// /post/{post_id}
// Takes in GET request and post_id from URL
// On success, returns 200 OK with JSON encoded Post in body
// If post_id does not exist, returns 404 Not Found
async fn read_post(
    Path(post_id): Path<i64>,
    db: Data<Db>,
) -> Result<Json<Post>, InternalError<StdErr>> {
    db.read_post(post_id).await.map(Json).map_err(to_not_found)
}

// /post/{post_id}
// Takes in PATCH request with JSON encoded UpdatePost in body and post_id from URL
// On success, updates the post and returns 200 OK with JSON encoded Post in body
// If post_id does not exist, returns 404 Not Found
async fn update_post(
    update_post: Json<UpdatePost>,
    Path(post_id): Path<i64>,
    db: Data<Db>,
) -> Result<Json<Post>, InternalError<StdErr>> {
    db.update_post(post_id, update_post.0)
        .await
        .map(Json)
        .map_err(to_not_found)
}

// /post/{post_id}
// Takes in DELETE request and post_id from URL =
// On success, deletes the Post and returns 200 OK
// If post_id does not exist, returns 404 Not Found
async fn delete_post(
    Path(post_id): Path<i64>,
    db: Data<Db>,
) -> Result<HttpResponse, InternalError<StdErr>> {
    db.delete_post(post_id)
        .await
        .map(to_ok)
        .map_err(to_not_found)
}

//
// --------------------------- REPLIES ---------------------------
//

// /post/{post_id}/reply
// Takes in POST request with JSON encoded CreateReply in body and post_id from URL
// On success, returns 200 OK with JSON encoded Reply in body
// If post_id does not exist, returns 404 Not Found
async fn create_reply(
    create_reply: Json<CreateReply>,
    Path(post_id): Path<i64>,
    db: Data<Db>,
) -> Result<Json<Reply>, InternalError<StdErr>> {
    db.create_reply(create_reply.0, post_id)
        .await
        .map(Json)
        .map_err(to_not_found)
}

// /post/{post_id}/reply/all
// Takes in GET request and post_id from URL
// On success, returns 200 OK with JSON encoded Replies in body
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

// /post/{post_id}/reply/{reply_id}
// Takes in GET request and post_id, reply_id from URL
// On success, returns 200 OK with JSON encoded Reply in body
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

// /post/{post_id}/reply/{reply_id}
// Takes in PATCH request with JSON encoded UpdateReply in body and post_id, reply_id from URL
// On success, updates the Reply and returns 200 OK with JSON encoded Reply in body
// If post_id, reply_id does not exist, returns 404 Not Found
async fn update_reply(
    update_reply: Json<UpdateReply>,
    Path((post_id, reply_id)): Path<(i64, i64)>,
    db: Data<Db>,
) -> Result<Json<Reply>, InternalError<StdErr>> {
    db.update_reply(reply_id, post_id, update_reply.0)
        .await
        .map(Json)
        .map_err(to_not_found)
}

// /post/{post_id}/reply/{reply_id}
// Takes in DELETE request and post_id, reply_id from URL
// On success, deletes the Reply and returns 200 OK
// If post_id, reply_id does not exist, returns 404 Not Found
async fn delete_reply(
    Path((post_id, reply_id)): Path<(i64, i64)>,
    db: Data<Db>,
) -> Result<HttpResponse, InternalError<StdErr>> {
    db.delete_reply(reply_id, post_id)
        .await
        .map(to_ok)
        .map_err(to_not_found)
}

// Configure API routes
pub fn api() -> impl HttpServiceFactory + 'static {
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
        )
}
