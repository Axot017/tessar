import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { Function, Code, Runtime, Architecture, LayerVersion } from 'aws-cdk-lib/aws-lambda';
import { RetentionDays } from 'aws-cdk-lib/aws-logs';
import { RestApi, LambdaIntegration } from 'aws-cdk-lib/aws-apigateway';

export class TessarStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const flutterLayer = new LayerVersion(this, 'FlutterLayer', {
      code: Code.fromAsset('target/layer/flutter_layer.zip'),
      compatibleRuntimes: [Runtime.PROVIDED_AL2],
    })

    const dartVersionV1 = new Function(this, 'DartVersionV1', {
      description: 'Check dart version',
      code: Code.fromAsset(
        'target/lambdas/dart_version_v1.zip'
      ),
      runtime: Runtime.PROVIDED_AL2,
      handler: 'not.required',
      architecture: Architecture.ARM_64,
      environment: {
        RUST_BACKTRACE: '1',
      },
      logRetention: RetentionDays.ONE_WEEK,
      layers: [flutterLayer],
    });

    const api = new RestApi(this, "TessarRestApi", {
      restApiName: "Tessar Api",
      deployOptions: {
        stageName: 'dev',
      }
    });

    const v1 = api.root.addResource('v1');

    const dart = v1.addResource('dart');
    dart.addResource('version').addMethod("GET", new LambdaIntegration(dartVersionV1, { proxy: true }));
  }

}
