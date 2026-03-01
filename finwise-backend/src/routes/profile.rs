use actix_web::{web, HttpResponse};
use mongodb::bson::doc;

use crate::db::{Database, collections};
use crate::models::profile::{ProfileDocument, ProfileResponse};
use crate::models::piggy::PiggyBankDocument;
use crate::utils::{AppError, AppResult};

pub async fn get_profile(
    db: web::Data<Database>,
    path: web::Path<String>,
) -> AppResult<HttpResponse> {
    let user_id = path.into_inner();

    // Fetch profile
    let profiles = db.collection::<ProfileDocument>(collections::PROFILES);
    let profile = profiles
        .find_one(doc! { "user_id": &user_id })
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Profile not found for user {}", user_id)))?;

    // Fetch piggy bank stats (optional — defaults to 0)
    let piggies = db.collection::<PiggyBankDocument>(collections::PIGGY_BANKS);
    let piggy = piggies
        .find_one(doc! { "user_id": &user_id })
        .await?
        .unwrap_or_else(|| PiggyBankDocument {
            id: None,
            user_id: user_id.clone(),
            total_saved: 0.0,
            current_streak: 0,
            longest_streak: 0,
            last_deposit_ts: None,
            reward_points: 0,
        });

    Ok(HttpResponse::Ok().json(ProfileResponse {
        user_id: profile.user_id,
        latest_advice: profile.latest_advice,
        total_analyses: profile.total_analyses,
        total_saved: piggy.total_saved,
        current_streak: piggy.current_streak,
        longest_streak: piggy.longest_streak,
        reward_points: piggy.reward_points,
    }))
}