use proto::{
    calculator_server::{Calculator, CalculatorServer},
    CalculationResponse,
};
use tonic::transport::Server;

mod proto {
    tonic::include_proto!("calculator");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("calculator_descriptor");
}

#[derive(Debug, Default)]
struct CalculatorService {}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<CalculationResponse>, tonic::Status> {
        println!("Got a new request: {:?}", request);
        let input = request.get_ref();
        let response = proto::CalculationResponse {
            result: input.a + input.b,
        };
        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() {
    // Reflection
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    // gRPC server
    let addr = "[::1]:50051".parse().unwrap();
    let calc = CalculatorService::default();
    Server::builder()
        .add_service(service)
        .add_service(CalculatorServer::new(calc))
        .serve(addr)
        .await
        .unwrap();
}
