use reqwest::blocking::Client;
use serde_json::Value;
use std::error::Error;
use std::time::Duration;

/// Extracts the elevation (meters above sea level) from an Open-Elevation response.
fn parse_elevation(body: &Value) -> Result<f64, Box<dyn Error>> {
    body["results"][0]["elevation"]
        .as_f64()
        .ok_or_else(|| "elevation missing from Open-Elevation response".into())
}

/// Gets altitude for a given latitude and longitude.
/// Result is returned in meters above sea level.
pub fn get_altitude(
    latitude: f64,
    longitude: f64,
) -> Result<f64, Box<dyn Error>> {
    let url = format!(
        "https://api.open-elevation.com/api/v1/lookup?locations={},{}",
        latitude, longitude
    );

    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let body = client
        .get(&url)
        .send()?
        .error_for_status()?
        .json::<Value>()?;

    parse_elevation(&body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_elevation_ok() {
        let body = json!({
            "results": [{ "latitude": 43.084128, "longitude": 25.5919228, "elevation": 265.0 }]
        });
        assert_eq!(parse_elevation(&body).unwrap(), 265.0);
    }

    #[test]
    fn test_parse_elevation_missing_is_error() {
        let body = json!({ "results": [] });
        assert!(parse_elevation(&body).is_err());
    }

    /// Hits the real API, so it can fail when the service is down or its data changes.
    /// Run manually with: cargo test -p libswe-sys -- --ignored
    #[test]
    #[ignore = "network: calls api.open-elevation.com"]
    fn test_get_altitude_live() {
        let altitude = get_altitude(43.084128, 25.5919228).unwrap();
        assert!(
            (altitude - 265.0).abs() < 5.0,
            "{}",
            "unexpected altitude: {altitude}"
        );
    }
}
