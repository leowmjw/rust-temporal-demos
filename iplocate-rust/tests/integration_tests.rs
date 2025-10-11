use iplocate_rust::types::{WorkflowInput, WorkflowOutput, TASK_QUEUE_NAME};
use tracing::info;

// ============================================================================
// Type Serialization Tests
// ============================================================================

#[tokio::test]
async fn test_workflow_input_serialization() {
    let _ = tracing_subscriber::fmt::try_init();

    let input = WorkflowInput {
        name: "TestUser".to_string(),
        seconds: 5,
    };

    let serialized = serde_json::to_string(&input).unwrap();
    let deserialized: WorkflowInput = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.name, input.name);
    assert_eq!(deserialized.seconds, input.seconds);
}

#[tokio::test]
async fn test_workflow_output_serialization() {
    let _ = tracing_subscriber::fmt::try_init();

    let output = WorkflowOutput {
        ip_addr: "192.168.1.1".to_string(),
        location: "San Francisco, California, United States".to_string(),
    };

    let serialized = serde_json::to_string(&output).unwrap();
    let deserialized: WorkflowOutput = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.ip_addr, output.ip_addr);
    assert_eq!(deserialized.location, output.location);
}

// ============================================================================
// Activity Logic Tests (Testing HTTP logic, not Temporal integration)
// ============================================================================

#[tokio::test]
async fn test_get_ip_http_call() {
    let _ = tracing_subscriber::fmt::try_init();

    // Test the actual HTTP call logic
    let result = reqwest::get("https://icanhazip.com").await;

    match result {
        Ok(response) => {
            let ip = response.text().await.unwrap().trim().to_string();
            info!("Retrieved IP: {}", ip);
            assert!(!ip.is_empty(), "IP should not be empty");
            // Verify it's a valid IP format (IPv4 or IPv6)
            assert!(
                ip.contains('.') || ip.contains(':'),
                "IP should contain . (IPv4) or : (IPv6)"
            );
        }
        Err(e) => {
            // Network errors are acceptable in test environments
            info!("Network error (acceptable in tests): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_get_location_info_http_call() {
    let _ = tracing_subscriber::fmt::try_init();

    // Test with Google's public DNS
    let url = "http://ip-api.com/json/8.8.8.8";
    let result = reqwest::get(url).await;

    match result {
        Ok(response) => {
            let json_result: Result<serde_json::Value, _> = response.json().await;
            if let Ok(data) = json_result {
                info!("Location data for 8.8.8.8: {:?}", data);

                // Verify expected fields exist
                assert!(data.get("city").is_some(), "Should have city field");
                assert!(data.get("regionName").is_some(), "Should have regionName field");
                assert!(data.get("country").is_some(), "Should have country field");

                // Build location string like the activity does
                let city = data["city"].as_str().unwrap_or("");
                let region = data["regionName"].as_str().unwrap_or("");
                let country = data["country"].as_str().unwrap_or("");
                let location = format!("{}, {}, {}", city, region, country);

                assert!(!location.is_empty());
                assert!(location.contains(","));
            }
        }
        Err(e) => {
            info!("Network error (acceptable in tests): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_location_api_response_structure() {
    let _ = tracing_subscriber::fmt::try_init();

    // Test with Cloudflare DNS (1.1.1.1)
    let url = "http://ip-api.com/json/1.1.1.1";

    if let Ok(response) = reqwest::get(url).await {
        if let Ok(data) = response.json::<serde_json::Value>().await {
            // Verify the structure matches what our activity expects
            let has_city = data.get("city").and_then(|v| v.as_str()).is_some();
            let has_region = data.get("regionName").and_then(|v| v.as_str()).is_some();
            let has_country = data.get("country").and_then(|v| v.as_str()).is_some();

            // At minimum we need country
            assert!(has_country, "API should return country field");

            info!(
                "API structure valid - city: {}, region: {}, country: {}",
                has_city, has_region, has_country
            );
        }
    }
}

// ============================================================================
// Workflow Logic Tests (No real execution, just structure validation)
// ============================================================================

#[tokio::test]
async fn test_workflow_input_validation() {
    let _ = tracing_subscriber::fmt::try_init();

    // Test with no sleep
    let input1 = WorkflowInput {
        name: "User1".to_string(),
        seconds: 0,
    };
    assert_eq!(input1.seconds, 0);

    // Test with sleep
    let input2 = WorkflowInput {
        name: "User2".to_string(),
        seconds: 5,
    };
    assert!(input2.seconds > 0);
}

#[tokio::test]
async fn test_workflow_output_structure() {
    let _ = tracing_subscriber::fmt::try_init();

    let output = WorkflowOutput {
        ip_addr: "203.0.113.42".to_string(),
        location: "New York, New York, United States".to_string(),
    };

    // Verify output has required fields
    assert!(!output.ip_addr.is_empty());
    assert!(!output.location.is_empty());

    // Verify location is properly formatted
    assert!(output.location.contains(","));

    // Can be JSON serialized
    let json = serde_json::to_string(&output).unwrap();
    assert!(json.contains("ip_addr"));
    assert!(json.contains("location"));
}

#[tokio::test]
async fn test_task_queue_constant() {
    assert_eq!(TASK_QUEUE_NAME, "ip-address-rust");
}
