use std::sync::Arc;
use azure_identity::client_credentials_flow;
use log::{debug, info};
use reqwest::header::AUTHORIZATION;
use time::OffsetDateTime;
use crate::entities::{AzureAccessToken, AzureError, Error, JobExecutionStatusResponse, JobStartResponse, JobTemplate};

pub struct AzureContainerAppClient {
    client: Arc<reqwest::Client>,
    client_id: String,
    client_secret: String,
    tenant_id: String,
    subscription_id: String,
    resource_group: String,
    access_token: Option<AzureAccessToken>,
}

impl AzureContainerAppClient {
    pub fn new(
        client_id: String,
        client_secret: String,
        tenant_id: String,
        subscription_id: String,
        resource_group: String,
    ) -> Self {
        Self {
            client: Arc::new(reqwest::Client::new()),
            client_id,
            client_secret,
            tenant_id,
            subscription_id,
            resource_group,
            access_token: None,
        }
    }
    //
    //  Get access token
    //
    async fn get_access_token(&mut self) -> Result<AzureAccessToken, AzureError> {
        debug!("Get access token");
        if let Some(token) = self.access_token.as_ref() {
            debug!("Check access token expiry");
            if !token.expired_on.lt(&OffsetDateTime::now_utc()) {
                debug!("Reuse access token");
                return Ok(token.clone());
            }
        }
        debug!("Get new access token");
        let req = client_credentials_flow::perform(
            self.client.clone(),
            &self.client_id,
            &self.client_secret,
            &["https://management.azure.com/.default"],
            &self.tenant_id,
        )
            .await;
        match req {
            Ok(token) => {
                debug!("Login Response : {:#?}", token);

                let access_token = AzureAccessToken {
                    access_token: token.access_token.secret().to_string(),
                    expired_on: token.expires_on.unwrap_or(OffsetDateTime::now_utc()),
                };
                Ok(access_token)
            }
            Err(err) => {
                let error_message = err.to_string();
                let azure_error = AzureError {
                    error: Error {
                        code: Some("AzureError".to_string()),
                        inner_error: None,
                        message: Some(error_message),
                        target: None,
                        details: None,
                    },
                };
                Err(azure_error)
            }
        }
    }
    //
    // Start Job
    //
    pub async fn start_job(
        &mut self,
        job_name: &str,
        job_req: JobTemplate,
    ) -> Result<JobStartResponse, AzureError> {
        debug!("Start Job: {:?}", job_name);

        let token = self.get_access_token().await?;
        let subscription_id = &self.subscription_id;
        let resource_group = &self.resource_group;
        let start_job_url =
            format!("https://management.azure.com/subscriptions/{subscription_id}/resourceGroups/{resource_group}/providers/Microsoft.App/jobs/{job_name}/start?api-version=2024-03-01");

        debug!("Start Job URL: {:?}", start_job_url);

        let req = self
            .client
            .post(&start_job_url)
            .header(AUTHORIZATION, format!("Bearer {}", token.access_token))
            .json(&job_req)
            .build()
            .expect("Failed to build request");
        debug!("Request: {:#?}", req);
        let response = self.client.execute(req).await;
        match response {
            Ok(response) => {
                if response.status().is_success() {
                    let job_start_response = response.json::<JobStartResponse>().await.unwrap();
                    Ok(job_start_response)
                } else {
                    let err = response.json::<AzureError>().await.unwrap();
                    Err(err)
                }
            }
            Err(err) => {
                let error_message = err.to_string();
                let azure_error = AzureError {
                    error: Error {
                        code: Some("AzureError".to_string()),
                        inner_error: None,
                        message: Some(error_message),
                        target: None,
                        details: None,
                    },
                };
                Err(azure_error)
            }
        }
    }
    //
    // Get Job Execution Status
    //
    pub async fn get_job_execution_status(&mut self,job_execution:  &JobStartResponse) -> Result<JobExecutionStatusResponse, AzureError> {
        let token = self.get_access_token().await?;
        let url = format!(
            "https://management.azure.com{}?api-version=2024-03-01",job_execution.id);
        debug!("Get Job Status URL: {:?}", url);
        let req = self
            .client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token.access_token))
            .build()
            .expect("Failed to build request");
        let response = self.client.execute(req).await;
        return match response {
            Ok(response) => {
                if response.status().is_success() {
                    let job_status = response.json::<JobExecutionStatusResponse>().await.unwrap();
                    Ok(job_status)
                } else {
                    let err = response.json::<AzureError>().await.unwrap();
                    Err(err)
                }
            }
            Err(err) => {
                let error_message = err.to_string();
                let azure_error = AzureError {
                    error: Error {
                        code: Some("AzureError".to_string()),
                        inner_error: None,
                        message: Some(error_message),
                        target: None,
                        details: None,
                    },
                };
                Err(azure_error)
            }
        }
    }
}