use anyhow::{Result, anyhow};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::time::Duration;

// Surfline sits behind Cloudflare bot management, which blocks non-browser TLS
// fingerprints. We route requests through a FlareSolverr instance (headless
// Chromium) that solves the challenge and returns the rendered page.
const DEFAULT_FLARESOLVERR_URL: &str = "http://localhost:8191/v1";
const MAX_TIMEOUT_MS: u64 = 60_000;

// Subset of the FlareSolverr `/v1` response envelope we care about.
#[derive(Deserialize)]
struct FlareSolverrResponse {
    status: String,
    #[serde(default)]
    message: String,
    solution: Option<FlareSolverrSolution>,
}

#[derive(Deserialize)]
struct FlareSolverrSolution {
    status: u16,
    response: String,
}

pub async fn fetch<T: DeserializeOwned>(url: &str) -> Result<T> {
    let endpoint = std::env::var("FLARESOLVERR_URL").unwrap_or_else(|_| DEFAULT_FLARESOLVERR_URL.to_string());

    // FlareSolverr solves take several seconds; allow headroom over maxTimeout.
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(MAX_TIMEOUT_MS + 30_000))
        .build()?;

    let body = serde_json::json!({
        "cmd": "request.get",
        "url": url,
        "maxTimeout": MAX_TIMEOUT_MS,
    });

    let response = client.post(&endpoint).json(&body).send().await?;
    if !response.status().is_success() {
        return Err(anyhow!(
            "FlareSolverr request failed with status: {}",
            response.status()
        ));
    }

    let envelope: FlareSolverrResponse = response.json().await?;
    if envelope.status != "ok" {
        return Err(anyhow!(
            "FlareSolverr returned status '{}': {}",
            envelope.status,
            envelope.message
        ));
    }

    let solution = envelope
        .solution
        .ok_or_else(|| anyhow!("FlareSolverr response missing solution"))?;
    if solution.status != 200 {
        return Err(anyhow!("Surfline returned HTTP {} via FlareSolverr", solution.status));
    }

    let json = extract_json(&solution.response)?;
    Ok(serde_json::from_str(&json)?)
}

// FlareSolverr returns the browser-rendered page, so a raw JSON endpoint comes
// back wrapped in HTML (`<html>…<pre>{…}</pre>…</html>`) with `<`, `>`, and `&`
// HTML-escaped inside string values. Recover the JSON by slicing from the first
// `{` to the last `}` and unescaping those entities.
fn extract_json(html: &str) -> Result<String> {
    let start = html
        .find('{')
        .ok_or_else(|| anyhow!("No JSON object found in FlareSolverr response"))?;
    let end = html
        .rfind('}')
        .ok_or_else(|| anyhow!("No JSON object found in FlareSolverr response"))?;
    if end < start {
        return Err(anyhow!("Malformed JSON boundaries in FlareSolverr response"));
    }
    Ok(html_unescape(&html[start..=end]))
}

fn html_unescape(s: &str) -> String {
    // `&amp;` must be replaced last to avoid double-unescaping.
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&#x27;", "'")
        .replace("&amp;", "&")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Exact wrapper format observed from FlareSolverr + Chrome for a JSON endpoint.
    const WRAPPED: &str = r#"<html><head><meta name="color-scheme" content="light dark"><meta charset="utf-8"></head><body><pre>{"data":{"value":1}}</pre><div class="json-formatter-container"></div></body></html>"#;

    #[test]
    fn extracts_json_from_flaresolverr_html_wrapper() {
        let json = extract_json(WRAPPED).unwrap();
        assert_eq!(json, r#"{"data":{"value":1}}"#);
    }

    #[test]
    fn unescapes_html_entities_in_json_string_values() {
        let wrapped = r#"<pre>{"name":"Jack &amp; Jill &lt;3&gt;"}</pre>"#;
        let json = extract_json(wrapped).unwrap();
        assert_eq!(json, r#"{"name":"Jack & Jill <3>"}"#);
        // Must be valid, parseable JSON after unescaping.
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["name"], "Jack & Jill <3>");
    }

    #[test]
    fn parses_wrapped_wave_payload_into_struct() {
        use crate::surfline_types::wave::WaveResult;
        let wrapped = r#"<html><body><pre>{"data":{"wave":[{"timestamp":1787814000,"surf":{"min":3,"max":4,"plus":false,"humanRelation":"Waist to chest"}}]}}</pre></body></html>"#;
        let json = extract_json(wrapped).unwrap();
        let result: WaveResult = serde_json::from_str(&json).unwrap();
        assert_eq!(result.data.wave.len(), 1);
        assert_eq!(result.data.wave[0].surf.max, 4);
        assert_eq!(result.data.wave[0].surf.human_relation, "Waist to chest");
    }

    #[test]
    fn errors_when_no_json_object_present() {
        assert!(extract_json("<html><body>no json here</body></html>").is_err());
    }
}
