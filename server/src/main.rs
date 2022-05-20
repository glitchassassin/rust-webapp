use std::{collections::HashMap, convert::Infallible, sync::Arc};
use warp_reverse_proxy::reverse_proxy_filter;

use chat_server::ChatServer;
use tokio::sync::Mutex;
use warp::Filter;

mod ws;
mod chat_server;

#[tokio::main]
async fn main() {
  let server = Arc::new(Mutex::new(ChatServer {
    clients: HashMap::new()
  }));

  let ws_route = warp::path("ws")
    .and(warp::ws())
    .and(with_chat_server(server.clone()))
    .and_then(ws::ws_handler);

  let default_route = warp::get()
    .and(reverse_proxy_filter("".to_string(), "http://127.0.0.1:8080/".to_string()));

  let routes = ws_route
    .or(default_route)
    .with(warp::cors().allow_any_origin());

  warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_chat_server(server: Arc<Mutex<ChatServer>>) -> impl Filter<Extract = (Arc<Mutex<ChatServer>>,), Error = Infallible> + Clone {
  warp::any().map(move || server.clone())
}
