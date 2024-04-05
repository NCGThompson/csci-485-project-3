mod tests;

const REMOTE_SERVER_URL: &str = "http://example.com/upload";

pub fn send_to_remote_server(data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // Implement HTTP request to send data to remote server
    let client = reqwest::blocking::Client::new();
    client.post(REMOTE_SERVER_URL).body(data.to_vec()).send()?;
    Ok(())
}
