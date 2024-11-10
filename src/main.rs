pub mod grpc_message {
    tonic::include_proto!("org.apache.seata.protocol.protobuf.grcp_message");
}

pub mod seata_protobuf {
    tonic::include_proto!("org.apache.seata.protocol.protobuf");
}

use std::collections::HashMap;
use seata_protobuf::{MessageTypeProto, RegisterTmRequestProto, AbstractMessageProto, AbstractIdentifyRequestProto, RegisterTmResponseProto};
use tokio_stream::{Stream, StreamExt};
use tonic::transport::Channel;
use grpc_message::{seata_service_client::SeataServiceClient, GrpcMessageProto};
use prost::alloc::string::String;
use prost::alloc::boxed::Box;
use prost::Message;
use prost_types::Any;

fn seata_requests_iter() -> impl Stream<Item=GrpcMessageProto> {
    let abstract_identify_request_proto = AbstractIdentifyRequestProto {
        abstract_message: None,
        version: String::new(),
        application_id: String::from("test-applicationId"),
        transaction_service_group: String::new(),
        extra_data: String::new(),
    };

    let mut head_map = HashMap::new();
    let byte_value = (0x128 & 0xFF) as u8;
    let register_tm_request_proto = RegisterTmRequestProto {
        abstract_identify_request: Some(abstract_identify_request_proto),
    };
    head_map.insert("codec-type".to_string(), byte_value.to_string());

    let mut buf = Vec::new();
    register_tm_request_proto.encode(&mut buf).unwrap();
    let any_message = Any {
        type_url: "type.googleapis.com/org.apache.seata.protocol.protobuf.RegisterTMRequestProto".to_string(),
        value: buf,
    };
    let grpc_message_proto = GrpcMessageProto {
        id: 1,
        message_type: 2,
        head_map: head_map,
        body: any_message.encode_to_vec(),
    };
    tokio_stream::iter(1..usize::MAX).map(move |i| grpc_message_proto.clone())
}

async fn seata_streaming_echo(client: &mut SeataServiceClient<Channel>, num: usize) {
    let in_stream = seata_requests_iter().take(num);

    let response = client
        .send_request(in_stream)
        .await
        .unwrap();

    let mut resp_stream = response.into_inner();

    while let Some(received) = resp_stream.next().await {
        let received = received.unwrap();
        let decoded_message: Any = Any::decode(&*received.body).unwrap();
        let response = RegisterTmResponseProto::decode(&*decoded_message.value).unwrap();
        println!("\treceived message: `{}`", response.abstract_identify_response.unwrap().version);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SeataServiceClient::connect("http://127.0.0.1:8091").await?;
    println!("\r\nseata stream echo:");
    seata_streaming_echo(&mut client, 1).await;

    Ok(())
}
