use std::collections::{HashMap, hash_map::Keys};
use crate::algess::structs::{ Data2D };

use std::sync::{ Mutex, OnceLock };

static CHANNEL_MANAGER: OnceLock<Mutex<ChannelManager>> = OnceLock::new();
pub fn get_channel_manager() -> &'static Mutex<ChannelManager> {
    CHANNEL_MANAGER.get_or_init(|| Mutex::new(ChannelManager::new()))
}

pub struct ChannelManager {
    channel_map: HashMap<String, Channel>
}
impl ChannelManager {
    fn new() -> Self {
        ChannelManager { channel_map: HashMap::new() }
    }

    pub fn create_channel(&mut self, name: String) {
        let channel = Channel::from(name.clone());
        self.channel_map.entry(name).or_insert(channel);
    }

    pub fn get_channels(&self) -> Vec<String> {
        self.channel_map.keys().cloned().collect()
    }
}

pub struct Channel {
    pub name: String,
    pub table: Vec<Data2D>,
}
impl Channel {
    fn from(name: String) -> Self {
        Self {
            name: name.to_string(),
            table: Vec::new(),
        }
    }
}