use crate::types::messages::FragmentRequest;

pub trait FragmentRequestOperation {
    fn new(worker_name: String, maximal_worker_load: u32) -> Self;

    fn serialize(&self) -> Result<String, serde_json::Error>;
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
}
