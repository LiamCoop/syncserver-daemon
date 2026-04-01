use futures_util::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub async fn open_ws_conn(
    url: &str,
) -> Result<
    (
        SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ),
    Box<dyn std::error::Error>,
> {
    let (stream, _) = connect_async(url).await?;
    let (sink, stream) = stream.split();
    return Ok((sink, stream));
}

#[cfg(test)]
mod tests {
    use tokio::net::TcpListener;
    use tokio_tungstenite::accept_async;

    use super::*;

    // This attribute tells tokio to set up an async runtime just for this test.
    // Equivalent to wrapping your test in an async IIFE in JS.
    #[tokio::test]
    async fn test_open_ws_conn_succeeds() {
        // Spin up a real local WebSocket server on a random port
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let url = format!("ws://{}", addr);

        // Spawn the mock server as a concurrent task — like Promise.all() in JS
        // It just needs to accept the connection so our client doesn't hang
        tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            accept_async(stream).await.unwrap();
            // Server just holds the connection open, does nothing
        });

        let result = open_ws_conn(&url).await;
        assert!(result.is_ok(), "should connect successfully");
    }

    #[tokio::test]
    async fn test_open_ws_conn_returns_split_halves() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let url = format!("ws://{}", addr);

        tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            accept_async(stream).await.unwrap();
        });

        // Destructure the split halves — this verifies your return type is correct
        let (mut sender, mut receiver) = open_ws_conn(&url).await.unwrap();

        // Compiler will catch it if the halves are the wrong types
        let _ = (&mut sender, &mut receiver);
    }

    #[tokio::test]
    async fn test_open_ws_conn_bad_url_fails() {
        let result = open_ws_conn("ws://127.0.0.1:1").await;
        assert!(result.is_err(), "should fail on bad url");
    }
}
