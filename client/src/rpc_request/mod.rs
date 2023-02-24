pub mod get_path;
pub mod get_root;
pub mod send_proof;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
