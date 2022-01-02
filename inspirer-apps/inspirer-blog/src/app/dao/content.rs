use inspirer_core::dao::DaoProject;
use sea_orm::{ConnectionTrait, EntityTrait, Set};

use crate::app::model::{content, content_body};
use crate::Result;

#[async_trait]
pub trait InspirerBlogContentDao {
    async fn create_content(
        &self,
        title: &str,
        keywords: &str,
        description: &str,
        body: &str,
        user_id: u64,
        status: u16,
        name: Option<&str>,
    ) -> Result<u64>;

    async fn find_content_by_id(
        &self,
        id: u64,
    ) -> Result<Option<(content::Model, Option<content_body::Model>)>>;
}

#[async_trait]
impl<'a, C> InspirerBlogContentDao for DaoProject<'a, C>
where
    C: ConnectionTrait<'a>,
{
    async fn create_content(
        &self,
        title: &str,
        keywords: &str,
        description: &str,
        body: &str,
        user_id: u64,
        status: u16,
        name: Option<&str>,
    ) -> Result<u64> {
        let content = content::ActiveModel {
            title: Set(title.into()),
            keywords: Set(keywords.into()),
            description: Set(description.into()),
            user_id: Set(user_id),
            status: Set(status),
            name: Set(name.map(Into::into)),
            ..Default::default()
        };

        let id = content::Entity::insert(content)
            .exec(self.connection())
            .await?
            .last_insert_id;

        let content_body = content_body::ActiveModel {
            id: Set(id),
            content: Set(body.into()),
        };

        content_body::Entity::insert(content_body)
            .exec(self.connection())
            .await?;

        Ok(id)
    }

    async fn find_content_by_id(
        &self,
        id: u64,
    ) -> Result<Option<(content::Model, Option<content_body::Model>)>> {
        content::Entity::find_by_id(id)
            .find_also_related(content_body::Entity)
            .one(self.connection())
            .await
            .map_err(Into::into)
    }
}
