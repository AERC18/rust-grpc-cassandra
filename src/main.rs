extern crate cdrs;

use tonic::{transport::Server, Request, Response, Status};

use blocking_service::blocking_service_server::{BlockingService, BlockingServiceServer};
use blocking_service::*;

use reporting_service::reporting_service_server::{ReportingService, ReportingServiceServer};
use reporting_service::*;

mod blocking_users_service;
mod cassandra_session;
mod reporting_users_service;

use crate::blocking_users_service::{create_blocking_ks, create_blocking_table, block_user, unblock_user, are_this_users_blocked};
use crate::reporting_users_service::{create_reporting_ks, create_reporting_table, report_user};

pub mod blocking_service {
    tonic::include_proto!("blocking_service");
}

pub mod reporting_service {
    tonic::include_proto!("reporting_service");
}

// -----------------------------------BLOCKING SERVICE----------------------------------------------
pub struct MyBlockingService {}

#[tonic::async_trait]
impl BlockingService for MyBlockingService {
    async fn block_user(
        &self,
        request: Request<BlockUserInfo>
    ) -> Result<Response<BlockUserResponse>, Status> {
        println!("Got a request: {:?}", request);
        block_user(&request.into_inner());
        
        let reply = blocking_service::BlockUserResponse {
            acknowledge: true,
            response: String::from("This is a response from the Rust gRPc Server")	
        };  
        
        Ok(Response::new(reply)) 
    }

    async fn unblock_user(
        &self,
        request: Request<UnblockUserInfo>
    ) -> Result<Response<UnblockUserResponse>, Status> {
        println!("Got a request: {:?}", request);
        unblock_user(&request.into_inner());
        
        let reply = blocking_service::UnblockUserResponse {
            acknowledge: true,
            response: String::from("This is a response from the Rust gRPc Server")	
        };  
        
        Ok(Response::new(reply)) 
    }

    async fn are_this_users_blocked(
        &self,
        request: Request<AreThisUsersBlockedRequest>
    ) -> Result<Response<AreThisUsersBlockedResponse>, Status> {
        println!("Got a request: {:?}", request);
        let reply = are_this_users_blocked(&request.into_inner());  
        Ok(Response::new(reply)) 
    }
}

// -----------------------------------REPORTING SERVICE---------------------------------------------
pub struct  MyReportingService {}

#[tonic::async_trait]
impl ReportingService for MyReportingService {
    async fn report_user(
        &self,
        request: Request<ReportUserRequest>
    ) -> Result<Response<ReportUserResponse>, Status> {
        println!("Got a request: {:?}", request);
        report_user(&request.into_inner());

        let reply = reporting_service::ReportUserResponse {
            acknowledge: true,
            response: String::from("This is a response from the Rust reporting gRPc Server")
        };

        Ok(Response::new(reply))
    }
}
// -----------------------------------MAIN FUNCTION-------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "0.0.0.0:50051".parse()?;
    let blocking_service = MyBlockingService {};
    let reporting_service = MyReportingService{};
    // ----------------- Testing space ---------------
    // ----------------- End of testing space --------
    // Create the cassandra keyspace and table
    create_blocking_ks();
    create_blocking_table();
    create_reporting_ks();
    create_reporting_table();

    Server::builder()
	.add_service(BlockingServiceServer::new(blocking_service))
        .add_service(ReportingServiceServer::new(reporting_service))
	.serve(address)
	.await?;
    Ok(())

}
