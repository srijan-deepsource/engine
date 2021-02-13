extern crate test_utilities;

use std::fs::File;
use std::io::Read;

use test_utilities::digitalocean::DO_KUBERNETES_VERSION;
use tracing::{span, Level};

use qovery_engine::cloud_provider::digitalocean::kubernetes::DOKS;

use self::test_utilities::cloudflare::dns_provider_cloudflare;
use self::test_utilities::utilities;
use self::test_utilities::utilities::engine_run_test;
use qovery_engine::transaction::TransactionResult;

fn create_and_destroy_doks_cluster(region: &str, test_name: &str) {
    engine_run_test(|| {
        let span = span!(Level::INFO, "test", name = test_name);
        let _enter = span.enter();

        let context = test_utilities::utilities::context();

        let engine = test_utilities::digitalocean::docker_cr_do_engine(&context);
        let session = engine.session().unwrap();
        let mut tx = session.transaction();

        let digitalocean = test_utilities::digitalocean::cloud_provider_digitalocean(&context);
        let nodes = test_utilities::digitalocean::do_kubernetes_nodes();

        let cloudflare = dns_provider_cloudflare(&context);

        let mut file = File::open("tests/assets/do-options.json").unwrap();
        let mut read_buf = String::new();
        file.read_to_string(&mut read_buf).unwrap();

        let options_result =
            serde_json::from_str::<qovery_engine::cloud_provider::digitalocean::kubernetes::Options>(read_buf.as_str());

        let kubernetes = DOKS::new(
            context.clone(),
            utilities::generate_cluster_id(region).as_str(),
            utilities::generate_cluster_id(region).as_str(),
            DO_KUBERNETES_VERSION,
            region,
            &digitalocean,
            &cloudflare,
            options_result.expect("Oh my satan an error in test... Options options options"),
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

        // // TESTING: Kube cluster UUID is OK ?
        // let res_uuid = get_uuid_of_cluster_from_name(digital_ocean_token().as_str(), cluster_name.clone());
        // match res_uuid {
        //     Ok(uuid) => assert_eq!(get_kube_cluster_name_from_uuid(uuid.as_str()), cluster_name.clone()),
        //     Err(e) => {
        //         error!("{:?}", e.message);
        //         assert!(false);
        //     }
        // }

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

#[cfg(feature = "test-do-infra")]
#[test]
fn create_and_destroy_doks_cluster_in_fra1() {
    let region = "fra1";
    create_and_destroy_doks_cluster(
        region.clone(),
        &format!("create_and_destroy_doks_cluster_in_{}", region),
    );
}
