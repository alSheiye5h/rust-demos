use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

type SharedDb = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;

fn new_shared_db(num_shards: usize) -> SharedDb {
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        let db = db .clone();

        println!("Accepted");
        tokio::spawn(async move {
            process(socket, db).await
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
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