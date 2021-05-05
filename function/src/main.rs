use aws_lambda_events::event::apigw::{ApiGatewayV2httpRequest};
use lambda_runtime::{handler_fn, Context, Error};
use log::LevelFilter;
use serde;
use serde_json::{json, Value};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
  SimpleLogger::new()
    .with_level(LevelFilter::Info)
    .init()
    .unwrap();

  let func = handler_fn(handler);
  lambda_runtime::run(func).await?;
  Ok(())
}

async fn handler(event: ApiGatewayV2httpRequest, _: Context) -> Result<Value, Error> {
  log::info!("request body: {:?}", event.body);
  let name = event.query_string_parameters["name"].as_str();
  let body: CalculatorRequest = serde_json::from_str(&event.body.unwrap()).unwrap();
  let response = format!("Hello, {}, you sent me {:?}", name, body);
  log::info!("{}", response);

  Ok(json!({ "response": response }))
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct CalculatorRequest {
  num_one: i32,
  num_two: i32
}