// src/routes/profile.rs    
use actix_web::{web, HttpResponse};
use actix_session::Session;
use mongodb::bson::{doc, oid::ObjectId}; 
use chrono::Utc;                           

use crate::db::Database;
use crate::models::profile::{ProfileDocument, ProfileResponse};
use crate::models::user::User;             
use crate::utils::{AppError, AppResult};

pub async fn get_profile(
    db: web::Data<Database>,
    session: Session,
) -> AppResult<HttpResponse> {

    let user_id = session
        .get::<ObjectId>("user_id")
        .map_err(|_| AppError::NotFound("Session error".into()))? // ? won't work directly
        .ok_or_else(|| AppError::NotFound("Not logged in".into()))?;

    let users = db.collection::<User>("users");

    let user = users
        .find_one(doc! { "_id": user_id })
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    let profiles = db.collection::<ProfileDocument>("profiles");

    let profile = profiles
        .find_one(doc! { "user_id": &user.email })
        .await?;

    if let Some(profile) = profile {
        return Ok(HttpResponse::Ok().json(ProfileResponse {
            user_id: profile.user_id,
            latest_advice: profile.latest_advice,
            total_analyses: profile.total_analyses,
            total_saved: profile.total_saved,
            current_streak: profile.current_streak,
            longest_streak: profile.longest_streak,
            reward_points: profile.reward_points,
        }));
    }

    let new_profile = ProfileDocument {
        id: None,
        user_id: user.email.clone(),
        latest_advice: None,
        total_analyses: 0,
        updated_at: Utc::now(),
        total_saved: 0.0,       
        current_streak: 0,      
        longest_streak: 0,      
        reward_points: 0,         
    };

    profiles.insert_one(&new_profile).await?;

    Ok(HttpResponse::Ok().json(ProfileResponse {
        user_id: new_profile.user_id,
        latest_advice: new_profile.latest_advice,
        total_analyses: new_profile.total_analyses,
        total_saved: new_profile.total_saved,
        current_streak: new_profile.current_streak,
        longest_streak: new_profile.longest_streak,
        reward_points: new_profile.reward_points,
    }))
}