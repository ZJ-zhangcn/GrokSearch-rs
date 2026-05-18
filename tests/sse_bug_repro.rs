// 复现/回归：上游返回 SSE，并且发完 completed 后不主动关闭连接。

use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use grok_search_rs::providers::http::{build_client, post_json};
use serde_json::json;

async fn spawn_sse_server(expected_stream: bool) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        loop {
            let (mut sock, _) = match listener.accept().await {
                Ok(s) => s,
                Err(_) => return,
            };
            tokio::spawn(async move {
                let mut buf = [0u8; 4096];
                let n = sock.read(&mut buf).await.unwrap_or(0);
                let request = String::from_utf8_lossy(&buf[..n]);
                let expected = format!(r#""stream":{}"#, expected_stream);
                if !request.contains(&expected) {
                    let _ = sock
                        .write_all(
                            b"HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                        )
                        .await;
                    return;
                }

                let body = b"event: response.created\n\
data: {\"type\":\"response.created\"}\n\n\
event: response.output_text.delta\n\
data: {\"type\":\"response.output_text.delta\",\"delta\":\"streamed answer\"}\n\n\
event: response.completed\n\
data: {\"type\":\"response.completed\"}\n\n";

                let resp = "HTTP/1.1 200 OK\r\n\
                     Content-Type: text/event-stream\r\n\
                     Connection: keep-alive\r\n\r\n";
                let _ = sock.write_all(resp.as_bytes()).await;
                let _ = sock.write_all(body).await;

                // 不 shutdown：服务端保持连接，客户端必须在 completed 后主动结束读取。
                let mut one = [0u8; 1];
                let _ = sock.read(&mut one).await;
            });
        }
    });

    format!("http://{}", addr)
}

#[tokio::test]
async fn post_json_returns_when_stream_true_sse_completed_without_connection_close() {
    let base = spawn_sse_server(true).await;
    let client = build_client(Duration::from_secs(5));

    let raw = post_json(
        &client,
        &format!("{}/v1/responses", base),
        "dummy-key",
        &json!({"model": "grok-4-fast", "input": "test", "stream": true}),
        "Grok Responses",
    )
    .await
    .expect("SSE stream should be normalized into JSON");

    assert_eq!(raw["output_text"], "streamed answer");
}

#[tokio::test]
async fn post_json_returns_when_stream_false_still_gets_sse() {
    let base = spawn_sse_server(false).await;
    let client = build_client(Duration::from_secs(5));

    let raw = post_json(
        &client,
        &format!("{}/v1/responses", base),
        "dummy-key",
        &json!({"model": "grok-4-fast", "input": "test", "stream": false}),
        "Grok Responses",
    )
    .await
    .expect("SSE stream should be normalized even when stream:false is ignored");

    assert_eq!(raw["output_text"], "streamed answer");
}
