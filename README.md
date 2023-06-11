## FOLLOW DOC WITH
https://docs.rs/poem/latest/poem/test/index.html
https://github.com/poem-web/poem/blob/master/examples/poem/tls-reload/Cargo.toml
https://github.com/poem-web/poem/blob/master/examples/poem/tls-reload/src/main.rs

* test code
```rs
#[tokio::test]
async fn it_key() {
    dotenv::dotenv().ok();
    let app = App::create();
    let cli = TestClient::new(app);

    // send request
    // check the status code
    // check the body string
    let resp = cli.get("/1").send().await;
    resp.assert_status_is_ok();
    insta::assert_debug_snapshot!(resp.json().await);
}
```