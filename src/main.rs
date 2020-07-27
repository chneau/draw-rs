use futures::{FutureExt, StreamExt};
use warp::ws::WebSocket;
use warp::Filter;

struct Manager {}

impl Manager {
    async fn handle_ws(self, ws: WebSocket) {
        let (tx, mut rx) = ws.split();
        // let fut = rx.forward(tx).map(|_| ());
        loop {
            let msg = match rx.next().await {
                Some(x) => match x {
                    Ok(x) => match x.to_str() {
                        Ok(x) => x.to_owned(),
                        Err(_) => return,
                    },
                    Err(_) => return,
                },
                None => return,
            };
            println!("{:?}", msg)
        }
    }
}
async fn handle_ws(ws: WebSocket) {
    let (tx, mut rx) = ws.split();
    // let fut = rx.forward(tx).map(|_| ());
    loop {
        let msg = match rx.next().await {
            Some(x) => match x {
                Ok(x) => match x.to_str() {
                    Ok(x) => x.to_owned(),
                    Err(_) => return,
                },
                Err(_) => return,
            },
            None => return,
        };
        println!("{:?}", msg)
    }
}

#[tokio::main]
async fn main() {
    let manager = Manager {};
    let home = warp::path("static").and(warp::fs::dir("static"));
    let wsconn = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(|websocket| handle_ws(websocket)));
    warp::serve(wsconn.or(home))
        .run(([127, 0, 0, 1], 8000))
        .await;
}
