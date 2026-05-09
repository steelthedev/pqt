use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Request {
    Ping,
    Add { cmd: Vec<String>, cwd: PathBuf },
    List,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Response {
    Pong,
    Added { id: JobId },
    Jobs { jobs: Vec<JobSummary> },
    Error { message: String },
}

pub type JobId = u64;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum JobStatus {
    Queued,
    Running,
    Success,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobSummary {
    pub id: JobId,
    pub status: JobStatus,
    pub cmd: Vec<String>,
    pub cwd: PathBuf,
}
