use actix_web::{web, HttpResponse};
use chrono::Utc;
use mongodb::bson::doc;

use crate::db::{Database, collections};
use crate::models::analysis::{AnalysisDocument, AnalyzeRequest, AnalyzeResponse, FinancialAdvice};
use crate::models::profile::ProfileDocument;
use crate::services::{AiService, parse_financial_text};
use crate::utils::{AppError, AppResult};

pub async fn analyze(
    db: web::Data<Database>,
    body: web::Json<AnalyzeRequest>,
) -> AppResult<HttpResponse> {
    // Input validation
    if body.user_id.trim().is_empty() {
        return Err(AppError::Validation("user_id cannot be empty".to_string()));
    }
    if body.financial_text.trim().len() < 10 {
        return Err(AppError::Validation(
            "financial_text must be at least 10 characters".to_string(),
        ));
    }

    // Step 1: Extract structured data from free text
    let structured_data = parse_financial_text(&body.financial_text);
    log::info!("Structured data extracted for user {}", body.user_id);

    // Step 2: Call Gemini API for advice
    // Explicit type annotation to resolve E0282
let ai = AiService::new();
let advice: FinancialAdvice = ai.get_financial_advice(&structured_data).await?;
    log::info!("Gemini advice received for user {}", body.user_id);

    // Step 3: Store analysis in MongoDB
    let analyses = db.collection::<AnalysisDocument>(collections::ANALYSES);
    let analysis_doc = AnalysisDocument {
        id: None,
        user_id: body.user_id.clone(),
        original_text: body.financial_text.clone(),
        structured_data: structured_data.clone(),
        advice: advice.clone(),
        created_at: Utc::now(),
    };

    let insert_result = analyses.insert_one(&analysis_doc).await?;
    let analysis_id = insert_result.inserted_id.as_object_id()
        .map(|oid| oid.to_hex())
        .unwrap_or_default();

    // Step 4: Upsert user profile with latest advice
    // Convert advice to Bson manually, mapping the error into AppError
    let advice_bson = mongodb::bson::to_bson(&advice)
        .map_err(|e| AppError::OllamaApi(format!("Failed to serialize advice to BSON: {}", e)))?;

    let profiles = db.collection::<ProfileDocument>(collections::PROFILES);
    let filter = doc! { "user_id": &body.user_id };
    let update = doc! {
        "$set": {
            "latest_advice": advice_bson,
            "updated_at": Utc::now().to_rfc3339(),
        },
        "$inc": { "total_analyses": 1_i32 }
    };

    // MongoDB 3.x API: update_one takes (filter, update) directly
    profiles
        .update_one(filter, update)
        .upsert(true)
        .await?;

    Ok(HttpResponse::Ok().json(AnalyzeResponse {
        success: true,
        user_id: body.user_id.clone(),
        structured_data,
        advice,
        analysis_id,
    }))
}