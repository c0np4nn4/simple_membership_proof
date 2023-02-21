use ark_std::test_rng;
use bytes::Bytes;
use handler::{get_path, get_root, send_proof};
use hyper::{
    body::to_bytes,
    service::{make_service_fn, service_fn},
    Body, Request, Server,
};
use payment::ledger::{Parameters, State};
use route_recognizer::Params;
use router::Router;
use std::sync::{Arc, Mutex};

mod handler;
mod payment;
mod router;

type Response = hyper::Response<hyper::Body>;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Clone)]
pub struct AppState {
    pub state_thing: Arc<Mutex<State>>,
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

// struct AccountInfo {
//     id: AccountId,
//     pk: AccountPublicKey,
//     sk: AccountSecretKey,
// }

// impl AccountInfo {
//     pub fn serialize(&self) {
//         let id: u8 = self.id.0;
//         let pk = self.pk.to_string();
//         let sk_pk = self.sk.public_key.to_string();
//         let sk_sk = self.sk.secret_key.to_string();

//         println!("{:?}\n{:?}\n{:?}\n{:?}", id, pk, sk_pk, sk_sk);
//     }
// }

pub const TREE_SIZE: usize = 16;

#[tokio::main]
async fn main() {
    let mut rng = test_rng();
    let pp = Parameters::sample(&mut rng);
    let mut state = State::new(TREE_SIZE * 2, pp.clone());

    for _ in 0..TREE_SIZE - 1 {
        let (_id, _pk, _sk) = state.sample_keys_and_register(&pp, &mut rng).unwrap();

        // let _acc_info = AccountInfo { id, pk, sk };
    }

    println!("[!] Tree has been initialized");

    // println!("state size: {:?}", state.account_merkle_tree.height());

    let runtime_state = Arc::new(Mutex::new(state));

    let mut router: Router = Router::new();

    // get
    router.get("/get_path", Box::new(get_path));
    router.get("/get_root", Box::new(get_root));

    // post
    router.post("/send_proof", Box::new(send_proof));

    let shared_router = Arc::new(router);

    let new_service = make_service_fn(move |_| {
        let app_state = AppState {
            state_thing: runtime_state.clone(),
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
    println!("[!] route has been invoked!, req: {:?}", req);

    let found_handler = router.route(req.uri().path(), req.method());

    let resp = found_handler
        .handler
        .invoke(Context::new(
            //
            app_state,
            req,
            found_handler.params,
        ))
        .await;

    Ok(resp)
}
