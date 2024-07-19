use crate::apis::AzureContainerAppClient;
use crate::entities::{
    ContainerResources, EnvironmentVar, JobExecutionContainer, JobExecutionStatus, JobTemplate,
};
use log::{error, info};

mod apis;
mod entities;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    dotenv::dotenv().ok();
    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
    let tenant_id = std::env::var("TENANT_ID").expect("TENANT_ID must be set");
    let subscription_id = std::env::var("SUBSCRIPTION_ID").expect("SUBSCRIPTION_ID must be set");
    let resource_group = std::env::var("RESOURCE_GROUP").expect("RESOURCE_GROUP must be set");
    let application_insights_connection_string =
        std::env::var("APPLICATIONINSIGHTS_CONNECTION_STRING")
            .expect("APPLICATIONINSIGHTS_CONNECTION_STRING must be set");

    let mut client = AzureContainerAppClient::new(
        client_id,
        client_secret,
        tenant_id,
        subscription_id,
        resource_group,
    );

    // start job
    let start = client
        .start_job(
            "nick-aca-job-dev001",
            JobTemplate {
                containers: vec![JobExecutionContainer {
                    image: "nickmsft/batch_demo:latest".to_string(),
                    name: "my-job-batch-demo001".to_string(),
                    resources: ContainerResources {
                        cpu: 0.5,
                        ephemeral_storage: None,
                        memory: "1Gi".to_string(),
                    },
                    command: None,
                    args: None,
                    env: Some(vec![EnvironmentVar {
                        name: "APPLICATIONINSIGHTS_CONNECTION_STRING".to_string(),
                        secret_ref: None,
                        value: application_insights_connection_string.clone(),
                    }]),
                }],
                init_containers: vec![JobExecutionContainer {
                    image: "nickmsft/batch_demo:latest".to_string(),
                    name: "my-job-batch-demo002".to_string(),
                    resources: ContainerResources {
                        cpu: 0.5,
                        ephemeral_storage: None,
                        memory: "1Gi".to_string(),
                    },
                    command: None,
                    args: None,
                    env: Some(vec![EnvironmentVar {
                        name: "APPLICATIONINSIGHTS_CONNECTION_STRING".to_string(),
                        secret_ref: None,
                        value: application_insights_connection_string.clone(),
                    }]),
                }],
            },
        )
        .await;

    match start {
        Ok(job_start_response) => {
            info!("Job Start Response: {:#?}", job_start_response);
            //waiting for job status
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                let res = client.get_job_execution_status(&job_start_response).await;
                match res {
                    Ok(job_status) => {
                        if job_status.properties.status == JobExecutionStatus::Succeeded {
                            info!("Job Succeeded");
                            break;
                        } else if job_status.properties.status == JobExecutionStatus::Failed {
                            info!("Job Failed");
                            break;
                        } else {
                            info!("Job Running");
                        }
                    }
                    Err(err) => {
                        error!("Error: {:?}", err);
                        break;
                    }
                }
            }
        }
        Err(err) => {
            error!("Error: {:?}", err);
        }
    }
}
