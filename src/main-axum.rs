// use key_derive::{encode, CONFIG};

// use axum::{extract::Path, http::Error, routing::get, Router};

// // #[cfg(test)]
// // #[handler]
// fn derive_child_key(Path(index): Path<u32>) -> String {
//     let result_tuple = CONFIG.derive_child_key(index);
//     let mut serialized_key = Vec::new();
//     serialized_key.extend_from_slice(&result_tuple.0[..]);
//     let response_body = format!(
//         "child_key={}\nchild_chain_code={}\nchild_public_key={}",
//         encode(result_tuple.0.as_ref()),
//         encode(result_tuple.1),
//         result_tuple.2,
//     );
//     response_body
// }
// // t
// // #[tokio::test]
// // async fn it_key() {
// //     dotenv::dotenv().ok();
// //     let app = App::create();
// //     let cli = TestClient::new(app);

// //     // send request
// //     // check the status code
// //     // check the body string
// //     let resp = cli.get("/1").send().await;
// //     resp.assert_status_is_ok();
// //     insta::assert_debug_snapshot!(resp
// //         .0
// //         .into_body()
// //         .into_string()
// //         .await
// //         .unwrap()
// //         .chars()
// //         .rev()
// //         .take(30)
// //         .collect::<String>());
// // }

// struct App;
// impl App {
//     fn create() -> Router {
//         Router::new().route("/:index", get(derive_child_key))
//     }
//     async fn run(app: Router) -> Result<(), Error> {
//         axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//             .serve(app.into_make_service())
//             .await
//     }
// }

// // #[tokio::main]
// // async fn main() -> Result<(), std::io::Error> {
// //     dotenv::dotenv().ok();
// //     App::run(App::create()).await
// // }
