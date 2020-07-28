use futures::StreamExt;
use warp::ws::WebSocket;
use warp::Filter;

#[derive(Debug, Clone, Copy)]
struct Manager {} // TODO: add a list of tx

impl Manager {
    async fn handle_ws(self, ws: WebSocket) {
        let (_tx, mut rx) = ws.split();
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
            // TODO: send to list of tx, if it fails, remove tx.
            println!("{:?}", msg)
        }
    }
}

async fn handle_ws(ws: WebSocket) {
    let (tx, mut rx) = ws.split();
    // let fut = rx.forward(tx).map(|_| ());
    loop {
        let msg = match rx.next().await {
            Some(x) => x,
            None => return,
        };
        let msg = if let Ok(x) = msg { x } else { return };
        println!("{:?}", msg)
    }
}

#[tokio::main]
async fn main() {
    let manager = Manager {};
    // let manager = Arc::new(manager);
    let home = warp::path("static").and(warp::fs::dir("static"));
    let wsconn = warp::path("ws").and(warp::ws()).map(|ws: warp::ws::Ws| {
        let res = ws.on_upgrade(|websocket: WebSocket| {
            let res = handle_ws(websocket);
            // let res = manager.handle_ws(websocket); // TODO: can't figure out how to put it here ...
            res
        });
        res
    });
    warp::serve(wsconn.or(home))
        .run(([127, 0, 0, 1], 8000))
        .await;
    /*
    In case this is total crap here is the story:
    A user open the web page.
    websockets connect.
    websocket (TX) is saved on a collection.
    websocket (RX) is listened to and dispatched to others.
    if dispatch fail, remove tx from collection.
    */
}
