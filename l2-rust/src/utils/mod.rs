pub mod db;
pub mod error;
pub mod response;
pub mod routes;

// 重导出常用类型，方便使用
pub use response::{ApiResponse, IntoApiResponse};