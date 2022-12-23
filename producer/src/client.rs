use payments::bitcoin_client::BitcoinClient;
use payments::BtcPayRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut client = BitcoinClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(BtcPayRequest {
        amount: 100,
        from_addr: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".into(),
        to_addr: "asdNa".into(),
    });
    let response = client.send_payment(request).await?;
    println!("RESPONSE={:?}", response);
    Ok(())

}