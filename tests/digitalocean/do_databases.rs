use test_utilities::utilities::{context, engine_run_test, generate_id};
use tracing::{span, Level};

use qovery_engine::models::{
    Action, Clone2, Context, Database, DatabaseKind, Environment, EnvironmentAction, EnvironmentVariable, Kind,
};
use qovery_engine::transaction::TransactionResult;
use ::function_name::named;
use crate::digitalocean::do_environment::{deploy_environment, delete_environment};

//TODO: Do you wanna play a game ?
// fn deploy_one_postgresql() {
//     engine_run_test(|| {
//         let span = span!(Level::INFO, "deploy_one_postgresql");
//         let _enter = span.enter();
//
//         let context = context();
//         let context_for_deletion = context.clone_not_same_execution_id();
//
//         let environment = test_utilities::aws::working_minimal_environment(&context);
//
//         let mut environment_delete = environment.clone();
//         environment_delete.action = Action::Delete;
//         let ea = EnvironmentAction::Environment(environment);
//         let ea_delete = EnvironmentAction::Environment(environment_delete);
//
//         match deploy_environment(&context, &ea) {
//             TransactionResult::Ok => assert!(true),
//             TransactionResult::Rollback(_) => assert!(false),
//             TransactionResult::UnrecoverableError(_, _) => assert!(false),
//         };
//
//         match deploy_environment(&context_for_deletion, &ea_delete) {
//             TransactionResult::Ok => assert!(true),
//             TransactionResult::Rollback(_) => assert!(false),
//             TransactionResult::UnrecoverableError(_, _) => assert!(false),
//         };
//         return "deploy_one_postgresql".to_string();
//     })
// }

/**
**
** PostgreSQL tests
**
**/

fn test_postgresql_configuration(context: Context, mut environment: Environment, version: &str, test_name: &str) {
    engine_run_test(|| {
        let span = span!(Level::INFO, "test", name = test_name);
        let _enter = span.enter();
        let context_for_delete = context.clone_not_same_execution_id();

        let app_name = format!("postgresql-app-{}", generate_id());
        let database_host = "postgres-".to_string() + generate_id().as_str() + ".CHANGE-ME/DEFAULT_TEST_DOMAIN";
        let database_port = 5432;
        let database_db_name = "postgres".to_string();
        let database_username = "superuser".to_string();
        let database_password = generate_id();

        let _is_rds = match environment.kind {
            Kind::Production => true,
            Kind::Development => false,
        };

        environment.databases = vec![Database {
            kind: DatabaseKind::Postgresql,
            action: Action::Create,
            id: generate_id(),
            name: database_db_name.clone(),
            version: version.to_string(),
            fqdn_id: "postgresql-".to_string() + generate_id().as_str(),
            fqdn: database_host.clone(),
            port: database_port.clone(),
            username: database_username.clone(),
            password: database_password.clone(),
            total_cpus: "100m".to_string(),
            total_ram_in_mib: 512,
            disk_size_in_gib: 10,
            database_instance_type: "db.t2.micro".to_string(),
            database_disk_type: "gp2".to_string(),
        }];
        environment.applications = environment
            .applications
            .into_iter()
            .map(|mut app| {
                app.branch = app_name.clone();
                app.commit_id = "ad65b24a0470e7e8aa0983e036fb9a05928fd973".to_string();
                app.private_port = Some(1234);
                app.dockerfile_path = Some(format!("Dockerfile-{}", version));
                app.environment_variables = vec![
                    EnvironmentVariable {
                        key: "PG_HOST".to_string(),
                        value: database_host.clone(),
                    },
                    EnvironmentVariable {
                        key: "PG_PORT".to_string(),
                        value: database_port.clone().to_string(),
                    },
                    EnvironmentVariable {
                        key: "PG_DBNAME".to_string(),
                        value: database_db_name.clone(),
                    },
                    EnvironmentVariable {
                        key: "PG_USERNAME".to_string(),
                        value: database_username.clone(),
                    },
                    EnvironmentVariable {
                        key: "PG_PASSWORD".to_string(),
                        value: database_password.clone(),
                    },
                ];
                app
            })
            .collect::<Vec<qovery_engine::models::Application>>();
        environment.routers[0].routes[0].application_name = app_name.clone();

        let mut environment_delete = environment.clone();
        environment_delete.action = Action::Delete;
        let ea = EnvironmentAction::Environment(environment);
        let ea_delete = EnvironmentAction::Environment(environment_delete);

        match deploy_environment(&context, &ea) {
            TransactionResult::Ok => assert!(true),
            TransactionResult::Rollback(_) => assert!(false),
            TransactionResult::UnrecoverableError(_, _) => assert!(false),
        };

        // todo: check the database disk is here and with correct size

        match delete_environment(&context_for_delete, &ea_delete) {
            TransactionResult::Ok => assert!(true),
            TransactionResult::Rollback(_) => assert!(false),
            TransactionResult::UnrecoverableError(_, _) => assert!(true),
        };
        return test_name.to_string();
    })
}

// Postgres environment environment
#[named]
#[cfg(feature = "test-do-self-hosted")]
#[test]
fn do_postgresql_v10_deploy_a_working_dev_environment() {
    let context = context();
    let environment = test_utilities::aws::working_minimal_environment(&context);
    test_postgresql_configuration(
        context,
        environment,
        "10",
        function_name!(),
    );
}

#[named]
#[cfg(feature = "test-do-self-hosted")]
#[test]
fn do_postgresql_v11_deploy_a_working_dev_environment() {
    let context = context();
    let environment = test_utilities::aws::working_minimal_environment(&context);
    test_postgresql_configuration(
        context,
        environment,
        "11",
        function_name!(),
    );
}

#[named]
#[cfg(feature = "test-do-self-hosted")]
#[test]
fn do_postgresql_v12_deploy_a_working_dev_environment() {
    let context = context();
    let environment = test_utilities::aws::working_minimal_environment(&context);
    test_postgresql_configuration(
        context,
        environment,
        "12",
        function_name!(),
    );
}
