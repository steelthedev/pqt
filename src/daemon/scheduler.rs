use std::{collections::VecDeque, path::PathBuf};

use crate::{JobId, JobStatus, JobSummary};

#[derive(Default)]
pub struct Queue {
    next_id: JobId,
    jobs: VecDeque<JobSummary>,
}

impl Queue {
    pub fn add(&mut self, cmd: Vec<String>, cwd: PathBuf) -> JobId {
        self.next_id += 1;
        let id = self.next_id;
        self.jobs.push_back(JobSummary {
            id,
            status: JobStatus::Queued,
            cmd,
            cwd,
        });
        id
    }

    pub fn list(&self) -> Vec<JobSummary> {
        self.jobs.iter().cloned().collect()
    }
}
