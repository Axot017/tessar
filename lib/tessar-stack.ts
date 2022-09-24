import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { Function, Code, Runtime, Architecture, LayerVersion } from 'aws-cdk-lib/aws-lambda';
import { RetentionDays } from 'aws-cdk-lib/aws-logs';
import { RestApi, LambdaIntegration } from 'aws-cdk-lib/aws-apigateway';

export class TessarStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const dartLayer = new LayerVersion(this, 'DartLayer', {
      code: Code.fromAsset('target/layer/dart.zip'),
      compatibleRuntimes: [Runtime.PROVIDED_AL2],
    })

    const dartProjectLayer = new LayerVersion(this, 'DartProjectLayer', {
      code: Code.fromAsset('target/layer/dart_project.zip'),
      compatibleRuntimes: [Runtime.PROVIDED_AL2],
    })

    const dartAnalyzeV1 = new Function(this, 'DartAnalyzeV1', {
      description: 'Analyze dart code',
      code: Code.fromAsset(
        'target/lambdas/dart_analyze_v1.zip'
      ),
      runtime: Runtime.PROVIDED_AL2,
      handler: 'not.required',
      architecture: Architecture.ARM_64,
      environment: {
        RUST_BACKTRACE: '1',
      },
      logRetention: RetentionDays.ONE_WEEK,
      layers: [dartLayer, dartProjectLayer],
    });

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
      layers: [dartLayer],
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
    dart.addResource('analyze').addMethod("GET", new LambdaIntegration(dartAnalyzeV1, { proxy: true }))
  }

}
