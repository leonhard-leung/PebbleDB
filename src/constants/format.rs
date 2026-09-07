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
pub const RECORD_MAGIC_NUMBER: &[u8; 4] = b"REC1";
pub const RECORD_EMPTY_MAGIC_NUMBER: &[u8; 4] = b"REC0";

// =================================================================================================
// Sizes
// =================================================================================================

// File Header
pub const FILE_HEADER_SIZE: usize =
    FILE_MAGIC_NUMBER_SIZE + FILE_VERSION_SIZE;
pub const FILE_MAGIC_NUMBER_SIZE: usize = FILE_MAGIC_NUMBER.len();
pub const FILE_VERSION_SIZE: usize = 1;

// Database
pub const DATABASE_HEADER_SIZE: usize =
    DATABASE_MAGIC_NUMBER_SIZE + TABLE_COUNT_SIZE;
pub const DATABASE_MAGIC_NUMBER_SIZE: usize = DATABASE_MAGIC_NUMBER.len();
pub const TABLE_COUNT_SIZE: usize = size_of::<u8>();

// Table
pub const TABLE_BLOCK_SIZE: usize = 8192;
pub const TABLE_HEADER_SIZE: usize =
    TABLE_MAGIC_NUMBER_SIZE + TABLE_NAME_SIZE + COLUMN_COUNT_SIZE + ROW_COUNT_SIZE;
pub const TABLE_MAGIC_NUMBER_SIZE: usize = TABLE_MAGIC_NUMBER.len();
pub const TABLE_NAME_SIZE: usize = 32;
pub const COLUMN_COUNT_SIZE: usize = size_of::<u8>();
pub const ROW_COUNT_SIZE: usize = size_of::<u32>();

// Column
pub const COLUMN_DEFINITION_SIZE: usize =
    COLUMN_NAME_SIZE + COLUMN_DATA_TYPE_SIZE;
pub const COLUMN_NAME_SIZE: usize = 32;
pub const COLUMN_DATA_TYPE_SIZE: usize = size_of::<u8>();

// Record
pub const RECORD_MAGIC_NUMBER_SIZE: usize = RECORD_MAGIC_NUMBER.len();
pub const RECORD_LENGTH_SIZE: usize = size_of::<u32>();

// =================================================================================================
// Pointers
// =================================================================================================

// Database
pub const DATABASE_HEADER_OFFSET: u64 = FILE_HEADER_SIZE as u64;
pub const DATABASE_MAGIC_NUMBER_OFFSET: u64 = DATABASE_HEADER_OFFSET;

// Table
pub const TABLE_BLOCK_START_OFFSET: u64 =
    (FILE_HEADER_SIZE + DATABASE_HEADER_SIZE) as u64;
pub const COLUMN_DEFINITION_OFFSET: u64 =
    TABLE_BLOCK_START_OFFSET + TABLE_HEADER_SIZE as u64;

pub const RECORD_BLOCK_START_OFFSET: u64 =
    COLUMN_DEFINITION_OFFSET + (COLUMN_DEFINITION_SIZE as u64 * 20u64);