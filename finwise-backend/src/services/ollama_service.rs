use reqwest::Client;
use serde_json::{json, Value};

use crate::models::analysis::{FinancialAdvice, StructuredFinancialData, BudgetPlan};
use crate::utils::{AppError, AppResult};

pub struct AiService {
    client: Client,
}

impl AiService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_financial_advice(
        &self,
        data: &StructuredFinancialData,
    ) -> AppResult<FinancialAdvice> {

        let prompt = self.build_prompt(data);

        let api_key = std::env::var("GROQ_API_KEY")
            .map_err(|_| AppError::OllamaApi("Missing GROQ_API_KEY".into()))?;

        let body = json!({
            "model": "llama-3.3-70b-versatile",
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": 0.3
        });

        let response = self
            .client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .bearer_auth(api_key)
            .json(&body)
            .send()
            .await
            .map_err(AppError::HttpRequest)?;

        if !response.status().is_success() {
            let err = response.text().await.unwrap_or_default();
            return Err(AppError::OllamaApi(format!("Groq error: {}", err)));
        }

        let response_json: Value = response.json().await.map_err(AppError::HttpRequest)?;

        let text = response_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| AppError::OllamaApi("No response from Groq".into()))?;
        
        println!("Groq response: {}", text);

        self.parse_advice(text)
    }

    fn build_prompt(&self, data: &StructuredFinancialData) -> String {
        format!(
            r#"
You are a professional financial advisor.

Analyze this structured financial data and respond ONLY in valid JSON:

{}

Required JSON format:

{{
  "risk_level": "Low" | "Medium" | "High",
  "tax_saving_suggestions": ["string"],
  "budget_plan": {{
    "needs": number,
    "wants": number,
    "savings": number
  }},
  "income_growth_suggestions": ["string"],
  "debt_strategy": "string"
}}

Ensure needs + wants + savings = 100.
Do not include explanations.
"#,
            serde_json::to_string_pretty(data).unwrap()
        )
    }

    fn parse_advice(&self, text: &str) -> AppResult<FinancialAdvice> {

        let cleaned = text
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        let value: Value = serde_json::from_str(cleaned)
            .map_err(|e| AppError::OllamaApi(format!("JSON parse failed: {}", e)))?;

        Ok(FinancialAdvice {
            risk_level: value["risk_level"].as_str().unwrap_or("Medium").to_string(),

            tax_saving_suggestions: value["tax_saving_suggestions"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect::<Vec<String>>()
                })
                .unwrap_or_default(),

            income_growth_suggestions: value["income_growth_suggestions"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect::<Vec<String>>()
                })
                .unwrap_or_default(),

            debt_strategy: value["debt_strategy"]
                .as_str()
                .unwrap_or("")
                .to_string(),

            budget_plan: BudgetPlan {
                needs: value["budget_plan"]["needs"].as_f64().unwrap_or(50.0),
                wants: value["budget_plan"]["wants"].as_f64().unwrap_or(30.0),
                savings: value["budget_plan"]["savings"].as_f64().unwrap_or(20.0),
            },
        })
    }
}