use futures_util::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::{mpsc};
use warp::ws::{Message, WebSocket};
use warp::Filter;

use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

#[tokio::main]
async fn main() {
    // GET /chat -> websocket upgrade
    let chat = warp::path("chat")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| user_connected(socket))
        });

    // GET / -> index html  
    let index = warp::path::end().map(|| warp::reply::html(INDEX_HTML));

    let routes = index.or(chat);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn user_connected(ws: WebSocket) {
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
    let mut child = Command::new("tail")
        .arg("-f")
        .arg("/Users/fuhui/file.txt")
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
        println!("read file line {}", msg.clone());
        match result {
            Ok(msg) => {
            }
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
        tokio::task::spawn(async move {
            start(tx).await
        });    
    }

    // New message from this user, send it to everyone else (except same uid)...
    // loop {
    //     println!("start loop");
    //     match rx.lock().await.recv().await {
    //         Some(msg) => {
    //             if let Err(_disconnected) = tx.send(Message::text(msg.clone())) {
    //                 // The tx is disconnected, our `user_disconnected` code
    //                 // should be happening in another task, nothing more to
    //                 // do here.
    //             }
    //         }
    //         None => {
    //             break // 通道关闭
    //         }
    //     }
    //     println!("end loop");
    // }
}

static INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <title>Warp Chat</title>
    </head>
    <body>
        <h1>Warp chat</h1>
        <div id="chat">
            <p><em>Connecting...</em></p>
        </div>
        <input type="text" id="text" />
        <button type="button" id="send">Send</button>
        <script type="text/javascript">
        const chat = document.getElementById('chat');
        const text = document.getElementById('text');
        const uri = 'ws://' + location.host + '/chat';
        const ws = new WebSocket(uri);

        function message(data) {
            const line = document.createElement('p');
            line.innerText = data;
            chat.appendChild(line);
        }

        ws.onopen = function() {
            chat.innerHTML = '<p><em>Connected!</em></p>';
        };

        ws.onmessage = function(msg) {
            message(msg.data);
        };

        ws.onclose = function() {
            chat.getElementsByTagName('em')[0].innerText = 'Disconnected!';
        };

        send.onclick = function() {
            const msg = text.value;
            ws.send(msg);
            text.value = '';

            message('<You>: ' + msg);
        };
        </script>
    </body>
</html>
"#;