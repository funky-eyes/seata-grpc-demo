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

fn seata_requests_iter() -> impl Stream<Item = GrpcMessageProto> {
    let abstract_identify_request_proto = AbstractIdentifyRequestProto {
        abstract_message: None,
        version: String::new(),
        application_id: String::from("test-applicationId"),
        transaction_service_group: String::new(),
        extra_data: String::new(),
    };

    let mut head_map = HashMap::new();
    let result = 2f64.powi(1);
    let register_tm_request_proto = RegisterTmRequestProto {
        abstract_identify_request: Some(abstract_identify_request_proto),
    };
    head_map.insert("codec-type".to_string(), result.to_string());
    let bytes = register_tm_request_proto.encode_to_vec();
    let grpc_message_proto = GrpcMessageProto {
        id: 1,
        message_type: 2,
        head_map: head_map,
        body: bytes,
    };
    let test =  RegisterTmRequestProto::decode(grpc_message_proto.body.as_slice()).unwrap();
    println!("\treceived message: `{}`", test.abstract_identify_request.unwrap().application_id);
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
        println!("\treceived message: `{}`", received.message_type);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SeataServiceClient::connect("http://127.0.0.1:8091").await?;
    println!("\r\nseata stream echo:");
    seata_streaming_echo(&mut client, 1).await;

    Ok(())
}
