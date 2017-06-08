pub mod innerptr;
pub mod empty;
pub mod branch_bitmap;
pub mod branch_linear;
pub mod branch_uncompressed;
pub mod leaf_bitmap;
pub mod leaf_linear;
pub mod jpm_root;
pub mod traits;

pub use self::jpm_root::Jpm;
