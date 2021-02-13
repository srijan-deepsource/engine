use chrono::offset::utc::Utc;
use core::option::Option::{None, Some};
use qovery_engine::models::{Action, Application, Context, Environment, GitCredentials, Kind, Route, Router};
use test_utilities::aws::ORGANIZATION_ID;
use test_utilities::utilities::generate_id;

pub fn working_minimal_environment(context: &Context) -> Environment {
    let suffix = generate_id();
    Environment {
        execution_id: context.execution_id().to_string(),
        id: generate_id(),
        kind: Kind::Development,
        owner_id: generate_id(),
        project_id: generate_id(),
        organization_id: ORGANIZATION_ID.to_string(),
        action: Action::Create,
        applications: vec![Application {
            id: generate_id(),
            name: format!("{}-{}", "simple-app".to_string(), &suffix),
            /*name: "simple-app".to_string(),*/
            git_url: "https://github.com/Qovery/engine-testing.git".to_string(),
            commit_id: "fc575a2f3be0b9100492c8a463bf18134a8698a5".to_string(),
            dockerfile_path: Some("Dockerfile".to_string()),
            action: Action::Create,
            git_credentials: Some(GitCredentials {
                login: "x-access-token".to_string(),
                access_token: "xxx".to_string(),
                expired_at: Utc::now(),
            }),
            storage: vec![],
            environment_variables: vec![],
            branch: "basic-app-deploy".to_string(),
            private_port: Some(80),
            total_cpus: "100m".to_string(),
            total_ram_in_mib: 256,
            total_instances: 2,
            cpu_burst: "100m".to_string(),
            start_timeout_in_seconds: 60,
        }],
        routers: vec![Router {
            id: generate_id(),
            name: "main".to_string(),
            action: Action::Create,
            default_domain: generate_id() + ".oom.sh",
            public_port: 443,
            custom_domains: vec![],
            routes: vec![Route {
                path: "/".to_string(),
                application_name: format!("{}-{}", "simple-app".to_string(), &suffix),
            }],
        }],
        databases: vec![],
        external_services: vec![],
        clone_from_environment_id: None,
    }
}
