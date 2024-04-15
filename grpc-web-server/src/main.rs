use tonic::{codec::CompressionEncoding, transport::Server};
use tonic_web::enable;

use crate::{
    proto::{echo_service_server::EchoServiceServer, greet_service_server::GreetServiceServer},
    server::{Echo, Greet},
};

mod proto;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse().unwrap();
    let echo = EchoServiceServer::new(Echo)
        .accept_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Zstd);
    let greet = GreetServiceServer::new(Greet)
        .accept_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Zstd);

    eprintln!("Listening on: http://{}", addr);

    Server::builder()
        .accept_http1(true)
        .add_service(enable(echo))
        .add_service(enable(greet))
        .serve(addr)
        .await?;

    Ok(())
}
