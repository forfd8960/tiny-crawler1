use std::collections::HashSet;
use tokio::sync::{Mutex, mpsc, mpsc::error::SendError};

use crate::errors::Errors;

#[derive(Debug, Clone)]
pub struct Link {
    pub url: String,
    pub base: String,
    pub depth: usize,
}

#[derive(Debug)]
pub struct URLQueue {
    sender: mpsc::Sender<Link>,
    receiver: Mutex<mpsc::Receiver<Link>>,
    visited: Mutex<HashSet<String>>,
    max_depth: usize,
}

impl URLQueue {
    pub fn new(max_depth: usize, queue_size: usize) -> Self {
        let (sender, receiver) = mpsc::channel(queue_size);

        Self {
            sender: sender,
            receiver: Mutex::new(receiver),
            visited: Mutex::new(HashSet::with_capacity(queue_size)),
            max_depth: max_depth,
        }
    }

    pub async fn add_url(&self, link: &Link) -> Result<bool, Errors> {
        let mut visited = self.visited.lock().await;
        if visited.contains(&link.url) {
            return Ok(false);
        }

        visited.insert(link.url.clone());
        drop(visited);

        self.sender.send(link.clone()).await?;
        Ok(true)
    }

    pub async fn get_next_link(&self) -> Option<Link> {
        let mut receiver = self.receiver.lock().await;
        receiver.recv().await
    }
}
