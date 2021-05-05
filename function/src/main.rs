use aws_lambda_events::event::apigw;
use lambda_runtime::{handler_fn, Context, Error};
use log::LevelFilter;
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

async fn handler(event: apigw::ApiGatewayV2httpRequest, _: Context) -> Result<Value, Error> {
  println!("{:?}", event);
  let name = event.query_string_parameters["name"].as_str();
  let response = format!("Hello, {}!", name);
  log::info!("{}", response);

  Ok(json!({ "response": response }))
}
