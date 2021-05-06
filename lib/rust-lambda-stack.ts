import * as cdk from '@aws-cdk/core'
import * as lambda from '@aws-cdk/aws-lambda'
import * as apig from '@aws-cdk/aws-apigatewayv2'
import * as ints from '@aws-cdk/aws-apigatewayv2-integrations'

export class RustLambdaStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props)

    const func = new lambda.Function(this, 'hello_rust', {
      functionName: 'hello_rust',
      description:
        'a Rust function deployed with CDK using a custom Lambda runtime',
      code: lambda.Code.fromAsset('function/target/lambda'),
      runtime: lambda.Runtime.PROVIDED_AL2,
      handler: 'not.required',
      environment: {
        RUST_BACKTRACE: '1'
      }
    })

    const integration = new ints.LambdaProxyIntegration({
      handler: func,
      payloadFormatVersion: apig.PayloadFormatVersion.VERSION_2_0
    })

    const api = new apig.HttpApi(this, 'hello_rust_api')
    api.addRoutes({
      path: '/calculate',
      integration
    })
  }
}
