use ark_std::test_rng;
use bytes::Bytes;
use hyper::{
    body::to_bytes,
    service::{make_service_fn, service_fn},
    Body, Request, Server,
};
use payment::ledger::{Parameters, State, Amount};
use route_recognizer::Params;
use router::Router;
use std::sync::{Arc, Mutex};

mod handler;
mod router;
mod payment;


type Response = hyper::Response<hyper::Body>;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Clone)]
pub struct AppState {
    pub state_thing: Arc<Mutex<State>>,
}

#[tokio::main]
async fn main() {
    //
    let mut rng = test_rng();
    let pp = Parameters::sample(&mut rng);
    let mut state = State::new(32, &pp);

    let (alice_id, _alice_pk, _alice_sk) =
        state.sample_keys_and_register(&pp, &mut rng).unwrap();
    // Let's give her some initial balance to start with.
    state
        .update_balance(alice_id, Amount(100))
        .expect("Alice's account should exist");
    // Let's make an account for Bob.
    let (bob_id, _bob_pk, bob_sk) = state.sample_keys_and_register(&pp, &mut rng).unwrap();

    let some_state = Arc::new(Mutex::new(state));

    //
    let mut router: Router = Router::new();
    router.get("/get_balance", Box::new(handler::get_balance));
    router.post("/add_balance", Box::new(handler::add_balance));

    let shared_router = Arc::new(router);
    let new_service = make_service_fn(move |_| {
        let app_state = AppState {
            state_thing: some_state.clone(),
        };

        let router_capture = shared_router.clone();
        async {
            Ok::<_, Error>(service_fn(move |req| {
                route(router_capture.clone(), req, app_state.clone())
            }))
        }
    });

    let addr = "127.0.0.1:8080".parse().expect("address creation works");
    let server = Server::bind(&addr).serve(new_service);
    println!("Listening on http://{}", addr);
    let _ = server.await;
}

async fn route(
    router: Arc<Router>,
    req: Request<hyper::Body>,
    app_state: AppState,
) -> Result<Response, Error> {
    let found_handler = router.route(req.uri().path(), req.method());
    let resp = found_handler
        .handler
        .invoke(Context::new(app_state, req, found_handler.params))
        .await;
    Ok(resp)
}

pub struct Context {
    pub state: AppState,
    pub req: Request<Body>,
    pub params: Params,
    body_bytes: Option<Bytes>,
}

impl Context {
    pub fn new(state: AppState, req: Request<Body>, params: Params) -> Context {
        Context {
            state,
            req,
            params,
            body_bytes: None,
        }
    }

    pub async fn body_json<T: serde::de::DeserializeOwned>(&mut self) -> Result<T, Error> {
        let body_bytes = match self.body_bytes {
            Some(ref v) => v,
            _ => {
                let body = to_bytes(self.req.body_mut()).await?;
                self.body_bytes = Some(body);
                self.body_bytes.as_ref().expect("body_bytes was set above")
            }
        };
        Ok(serde_json::from_slice(&body_bytes)?)
    }
}

