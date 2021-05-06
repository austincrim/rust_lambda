# Rust + Lambda + API Gateway PoC  

## Example Request

```
POST https://jd5a45slm6.execute-api.us-east-1.amazonaws.com/calculate?name=Austin

{
  "num1": 2,
  "num2": 10
}
```

## Useful commands

 * `npm run build`   compile typescript to js
 * `npm run watch`   watch for changes and compile
 * `npm run test`    perform the jest unit tests
 * `cdk deploy`      deploy this stack to your default AWS account/region
 * `cdk diff`        compare deployed stack with current state
 * `cdk synth`       emits the synthesized CloudFormation template
