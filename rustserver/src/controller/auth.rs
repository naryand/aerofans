use actix_web::{
    cookie::{Cookie, SameSite},
    dev::Payload,
    error::InternalError,
    http::StatusCode,
    web::{Data, Json},
    FromRequest, HttpRequest, HttpResponse,
};
use bcrypt::{hash, hash_with_salt, verify, DEFAULT_COST};
use chrono::Utc;
use futures::{future, Future, FutureExt};
use sea_orm::{
    prelude::Uuid, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};

use crate::model::{
    token,
    user::{self, LoginResponse},
};

// POST /register
// Takes in JSON encoded user Input
// On success, returns 200 OK with JSON encoded LoginResponse
// On error, returns 500 Internal Server Error
pub async fn create(
    Json(mut input_user): Json<user::Input>,
    db: Data<DatabaseConnection>,
) -> Json<LoginResponse> {
    input_user.password = match hash(input_user.password, DEFAULT_COST) {
        Ok(p) => p,
        Err(_) => {
            return Json(LoginResponse {
                status: false,
                message: "invalid registration info",
            })
        }
    };

    let input_user = input_user.into_active_model();
    let input_user = user::ActiveModel {
        created_at: Set(Utc::now().naive_utc()),
        ..input_user
    };

    match input_user.insert(db.get_ref()).await {
        Ok(_) => Json(LoginResponse {
            status: true,
            message: "registration successful",
        }),
        Err(_) => Json(LoginResponse {
            status: false,
            message: "username is taken",
        }),
    }
}

// POST /login
// Takes in JSON encoded user Input
// On success, returns 200 OK with JSON encoded LoginResponse and cookie
// On error, returns 500 Internal Server Error
pub async fn login(
    Json(login_user): Json<user::Input>,
    db: Data<DatabaseConnection>,
) -> HttpResponse {
    let user = match user::Entity::find()
        .filter(user::Column::Username.eq(login_user.username))
        .one(db.get_ref())
        .await
    {
        Ok(Some(u)) => u,
        Err(_) | Ok(None) => {
            return HttpResponse::build(StatusCode::OK).json(LoginResponse {
                status: false,
                message: "username doesn't exist",
            })
        }
    };

    if let Some(true) = verify(login_user.password, &user.password).ok() {
        // Build response
        let mut response = HttpResponse::build(StatusCode::OK).json(LoginResponse {
            status: true,
            message: "login successful",
        });

        // Generate token id and add to response as cookie
        let uuid = Uuid::new_v4();

        let hash = hash_with_salt(uuid.as_bytes(), 4, &[0; 16][..])
            .unwrap()
            .to_string();

        let mut buf = [b'x'; 36];
        let str = uuid.to_hyphenated().encode_lower(&mut buf);

        let cookie = Cookie::build("token", &*str)
            .secure(true)
            .permanent()
            .same_site(SameSite::None)
            .finish();
        response.add_cookie(&cookie).unwrap();

        // Set expiry
        let expiry = chrono::Utc::now() + chrono::Duration::weeks(1240);

        // Build
        let token = token::Model {
            hash,
            user_id: user.id,
            expires_at: expiry.naive_utc(),
        };

        // Put token in database
        let _ = token.into_active_model().insert(db.get_ref()).await;

        response
    } else {
        HttpResponse::build(StatusCode::OK).json(LoginResponse {
            status: false,
            message: "invalid login info",
        })
    }
}

// Implements user authentication
// Takes token from cookie and checks against database
impl FromRequest for token::Model {
    type Error = InternalError<&'static str>;

    type Future = futures::future::Either<
        future::Ready<Result<Self, Self::Error>>,
        std::pin::Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>,
    >;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let uuid = match req
            .cookie("token")
            .as_ref()
            .map(Cookie::value)
            .map(Uuid::parse_str)
        {
            Some(Ok(t)) => t,
            _ => {
                return future::err(InternalError::new(
                    "no valid token",
                    StatusCode::BAD_REQUEST,
                ))
                .left_future()
            }
        };

        let db = req
            .app_data::<Data<DatabaseConnection>>()
            .unwrap()
            .as_ref()
            .clone();

        let hash = hash_with_salt(uuid.as_bytes(), 4, &[0; 16][..])
            .unwrap()
            .to_string();

        async move {
            Ok(match token::Entity::find_by_id(hash).one(&db).await {
                Ok(Some(t)) if t.expires_at > chrono::Utc::now().naive_utc() => t,
                _ => {
                    return Err(InternalError::new(
                        "invalid token",
                        StatusCode::UNAUTHORIZED,
                    ))
                }
            })
        }
        .boxed_local()
        .right_future()
    }
}
