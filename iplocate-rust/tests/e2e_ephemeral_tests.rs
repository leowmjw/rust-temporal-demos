/// End-to-End Integration tests with ephemeral Temporal server
/// These tests are fully self-contained and spin up their own Temporal server
/// Perfect for CI/CD pipelines - no external dependencies needed!

use iplocate_rust::{
    activities::{get_ip, get_location_info},
    workflows::get_address_from_ip,
    WorkflowInput, WorkflowOutput,
};
use std::sync::Arc;
use std::time::Duration;
use temporal_client::{WfClientExt, WorkflowClientTrait, WorkflowOptions};
use temporal_sdk::{ActContext, ActivityError, Worker};
use temporal_sdk_core::{
    ephemeral_server::{TemporalDevServerConfigBuilder, default_cached_download},
    init_worker, ClientOptionsBuilder, CoreRuntime, Url, WorkerConfigBuilder,
};
use temporal_sdk_core_api::{
    telemetry::TelemetryOptionsBuilder, worker::WorkerVersioningStrategy,
};
use temporal_sdk_core_protos::coresdk::{AsJsonPayloadExt, FromJsonPayloadExt};

/// ✅ HAPPY PATH: Full end-to-end test with ephemeral server
#[tokio::test]
#[ignore] // Only run explicitly in CI with: cargo test --test e2e_ephemeral_tests -- --ignored
async fn e2e_iplocate_happy_path() {
    // 1. Start ephemeral Temporal server
    let server_config = TemporalDevServerConfigBuilder::default()
        .exe(default_cached_download())
        .build()
        .unwrap();

    let mut server = server_config.start_server().await.unwrap();
    println!("✅ Ephemeral Temporal server started on {}", server.target);

    // 2. Create client connected to ephemeral server
    let client = ClientOptionsBuilder::default()
        .identity("e2e-test-worker".to_string())
        .target_url(Url::parse(&format!("http://{}", server.target)).unwrap())
        .client_name("iplocate-e2e-test".to_string())
        .client_version("0.1.0".to_string())
        .build()
        .unwrap()
        .connect("default", None)
        .await
        .unwrap();

    // 3. Create worker
    let telemetry = TelemetryOptionsBuilder::default().build().unwrap();
    let runtime = CoreRuntime::new_assume_tokio(telemetry).unwrap();

    let task_queue = "e2e-test-happy";
    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue(task_queue)
        .versioning_strategy(WorkerVersioningStrategy::None {
            build_id: "e2e-test".to_owned(),
        })
        .build()
        .unwrap();

    let core_worker = init_worker(&runtime, worker_config, client.clone()).unwrap();
    let mut worker = Worker::new_from_core(Arc::new(core_worker), task_queue);

    // 4. Register workflow
    worker.register_wf(
        "get_address_from_ip",
        |ctx: temporal_sdk::WfContext| async move {
            let input = WorkflowInput::from_json_payload(
                ctx.get_args().first().expect("Missing input"),
            )
            .expect("Failed to deserialize");
            get_address_from_ip(ctx, input).await
        },
    );

    // 5. Register real activities
    worker.register_activity("get_ip", |ctx: ActContext, _: ()| async move {
        get_ip(ctx).await
    });
    worker.register_activity(
        "get_location_info",
        |ctx: ActContext, ip: String| async move { get_location_info(ctx, ip).await },
    );

    // 6. Start workflow
    let workflow_id = format!("e2e-happy-{}", uuid::Uuid::new_v4());
    let input = WorkflowInput {
        name: "E2E-Test".to_string(),
        seconds: 0,
    };

    let handle = client
        .start_workflow(
            vec![input.as_json_payload().unwrap()],
            task_queue.to_string(),
            workflow_id.clone(),
            "get_address_from_ip".to_string(),
            None,
            WorkflowOptions::default(),
        )
        .await
        .expect("Failed to start workflow");

    println!("✅ Workflow started: {}", workflow_id);

    // 7. Run worker and wait for result concurrently
    let worker_fut = worker.run();
    let wf_handle = client.get_untyped_workflow_handle(&workflow_id, handle.run_id.clone());
    let result_fut = wf_handle.get_workflow_result(Default::default());

    let result = tokio::select! {
        res = result_fut => res.expect("Failed to get workflow result"),
        _ = worker_fut => panic!("Worker stopped unexpectedly"),
    };

    // 8. Verify result
    match result {
        temporal_client::WorkflowExecutionResult::Succeeded(payloads) => {
            let output = WorkflowOutput::from_json_payload(payloads.first().unwrap()).unwrap();

            assert!(!output.ip_addr.is_empty(), "IP should not be empty");
            assert!(
                output.ip_addr.contains('.') || output.ip_addr.contains(':'),
                "IP should be valid format"
            );
            assert!(!output.location.is_empty(), "Location should not be empty");

            println!(
                "✅ E2E Happy Path Success!\n   IP: {}\n   Location: {}",
                output.ip_addr, output.location
            );
        }
        _ => panic!("Workflow should have succeeded"),
    }

    // 9. Cleanup: shutdown ephemeral server
    server.shutdown().await.unwrap();
    println!("✅ Ephemeral server shut down cleanly");
}

/// ❌ PERMANENT FAILURE: Test with ephemeral server
#[tokio::test]
#[ignore] // Only run explicitly in CI with: cargo test --test e2e_ephemeral_tests -- --ignored
async fn e2e_iplocate_permanent_failure() {
    // 1. Start ephemeral server
    let server_config = TemporalDevServerConfigBuilder::default()
        .exe(default_cached_download())
        .build()
        .unwrap();

    let mut server = server_config.start_server().await.unwrap();

    // 2. Create client
    let client = ClientOptionsBuilder::default()
        .identity("e2e-test-worker".to_string())
        .target_url(Url::parse(&format!("http://{}", server.target)).unwrap())
        .client_name("iplocate-e2e-test".to_string())
        .client_version("0.1.0".to_string())
        .build()
        .unwrap()
        .connect("default", None)
        .await
        .unwrap();

    // 3. Create worker
    let telemetry = TelemetryOptionsBuilder::default().build().unwrap();
    let runtime = CoreRuntime::new_assume_tokio(telemetry).unwrap();

    let task_queue = "e2e-test-failure";
    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue(task_queue)
        .versioning_strategy(WorkerVersioningStrategy::None {
            build_id: "e2e-test".to_owned(),
        })
        .build()
        .unwrap();

    let core_worker = init_worker(&runtime, worker_config, client.clone()).unwrap();
    let mut worker = Worker::new_from_core(Arc::new(core_worker), task_queue);

    // 4. Register workflow
    worker.register_wf(
        "get_address_from_ip",
        |ctx: temporal_sdk::WfContext| async move {
            let input = WorkflowInput::from_json_payload(
                ctx.get_args().first().expect("Missing input"),
            )
            .expect("Failed to deserialize");
            get_address_from_ip(ctx, input).await
        },
    );

    // 5. Register FAILING activity (permanent network error)
    worker.register_activity("get_ip", |_ctx: ActContext, _: ()| async move {
        Err::<String, _>(ActivityError::NonRetryable(anyhow::anyhow!(
            "WIFI_DISABLED: No network connection available"
        )))
    });

    worker.register_activity(
        "get_location_info",
        |ctx: ActContext, ip: String| async move { get_location_info(ctx, ip).await },
    );

    // 6. Start workflow
    let workflow_id = format!("e2e-failure-{}", uuid::Uuid::new_v4());
    let input = WorkflowInput {
        name: "FailureTest".to_string(),
        seconds: 0,
    };

    let handle = client
        .start_workflow(
            vec![input.as_json_payload().unwrap()],
            task_queue.to_string(),
            workflow_id.clone(),
            "get_address_from_ip".to_string(),
            None,
            WorkflowOptions::default(),
        )
        .await
        .expect("Failed to start workflow");

    // 7. Run worker and wait for result
    let worker_fut = worker.run();
    let wf_handle = client.get_untyped_workflow_handle(&workflow_id, handle.run_id.clone());
    let result_fut = wf_handle.get_workflow_result(Default::default());

    let result = tokio::select! {
        res = result_fut => res.expect("Failed to get workflow result"),
        _ = worker_fut => panic!("Worker stopped unexpectedly"),
    };

    // 8. Verify failure
    match result {
        temporal_client::WorkflowExecutionResult::Failed(failure) => {
            println!("✅ E2E Permanent Failure Test Passed!");
            println!("   Workflow failed as expected: {}", failure.message);
            assert!(
                failure.message.contains("WIFI_DISABLED")
                    || failure.message.contains("No network")
                    || failure.message.contains("Activity task failed"),
                "Failure message should indicate network error"
            );
        }
        _ => panic!("Workflow should have failed"),
    }

    // 9. Cleanup
    server.shutdown().await.unwrap();
}

/// 🔄 RETRYABLE FAILURE: Test retry logic with ephemeral server
#[tokio::test]
#[ignore] // Only run explicitly in CI with: cargo test --test e2e_ephemeral_tests -- --ignored
async fn e2e_iplocate_retryable_success() {
    // 1. Start ephemeral server
    let server_config = TemporalDevServerConfigBuilder::default()
        .exe(default_cached_download())
        .build()
        .unwrap();

    let mut server = server_config.start_server().await.unwrap();

    // 2. Create client
    let client = ClientOptionsBuilder::default()
        .identity("e2e-test-worker".to_string())
        .target_url(Url::parse(&format!("http://{}", server.target)).unwrap())
        .client_name("iplocate-e2e-test".to_string())
        .client_version("0.1.0".to_string())
        .build()
        .unwrap()
        .connect("default", None)
        .await
        .unwrap();

    // 3. Create worker
    let telemetry = TelemetryOptionsBuilder::default().build().unwrap();
    let runtime = CoreRuntime::new_assume_tokio(telemetry).unwrap();

    let task_queue = "e2e-test-retry";
    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue(task_queue)
        .versioning_strategy(WorkerVersioningStrategy::None {
            build_id: "e2e-test".to_owned(),
        })
        .build()
        .unwrap();

    let core_worker = init_worker(&runtime, worker_config, client.clone()).unwrap();
    let mut worker = Worker::new_from_core(Arc::new(core_worker), task_queue);

    // 4. Track retries
    let retry_count = Arc::new(std::sync::atomic::AtomicU32::new(0));
    let retry_count_clone = retry_count.clone();

    // 5. Register workflow
    worker.register_wf(
        "get_address_from_ip",
        |ctx: temporal_sdk::WfContext| async move {
            let input = WorkflowInput::from_json_payload(
                ctx.get_args().first().expect("Missing input"),
            )
            .expect("Failed to deserialize");
            get_address_from_ip(ctx, input).await
        },
    );

    // 6. Register activity that fails twice then succeeds
    worker.register_activity("get_ip", move |_ctx: ActContext, _: ()| {
        let count = retry_count_clone.clone();
        async move {
            let attempt = count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if attempt < 2 {
                println!("   Attempt {}: Simulating transient network failure...", attempt + 1);
                Err::<String, _>(ActivityError::Retryable {
                    source: anyhow::anyhow!("Temporary DNS resolution failure"),
                    explicit_delay: Some(Duration::from_millis(100)),
                })
            } else {
                println!("   Attempt {}: Success!", attempt + 1);
                Ok("203.0.113.42".to_string())
            }
        }
    });

    worker.register_activity(
        "get_location_info",
        |_ctx: ActContext, _ip: String| async move {
            Ok("Mountain View, California, United States".to_string())
        },
    );

    // 7. Start workflow
    let workflow_id = format!("e2e-retry-{}", uuid::Uuid::new_v4());
    let input = WorkflowInput {
        name: "RetryTest".to_string(),
        seconds: 0,
    };

    let handle = client
        .start_workflow(
            vec![input.as_json_payload().unwrap()],
            task_queue.to_string(),
            workflow_id.clone(),
            "get_address_from_ip".to_string(),
            None,
            WorkflowOptions::default(),
        )
        .await
        .expect("Failed to start workflow");

    // 8. Run worker and wait for result
    let worker_fut = worker.run();
    let wf_handle = client.get_untyped_workflow_handle(&workflow_id, handle.run_id.clone());
    let result_fut = wf_handle.get_workflow_result(Default::default());

    let result = tokio::select! {
        res = result_fut => res.expect("Failed to get workflow result"),
        _ = worker_fut => panic!("Worker stopped unexpectedly"),
    };

    // 9. Verify success after retries
    match result {
        temporal_client::WorkflowExecutionResult::Succeeded(payloads) => {
            let output = WorkflowOutput::from_json_payload(payloads.first().unwrap()).unwrap();

            assert_eq!(output.ip_addr, "203.0.113.42");
            assert_eq!(output.location, "Mountain View, California, United States");

            let attempts = retry_count.load(std::sync::atomic::Ordering::SeqCst);
            assert!(attempts >= 3, "Should have at least 3 attempts, got {}", attempts);

            println!("✅ E2E Retry Test Success!");
            println!("   Total attempts: {}", attempts);
            println!("   IP: {}", output.ip_addr);
            println!("   Location: {}", output.location);
        }
        _ => panic!("Workflow should have succeeded after retries"),
    }

    // 10. Cleanup
    server.shutdown().await.unwrap();
}
