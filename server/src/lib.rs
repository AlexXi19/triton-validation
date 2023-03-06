pub mod RequestBody {
    use std::str::FromStr;

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct LoadTestConfig {
        pub model_name: String,
        pub model_version: String,
        pub concurrency: u32,
        pub duration: u32,
        pub request_rate: u32,
        pub request_size: u32,
        pub response_size: u32,
    }

    #[derive(Deserialize)]
    pub struct CreateJobBody {
        pub name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct JobStatusBody {
        pub name: Option<String>,
        pub id: String,
        pub status: String,
    }

    pub enum JobStatus {
        Completed,
        Failed,
    }
    impl FromStr for JobStatus {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                "completed" => Ok(JobStatus::Completed),
                "failed" => Ok(JobStatus::Failed),
                _ => Err(format!("invalid job status: {}", s)),
            }
        }
    }

    impl ToString for JobStatus {
        fn to_string(&self) -> String {
            match self {
                JobStatus::Completed => "completed".to_string(),
                JobStatus::Failed => "failed".to_string(),
            }
        }
    }
}
