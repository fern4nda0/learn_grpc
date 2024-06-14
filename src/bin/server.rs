use tonic::{transport::Server, Request, Response, Status};
use test::hello_server::{Hello, HelloServer};
use test::{HelloRequest, HelloReply};

pub mod test {
    tonic::include_proto!("test");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Hello for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        
        let name = request.into_inner().name;

        println!("Client said {}",&name);
        
        let reply = HelloReply {
            message: format!("Hello, {}!",name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(HelloServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
