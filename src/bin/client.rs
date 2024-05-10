use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

enum Command {
    Get {
        key: String,
        responder: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        responder: Responder<()>,
    },
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, responder } => {
                    let _ = responder.send(client.get(&key).await);
                }
                Command::Set { key, val, responder } => {
                    let _ = responder.send(client.set(&key, val).await);
                }
            }
        }
    });

    let tx2 = tx.clone();
    let t1 = tokio::spawn(async move {
        let (responder, rx) = oneshot::channel();

        let cmd = Command::Get {
            key: "foo".to_string(),
            responder,
        };

        tx.send(cmd).await.unwrap();

        let value = rx.await.unwrap();
        println!("Got after Get = {:?}", value);
    });

    let t2 = tokio::spawn(async move {
        let (responder, rx) = oneshot::channel();

        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            responder,
        };

        tx2.send(cmd).await.unwrap();

        let res = rx.await.unwrap();
        println!("Got after Set = {:?}", res);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
