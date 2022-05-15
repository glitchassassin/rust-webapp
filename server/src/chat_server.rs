use std::collections::HashMap;

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
  pub clients: HashMap<String, Client>
}

impl ChatServer {
  pub async fn add_client(&mut self, sender: ClientSender) -> String {
    let uuid = Uuid::new_v4().simple().to_string();
    let nick = format!("user{}", &uuid[0..5]);
    let client = Client {
      user_id: uuid.clone(),
      channel: String::from("general"),
      sender,
      nick
    };
    self.clients.insert(
      uuid.clone(),
      client
    );

    // self.send_system_message(client.nick.clone(), String::from("Welcome to the server")).await;

    uuid
  }

  pub async fn remove_client(&mut self, id: String) {
    self.clients.remove(&id);
  }

  pub async fn handle_message(&mut self, id: String, msg: Message) {
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
  
      match command {
        "/nick" => {
          if let Some(client) = self.clients.get(&id) { 
            self.nick(client, params).await 
          }
          if let Some(client) = self.clients.get_mut(&id) { 
            if let Some(nick) = params {
              client.nick = nick.to_string();
            }
          }
        },
        "/join" => {
          if let Some(client) = self.clients.get(&id) { 
            self.join(client, params).await
          }
          if let Some(client) = self.clients.get_mut(&id) { 
            if let Some(channel) = params {
              client.channel = channel.to_string();
            }
          }
        },
        "/ping" => (),
        "/users" => if let Some(client) = self.clients.get(&id) { 
          self.users(client).await
        },
        "/list" => if let Some(client) = self.clients.get(&id) { 
          self.list(client).await
        },
        _ => self.send_system_message(
          id.to_string(), 
          format!("Unknown command '{}'", command)
        ).await,
      }
    } else {
      // Publish message
      if let Some(client) = self.clients.get(&id) { 
        self.publish_message(client.channel.clone(), client.nick.clone(), message.to_string()).await;
      }
    }
  }

  pub async fn nick(&self, client: &Client, nick: Option<&str>) {
    if let Some(set_nick) = nick {
      self.send_system_message(
        client.channel.to_string(), 
        format!("{} changed nick to {}", client.nick, set_nick)
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
    let channels_list = self.clients.iter()
      .map(|(_, c)| format!("{}\n", c.channel))
      .unique()
      .collect();
    self.send_system_message(client.user_id.clone(), channels_list).await;
  }
  
  /// Join (or create) a channel
  pub async fn join(&self, client: &Client, channel: Option<&str>) {
    if let Some(set_channel) = channel {
      if set_channel != client.channel {
        self.send_system_message(
          client.channel.clone(), 
          format!("{} left the channel", client.nick)
        ).await;
        self.send_system_message(
          set_channel.to_string(), 
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
    let users_list = self.clients.iter()
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
  
  pub async fn publish_message(&self, channel: String, nick: String, message: String) {
    let formatted_message = format!("{}: {}", nick, message);
    println!("Queuing: [{}]", &formatted_message);

    self.clients
      .iter()
      .filter(|(_, client)| client.channel == channel || client.user_id == channel)
      .for_each(|(_, client)| {
        println!("Sending: [{}]", &formatted_message);
        let _ = client.sender.send(Ok(Message::text(&formatted_message)));
      });
  }
}