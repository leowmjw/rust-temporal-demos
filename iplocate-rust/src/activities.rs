use anyhow::Result;
use serde::{Deserialize, Serialize};
use temporal_sdk::{ActContext, ActivityError};
use tracing::info;

/// Activity to fetch the public IP address
pub async fn get_ip(_ctx: ActContext) -> Result<String, ActivityError> {
    info!("Fetching public IP address");

    let response = reqwest::get("https://icanhazip.com")
        .await
        .map_err(|e| ActivityError::NonRetryable(e.into()))?;

    let ip = response
        .text()
        .await
        .map_err(|e| ActivityError::NonRetryable(e.into()))?
        .trim()
        .to_string();

    info!("Retrieved IP address: {}", ip);
    Ok(ip)
}

/// Location information from the IP API
#[derive(Debug, Deserialize, Serialize)]
struct LocationData {
    city: String,
    #[serde(rename = "regionName")]
    region_name: String,
    country: String,
}

/// Activity to get location information from an IP address
pub async fn get_location_info(_ctx: ActContext, ip: String) -> Result<String, ActivityError> {
    info!("Fetching location info for IP: {}", ip);

    let url = format!("http://ip-api.com/json/{}", ip);

    let response = reqwest::get(&url)
        .await
        .map_err(|e| ActivityError::NonRetryable(e.into()))?;

    let location_data: LocationData = response
        .json()
        .await
        .map_err(|e| ActivityError::NonRetryable(e.into()))?;

    let location = format!(
        "{}, {}, {}",
        location_data.city, location_data.region_name, location_data.country
    );

    info!("Retrieved location: {}", location);
    Ok(location)
}
