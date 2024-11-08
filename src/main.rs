pub mod pb {
    tonic::include_proto!("org.apache.seata.protocol.protobuf.grcp_message");
}
use std::collections::HashMap;
use prost::alloc::vec::Vec;
use std::time::Duration;
use org::apache::seata::protocol::protobuf::{RegisterTmRequestProto, AbstractIdentifyRequestProto, RegisterTmResponseProto};
use tokio_stream::{Stream, StreamExt};
use tonic::transport::Channel;
use pb::{seata_service_client::SeataServiceClient, GrpcMessageProto};
use prost::alloc::string::String;
use prost::alloc::boxed::Box;

async fn streaming_msg(client: &mut SeataServiceClient<Channel>, num: usize) {
    let register_tm_request_proto = RegisterTmRequestProto {
        abstract_identify_request: Some(AbstractIdentifyRequestProto {
            abstract_message: Some(AbstractMessageProto {
                message_type: MessageTypeProto::TypeGlobalBegin as i32,
            }),
            version: String::from("1.0"),
            application_id: String::from("app_id"),
            transaction_service_group: String::from("tx_group"),
            extra_data: String::from("extra_data"),
        }),
    };
    let mut head_map = HashMap::new();
    let grpc_message_proto = GrpcMessageProto {
        id: 1,
        message_type: 2,
        head_map: head_map,
        body: Vec::from("example body"),
    };
    let stream = client
        .send_request(grpc_message_proto)
        .await
        .unwrap()
        .into_inner();

    // stream is infinite - take just 5 elements and then disconnect
    let mut stream = stream.take(num);
    while let Some(item) = stream.next().await {
        println!("\treceived: {}", item.unwrap().message);
    }
    // stream is dropped here and the disconnect info is sent to server
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SeataServiceClient::connect("http://127.0.0.1:8091").await?;


    Ok(())
}
