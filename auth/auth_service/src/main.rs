mod components;
use components::rpc::auth_rpc::AuthService;
use crate::components::rpc::auth_rpc::auth::auth_server::AuthServer;
use tonic::{transport::Server, Request, Response, Status};
use redis::{Client, Connection, Commands};
use tokio::sync::Mutex;
use tonic::codegen::Arc;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let mut conn = client.get_connection().unwrap();
    let addr = "127.0.0.1:50051".parse()?;
    let auth_service = AuthService{conn: Arc::new(Mutex::new(conn))};

    Server::builder()
        .add_service(AuthServer::new(auth_service))

        .serve(addr)

        .await?;

    Ok(())
}