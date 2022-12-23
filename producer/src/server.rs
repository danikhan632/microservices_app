use tonic::{transport::Server, Request, Response, Status};
use payments::bitcoin_server::{Bitcoin, BitcoinServer};
use payments::{BtcPayRequest, BTCPayResponse};
use rand::Rng;
pub mod payments {
    tonic::include_proto!("payments");
}
#[derive(Debug, Default)]
pub struct BitcoinService {}

impl Bitcoin for BitcoinService {
    async fn SendPayment(&self, request: Request<BtcPayRequest>) -> Result<Response<BTCPayResponse>, Status> {
        // println!("Got a request: {:?}", request);
        let mut rng = rand::thread_rng();
        let reply = payments::BTCPayResponse {
            success:true,
            message: ("Hello from the server: "+ (rng.gen_range(0, 99999))).into(),
        };
        Ok(Response::new(reply))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let bitcoin = BitcoinService::default();
    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(BitcoinServer::new(bitcoin))
        .serve(addr)
        .await?;
    Ok(())
}
