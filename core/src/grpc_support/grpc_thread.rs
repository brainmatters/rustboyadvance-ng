use tonic::{transport::Server, Request, Response, Status};

use rustnodegrpc::commands_server::{Commands, CommandsServer};
use rustnodegrpc::{MeanRequest, MeanResponse};

use std::net::SocketAddr;
use std::sync::mpsc::{channel, SyncSender};
 
 
//use crate::rustboyadvance_core::{ GameBoyAdvance  };
 use rustboyadvance_core::GameBoyAdvance;
mod rustnodegrpc;

//#[derive(Default)]
pub struct RpcCommandServer {
 
    

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

       // self.sender.send(reply.mean.to_string() );

        Ok(Response::new(reply))
    }
}



//need this to be non-blocking

#[tokio::main]
pub async fn start_grpc_server(gba: &mut GameBoyAdvance, port:u32 ) -> Result<(), Box<dyn std::error::Error>> {
    let addr:SocketAddr = "127.0.0.1:9800".parse().unwrap();
    let server = RpcCommandServer {
          
    };
 

    println!("StatsServer listening on {}", addr);

    Server::builder()
        .add_service(CommandsServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}