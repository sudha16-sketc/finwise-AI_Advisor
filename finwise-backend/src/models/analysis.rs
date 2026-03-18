// src/models/analysis.rs
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, Utc};

/// Incoming request from the client
#[derive(Debug, Deserialize)]
pub struct AnalyzeRequest {
    pub user_id: String,
    pub financial_text: String,
}

/// Structured financial data extracted from free text
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StructuredFinancialData {
    pub income: f64,
    pub country: String,
    pub family_members: u32,
    pub loans: Vec<String>,
    pub financial_challenges: Vec<String>,
    pub goals: Vec<String>,
}

/// AI-generated budget plan breakdown
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BudgetPlan {
    pub needs: f64,
    pub wants: f64,
    pub savings: f64,
}

/// Full AI financial advice response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FinancialAdvice {
    pub risk_level: String,
    pub tax_saving_suggestions: Vec<String>,
    pub budget_plan: BudgetPlan,
    pub income_growth_suggestions: Vec<String>,
    pub debt_strategy: String,
}

/// MongoDB document for storing analysis results
#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String,
    pub original_text: String,
    pub structured_data: StructuredFinancialData,
    pub advice: FinancialAdvice,
    pub created_at: DateTime<Utc>,
}

/// Response sent back to the client
#[derive(Debug, Serialize)]
pub struct AnalyzeResponse {
    pub success: bool,
    pub user_id: String,
    pub structured_data: StructuredFinancialData,
    pub advice: FinancialAdvice,
    pub analysis_id: String,
}