pub mod get_hash_params;
pub mod get_root;
pub mod get_tree;
pub mod send_proof;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
