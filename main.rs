use spiceai::ClientBuilder;

#[tokio::main]
async fn main() {
  let mut client = ClientBuilder::new()
    .flight_url("http://localhost:50051")
    .build()
    .await
    .unwrap();

  let data = client.query("SELECT trip_distance, total_amount FROM taxi_trips ORDER BY trip_distance DESC LIMIT 10;").await;
  print!("{}", data);
}
