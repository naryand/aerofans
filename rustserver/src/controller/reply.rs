use actix_web::{
    error::InternalError,
    http::StatusCode,
    web::{Data, Json, Path},
    HttpResponse,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter, QuerySelect, RelationTrait, Set,
};

use crate::model::{reply, token, user};

use super::{to_internal_error, to_not_found, to_ok};

// POST /post/{post_id}/reply
// Takes in JSON encoded reply Input and user auth
// On success, returns 200 OK with JSON encoded reply Output
// If post_id does not exist, returns 404 Not Found
pub async fn create(
    Json(input_reply): Json<reply::Input>,
    param: Path<i64>,
    db: Data<DatabaseConnection>,
    token: token::Model,
) -> Result<Json<reply::Output>, InternalError<DbErr>> {
    let post_id = param.into_inner();

    let input_reply = input_reply.into_active_model();
    let input_reply = reply::ActiveModel {
        user_id: Set(token.user_id),
        post_id: Set(post_id),
        created_at: Set(Utc::now().naive_utc()),
        ..input_reply
    };

    let reply = input_reply
        .insert(db.as_ref())
        .await
        .map_err(to_internal_error)?;

    reply::Entity::find_by_id(reply.id)
        .join(sea_orm::JoinType::InnerJoin, reply::Relation::User.def())
        .column(user::Column::Username)
        .into_model::<reply::Output>()
        .one(db.as_ref())
        .await
        .transpose()
        .ok_or_else(|| DbErr::RecordNotFound(String::new()))
        .and_then(std::convert::identity)
        .map(Json)
        .map_err(to_not_found)
}

// GET /post/{post_id}/reply/all
// On success, returns 200 OK with JSON encoded reply Outputs
// If post_id does not exist, returns 404 Not Found
pub async fn read_all(
    param: Path<i64>,
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<reply::Output>>, InternalError<DbErr>> {
    let post_id = param.into_inner();

    reply::Entity::find()
        .filter(reply::Column::PostId.eq(post_id))
        .join(sea_orm::JoinType::InnerJoin, reply::Relation::User.def())
        .column(user::Column::Username)
        .into_model::<reply::Output>()
        .all(db.as_ref())
        .await
        .map(Json)
        .map_err(to_not_found)
}

// GET /post/{post_id}/reply/{reply_id}
// On success, returns 200 OK with JSON encoded reply Output
// If post_id, reply_id does not exist, returns 404 Not Found
pub async fn read(
    param: Path<(i64, i64)>,
    db: Data<DatabaseConnection>,
) -> Result<Json<reply::Output>, InternalError<DbErr>> {
    let (post_id, reply_id) = param.into_inner();

    reply::Entity::find_by_id(reply_id)
        .filter(reply::Column::PostId.eq(post_id))
        .join(sea_orm::JoinType::InnerJoin, reply::Relation::User.def())
        .column(user::Column::Username)
        .into_model::<reply::Output>()
        .one(db.as_ref())
        .await
        .transpose()
        .ok_or_else(|| DbErr::RecordNotFound(String::new()))
        .and_then(std::convert::identity)
        .map(Json)
        .map_err(to_not_found)
}

// PATCH /post/{post_id}/reply/{reply_id}
// Takes in JSON encoded reply Input and user auth
// On success, updates and returns 200 OK with JSON encoded reply Output
// If post_id, reply_id does not exist, returns 404 Not Found
pub async fn update(
    Json(input_reply): Json<reply::Input>,
    param: Path<(i64, i64)>,
    db: Data<DatabaseConnection>,
    token: token::Model,
) -> Result<Json<reply::Output>, InternalError<DbErr>> {
    let (post_id, reply_id) = param.into_inner();

    let reply = reply::Entity::find_by_id(reply_id)
        .filter(reply::Column::PostId.eq(post_id))
        .one(db.as_ref())
        .await
        .transpose()
        .ok_or_else(|| DbErr::RecordNotFound(String::new()))
        .and_then(std::convert::identity)
        .map_err(to_not_found)?;

    if reply.user_id != token.user_id {
        return Err(InternalError::new(
            DbErr::Custom("not real author".to_string()),
            StatusCode::UNAUTHORIZED,
        ));
    }

    let input_reply = input_reply.into_active_model();
    let input_reply = reply::ActiveModel {
        id: Set(reply_id),
        post_id: Set(post_id),
        user_id: Set(token.user_id),
        ..input_reply
    };

    input_reply
        .save(db.as_ref())
        .await
        .map_err(to_internal_error)?;

    reply::Entity::find_by_id(reply_id)
        .filter(reply::Column::PostId.eq(post_id))
        .join(sea_orm::JoinType::InnerJoin, reply::Relation::User.def())
        .column(user::Column::Username)
        .into_model::<reply::Output>()
        .one(db.as_ref())
        .await
        .transpose()
        .ok_or_else(|| DbErr::RecordNotFound(String::new()))
        .and_then(std::convert::identity)
        .map_err(to_internal_error)
        .map(Json)
}

// DELETE /post/{post_id}/reply/{reply_id}
// Takes in user auth
// On success, deletes and returns 200 OK
// If post_id, reply_id does not exist, returns 404 Not Found
pub async fn delete(
    param: Path<(i64, i64)>,
    db: Data<DatabaseConnection>,
    token: token::Model,
) -> Result<HttpResponse, InternalError<DbErr>> {
    let (post_id, reply_id) = param.into_inner();

    let reply = reply::Entity::find_by_id(reply_id)
        .filter(reply::Column::PostId.eq(post_id))
        .one(db.as_ref())
        .await
        .transpose()
        .ok_or_else(|| DbErr::RecordNotFound(String::new()))
        .and_then(std::convert::identity)
        .map_err(to_not_found)?;

    if reply.user_id != token.user_id {
        return Err(InternalError::new(
            DbErr::Custom("not real author".to_string()),
            StatusCode::UNAUTHORIZED,
        ));
    }

    reply
        .into_active_model()
        .delete(db.as_ref())
        .await
        .map(to_ok)
        .map_err(to_internal_error)
}
