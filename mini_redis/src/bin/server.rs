// use bytes::Bytes;
// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};
// use tokio::net::{TcpListener, TcpStream};
// use mini_redis::{Connection, Frame};

// type Db = Arc<Mutex<HashMap<String, Bytes>>>;


// #[tokio::main]
// async fn main() {
//     let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

//     println!("Listening");

//     let db = Arc::new(Mutex::new(HashMap::new()));

//     loop {
//         let (socket, _) = listener.accept().await.unwrap();

//         let db = db .clone();

//         println!("Accepted");
//         tokio::spawn(async move {
//             process(socket, db).await
//         });
//     }
// }

// async fn process(socket: TcpStream, db: Db) {
//     use mini_redis::Command::{self, Get, Set};

//     let mut connection = Connection::new(socket);

//     while let Some(frame) = connection.read_frame().await.unwrap() {
//         let response = match Command::from_frame(frame).unwrap() {
//             Set(cmd) => {
//                 let mut db = db.lock().unwrap();
//                 db.insert(cmd.key().to_string(), cmd.value().clone());
//                 Frame::Simple("OK".to_string())
//             }
//             Get(cmd) => {
//                 let db = db.lock().unwrap();
//                 if let Some(value) = db.get(cmd.key()) {
//                     Frame::Bulk(value.clone())
//                 } else {
//                     Frame::Null
//                 }
//             }
//             cmd => panic!("unimplemented {:?}", cmd),
//         };

//         connection.write_frame(&response).await.unwrap();
//     }
// }
let t1 = tokio::spawn(async move {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = Command::Get {
        key: "foo".to_string(),
        resp: resp_tx,
    };

    // Send the GET request
    tx.send(cmd).await.unwrap();

    // Await the response
    let res = resp_rx.await;
    println!("GOT = {:?}", res);
});

let t2 = tokio::spawn(async move {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = Command::Set {
        key: "foo".to_string(),
        val: "bar".into(),
        resp: resp_tx,
    };

    // Send the SET request
    tx2.send(cmd).await.unwrap();

    // Await the response
    let res = resp_rx.await;
    println!("GOT = {:?}", res);
});

/// Provided by the requester and used by the manager task to send
/// the command response back to the requester.
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

while let Some(cmd) = rx.recv().await {
    match cmd {
        Command::Get { key, resp } => {
            let res = client.get(&key).await;
            // Ignore errors
            let _ = resp.send(res);
        }
        Command::Set { key, val, resp } => {
            let res = client.set(&key, val).await;
            // Ignore errors
            let _ = resp.send(res);
        }
    }
}








































// use tokio::net::{TcpListener, TcpStream};
// use mini_redis::{Connection, Frame};

// #[tokio::main]
// async fn main() {
//     // bindi l ip w lport => yaeni launchi server li ghyaccepti lconnections
//     let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

//     loop {
//         // hna mn listener ghanakhod socket, w dk l element tani fih l ip w lport dial l client
//         let (socket, _) = listener.accept().await.unwrap();
//         process(socket).await;
//     }
// }

// async fn process(socket: TcpStream) {
//     use mini_redis::Command::{self, Get, Set};
//     use std::collections::HashMap;

//     // hna decalarina wahed lhashmap li katstore json data
//     let mut db = HashMap::new();

//     // hna khdina socket w dkhlnah w pipe wla connection jdida
//     let mut connection = Connection::new(socket);

//     while let Some(frame) = connection.read_frame().await.unwrap() {
//         let response =  match Command::from_frame(frame).unwrap() {
//             Set(cmd) => {
//                 db.insert(cmd.key().to_string(), cmd.value().to_vec());
//                 Frame::Simple("OK".to_string()) 
//             }
//             Get(cmd) => {
//                 if let Some(value) = db.get(cmd.key()) {
//                     Frame::Bulk(value.clone().into())
//                 } else {
//                     Frame::Null
//                 }
//             }
//             cmd => panic!("unimplemented {:?}", cmd),
//         };

//         connection.write_frame(&response).await.unwrap();

//     }
// }