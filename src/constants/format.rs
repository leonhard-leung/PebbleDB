/// FILE
pub const FILE_EXTENSION: &str = "peb";
pub const FILE_FORMAT_VERSION: u8 = 1;

// =================================================================================================
// Magic Number
// =================================================================================================
pub const FILE_MAGIC_NUMBER: &[u8; 4] = b"PBDB";
pub const DATABASE_MAGIC_NUMBER: &[u8; 4] = b"DBMG";
pub const TABLE_MAGIC_NUMBER: &[u8; 4] = b"TBL1";
pub const TABLE_EMPTY_MAGIC_NUMBER: &[u8; 4] = b"TBL0";

// =================================================================================================
// Sizes
// =================================================================================================

// File Header
pub const FILE_HEADER_SIZE: usize = 64;
pub const FILE_MAGIC_NUMBER_SIZE: usize = FILE_MAGIC_NUMBER.len();
pub const FILE_VERSION_SIZE: usize = 1;

// Database
pub const DATABASE_HEADER_SIZE: usize = 512;
pub const TABLE_COUNT_SIZE: usize = size_of::<u32>();

// Table
pub const TABLE_BLOCK_SIZE: usize = 4096;
pub const TABLE_HEADER_SIZE: usize = 128;
pub const TABLE_MAGIC_NUMBER_SIZE: usize = TABLE_MAGIC_NUMBER.len();
pub const TABLE_NAME_SIZE: usize = 16;
pub const COLUMN_COUNT_SIZE: usize = size_of::<u32>();
pub const ROW_COUNT_SIZE: usize = size_of::<u32>();

// Column
pub const COLUMN_DEFINITION_SIZE: usize = COLUMN_NAME_SIZE + COLUMN_DATA_TYPE_SIZE;
pub const COLUMN_NAME_SIZE: usize = 16;
pub const COLUMN_DATA_TYPE_SIZE: usize = size_of::<u32>();

// =================================================================================================
// Pointers
// =================================================================================================
pub const DATABASE_HEADER_OFFSET: u64 = FILE_HEADER_SIZE as u64;
pub const TABLE_BLOCK_START_OFFSET: u64 = (FILE_HEADER_SIZE + DATABASE_HEADER_SIZE) as u64;
pub const COLUMN_DEFINITION_OFFSET: u64 = TABLE_BLOCK_START_OFFSET + TABLE_HEADER_SIZE as u64;