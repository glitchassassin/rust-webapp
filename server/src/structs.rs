use tokio::sync::mpsc;

use warp::ws::Message;

pub struct Client {
    pub user_id: usize,
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

pub struct RegisterRequest {
    pub user_id: usize,
}

#[derive(serde::Serialize)]
pub struct RegisterResponse {
    pub url: String,
}

pub struct Event {
    pub topic: String,
    pub user_id: Option<usize>,
    pub message: String,
}

pub struct TopicsRequest {
    pub topics: Vec<String>
}