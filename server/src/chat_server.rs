use std::{sync::Arc, collections::HashMap};

use tokio::sync::Mutex;
use uuid::Uuid;
use warp::ws::Message;
use itertools::Itertools;

type ClientSender = tokio::sync::mpsc::UnboundedSender<std::result::Result<warp::ws::Message, warp::Error>>;

#[derive(Clone)]
pub struct Client {
    pub user_id: String,
    pub nick: String,
    pub channel: String,
    pub sender: ClientSender,
}

#[derive(Clone)]
pub struct ChatServer {
  pub clients: Arc<Mutex<HashMap<String, Client>>>
}

impl ChatServer {
  pub async fn add_client(&self, sender: ClientSender) -> String {
    let uuid = Uuid::new_v4().simple().to_string();
    let nick = format!("user{}", &uuid[0..5]);
    let client = Client {
      user_id: uuid.clone(),
      channel: String::from("general"),
      sender,
      nick
    };
    self.clients.lock().await.insert(
      uuid.clone(),
      client.clone()
    );

    self.send_system_message(client.nick.clone(), String::from("Welcome to the server")).await;

    uuid
  }

  pub async fn remove_client(&self, id: String) {
    self.clients.lock().await.remove(&id);
  }

  pub async fn handle_message(&self, id: String, msg: Message) {
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
  
      if let Some(client) = self.clients.lock().await.get_mut(&id) {
        match command {
          "/nick" => self.nick(client, params).await,
          "/join" => self.join(client, params).await,
          "/ping" => (),
          "/users" => self.users(client).await,
          "/list" => self.list(client).await,
          _ => self.send_system_message(
            id.to_string(), 
            format!("Unknown command '{}'", command)
          ).await,
        }
      }
    } else {
      // Publish message
    }
  }

  pub async fn nick(&self, client: &mut Client, nick: Option<&str>) {
    if let Some(set_nick) = nick {
      let old_nick = client.nick.clone();
      client.nick = set_nick.to_string();
      self.send_system_message(
        client.channel.to_string(), 
        format!("{} changed nick to {}", old_nick, client.nick)
      ).await;
    } else {
      self.send_system_message(
        client.user_id.to_string(), 
        format!("Current nick: {}", client.nick)
      ).await;
    }
  }
  
  /// List channels
  pub async fn list(&self, client: &Client) {
    let channels_list = self.clients.lock().await.iter()
      .map(|(_, c)| format!("{}\n", c.channel))
      .unique()
      .collect();
    self.send_system_message(client.user_id.clone(), channels_list).await;
  }
  
  /// Join (or create) a channel
  pub async fn join(&self, client: &mut Client, channel: Option<&str>) {
    if let Some(set_channel) = channel {
      if set_channel != client.channel {
        self.send_system_message(
          client.channel.clone(), 
          format!("{} left the channel", client.nick)
        ).await;
        client.channel = set_channel.to_string();
        self.send_system_message(
          client.channel.clone(), 
          format!("{} joined the channel", client.nick)
        ).await;
      } else {
        self.send_system_message(
          client.user_id.clone(), 
          format!("Already in channel '{}'", client.channel)
        ).await;
      }
    } else {
      self.send_system_message(
        client.user_id.clone(), 
        format!("Current channel: '{}'", client.channel)
      ).await;
    }
  }
  
  /// List users in the current channel
  pub async fn users(&self, client: &Client) {
    let users_list = self.clients.lock().await.iter()
      .filter(|(_, c)| c.channel == client.channel)
      .map(|(_, c)| format!("{}\n", c.nick))
      .collect();
    self.send_system_message(client.user_id.clone(), users_list).await;
  }
  
  pub async fn send_system_message(&self, channel: String, message: String) {
    self.publish_message(
      channel.to_string(),
      String::from("System"),
      message.to_string()
    ).await;
  }
  
  pub async fn publish_message(&self, channel: String, message: String, nick: String) {
    self.clients
      .lock()
      .await
      .iter_mut()
      .filter(|(_, client)| client.channel == channel || client.user_id == channel)
      .for_each(|(_, client)| {
        let _ = client.sender.send(Ok(Message::text(format!("{}: {}", nick, message))));
      });
  }
}