use tonic::{Request, Response, Status};

use crate::proto::{
    echo_service_server::EchoService, greet_service_server::GreetService, EchoRequest,
    EchoResponse, GreetRequest, GreetResponse,
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
