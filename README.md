# azurecontainerapp-job-rs
This project use Azure REST API to start Azure Container App Job and get the job status.

## Prerequisites
- Create Application Registration in Azure AD and get the client id and client secret.
- Create Role Policy in Azure Container App and assign the Application Registration to the Role Policy.
```chatinput
{
  "Name": "CustomContainerAppsJobStarter",
  "IsCustom": true,
  "Description": "Custom role to start jobs in Azure Container Apps",
  "Actions": [
    "Microsoft.App/containerApps/read",
    "Microsoft.App/containerApps/write",
    "Microsoft.App/jobs/start/action" 
  ],
  "NotActions": [],
  "DataActions": [],
  "NotDataActions": [],
  "AssignableScopes": [
    "/subscriptions/<<subscription_id>>/resourceGroups/<<resource_group_name>>"
  ]
}
```
Create custom role definition in Azure AD
```chatinput
az role definition create --role-definition ./customRoleDefinition.json
```

```chatinput
az role assignment create --assignee abd73c26-934b-40b6-931a-44fba2cb6a47 --role CustomContainerAppsJobStarter --scope /subscriptions/<<subscription_id>>/resourceGroups/<<resource_group_name>>
```
## Example Code
```chatinput

    // Get the environment variables
    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
    let tenant_id = std::env::var("TENANT_ID").expect("TENANT_ID must be set");
    let subscription_id = std::env::var("SUBSCRIPTION_ID").expect("SUBSCRIPTION_ID must be set");
    let resource_group = std::env::var("RESOURCE_GROUP").expect("RESOURCE_GROUP must be set");
    let application_insights_connection_string =
        std::env::var("APPLICATIONINSIGHTS_CONNECTION_STRING")
            .expect("APPLICATIONINSIGHTS_CONNECTION_STRING must be set");
    
    // Create Azure Container App Client
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
             JobTemplate{
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
                let res = client.get_job_execution_status(
                    &job_start_response,
                ).await;
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
```

## Demo
[Watch the video](./demo.mov)