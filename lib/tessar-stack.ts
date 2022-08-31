import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { Function, Code, Runtime, Architecture } from 'aws-cdk-lib/aws-lambda';
import { RetentionDays } from 'aws-cdk-lib/aws-logs';
import { RestApi, LambdaIntegration } from 'aws-cdk-lib/aws-apigateway';

export class TessarStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);
    const helloLambda = new Function(this, 'hello', {
      description: 'Deploying a Rust function on Lambda using the custom runtime',
      code: Code.fromAsset(
        'target/lambdas/hello.zip'
      ),
      runtime: Runtime.PROVIDED_AL2,
      handler: 'not.required',
      architecture: Architecture.ARM_64,
      environment: {
        RUST_BACKTRACE: '1',
      },
      logRetention: RetentionDays.ONE_WEEK,
    });
    const api = new RestApi(this, "tessar-restapi", {
      restApiName: "Tessar Api",
    });

    api.root.addMethod("GET", new LambdaIntegration(helloLambda, { proxy: true }))
  }

}
