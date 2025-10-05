use crate::types::{WorkflowInput, WorkflowOutput};
use anyhow::Result;
use temporal_sdk::{WfContext, WfExitValue, ActivityOptions, TimerOptions};
use temporal_sdk_core_protos::coresdk::{AsJsonPayloadExt, FromJsonPayloadExt};
use tokio::time::Duration;
use tracing::info;

/// Workflow that retrieves IP address and location information
pub async fn get_address_from_ip(ctx: WfContext, input: WorkflowInput) -> Result<WfExitValue<WorkflowOutput>> {
    info!("Starting workflow for user .. cool! run tests: {}", input.name);

    // Execute GetIP activity
    let ip_result = ctx
        .activity(ActivityOptions {
            activity_type: "get_ip".to_string(),
            input: ().as_json_payload()?,
            start_to_close_timeout: Some(Duration::from_secs(60)),
            ..Default::default()
        })
        .await;

    // Handle activity result - propagate failures properly
    let ip_payload = ip_result
        .success_payload_or_error()?
        .ok_or_else(|| anyhow::anyhow!("get_ip returned no payload"))?;

    let ip = String::from_json_payload(&ip_payload)?;

    // Sleep if requested
    if input.seconds > 0 {
        info!("Sleeping for {} seconds", input.seconds);
        ctx.timer(TimerOptions {
            duration: Duration::from_secs(input.seconds),
            summary: None,
        }).await;
    }

    // Execute GetLocationInfo activity
    let location_result = ctx
        .activity(ActivityOptions {
            activity_type: "get_location_info".to_string(),
            input: ip.as_json_payload()?,
            start_to_close_timeout: Some(Duration::from_secs(60)),
            ..Default::default()
        })
        .await;

    // Handle activity result - propagate failures properly
    let location_payload = location_result
        .success_payload_or_error()?
        .ok_or_else(|| anyhow::anyhow!("get_location_info returned no payload"))?;

    let location = String::from_json_payload(&location_payload)?;

    let output = WorkflowOutput {
        ip_addr: ip,
        location,
    };

    info!("Workflow completed successfully");
    Ok(WfExitValue::Normal(output))
}
