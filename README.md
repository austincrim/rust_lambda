# Rust + Lambda + API Gateway PoC

Base URL:  
https://jd5a45slm6.execute-api.us-east-1.amazonaws.com

## Endpoints

`POST /calculate`
```json
{
  "num1": number,
  "num2": number
}
```

## Useful commands

 * `npm run build`   compile typescript to js
 * `npm run watch`   watch for changes and compile
 * `npm run test`    perform the jest unit tests
 * `cdk deploy`      deploy this stack to your default AWS account/region
 * `cdk diff`        compare deployed stack with current state
 * `cdk synth`       emits the synthesized CloudFormation template
