use reqwest;
use reqwest::blocking;
use serde_json::Value;
use std::error::Error;

/// Gets altitude for a given latitude and longitude.
/// Result seems to be returned meters above sea level
pub fn get_altitude(latitude: f64, longitude: f64) -> Result<f64, Box<dyn Error>> {
    let url = format!(
        "https://api.open-elevation.com/api/v1/lookup?locations={},{}",
        latitude,
        longitude
    );

    // Perform a blocking HTTP request
    let response = blocking::get(&url)?.json::<Value>()?;

    let elevation = response["results"][0]["elevation"].as_f64().unwrap_or(0.0);
    Ok(elevation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_get_altitude() {
        let latitude = 43.084128;
        let longitude = 25.5919228;

        let result = get_altitude(latitude, longitude);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 265.0);
    }
}
