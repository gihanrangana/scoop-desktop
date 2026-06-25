pub async fn fetch_logo_from_brandfetch(
    domain: &str,
    client: &reqwest::Client,
) -> Result<Option<Vec<u8>>, String> {
    // let config = app.config();
    // let client_id = config
    //     .get("brandfetch")
    //     .and_then(|bf| bf.get("clientId"))
    //     .and_then(|ci| ci.as_str())
    //     .ok_or_else(|| "Brandfetch client ID not found".to_string())?;

    let client_id =
        std::env::var("BRANDFETCH_CLIENT_ID").unwrap_or_else(|_| "1idpP9KxXLazNbJk3Vg".to_string());

    let url = format!(
        "https://cdn.brandfetch.io/domain/{}?c={}",
        domain, client_id
    );

    let response = client
        .get(url)
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| format!("Brandfetch request failed for {}: {}", domain, e))?;

    if !response.status().is_success() {
        return Ok(None);
    }

    let logo_data = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read logo data for {}: {}", domain, e))?;

    Ok(Some(logo_data.to_vec()))
}
