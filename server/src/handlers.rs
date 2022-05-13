use uuid::Uuid;
use warp::{Reply, reply::json, hyper::StatusCode, ws::{Ws, WebSocket}};

use crate::{structs::{RegisterRequest, RegisterResponse, Client}, Clients};

pub async fn register_handler(body: RegisterRequest, clients: Clients) -> impl Reply {
  let user_id = body.user_id;
  let uuid = Uuid::new_v4().simple().to_string();

  register_client(uuid.clone(), user_id, clients).await;

  json(&RegisterResponse {
    url: format!("ws://127.0.0.1:8000/ws/{}", uuid),
  })
}

async fn register_client(id: String, user_id: usize, clients: Clients) {
  clients.lock().await.insert(
    id,
    Client {
      user_id,
      topics: vec![String::from("general")],
      sender: None,
    }
  );
}

pub async fn unregister_handler(id: String, clients: Clients) -> impl Reply {
  clients.lock().await.remove(&id);
  StatusCode::OK
}

pub async fn ws_handler(ws: Ws, id: String, clients: Clients) -> Result<impl Reply, warp::reject::Rejection> {
  let client = clients.lock().await.get(&id).cloned();
  match client {
    Some(c) => Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, id, clients, c))),
    None => Err(warp::reject::not_found()),
  }
}

