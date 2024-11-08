use  seata_service::seata_service_client::SeataServiceClient;
use  seata_service::GrpcMessageProto;

pub mod seata_service {
    tonic::include_proto!("seataservice");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SeataServiceClient::connect("http://127.0.0.1:8091").await?;




    Ok(())
}
