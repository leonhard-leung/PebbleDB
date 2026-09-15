use axum::{
    Json
    ,
    Router,
    extract::Path,
    http::StatusCode,
    routing::{get, post, put, delete}};

use crate::database::api;
use crate::server::dto::{CreateDatabaseRequest, CreateTableRequest, DatabaseListResponse, InsertRecordRequest, MessageResponse, RecordDetailsResponse, TableDetailsResponse, TableListResponse, UpdateRecordRequest};

pub fn router() -> Router {
    Router::new()
        // database
        .route("/databases", get(list_databases))
        .route("/databases", post(create_database))
        .route("/databases/{database}", delete(drop_database))

        // table
        .route("/databases/{database}/tables", get(list_tables))
        .route("/databases/{database}/tables", post(create_table))
        .route("/databases/{database}/tables/{table}", get(describe_table))
        .route("/databases/{database}/tables/{table}", delete(drop_table))

        // record
        .route("/databases/{database}/tables/{table}/records", post(insert_record))
        .route("/databases/{database}/tables/{table}/records/{id}", get(select_record))
        .route("/databases/{database}/tables/{table}/records/{id}", put(update_record))
        .route("/databases/{database}/tables/{table}/records/{id}", delete(delete_record))
}


// =================================================================================================
// Database
// =================================================================================================
async fn list_databases() -> Result<Json<DatabaseListResponse>, StatusCode> {
    let list = api::list_databases()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(DatabaseListResponse { list }))
}

async fn create_database(
    Json(request): Json<CreateDatabaseRequest>,
) -> Result<Json<MessageResponse>, StatusCode> {
    api::create_database(&request.db_name)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(MessageResponse { message: format!("Database {} created successfully", request.db_name) }))
}

async fn drop_database(
    Path(db_name): Path<String>,
) -> Result<Json<MessageResponse>, StatusCode> {
    api::drop_database(&db_name)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(MessageResponse { message: format!("Database {} dropped successfully", db_name) }))
}

// =================================================================================================
// Table
// =================================================================================================
async fn list_tables(
    Path(db_name): Path<String>,
) -> Result<Json<TableListResponse>, StatusCode> {
    let tables = api::list_tables(&db_name)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(TableListResponse { list: tables }))
}

async fn describe_table(
    Path((db_name, table_name)): Path<(String, String)>,
) -> Result<Json<TableDetailsResponse>, StatusCode> {
    let metadata = api::describe_table(&table_name, &db_name)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let table_name = metadata[0].to_owned();

    let column_count = metadata[1].parse::<u8>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let row_count = metadata[2].parse::<u32>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let columns: Vec<(String, String)> = metadata[3..]
        .chunks(2)
        .map(|chunk| (chunk[0].to_owned(), chunk[1].to_owned()))
        .collect();

    Ok(Json(TableDetailsResponse {
        table_name,
        column_count,
        row_count,
        columns
    }))
}


async fn create_table(
    Path(db_name): Path<String>,
    Json(request): Json<CreateTableRequest>,
) -> Result<Json<MessageResponse>, StatusCode> {
    let mut data = Vec::new();

    let table_name = request.table_name;
    let columns = request.columns;

    data.push(table_name.clone());
    columns.iter().for_each(| (name, data_type) | {
        data.push(name.clone());
        data.push(data_type.clone());
    });

    api::create_table(data, &db_name)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(MessageResponse { message: format!("Table {} created successfully", table_name) }))
}

async fn drop_table(
    Path((db_name, table_name)): Path<(String, String)>,
) -> Result<Json<MessageResponse>, StatusCode> {
    api::drop_table(&table_name, &db_name)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(MessageResponse { message: format!("Table {} dropped successfully", table_name) }))
}

// =================================================================================================
// Record
// =================================================================================================
async fn insert_record(
    Path((db_name, table_name)): Path<(String, String)>,
    Json(request): Json<InsertRecordRequest>
) -> Result<Json<MessageResponse>, StatusCode> {
    api::insert_record(request.data, &table_name, &db_name)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(MessageResponse { message: "Record added successfully".to_string() }))
}

async fn select_record(
    Path((db_name, table_name, id)): Path<(String, String, u32)>,
) -> Result<Json<RecordDetailsResponse>, StatusCode> {
    let record = api::select_record(id, &table_name, &db_name)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(RecordDetailsResponse { record: record.to_pairs() }))
}

async fn update_record(
    Path((db_name, table_name, id)): Path<(String, String, u32)>,
    Json(request): Json<UpdateRecordRequest>
) -> Result<Json<MessageResponse>, StatusCode> {
    api::update_record(request.updated_data, id, &table_name, &db_name)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(MessageResponse { message: "Record updated successfully".to_string() }))
}

async fn delete_record(
    Path((db_name, table_name, id)): Path<(String, String, u32)>,
) -> Result<Json<MessageResponse>, StatusCode> {
    api::delete_record(&id, &table_name, &db_name)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(MessageResponse { message: "Record deleted successfully".to_string() }))
}