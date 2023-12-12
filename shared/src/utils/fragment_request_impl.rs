use crate::types::messages::FragmentRequest;

pub trait FragmentRequestOperation {
    fn new(worker_name: String, maximal_worker_load: u32) -> Self;

    /// Serializes a `FragmentRequest` into a JSON string.
    ///
    /// # Arguments
    ///
    /// * `request` - A reference to the `FragmentRequest` to be serialized.
    ///
    /// # Returns
    ///
    /// A `Result<String, serde_json::Error>` which is Ok containing the JSON string if serialization is successful,
    /// or an Err containing the `serde_json::Error` if it fails.
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
        let fragment_json = serde_json::json!({"FragmentRequest": &self});
        serde_json::to_string(&fragment_json)
    }

    fn deserialize(message: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(message)
    }
}

#[cfg(test)]
mod iterated_sinz_tests {
    use crate::types::messages::FragmentRequest;
    use crate::utils::fragment_request_impl::FragmentRequestOperation;

    #[test]
    fn test_new_has_correct_values() {
        let fragment_request = FragmentRequest::new("test".to_string(), 100);
        assert_eq!(fragment_request.worker_name, "test");
        assert_eq!(fragment_request.maximal_work_load, 100);
    }

    #[test]
    fn test_serialize_returns_json_in_string() {
        let fragment_request = FragmentRequest::new("fractal painter".to_string(), 1000);
        match fragment_request.serialize() {
            Ok(serialized) => {
                assert_eq!(serialized, "{\"FragmentRequest\":{\"maximal_work_load\":1000,\"worker_name\":\"fractal painter\"}}");
            }
            Err(_) => {}
        }
    }

    #[test]
    fn test_deserialized_returns_fragment_request() {
        let serialized = "{\"FragmentRequest\":{\"maximal_work_load\":1000,\"worker_name\":\"fractal painter\"}}";
        match FragmentRequest::deserialize(serialized) {
            Ok(deserialized) => {
                assert_eq!(deserialized, FragmentRequest::new("fractal painter".to_string(), 1000));
            }
            Err(_) => {}
        }
    }
}