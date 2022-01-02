use inspirer_core::Json;

use crate::Result;
use crate::app::model::response::content::ContentList;

pub async fn get_content_list() -> Result<Json<ContentList>> {
    Ok(Json(ContentList::default()))
}