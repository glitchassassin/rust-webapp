use std::{collections::HashMap, convert::Infallible,};
use tokio::sync::{Arc, Mutex};

use structs::Client;
use warp::{Rejection, Filter};

mod handlers;
mod ws;

mod structs;

type Result<T> = std::result::Result<T, Rejection>;
type Clients = Arc<Mutex<HashMap<String, Client>>>;

#[tokio::main]
async fn main() {
  let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

  let health_route = warp::path!("health").and_then(handlers::health_handler);

  let register = warp::path("register");

  let register_routes = register
    .and(warp::post())
    .and(warp::body::json())
    .and(with_clients(clients.clone()))
    .and_then(handlers::register_handler)
    .or(register
      .and(warp::delete())
      .and(warp::path::param())
      .and(with_clients(clients.clone()))
      .and_then(handlers::unregister_handler));

  let publish = warp::path!("publish")
    .and(warp::body::json())
    .and(with_clients(clients.clone()))
    .and_then(handlers::publish_handler);

  let ws_route = warp::path("ws")
    .and(warp::ws())
    .and(warp::path::param())
    .and(with_clients(clients.clone()))
    .and_then(handlers::ws_handler);

  let routes = health_route
    .or(register_routes)
    .or(ws_route)
    .or(publish)
    .with(warp::cors().allow_any_origin());

  warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
  warp::any().map(move || clients.clone())
}
