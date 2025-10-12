use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct KavenegarResponse {
    #[serde(rename = "return")]
    pub return_status: KavenegarReturnStatus,
    pub entries: Option<Vec<KavenegarEntry>>,
}

#[derive(Deserialize, Debug)]
pub struct KavenegarReturnStatus {
    pub status: u16,
    #[serde(rename = "message")]
    pub message_text: String,
}

#[derive(Deserialize, Debug)]
pub struct KavenegarEntry {
    pub messageid: u64,
    pub message: String,
    pub status: u16,
    pub statustext: String,
    pub sender: String,
    pub receptor: String,
    pub date: u64,
    pub cost: u32,
}

pub async fn send_sms(api_key: &str, sender: &str, receptor: &str, message: &str) -> Result<KavenegarResponse, reqwest::Error> {
    let client = Client::new();
    let url = format!("https://api.kavenegar.com/v1/{}/sms/send.json", api_key);

    let params = [
        ("sender", sender),
        ("receptor", receptor),
        ("message", message),
    ];

    let response = client.post(&url)
        .form(&params)
        .send()
        .await?;

    response.json::<KavenegarResponse>().await
}

