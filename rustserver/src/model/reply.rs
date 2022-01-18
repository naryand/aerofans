use super::*;

#[derive(Debug, Clone, Deserialize, DeriveIntoActiveModel)]
pub struct Input {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, FromQueryResult)]
pub struct Output {
    pub id: i64,
    pub post_id: i64,
    pub user_id: i64,
    pub username: String,
    pub text: String,
    pub created_at: DateTime,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "replies")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub post_id: i64,
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
    #[sea_orm(
        belongs_to = "super::post::Entity",
        from = "Column::PostId",
        to = "super::post::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Post,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
