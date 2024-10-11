use futures_util::{SinkExt, StreamExt, TryFutureExt};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};

pub async fn user_connected(ws: WebSocket) {
    // Split the socket into a sender and receive of messages.
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (tx, mut rx) = mpsc::channel::<String>(1);

    tokio::task::spawn(async move {
        while let Some(message) = rx.recv().await {
            user_ws_tx
                .send(Message::text(message))
                .unwrap_or_else(|e| {
                    eprintln!("websocket send error: {}", e);
                })
                .await;
        }
    });

    // Return a `Future` that is basically a state machine managing
    // this specific user's connection.

    // Every time the user sends a message, broadcast it to
    // all other users...
    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                break;
            }
        };
        user_message(msg, tx.clone()).await;
    }
}

async fn start(tx: mpsc::Sender<String>) {
    let mut child = Command::new("sh")
        .arg("-f")
        .arg("scripts/start_hugo.sh")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child process");

    let pid = child.id();
    println!("process id {}", pid);

    let stdout = child.stdout.take().expect("child did not have a stdout");
    let reader = BufReader::new(stdout);

    // 创建一个线程，专门处理输出
    for line in reader.lines() {
        let msg = line.unwrap();
        let result = tx.send(msg.clone()).await;
        match result {
            Ok(msg) => {}
            Err(err) => {
                println!("send error:{}", err)
            }
        }
    }
}

async fn user_message(msg: Message, tx: mpsc::Sender<String>) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    if msg == "start_hugo" {
        tokio::task::spawn(async move { start(tx).await });
    }
}
