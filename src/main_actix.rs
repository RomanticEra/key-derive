use actix_web::{
    web::{self, Path},
    App, HttpResponse, HttpServer,
};
use key_derive::CONFIG;

// async
fn derive_child_key(index: web::Path<u32>) -> String {
    // async fn derive_child_key(index: web::Path<u32>) -> HttpResponse {
    let result_tuple = CONFIG.derive_child_key(index.into_inner());

    let response_body = format!(
        "child_key={}\nchild_chain_code={}\nchild_public_key={}",
        String::from_utf8(result_tuple.0[..].to_vec()).unwrap(),
        String::from_utf8(result_tuple.1[..].to_vec()).unwrap(),
        result_tuple.2,
    );
    response_body

    // HttpResponse::Ok().body(response_body)
}

#[test]
fn it_key() {
    dbg!(derive_child_key(Path(1)));
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(web::resource("/{index}").to(derive_child_key)))
//         .bind("0.0.0.0:8000")?
//         .run()
//         .await
// }
