use std::collections::{ HashMap };
use crate::algess::structs::{ Data2D, DataTable };

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

    pub fn delete_channel(&mut self, name: String) {
        self.channel_map.remove(&name);
    }

    pub fn get_channel(&self, name: String) -> Option<&Channel> {
        match self.channel_map.get(&name) {
            Some(ch) => Some(ch),
            None => None
        }
    }

    pub fn get_channels(&self) -> Vec<&Channel> {
        self.channel_map.values().collect()
    }
}

pub struct Channel {
    pub name: String,
    pub table: Vec<DataTable>,
}
impl Channel {
    fn from(name: String) -> Self {
        Self {
            name: name.to_string(),
            table: Vec::new(),
        }
    }
}