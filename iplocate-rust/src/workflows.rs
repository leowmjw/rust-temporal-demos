use crate::types::{WorkflowInput, WorkflowOutput};
use anyhow::Result;
use temporal_sdk::{WfContext, WfExitValue, ActivityOptions, TimerOptions};
use tokio::time::Duration;
use tracing::info;

/// Workflow that retrieves IP address and location information
pub async fn get_address_from_ip(ctx: WfContext, input: WorkflowInput) -> Result<WfExitValue<WorkflowOutput>> {
    info!("Starting workflow for user: {}", input.name);

    // Execute GetIP activity
    let ip_result = ctx
        .activity(ActivityOptions {
            activity_type: "get_ip".to_string(),
            input: serde_json::to_vec(&())?.into(),
            start_to_close_timeout: Some(Duration::from_secs(60)),
            ..Default::default()
        })
        .await
        .unwrap_ok_payload();

    let ip: String = serde_json::from_slice(&ip_result.data)?;

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
            input: serde_json::to_vec(&ip)?.into(),
            start_to_close_timeout: Some(Duration::from_secs(60)),
            ..Default::default()
        })
        .await
        .unwrap_ok_payload();

    let location: String = serde_json::from_slice(&location_result.data)?;

    let output = WorkflowOutput {
        ip_addr: ip,
        location,
    };

    info!("Workflow completed successfully");
    Ok(WfExitValue::Normal(output))
}
