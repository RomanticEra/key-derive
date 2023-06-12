use std::sync::atomic::AtomicU32;

use key_derive::{encode, CONFIG};

#[cfg(test)]
pub use poem::test::TestClient;
#[cfg(test)]
mod test;

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
            TcpListener::bind("127.0.0.1:3000"), // .rustls(async_stream::stream! {
                                                 //     loop {
                                                 //         if let Ok(tls_config) = load_tls_config() {
                                                 //             yield tls_config;
                                                 //         } else {
                                                 //             panic!("No Cert Found!")
                                                 //         }
                                                 //         tokio::time::sleep(Duration::from_secs(20)).await;
                                                 //     }
                                                 // }),
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
