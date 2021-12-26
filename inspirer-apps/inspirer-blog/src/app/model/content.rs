use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "inspirer_blog_contents")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub name: Option<String>,
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub user_id: u64,
    pub status: u16,
    pub published_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::content_body::Entity")]
    Body
}

impl Related<super::content_body::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Body.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}