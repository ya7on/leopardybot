use crate::conf::get_config;
use crate::error::{Error, Result};
use crate::telebot;
use crate::telebot::typings::output::Message;
use serde::de::DeserializeOwned;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct JsonResponse<R> {
    pub ok: bool,
    pub result: Option<R>,
    pub error_code: Option<usize>,
    pub description: Option<String>,
}

#[derive(Clone)]
pub struct Client {
    token: String,
    client: reqwest::Client,
}

impl Client {
    pub async fn new(token: &str, url: &String) -> Result<Self> {
        let c = Self {
            token: token.to_owned(),
            client: reqwest::Client::new(),
        };

        let webhook_info = c.get_webhook_info().await?;
        if url != &webhook_info.result.ok_or_else(|| todo!())?.url {
            info!("Updating telegram webhook url to {}", &url);
            c.set_webhook_info(url).await?;
        }

        Ok(c)
    }

    async fn execute<R: DeserializeOwned>(
        &self,
        method: &str,
        form: &[(&str, String)],
    ) -> Result<JsonResponse<R>> {
        let response = self
            .client
            .post(format!(
                "https://api.telegram.org/bot{}/{}",
                self.token, method
            ))
            .form(&form)
            .send()
            .await
            .map_err(|err| {
                error!("Connection error: {}", err);
                Error::ConnectionError(format!("Connection error: {}", err))
            })?;
        response.json().await.map_err(|err| {
            error!("Cannot parse Telegram API response: {:?}", err);
            Error::SerializationError(format!("Cannot parse Telegram API response: {:?}", err))
        })
    }

    pub async fn get_webhook_info(
        &self,
    ) -> Result<JsonResponse<telebot::typings::output::WebhookInfo>> {
        let response = self
            .execute::<telebot::typings::output::WebhookInfo>("getWebhookInfo", &[])
            .await;
        debug!("get_webhook_info: {:?}", response);
        response
    }

    pub(crate) async fn set_webhook_info(&self, url: &str) -> Result<JsonResponse<Option<bool>>> {
        let response = self.execute("setWebhook", &[("url", url.to_owned())]).await;
        debug!("set_webhook_info: {:?}", response);
        response
    }

    pub async fn send_message(
        &self,
        chat_id: isize,
        text: &String,
    ) -> Result<JsonResponse<Message>> {
        let response = self
            .execute(
                "sendMessage",
                &[("chat_id", chat_id.to_string()), ("text", text.to_string())],
            )
            .await;
        debug!("send_message: {:?}", response);
        response
    }

    pub async fn send_quiz(
        &self,
        chat_id: isize,
        question: &String,
        options: &Vec<String>,
        correct_option_id: usize,
    ) -> Result<JsonResponse<Message>> {
        let c = get_config();
        let response = self
            .execute(
                "sendPoll",
                &[
                    ("chat_id", chat_id.to_string()),
                    ("question", question.to_string()),
                    (
                        "options",
                        serde_json::to_string(options).map_err(|err| {
                            error!("Cannot convert options to json array. {}", err);
                            Error::SerializationError(format!(
                                "Cannot convert options to json array. {}",
                                err
                            ))
                        })?,
                    ),
                    ("is_anonymous", "false".to_string()),
                    ("type", "quiz".to_string()),
                    ("correct_option_id", correct_option_id.to_string()),
                    ("open_period", c.quiz_round_time.to_string()),
                    ("protect_content", true.to_string()),
                ],
            )
            .await;
        debug!("send_quiz: {:?}", response);
        response
    }
}
