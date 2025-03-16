use crate::clients::Client;
use crate::Result;

use bytes::Bytes;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::oneshot;

#[derive(Debug)]
enum Command {
    Get(String),
    Set(String,Bytes),
}

type Message = (Command,oneshot::Sender<Result<Option<Bytes>>>);

async fn run(mut client: Client,mut rx: Receiver<Message>) {
    while let Some((cmd,tx)) = rx.recv().await {
        
        
    }
}
