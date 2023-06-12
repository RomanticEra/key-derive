use crate::*;

mod self_sign;
mod simple_self_sign;
// use crate::*;
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
