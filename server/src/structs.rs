use tokio::sync::mpsc;

use warp::ws::Message;

#[derive(Clone)]
pub struct Client {
    pub user_id: String,
    pub nick: String,
    pub channel: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

#[derive(serde::Serialize)]
pub struct RegisterResponse {
    pub url: String,
}

#[derive(serde::Deserialize)]
pub struct Event {
    pub channel: String,
    pub nick: Option<String>,
    pub message: String,
}