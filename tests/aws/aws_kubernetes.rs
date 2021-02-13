extern crate test_utilities;

use std::fs::File;
use std::io::Read;

use self::test_utilities::cloudflare::dns_provider_cloudflare;
use self::test_utilities::utilities::{context, engine_run_test, init};
use test_utilities::aws::AWS_KUBERNETES_VERSION;
use test_utilities::utilities;
use tracing::{span, Level};

use qovery_engine::cloud_provider::aws::kubernetes::EKS;
use qovery_engine::transaction::TransactionResult;
use utilities::generate_cluster_id;

pub const QOVERY_ENGINE_REPOSITORY_URL: &str = "CHANGE-ME";
pub const TMP_DESTINATION_GIT: &str = "/tmp/qovery-engine-main/";

fn create_and_destroy_eks_cluster(region: &str, test_name: &str) {
    engine_run_test(|| {
        init();

        let span = span!(Level::INFO, "test", name = test_name);
        let _enter = span.enter();

        let context = context();
        let engine = test_utilities::aws::docker_ecr_aws_engine(&context);
        let session = engine.session().unwrap();
        let mut tx = session.transaction();

        let aws = test_utilities::aws::cloud_provider_aws(&context);
        let nodes = test_utilities::aws::aws_kubernetes_nodes();

        let cloudflare = dns_provider_cloudflare(&context);

        let mut file = File::open("tests/assets/eks-options.json").unwrap();
        let mut read_buf = String::new();
        file.read_to_string(&mut read_buf).unwrap();

        let options_result =
            serde_json::from_str::<qovery_engine::cloud_provider::aws::kubernetes::Options>(read_buf.as_str());

        let kubernetes = EKS::new(
            context.clone(),
            generate_cluster_id(region).as_str(),
            generate_cluster_id(region).as_str(),
            AWS_KUBERNETES_VERSION,
            region,
            &aws,
            &cloudflare,
            options_result.expect("Oh my god an error in test... Options options options"),
            nodes,
        );

        match tx.create_kubernetes(&kubernetes) {
            Err(err) => panic!("{:?}", err),
            _ => {}
        }
        let _ = match tx.commit() {
            TransactionResult::Ok => assert!(true),
            TransactionResult::Rollback(_) => assert!(false),
            TransactionResult::UnrecoverableError(_, _) => assert!(false),
        };

        match tx.delete_kubernetes(&kubernetes) {
            Err(err) => panic!("{:?}", err),
            _ => {}
        }
        let _ = match tx.commit() {
            TransactionResult::Ok => assert!(true),
            TransactionResult::Rollback(_) => assert!(false),
            TransactionResult::UnrecoverableError(_, _) => assert!(false),
        };
        return test_name.to_string();
    })
}

/*
    TESTS NOTES:
    It is useful to keep 2 clusters deployment tests to run in // to validate there is no name collision (overlaping)
*/

#[cfg(feature = "test-aws-infra")]
#[test]
fn create_and_destroy_eks_cluster_in_eu_west_3() {
    let region = "eu-west-3";
    create_and_destroy_eks_cluster(
        region.clone(),
        &format!("create_and_destroy_eks_cluster_in_{}", region.replace("-", "_")),
    );
}

#[cfg(feature = "test-aws-infra")]
#[test]
fn create_and_destroy_eks_cluster_in_us_east_2() {
    let region = "us-east-2";
    create_and_destroy_eks_cluster(
        region.clone(),
        &format!("create_and_destroy_eks_cluster_in_{}", region.replace("-", "_")),
    );
}
