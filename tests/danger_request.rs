/// you need to setup a really web serve, and value would change.
// #[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建一个 Reqwest 的客户端，并将证书和私钥添加到客户端配置中
    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .danger_accept_invalid_certs(true)
        .build()?;

    // // 发送请求
    let response = client.get("https://127.0.0.1:3000/get/").send().await?;
    let body = response.text().await?;
    eprintln!("Response body: {}", body);
    insta::assert_debug_snapshot!(body);

    Ok(())
}
