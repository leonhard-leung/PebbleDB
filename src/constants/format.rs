/// FILE
pub const FILE_EXTENSION: &str = "pdb";

/// FILE HEADER
pub const FILE_HEADER_SIZE: usize = 64;
pub const FILE_MAGIC_SIZE: usize = FILE_MAGIC_NUMBER.len();
pub const FILE_VERSION_SIZE: usize = 1;
pub const FILE_MAGIC_NUMBER: &[u8; 4] = b"PBDB";

pub const FILE_FORMAT_VERSION: u8 = 1;

/// DATABASE
pub const DATABASE_HEADER_SIZE: usize = 512;
pub const TABLE_COUNT_SIZE: usize = std::mem::size_of::<u32>();

/// TABLE
pub const TABLE_BLOCK_SIZE: usize = 4096;
pub const TABLE_HEADER_SIZE: usize = 128;
pub const TABLE_MAGIC_SIZE: usize = TABLE_MAGIC_NUMBER.len();
pub const TABLE_NAME_SIZE: usize = 16;
pub const COLUMN_COUNT_SIZE: usize = std::mem::size_of::<u32>();
pub const ROW_COUNT_SIZE: usize = std::mem::size_of::<u32>();
pub const FIRST_FREE_ROW_SIZE: usize = std::mem::size_of::<u32>();
pub const TABLE_MAGIC_NUMBER: &[u8; 4] = b"TBL1";

/// COLUMN
pub const COLUMN_DEFINITION_SIZE: usize = COLUMN_NAME_SIZE + COLUMN_DATA_TYPE_SIZE;

pub const COLUMN_NAME_SIZE: usize = 16;
pub const COLUMN_DATA_TYPE_SIZE: usize = std::mem::size_of::<u32>();


