use super::*;

#[derive(Debug, Clone, Serialize)]
pub struct LoginResponse {
    pub status: bool,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, DeriveIntoActiveModel)]
pub struct Input {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(column_type = "Char(Some(60))")]
    pub password: String,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::token::Entity")]
    Token,
    #[sea_orm(has_many = "super::post::Entity")]
    Post,
    #[sea_orm(has_many = "super::reply::Entity")]
    Reply,
}

impl Related<super::token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Token.def()
    }
}
impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}
impl Related<super::reply::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Reply.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
