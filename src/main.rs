use http_body_util::Full;
use hyper::body::Bytes;
use hyper::header::CONTENT_TYPE;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto;
use serde::Serialize;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct HelloResponse {
    test: String,
}

fn json_response(status: StatusCode, body: Vec<u8>) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .header(CONTENT_TYPE, "application/json")
        .body(Full::new(Bytes::from(body)))
        .expect("response builder received a valid status and header")
}

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let payload = HelloResponse {
        test: "van".to_string(),
    };

    Ok(match serde_json::to_vec(&payload) {
        Ok(body) => json_response(StatusCode::OK, body),
        Err(err) => {
            eprintln!("Error serializing response: {}", err);
            json_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                br#"{"error":"internal server error"}"#.to_vec(),
            )
        }
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = auto::Builder::new(TokioExecutor::new())
                .serve_connection(io, service_fn(hello))
                .await
            {
                eprintln!("Error serving connection: {}", err);
            }
        });
    }
}
