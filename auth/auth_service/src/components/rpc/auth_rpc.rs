use tonic::{transport::Server, Request, Response, Status};

use auth::auth_server::{Auth, AuthServer};
use auth::{AuthRequest, AuthResponse};
use redis::{Client, Connection, Commands};
use tokio::sync::Mutex;
use tonic::codegen::Arc;
// use crate::components::redis::crud::{create, read, update, delete};
pub mod auth {
    tonic::include_proto!("auth");
}
// #[derive(Debug)]


pub struct AuthService{
    pub conn: Arc<Mutex<Connection>>,
}






#[tonic::async_trait]
impl Auth for AuthService{

    async fn validate_auth(&self, request: Request<AuthRequest>) -> Result<Response<AuthResponse>, Status> {
        println!("Got a request: {:?}", request);
        let mut success=true;
        let req = request.into_inner();
        
        let mut temp =self.conn.lock().await;
        let mut db_res = String::from("");
        let mut query: redis::RedisResult<String> = match temp.get(req.username){
            Ok(value) => Ok(value),
            Err(e) =>{ 
                success=false;
                Ok("null".to_string())
            },
        };
        if success{
            if query.unwrap() == req.password{
                db_res="success".to_string();
            }
            else{
                db_res="incorrect password error".to_string();
                success=false;
            }   
        }
        else{
            db_res= "user not found error".to_string();
        }

        println!("Query: {:?}", db_res);
        let reply = AuthResponse {
            successful: success,
            message: db_res.into(),
        };

        Ok(Response::new(reply))
    }




    async fn create_new_user(&self, request: Request<AuthRequest>) -> Result<Response<AuthResponse>, Status> {
        println!("Got a request: {:?}", request);
        let mut success=true;
        let req = request.into_inner();
        
        let mut temp =self.conn.lock().await;
        let mut db_res = String::from("");
        let mut check: redis::RedisResult<String> = match temp.get(req.username.clone()){
            Ok(value) => {
                success=false;
                Ok(value)
            },
            Err(e) =>{ Ok("DNE".to_string())},
        };
        if !success{
            let reply = AuthResponse {
                successful: false,
                message: "ERR; user already exists".into(),
            };
    
            return Ok(Response::new(reply));
        }

        let mut create: redis::RedisResult<String> = match temp.set(req.username.clone(), req.password.clone()){
            Ok(value) => Ok(value),
            Err(e) =>{ 
                success=false;
                Ok("null".to_string())
            },
        };

        let reply = AuthResponse {
            successful: true,
            message: create.unwrap().into(),
        };

        return Ok(Response::new(reply))
        
    }

    async fn delete_user(&self, request: Request<AuthRequest>) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();
        let reply = AuthResponse {
            successful: true,
            message: format!("Username: {} Password: {}", req.username, req.password).into(),
        };
        Ok(Response::new(reply))
    }
}
