use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone,PartialEq, Serialize, Deserialize)]
pub enum JobExecutionStatus {
    Running,
    Succeeded,
    Failed,
    Unknown,
}

impl JobExecutionStatus {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Running" => JobExecutionStatus::Running,
            "Succeeded" => JobExecutionStatus::Succeeded,
            "Failed" => JobExecutionStatus::Failed,
            _ => JobExecutionStatus::Unknown,
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            JobExecutionStatus::Running => "Running",
            JobExecutionStatus::Succeeded => "Succeeded",
            JobExecutionStatus::Failed => "Failed",
            JobExecutionStatus::Unknown => "Unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobTemplate {
    #[serde(rename = "containers")]
    pub containers: Vec<JobExecutionContainer>,
    #[serde(rename = "initContainers")]
    pub init_containers: Vec<JobExecutionContainer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobStartResponse {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobExecutionStatusResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "properties")]
    pub properties: JobExecutionStatusProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobExecutionStatusProperties {
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    #[serde(rename = "status")]
    pub status: JobExecutionStatus,
    #[serde(rename = "template")]
    pub template: JobTemplate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentVar {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "secretRef", skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<String>,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerResources {
    #[serde(rename = "cpu")]
    pub cpu: f64,
    #[serde(rename = "ephemeralStorage", skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<String>,
    #[serde(rename = "memory")]
    pub memory: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobExecutionContainer {
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "resources")]
    pub resources: ContainerResources,
    #[serde(rename = "command", skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(rename = "args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(rename = "env", skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<EnvironmentVar>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "code")]
    pub code: Option<String>,
    #[serde(rename = "innererror", skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<ErrorDetail>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureError {
    #[serde(rename = "error")]
    pub error: Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureAccessToken {
    pub(crate) access_token: String,
    pub(crate) expired_on: OffsetDateTime,
}

