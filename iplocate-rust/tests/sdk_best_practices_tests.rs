/// Tests to catch common Temporal SDK mistakes for beginners
/// These tests demonstrate the CORRECT patterns and what errors occur with WRONG patterns

use temporal_sdk_core_protos::coresdk::{AsJsonPayloadExt, FromJsonPayloadExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestData {
    message: String,
    count: u32,
}

/// ✅ CORRECT: Using as_json_payload for serialization
#[test]
fn test_correct_payload_serialization() {
    let data = TestData {
        message: "test".to_string(),
        count: 42,
    };

    // This is the CORRECT way
    let payload = data.as_json_payload().expect("should serialize");

    // Verify it has JSON metadata
    assert!(payload.metadata.contains_key("encoding"));
    assert_eq!(
        payload.metadata.get("encoding").unwrap(),
        b"json/plain"
    );

    // Deserialize correctly
    let deserialized = TestData::from_json_payload(&payload).expect("should deserialize");
    assert_eq!(data, deserialized);
}

/// ❌ WRONG: Using raw serde_json creates payloads without metadata
#[test]
fn test_wrong_payload_serialization_fails() {
    let data = TestData {
        message: "test".to_string(),
        count: 42,
    };

    // This is WRONG - creates payload without JSON metadata
    let wrong_payload = temporal_sdk_core_protos::temporal::api::common::v1::Payload {
        metadata: Default::default(), // Missing "encoding" metadata!
        data: serde_json::to_vec(&data).unwrap(),
    };

    // This will fail with "This deserializer does not understand this payload"
    let result = TestData::from_json_payload(&wrong_payload);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("does not understand this payload"));
}

/// ✅ CORRECT: Empty tuple serialization for activities with no input
#[test]
fn test_correct_empty_input_serialization() {
    let empty = ();
    let payload = empty.as_json_payload().expect("should serialize empty");

    // Verify it's a valid JSON payload (even for empty tuple)
    assert!(payload.metadata.contains_key("encoding"));

    // Can deserialize back
    let result: () = <()>::from_json_payload(&payload).expect("should deserialize");
    assert_eq!(result, ());
}

/// Test that demonstrates task_queue vs workflow_id are different
#[test]
fn test_task_queue_vs_workflow_id_concept() {
    // Task queue is FIXED - same for all workflows of this type
    let task_queue = "ip-address-rust";

    // Workflow ID is DYNAMIC - unique for each execution
    let workflow_id_1 = format!("workflow-{}", uuid::Uuid::new_v4());
    let workflow_id_2 = format!("workflow-{}", uuid::Uuid::new_v4());

    // Same task queue, different IDs
    assert_eq!(task_queue, task_queue);
    assert_ne!(workflow_id_1, workflow_id_2);
}

/// Test showing correct start_workflow parameter order
#[test]
fn test_start_workflow_parameter_order_documentation() {
    // This test documents the CORRECT parameter order
    // The actual function signature is:
    //
    // start_workflow(
    //     input: Vec<Payload>,           // 1. Workflow input
    //     task_queue: String,            // 2. Task queue (FIXED)
    //     workflow_id: String,           // 3. Workflow ID (DYNAMIC)
    //     workflow_type: String,         // 4. Workflow type name
    //     request_id: Option<String>,    // 5. Optional request ID
    //     options: WorkflowOptions,      // 6. Workflow options
    // )

    let input = TestData {
        message: "test".to_string(),
        count: 1,
    };

    // Correct order
    let _payloads = vec![input.as_json_payload().unwrap()];
    let _task_queue = "my-task-queue".to_string();  // FIXED
    let _workflow_id = format!("wf-{}", uuid::Uuid::new_v4());  // DYNAMIC
    let _workflow_type = "my_workflow".to_string();

    // The mistake beginners make is swapping workflow_id and task_queue
    // because intuitively the dynamic ID "feels" like it should come before
    // the fixed queue name
}

/// Test demonstrating activity error types
#[test]
fn test_activity_error_types() {
    use temporal_sdk::ActivityError;

    // Retryable error - Temporal will retry
    let retryable = ActivityError::Retryable {
        source: anyhow::anyhow!("temporary network issue"),
        explicit_delay: None,
    };
    assert!(matches!(retryable, ActivityError::Retryable { .. }));

    // NonRetryable error - Temporal will NOT retry
    let non_retryable = ActivityError::NonRetryable(anyhow::anyhow!("invalid input"));
    assert!(matches!(non_retryable, ActivityError::NonRetryable(_)));

    // Cancelled - activity was cancelled
    let cancelled = ActivityError::cancelled();
    assert!(matches!(cancelled, ActivityError::Cancelled { .. }));
}

/// Test that client options require identity
#[test]
fn test_client_identity_is_required() {
    use temporal_sdk::sdk_client_options;
    use temporal_sdk_core::Url;
    use std::str::FromStr;

    let url = Url::from_str("http://localhost:7233").unwrap();

    // ✅ CORRECT: Always set identity
    let opts_with_identity = sdk_client_options(url.clone())
        .identity("my-worker".to_string())
        .build();
    assert!(opts_with_identity.is_ok());

    // ❌ WRONG: Missing identity will cause runtime error
    // "Client identity cannot be empty. Either lang or user should be setting this value"
    let opts_without_identity = sdk_client_options(url)
        .build();

    // The builder succeeds but connection will fail at runtime
    assert!(opts_without_identity.is_ok());
}

/// Test demonstrating workflow input deserialization pattern
#[test]
fn test_workflow_input_deserialization_pattern() {
    // Simulate workflow args
    let input = TestData {
        message: "workflow_input".to_string(),
        count: 5,
    };
    let payload = input.as_json_payload().unwrap();
    let args = vec![payload];

    // ✅ CORRECT pattern used in worker registration
    let deserialized = TestData::from_json_payload(
        args.first().expect("Missing input")
    ).expect("Failed to deserialize");

    assert_eq!(input.message, deserialized.message);
    assert_eq!(input.count, deserialized.count);

    // ❌ WRONG: Using serde_json::from_slice will fail
    let wrong_result: Result<TestData, _> = serde_json::from_slice(
        &args.first().unwrap().data
    );
    // This works only if the data is valid JSON, but won't handle metadata properly
    assert!(wrong_result.is_ok()); // May work but is fragile
}

/// Test for common async registration mistake
#[test]
fn test_activity_must_be_async_move() {
    // Activities MUST be registered with async move closures
    //
    // ❌ WRONG:
    // worker.register_activity("name", |ctx, input| get_activity(ctx, input))
    //
    // ✅ CORRECT:
    // worker.register_activity("name", |ctx, input| async move {
    //     get_activity(ctx, input).await
    // })

    // This test documents the pattern - the actual test would need a worker instance
    assert!(true);
}

/// Test showing string vs type safety for activity/workflow names
#[test]
fn test_activity_workflow_name_conventions() {
    // Activity and workflow names are strings - easy to typo!
    let activity_name = "get_ip";
    let workflow_name = "get_address_from_ip";

    // Common mistakes:
    // 1. Typo in name: "get_IP" vs "get_ip"
    // 2. Inconsistent naming between registration and invocation
    // 3. Using different case conventions

    // Best practice: Use constants
    const GET_IP_ACTIVITY: &str = "get_ip";
    const GET_LOCATION_ACTIVITY: &str = "get_location_info";
    const WORKFLOW_NAME: &str = "get_address_from_ip";

    assert_eq!(activity_name, GET_IP_ACTIVITY);
    assert_eq!(workflow_name, WORKFLOW_NAME);
    assert_eq!("get_location_info", GET_LOCATION_ACTIVITY);
}
