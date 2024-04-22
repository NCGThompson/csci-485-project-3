mod tests;

const REMOTE_SERVER_URL: &str = "http://127.0.0.1";

pub fn send_to_remote_server(data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let message: &str = std::str::from_utf8(data)?;
    ureq::post(REMOTE_SERVER_URL).send_json(ureq::json!({
        "message": message,
        "github link": "https://github.com/NCGThompson/csci-485-project-3"
    }))?;
    Ok(())
}
