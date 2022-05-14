
use futures::{StreamExt, FutureExt};
use tokio::sync::mpsc;
use uuid::Uuid;
use warp::ws::{WebSocket, Message};
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{Clients, structs::Client, commands};

pub async fn client_connection(ws: WebSocket, clients: Clients) {
  let (client_ws_sender, mut client_ws_rcv) = ws.split();
  let (client_sender, client_rcv) = mpsc::unbounded_channel();
  let client_rcv = UnboundedReceiverStream::new(client_rcv);

  tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
    if let Err(e) = result { 
      eprintln!("error sending websocket msg: {}", e);
    }
  }));

  let uuid = Uuid::new_v4().simple().to_string();
  let nick = format!("user{}", &uuid[0..5]);
  let client = Client {
    user_id: uuid.clone(),
    channel: String::from("general"),
    sender: Some(client_sender),
    nick
  };
  clients.lock().await.insert(
    uuid.clone(),
    client.clone()
  );

  println!("{} connected", &uuid);

  commands::send_system_message(client.nick, String::from("Welcome to the server"), &clients).await;

  while let Some(result) = client_ws_rcv.next().await {
    let msg = match result {
      Ok(msg) => msg,
      Err(e) => {
        eprintln!("error receiving ws message for id: {} ({})", uuid.clone(), e);
        break;
      }
    };
    client_msg(&uuid, msg, &clients).await;
  }

  clients.lock().await.remove(&uuid);
  println!("{} disconnected", uuid)
}

async fn client_msg(id: &str, msg: Message, clients: &Clients) {
  println!("received message from {}: {:?}", id, msg);
  let message = match msg.to_str() {
    Ok(v) => v,
    Err(_) => return,
  };

  if message.starts_with('/') {
    let (command, params) = if let Some((c, p)) = message.split_once(' ') {
      (c, Some(p))
    } else {
      (message, None)
    };

    if let Some(client) = clients.lock().await.get_mut(id) {
      match command {
        "/nick" => if let Some(nick) = params {
            commands::set_nick(client, nick);
          } else {
            commands::send_system_message(
              id.to_string(), 
              format!("Current nick: '{}'", client.nick), 
              clients
            ).await;
          },
        "/join" => if let Some(channel) = params {
            commands::join(client, channel);
          } else {
            commands::send_system_message(
              id.to_string(), 
              format!("Current channel: '{}'", client.channel), 
              clients
            ).await;
          },
        "/ping" => (),
        "/users" => commands::users(client, clients).await,
        "/list" => commands::list(client, clients).await,
        _ => commands::send_system_message(
          id.to_string(), 
          format!("Unknown command '{}'", command), 
          clients
        ).await,
      }
    }
  } else {
    // Publish message
  }
}

