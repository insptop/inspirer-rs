use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "inspirer_blog_users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub nickname: String,
    pub user_type: u16,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}