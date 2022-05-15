
use std::sync::Arc;

use futures::{StreamExt, FutureExt};
use tokio::sync::{mpsc, Mutex};
use warp::ws::WebSocket;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{chat_server::ChatServer};

use warp::{Reply, ws::Ws, Rejection};

type Result<T> = std::result::Result<T, Rejection>;

pub async fn ws_handler(ws: Ws, server: Arc<Mutex<ChatServer>>) -> Result<impl Reply> {
  Ok(ws.on_upgrade(move |socket| client_connection(socket, server)))
}

pub async fn client_connection(ws: WebSocket, server: Arc<Mutex<ChatServer>>) {
  let (client_ws_sender, mut client_ws_rcv) = ws.split();
  let (client_sender, client_rcv) = mpsc::unbounded_channel();
  let client_rcv = UnboundedReceiverStream::new(client_rcv);

  tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
    if let Err(e) = result { 
      eprintln!("error sending websocket msg: {}", e);
    }
  }));

  let id = server.lock().await.add_client(client_sender).await;

  println!("{} connected", id);

  while let Some(result) = client_ws_rcv.next().await {
    let msg = match result {
      Ok(msg) => msg,
      Err(e) => {
        eprintln!("error receiving ws message for id: {} ({})", id.clone(), e);
        break;
      }
    };
    server.lock().await.handle_message(id.clone(), msg).await;
  }

  server.lock().await.remove_client(id.clone()).await;

  println!("{} disconnected", id);
}



