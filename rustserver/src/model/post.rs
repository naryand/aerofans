use super::*;

#[derive(Debug, Clone, Deserialize, DeriveIntoActiveModel)]
pub struct Input {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, FromQueryResult)]
pub struct Output {
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub text: String,
    pub created_at: DateTime,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    pub text: String,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
    #[sea_orm(has_many = "super::reply::Entity")]
    Reply,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
impl Related<super::reply::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Reply.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
