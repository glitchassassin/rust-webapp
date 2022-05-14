use std::{collections::HashMap, convert::Infallible, sync::Arc};

use structs::Client;
use tokio::sync::Mutex;
use warp::Filter;

mod handlers;
mod ws;
mod structs;
mod commands;

type Clients = Arc<Mutex<HashMap<String, Client>>>;

#[tokio::main]
async fn main() {
  let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

  let health_route = warp::path!("health").and_then(handlers::health_handler);

  let ws_route = warp::path("ws")
    .and(warp::ws())
    .and(with_clients(clients.clone()))
    .and_then(handlers::ws_handler);

  let routes = health_route
    .or(ws_route)
    .with(warp::cors().allow_any_origin());

  warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
  warp::any().map(move || clients.clone())
}
