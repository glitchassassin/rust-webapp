use warp::ws::Message;
use itertools::Itertools;

use crate::{structs::{Client, Event}, Clients};

pub fn set_nick(client: &mut Client, nick: &str) {
  client.nick = nick.to_string();
}

/// List channels
pub async fn list(client: &Client, clients: &Clients) {
  let channels_list = clients.lock().await.iter()
    .map(|(_, c)| format!("{}\n", c.channel))
    .unique()
    .collect();
  send_system_message(client.user_id.clone(), channels_list, clients).await;
}

/// Join (or create) a channel
pub fn join(client: &mut Client, channel: &str) {
  client.channel = channel.to_string();
}

/// List users in the current channel
pub async fn users(client: &Client, clients: &Clients) {
  let users_list = clients.lock().await.iter()
    .filter(|(_, c)| c.channel == client.channel)
    .map(|(_, c)| format!("{}\n", c.nick))
    .collect();
  send_system_message(client.user_id.clone(), users_list, clients).await;
}

pub async fn send_system_message(channel: String, message: String, clients: &Clients) {
  publish_message(Event {
    channel: channel.to_string(),
    nick: Some("System".to_string()),
    message: message.to_string()
  }, clients).await;
}

pub async fn publish_message(body: Event, clients: &Clients) {
  clients
    .lock()
    .await
    .iter_mut()
    .filter(|(_, client)| client.channel == body.channel || client.user_id == body.channel)
    .for_each(|(_, client)| {
      if let Some(sender) = &client.sender {
        let _ = sender.send(Ok(Message::text(body.message.clone())));
      }
    });
}