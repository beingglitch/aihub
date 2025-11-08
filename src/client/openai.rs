use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ResponseRequest {
    model: String,
    input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning: Option<Reasoning>,
}

#[derive(Serialize)]
struct Reasoning {
    effort: String, // "low", "medium", "high"
}

#[derive(Deserialize)]
struct Content {
    r#type: String,
    text: String,
    #[serde(default)]
    annotations: Vec<serde_json::Value>,
}

#[derive(Deserialize)]
struct OutputItem {
    id: String,
    r#type: String,
    #[serde(default)]
    content: Vec<Content>,
    #[serde(default)]
    role: String,
}

#[derive(Deserialize)]
struct ResponseData {
    id: String,
    status: String,
    output: Vec<OutputItem>,
}

pub struct OpenAIClient {
    client: reqwest::Client,
    api_key: String,
    model: String,
}

impl OpenAIClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
            model: "gpt-5".to_string(),
        }
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    pub async fn chat(
        &self,
        prompt: &str,
        instructions: Option<String>,
        effort: Option<String>,
    ) -> Result<String> {
        let request = ResponseRequest {
            model: self.model.clone(),
            input: prompt.to_string(),
            instructions,
            reasoning: effort.map(|e| Reasoning { effort: e }),
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/responses")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let response_data: ResponseData = response.error_for_status()?.json().await?;

        // Extract text from output
        let text = self.extract_text(&response_data)?;

        Ok(text)
    }

    fn extract_text(&self, response: &ResponseData) -> Result<String> {
        // Collect all text from message type outputs
        let mut texts = Vec::new();

        for output_item in &response.output {
            if output_item.r#type == "message" {
                for content in &output_item.content {
                    if content.r#type == "output_text" {
                        texts.push(content.text.clone());
                    }
                }
            }
        }

        if texts.is_empty() {
            anyhow::bail!("No text output found in response");
        }

        Ok(texts.join("\n"))
    }
}
