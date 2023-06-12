use std::sync::atomic::AtomicU32;

use key_derive::{encode, CONFIG};

#[cfg(test)]
pub use poem::test::TestClient;
#[cfg(test)]
mod self_sign;
mod simple_self_sign;
use poem::{
    get, handler,
    listener::{Listener, RustlsCertificate, RustlsConfig, TcpListener},
    web::Query,
    IntoResponse, Route, Server,
};

use serde::Deserialize;
use tokio::time::Duration;

#[handler]
fn derive_child_key() -> String {
    let index = INDEX.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let result_tuple = CONFIG.derive_child_key(index);
    let mut serialized_key = Vec::new();
    serialized_key.extend_from_slice(&result_tuple.0[..]);
    let response_body = format!(
        "your_index={index}\nchild_key={}\nchild_chain_code={}\nchild_public_key={}",
        encode(result_tuple.0.as_ref()),
        encode(result_tuple.1),
        result_tuple.2,
    );
    response_body
}
#[handler]
fn check_public_key(Query(CheckParams { index, pk }): Query<CheckParams>) -> impl IntoResponse {
    if index > INDEX.load(std::sync::atomic::Ordering::SeqCst) {
        return "2".to_string();
    }
    let remote_pub = CONFIG.derive_child_key(index).2;
    if remote_pub.to_string() == pk {
        "0".into()
    } else {
        "1".into()
    }
}

#[derive(Deserialize)]
struct CheckParams {
    index: u32,
    pk: String,
}

#[tokio::test]
async fn it_get() {
    dotenv::dotenv().ok();
    INDEX.store(0, std::sync::atomic::Ordering::SeqCst);
    let app = App::create();
    let cli = TestClient::new(app);

    // send request
    // check the status code
    // check the body string
    // "/get" would error
    let resp = cli.get("/get").send().await;
    resp.assert_status_is_ok();
    insta::assert_debug_snapshot!(resp
        .0
        .into_body()
        .into_string()
        .await
        .unwrap()
        .chars()
        .rev()
        .take(30)
        .collect::<String>());
}
#[tokio::test]
async fn it_check_success() {
    dotenv::dotenv().ok();
    let app = App::create();
    let cli = TestClient::new(app);

    let resp = cli
        .post(
            "/check",
            // "/check?index=4&pk=0312cd51815677e1c7d94f5784cdc08982d2eb0540bd5cc9e7432ab5bfcf7714e6",
        )
        .query("index", &"1")
        .query(
            "pk",
            &"02a23660e2be6f853515801ded42fd340d1f8fa910edda1edab61480851d9700c8",
        )
        .send()
        .await;
    resp.assert_status_is_ok();
    insta::assert_debug_snapshot!(resp
        .0
        .into_body()
        .into_string()
        .await
        .unwrap()
        .chars()
        .rev()
        .take(30)
        .collect::<String>());
}
#[tokio::test]
async fn it_check_fail() {
    dotenv::dotenv().ok();
    let app = App::create();
    let cli = TestClient::new(app);

    let resp = cli
        .post("/check")
        .query("index", &"4")
        .query(
            "pk",
            &"02a23660e2be6f853515801ded42fd340d1f8fa910edda1edab61480851d9700c8",
        )
        .send()
        .await;
    resp.assert_status_is_ok();
    insta::assert_debug_snapshot!(resp
        .0
        .into_body()
        .into_string()
        .await
        .unwrap()
        .chars()
        .rev()
        .take(30)
        .collect::<String>());
}

static INDEX: AtomicU32 = AtomicU32::new(100);

struct App;
impl App {
    fn create() -> Route {
        Route::new()
            .at("/get", get(derive_child_key))
            .at("/check", check_public_key)
    }
    async fn run(app: Route) -> Result<(), std::io::Error> {
        Server::new(
            TcpListener::bind("127.0.0.1:3000").rustls(async_stream::stream! {
                loop {
                    if let Ok(tls_config) = load_tls_config() {
                        yield tls_config;
                    } else {
                        panic!("No Cert Found!")
                    }
                    tokio::time::sleep(Duration::from_secs(20)).await;
                }
            }),
        )
        .run(app)
        .await
    }
}

fn load_tls_config() -> Result<RustlsConfig, std::io::Error> {
    Ok(RustlsConfig::new().fallback(
        RustlsCertificate::new()
            .cert(std::fs::read("s_ca.pem")?)
            .key(std::fs::read("s_key.pem")?),
    ))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    App::run(App::create()).await
}
