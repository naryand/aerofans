mod auth;
mod post;
mod reply;

use actix_web::{
    error::InternalError,
    http::StatusCode,
    web::{self, ServiceConfig},
    HttpResponse,
};

use sea_orm::DbErr;

use self::{post as route_post, reply as route_reply};

// Helper functions for returning status codes
fn to_internal_error(e: DbErr) -> InternalError<DbErr> {
    InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)
}

fn to_not_found(e: DbErr) -> InternalError<DbErr> {
    InternalError::new(e, StatusCode::NOT_FOUND)
}

fn to_ok<T>(_: T) -> HttpResponse {
    HttpResponse::new(StatusCode::OK)
}

// Configure API routes
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/post")
            .route("", web::post().to(route_post::create))
            .route("/all", web::get().to(route_post::read_all))
            .service(
                web::scope("/{post_id}")
                    .route("", web::get().to(route_post::read))
                    .route("", web::patch().to(route_post::update))
                    .route("", web::delete().to(route_post::delete))
                    .service(
                        web::scope("/reply")
                            .route("", web::post().to(route_reply::create))
                            .route("/all", web::get().to(route_reply::read_all))
                            .service(
                                web::scope("/{reply_id}")
                                    .route("", web::get().to(route_reply::read))
                                    .route("", web::patch().to(route_reply::update))
                                    .route("", web::delete().to(route_reply::delete)),
                            ),
                    ),
            ),
    )
    .service(web::resource("/register").route(web::post().to(auth::create)))
    .service(web::resource("/login").route(web::post().to(auth::login)));
}
