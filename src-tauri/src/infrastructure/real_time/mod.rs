//! 实时通信领域
//! 
//! 负责处理实时通信：
//! - WebSocket 连接管理
//! - 实时客户端数据
//! - 消息队列处理

pub mod websocket;
pub mod liveclient;

// Re-export services
pub use websocket::service as websocket_service;
pub use websocket::event_handler as websocket_event_handler;
pub use liveclient::service as liveclient_service;
