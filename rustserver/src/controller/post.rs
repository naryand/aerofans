use actix_web::{
    error::InternalError,
    http::StatusCode,
    web::{Data, Json, Path},
    HttpResponse,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, QuerySelect,
    RelationTrait, Set, Value,
};

use crate::model::{post, token, user};

use super::{to_internal_error, to_not_found, to_ok};

// POST /post
// Takes in JSON encoded post Input and user auth
// On success, returns 200 OK with JSON encoded post Output
// On error, returns 500 Internal Server Error
pub async fn create(
    Json(input_post): Json<post::Input>,
    db: Data<DatabaseConnection>,
    token: token::Model,
) -> Result<Json<post::Output>, InternalError<DbErr>> {
    let input_post = input_post.into_active_model();
    let input_post = post::ActiveModel {
        user_id: Set(token.user_id),
        created_at: Set(Utc::now().naive_utc()),
        ..input_post
    };

    let post = input_post
        .insert(db.as_ref())
        .await
        .map_err(to_internal_error)?;

    post::Entity::find_by_id(post.id)
        .join(sea_orm::JoinType::InnerJoin, post::Relation::User.def())
        .column(user::Column::Username)
        .into_model::<post::Output>()
        .one(db.as_ref())
        .await
        .transpose()
        .ok_or_else(|| DbErr::RecordNotFound(String::new()))
        .and_then(std::convert::identity)
        .map(Json)
        .map_err(to_internal_error)
}

// GET /post/all
// On success, returns 200 OK with JSON encoded post Outputs
// On error, returns 500 Internal Server Error
pub async fn read_all(
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<post::Output>>, InternalError<DbErr>> {
    post::Entity::find()
        .join(sea_orm::JoinType::InnerJoin, post::Relation::User.def())
        .column(user::Column::Username)
        .into_model::<post::Output>()
        .all(db.as_ref())
        .await
        .map(Json)
        .map_err(to_internal_error)
}

// GET /post/{post_id}
// On success, returns 200 OK with JSON encoded post Output
// If post_id does not exist, returns 404 Not Found
pub async fn read(
    param: Path<i64>,
    db: Data<DatabaseConnection>,
) -> Result<Json<post::Output>, InternalError<DbErr>> {
    let post_id = param.into_inner();

    post::Entity::find_by_id(post_id)
        .join(sea_orm::JoinType::InnerJoin, post::Relation::User.def())
        .column(user::Column::Username)
        .into_model::<post::Output>()
        .one(db.as_ref())
        .await
        .transpose()
        .ok_or_else(|| DbErr::RecordNotFound(String::new()))
        .and_then(std::convert::identity)
        .map(Json)
        .map_err(to_not_found)
}

// PATCH /post/{post_id}
// Takes in JSON encoded post Input and token
// On success, updates and returns 200 OK with JSON encoded post Output
// If post_id does not exist, returns 404 Not Found
pub async fn update(
    Json(input_post): Json<post::Input>,
    param: Path<i64>,
    db: Data<DatabaseConnection>,
    token: token::Model,
) -> Result<Json<post::Output>, InternalError<DbErr>> {
    let post_id = param.into_inner();

    let post = post::Entity::find_by_id(post_id)
        .one(db.as_ref())
        .await
        .transpose()
        .ok_or_else(|| DbErr::RecordNotFound(String::new()))
        .and_then(std::convert::identity)
        .map_err(to_not_found)?;

    if post.user_id != token.user_id {
        return Err(InternalError::new(
            DbErr::Custom("not real author".to_string()),
            StatusCode::UNAUTHORIZED,
        ));
    }

    let mut input_post = input_post.into_active_model();

    input_post.set(post::Column::Id, Value::BigInt(Some(post_id)));
    input_post.set(post::Column::UserId, Value::BigInt(Some(token.user_id)));

    let post = input_post
        .update(db.as_ref())
        .await
        .map_err(to_internal_error)?;

    post::Entity::find_by_id(post.id)
        .join(sea_orm::JoinType::InnerJoin, post::Relation::User.def())
        .column(user::Column::Username)
        .into_model::<post::Output>()
        .one(db.as_ref())
        .await
        .transpose()
        .ok_or_else(|| DbErr::RecordNotFound(String::new()))
        .and_then(std::convert::identity)
        .map(Json)
        .map_err(to_internal_error)
}

// DELETE /post/{post_id}
// Takes in token
// On success, deletes and returns 200 OK
// If post_id does not exist, returns 404 Not Found
pub async fn delete(
    param: Path<i64>,
    db: Data<DatabaseConnection>,
    token: token::Model,
) -> Result<HttpResponse, InternalError<DbErr>> {
    let post_id = param.into_inner();

    let post = post::Entity::find_by_id(post_id)
        .one(db.as_ref())
        .await
        .transpose()
        .ok_or_else(|| DbErr::RecordNotFound(String::new()))
        .and_then(std::convert::identity)
        .map_err(to_not_found)?;

    if post.user_id != token.user_id {
        return Err(InternalError::new(
            DbErr::Custom("not real author".to_string()),
            StatusCode::UNAUTHORIZED,
        ));
    }

    post.into_active_model()
        .delete(db.as_ref())
        .await
        .map(to_ok)
        .map_err(to_internal_error)
}
