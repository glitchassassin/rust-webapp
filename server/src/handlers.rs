use warp::{Reply, hyper::StatusCode, ws::Ws, Rejection};

use crate::{Clients, ws};

type Result<T> = std::result::Result<T, Rejection>;

pub async fn ws_handler(ws: Ws, clients: Clients) -> Result<impl Reply> {
  Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, clients)))
}

pub async fn health_handler() -> Result<impl Reply> {
  Ok(StatusCode::OK)
}