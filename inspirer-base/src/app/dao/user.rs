use crate::app::model::user;
use inspirer_core::{application::DaoProject, Result};
use sea_orm::{sea_query::Expr, ConnectionTrait, EntityTrait, QueryFilter, Set, ColumnTrait};

#[async_trait]
pub trait InspirerBaseUserTrait {
    async fn create_user(
        &self,
        username: String,
        password: String,
        email: Option<String>,
        nickname: String,
    ) -> Result<u64>;
    async fn find_user_by_id(&self, id: u64) -> Result<Option<user::Model>>;
    async fn update_password(&self, id: u64, new_password: String) -> Result<bool>;
    async fn update_nickname(&self, id: u64, new_nickname: String) -> Result<bool>;
    async fn update_email(&self, id: u64, new_email: String) -> Result<bool>;
}

#[async_trait]
impl<'a, C> InspirerBaseUserTrait for DaoProject<'a, C>
where
    C: ConnectionTrait<'a>,
{
    async fn create_user(
        &self,
        username: String,
        password: String,
        email: Option<String>,
        nickname: String,
    ) -> Result<u64> {
        let model = user::ActiveModel {
            username: Set(username),
            password: Set(password),
            email: Set(email),
            nickname: Set(nickname),
            ..Default::default()
        };

        user::Entity::insert(model)
            .exec(self.connection())
            .await
            .map(|res| res.last_insert_id)
            .map_err(Into::into)
    }

    async fn find_user_by_id(&self, id: u64) -> Result<Option<user::Model>> {
        user::Entity::find_by_id(id)
            .one(self.connection())
            .await
            .map_err(Into::into)
    }

    async fn update_password(&self, id: u64, new_password: String) -> Result<bool> {
        user::Entity::update_many()
            .col_expr(user::Column::Password, Expr::value(new_password))
            .filter(user::Column::Id.eq(id))
            .exec(self.connection())
            .await
            .map(|res| res.rows_affected > 0)
            .map_err(Into::into)
    }

    async fn update_nickname(&self, id: u64, new_nickname: String) -> Result<bool> {
        user::Entity::update_many()
            .col_expr(user::Column::Nickname, Expr::value(new_nickname))
            .filter(user::Column::Id.eq(id))
            .exec(self.connection())
            .await
            .map(|res| res.rows_affected > 0)
            .map_err(Into::into)
    }

    async fn update_email(&self, id: u64, new_email: String) -> Result<bool> {
        user::Entity::update_many()
            .col_expr(user::Column::Email, Expr::value(new_email))
            .filter(user::Column::Id.eq(id))
            .exec(self.connection())
            .await
            .map(|res| res.rows_affected > 0)
            .map_err(Into::into)
    }
}
