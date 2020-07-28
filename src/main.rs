use futures::prelude::*;
use gotham::hyper::{Body, HeaderMap, Response, StatusCode};
use gotham::state::{request_id, FromState, State};

mod ws;

fn main() {
    pretty_env_logger::init();

    let addr = "127.0.0.1:7878";
    println!("Listening on http://{}/", addr);

    gotham::start(addr, || Ok(handler));
}

fn handler(mut state: State) -> (State, Response<Body>) {
    let body = Body::take_from(&mut state);
    let headers = HeaderMap::take_from(&mut state);
    // let mut all_ws = Mutex::new(vec![]);
    if ws::requested(&headers) {
        let (response, ws) = match ws::accept(&headers, body) {
            Ok(res) => res,
            Err(_) => return (state, bad_request()),
        };

        let req_id = request_id(&state).to_owned();
        let ws = ws
            .map_err(|err| eprintln!("websocket init error: {}", err))
            .and_then(move |ws| connected(req_id, ws));

        tokio::spawn(ws);

        (state, response)
    } else {
        (state, Response::new(Body::from(INDEX_HTML)))
    }
}

async fn connected<S>(req_id: String, stream: S) -> Result<(), ()>
where
    S: Stream<Item = Result<ws::Message, ws::Error>> + Sink<ws::Message, Error = ws::Error>,
{
    let (mut sink, mut stream) = stream.split();

    println!("Client {} connected", req_id);

    while let Some(message) = stream
        .next()
        .await
        .transpose()
        .map_err(|error| println!("Websocket receive error: {}", error))?
    {
        // println!("{}: {:?}", req_id, message);
        match sink.send(message).await {
            Ok(()) => (),
            // this error indicates a successfully closed connection
            Err(ws::Error::ConnectionClosed) => break,
            Err(error) => {
                println!("Websocket send error: {}", error);
                return Err(());
            }
        }
    }

    println!("Client {} disconnected", req_id);
    Ok(())
}

fn bad_request() -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::empty())
        .unwrap()
}

const INDEX_HTML: &str = include_str!("../static/index.html");
