import * as cdk from '@aws-cdk/core'
import * as lambda from '@aws-cdk/aws-lambda'

export class RustLambdaStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props)

    new lambda.Function(this, 'hello_rust', {
      functionName: 'hello_rust',
      description: 'a Rust function deployed with CDK using a custom Lambda runtime',
      code: lambda.Code.fromAsset('function/target/lambda'),
      runtime: lambda.Runtime.PROVIDED_AL2,
      handler: 'not.required',
      environment: {
        RUST_BACKTRACE: '1'
      }
    })
  }
}
