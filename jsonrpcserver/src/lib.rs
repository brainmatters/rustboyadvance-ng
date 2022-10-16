use tonic::{transport::Server, Request, Response, Status};

use rustnodegrpc::commands_server::{Commands, CommandsServer};
use rustnodegrpc::{MeanRequest, MeanResponse};

use std::net::SocketAddr;
use std::sync::mpsc::{channel, SyncSender};
 

mod rustnodegrpc;

//#[derive(Default)]
pub struct RpcCommandServer {
 
    sender: SyncSender<String>

}
 

#[tonic::async_trait]
impl Commands for RpcCommandServer {



    async fn mean(
        &self,
        request: Request<MeanRequest>,
    ) -> Result<Response<MeanResponse>, Status> {
        let r = request.into_inner();
        println!("Got a request for: {:?}", &r);
        

       

        let reply = MeanResponse {
            mean: (r.a + r.b) as f64 / 2.0,
        };

        self.sender.send(reply.mean.to_string() );

        Ok(Response::new(reply))
    }

    async fn getGameTitle(
        &self,
        request: Request<MeanRequest>,
    ) -> Result<Response<MeanResponse>, Status> {
    } 

    async fn getMemory(
        &self,
        request: Request<GetMemoryRequest>,
    ) -> Result<Response<GetMemoryResponse>, Status> {
    }

    async fn setMemory(
        &self,
        request: Request<SetMemoryRequest>,
    ) -> Result<Response<SetMemoryResponse>, Status> {
    }

    async fn sendControllerCommand(
        &self,
        request: Request<ControllerCommandRequest>,
    ) -> Result<Response<ControllerCommandResponse>, Status> {
    }

}



//need this to be non-blocking

#[tokio::main]
pub async fn start_server(sender: SyncSender<String>) -> Result<(), Box<dyn std::error::Error>> {
    let addr:SocketAddr = "127.0.0.1:9800".parse().unwrap();
    let server = RpcCommandServer {
        sender 
    };
 

    println!("StatsServer listening on {}", addr);

    Server::builder()
        .add_service(CommandsServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}