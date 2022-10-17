


use rustboyadvance_utils::Shared;
use tonic::{transport::Server, Request, Response, Status};

use rustnodegrpc::commands_server::{Commands, CommandsServer};
use rustnodegrpc::{MeanRequest, MeanResponse};

use std::net::SocketAddr;
//use std::sync::mpsc::{channel, SyncSender};
 
use std::thread;



use std::sync::mpsc::{channel,sync_channel, SyncSender, Receiver, TryRecvError};


//send data btwn thread 
//https://stackoverflow.com/questions/59075477/what-are-idiomatic-ways-to-send-data-between-threads
 


//use crate::rustboyadvance_core::{ GameBoyAdvance  };
//use rustboyadvance_core::GameBoyAdvance;
use crate::GameBoyAdvance;
use super::SysBus;

mod rustnodegrpc;

//#[derive(Default)]
pub struct RpcCommandServer {
 
    sender:SyncSender<String> 

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

        //use a lock here to grab data from gba to send out ??


        self.sender.send(reply.mean.to_string() );

        Ok(Response::new(reply))
    }
}



//need this to be non-blocking

#[tokio::main]
pub async fn start_grpc_server(gba: &mut GameBoyAdvance, port:u32 ) -> Result<(), Box<dyn std::error::Error>> {
   
    let (txOne, rxOne) = sync_channel(40);

   // let (txTwo, rxTwo) = sync_channel(40);




    thread::spawn(move || { 
        let (txTwo, rxTwo) = sync_channel(40);
 
        boot_grpc_server( txOne.clone() );
    });
 
 
    match rxOne.try_recv() {
            Ok(res) => {
                println!("got recv {}",res );

                //txTwo.clone().send( "woah lad" );

               // input::set_key_state(gba.get_key_state_mut() , keytype , true );
            }
            Err(TryRecvError::Disconnected) =>   {
                println!("disconnected");
            }

            Err(TryRecvError::Empty) =>   {
             //   println!("empty");
            }
             
        } 
  

    Ok(())
}


async fn boot_grpc_server(  sender: SyncSender<String>  ) -> Result<(), Box<dyn std::error::Error>> {

    let addr:SocketAddr = "127.0.0.1:9800".parse().unwrap();

    println!("GRPC Server listening on {}", addr);

    let server = RpcCommandServer {
          sender 
    };

    Server::builder()
    .add_service(CommandsServer::new(server))
    .serve(addr)
    .await?;


    Ok(())
}