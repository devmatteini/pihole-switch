pub const PIHOLE_DEFAULT_HOST: &str = "pi.hole";

pub struct PiHoleConfig {
    pub api_token: String,
    pub api_url: String,
}

impl PiHoleConfig {
    pub fn new(api_token: String, api_url: String) -> PiHoleConfig {
        PiHoleConfig { api_token, api_url }
    }

    pub fn build_url(host: &str) -> String {
        format!("http://{}/admin/api.php", host)
    }
}
