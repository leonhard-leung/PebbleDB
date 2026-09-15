use serde::{Deserialize, Serialize};
// =================================================================================================
// Database
// =================================================================================================

// Requests
#[derive(Deserialize)]
pub struct CreateDatabaseRequest {
    pub db_name: String,
}

#[derive(Deserialize)]
pub struct DropDatabaseRequest {
    pub db_name: String,
}

// Responses
#[derive(Serialize)]
pub struct DatabaseListResponse {
    pub list: Vec<String>,
}

// =================================================================================================
// Table
// =================================================================================================

// Requests
#[derive(Deserialize)]
pub struct CreateTableRequest {
    pub table_name: String,
    pub columns: Vec<(String, String)>
}

// Responses
#[derive(Serialize)]
pub struct TableDetailsResponse {
    pub table_name: String,
    pub column_count: u8,
    pub row_count: u32,
    pub columns: Vec<(String, String)>
}

#[derive(Serialize)]
pub struct TableListResponse {
    pub list: Vec<String>,
}

// =================================================================================================
// Record
// =================================================================================================
// Requests
#[derive(Deserialize)]
pub struct InsertRecordRequest {
    pub data: Vec<String>,
}

#[derive(Deserialize)]
pub struct UpdateRecordRequest {
    pub updated_data: Vec<String>,
}

// Response
#[derive(Serialize)]
pub struct RecordDetailsResponse {
    pub record: Vec<(String, String)>
}

// =================================================================================================
// Shared
// =================================================================================================
#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}