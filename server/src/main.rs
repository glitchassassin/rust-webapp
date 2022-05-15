use std::{collections::HashMap, convert::Infallible, sync::Arc};

use chat_server::ChatServer;
use tokio::sync::Mutex;
use warp::Filter;

mod ws;
mod chat_server;

#[tokio::main]
async fn main() {
  let server = ChatServer {
    clients: Arc::new(Mutex::new(HashMap::new()))
  };

  let ws_route = warp::path("ws")
    .and(warp::ws())
    .and(with_chat_server(server.clone()))
    .and_then(ws::ws_handler);

  let routes = ws_route
    .with(warp::cors().allow_any_origin());

  warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_chat_server(server: ChatServer) -> impl Filter<Extract = (ChatServer,), Error = Infallible> + Clone {
  warp::any().map(move || server.clone())
}
