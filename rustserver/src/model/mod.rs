use sea_orm::{
    entity::prelude::*,
    sea_query::{Index, IndexType, PostgresQueryBuilder},
    ConnectionTrait, FromQueryResult, Schema, Statement,
};
use serde::{Deserialize, Serialize};

pub mod post;
pub mod reply;
pub mod token;
pub mod user;

pub async fn init(db: &DatabaseConnection) {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    let _ = db
        .execute(builder.build(&schema.create_table_from_entity(user::Entity)))
        .await;
    let _ = db
        .execute(builder.build(&schema.create_table_from_entity(token::Entity)))
        .await;
    let _ = db
        .execute(builder.build(&schema.create_table_from_entity(post::Entity)))
        .await;
    let _ = db
        .execute(builder.build(&schema.create_table_from_entity(reply::Entity)))
        .await;

    let stmt = Index::create()
        .name("idx-user-username")
        .table(user::Entity)
        .col(user::Column::Username)
        .index_type(IndexType::BTree)
        .build(PostgresQueryBuilder);
    let _ = db.execute(Statement::from_string(builder, stmt)).await;

    let stmt = Index::create()
        .name("idx-user-password")
        .table(user::Entity)
        .col(user::Column::Password)
        .index_type(IndexType::BTree)
        .build(PostgresQueryBuilder);
    let _ = db.execute(Statement::from_string(builder, stmt)).await;

    let stmt = Index::create()
        .name("idx-user-created_at")
        .table(user::Entity)
        .col(user::Column::CreatedAt)
        .index_type(IndexType::BTree)
        .build(PostgresQueryBuilder);
    let _ = db.execute(Statement::from_string(builder, stmt)).await;

    let stmt = Index::create()
        .name("idx-token-user_id")
        .table(token::Entity)
        .col(token::Column::UserId)
        .index_type(IndexType::BTree)
        .build(PostgresQueryBuilder);
    let _ = db.execute(Statement::from_string(builder, stmt)).await;

    let stmt = Index::create()
        .name("idx-reply-post_id")
        .table(reply::Entity)
        .col(reply::Column::PostId)
        .index_type(IndexType::BTree)
        .build(PostgresQueryBuilder);
    let _ = db.execute(Statement::from_string(builder, stmt)).await;

    let stmt = Index::create()
        .name("idx-reply-user_id")
        .table(reply::Entity)
        .col(reply::Column::UserId)
        .index_type(IndexType::BTree)
        .build(PostgresQueryBuilder);
    let _ = db.execute(Statement::from_string(builder, stmt)).await;

    let stmt = Index::create()
        .name("idx-reply-text")
        .table(reply::Entity)
        .col(reply::Column::Text)
        .index_type(IndexType::FullText)
        .build(PostgresQueryBuilder);
    let _ = db.execute(Statement::from_string(builder, stmt)).await;

    let stmt = Index::create()
        .name("idx-reply-created_at")
        .table(reply::Entity)
        .col(reply::Column::CreatedAt)
        .index_type(IndexType::BTree)
        .build(PostgresQueryBuilder);
    let _ = db.execute(Statement::from_string(builder, stmt)).await;

    let stmt = Index::create()
        .name("idx-post-user_id")
        .table(post::Entity)
        .col(post::Column::UserId)
        .index_type(IndexType::BTree)
        .build(PostgresQueryBuilder);
    let _ = db.execute(Statement::from_string(builder, stmt)).await;

    let stmt = Index::create()
        .name("idx-post-text")
        .table(post::Entity)
        .col(post::Column::Text)
        .index_type(IndexType::FullText)
        .build(PostgresQueryBuilder);
    let _ = db.execute(Statement::from_string(builder, stmt)).await;

    let stmt = Index::create()
        .name("idx-post-created_at")
        .table(post::Entity)
        .col(post::Column::CreatedAt)
        .index_type(IndexType::BTree)
        .build(PostgresQueryBuilder);
    let _ = db.execute(Statement::from_string(builder, stmt)).await;
}
