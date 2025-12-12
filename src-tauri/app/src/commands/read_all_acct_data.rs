use crate::commands::CommandError;
use crate::entities::acct_data;
use crate::factory::create_db_connection;
use migration::Condition;
use sea_orm::{
    ColumnTrait, DerivePartialModel, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder,
};
use serde::Serialize;

#[tauri::command]
pub async fn read_all_acct_data(
    app: tauri::AppHandle,
    search_term: String,
    page_index: u64,
    page_size: u64,
) -> Result<Vec<ReadAllAcctDataResult>, CommandError> {
    let db = create_db_connection(&app).await?;
    let acct_data_to_read_query = acct_data::Entity::find()
        .filter(
            Condition::any()
                .add(acct_data::Column::Platform.contains(&search_term))
                .add(acct_data::Column::UserName.contains(&search_term)),
        )
        .order_by(acct_data::Column::Id, Order::Asc)
        .into_partial_model()
        .paginate(&db, page_size);
    let acct_data_to_read = acct_data_to_read_query.fetch_page(page_index).await?;
    Ok(acct_data_to_read)
}

#[derive(DerivePartialModel, Serialize)]
#[sea_orm(entity = "acct_data::Entity")]
pub struct ReadAllAcctDataResult {
    pub id: i32,
    pub user_name: String,
    pub platform: String,
}
