use crate::types::messages::FragmentRequest;

pub trait FragmentRequestOperation {
    fn new(worker_name: String, maximal_worker_load: u32) -> Self;

    fn serialize(&self) -> Result<String, serde_json::Error>;

    fn deserialize(message: &str) -> Result<Self, serde_json::Error> where Self: Sized;
}

impl FragmentRequestOperation for FragmentRequest {
    fn new(worker_name: String, maximal_work_load: u32) -> Self {
        Self {
            worker_name,
            maximal_work_load,
        }
    }

    fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    fn deserialize(message: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(message)
    }
}
