use std::time::Duration;

use tokio::{
    sync::mpsc::{self, error::SendError},
    time::sleep,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::proto::{
    echo_service_server::EchoService, greet_service_server::GreetService, EchoRequest,
    EchoResponse, EchoStreamRequest, EchoStreamResponse, GreetRequest, GreetResponse,
};

#[derive(Debug, Default)]
pub struct Echo;

#[tonic::async_trait]
impl EchoService for Echo {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let request = request.into_inner();
        eprintln!("Received request: {:?}", request);
        let response = Response::new(EchoResponse {
            message: request.message,
        });

        Ok(response)
    }

    type EchoStreamStream = ReceiverStream<Result<EchoStreamResponse, Status>>;

    async fn echo_stream(
        &self,
        request: Request<EchoStreamRequest>,
    ) -> Result<Response<Self::EchoStreamStream>, Status> {
        let request = request.into_inner();

        let (tx, rx) = mpsc::channel(5);

        tokio::spawn(async move {
            for _ in 0..request.times {
                tx.send(Ok(EchoStreamResponse {
                    message: request.message.clone(),
                }))
                .await?;
                sleep(Duration::from_secs(1)).await;
            }
            Ok::<_, SendError<_>>(())
        });
        Ok(Response::new(Self::EchoStreamStream::new(rx)))
    }
}

#[derive(Debug, Default)]
pub struct Greet;

#[tonic::async_trait]
impl GreetService for Greet {
    async fn greet(
        &self,
        request: Request<GreetRequest>,
    ) -> Result<Response<GreetResponse>, Status> {
        let request = request.into_inner();
        eprintln!("Received request: {:?}", request);
        let response = Response::new(GreetResponse {
            greeting: format!("Hello, {}!", request.name),
        });

        Ok(response)
    }
}
