use actix_web::{web, HttpResponse};

use crate::db::Database;
use crate::models::piggy::{DepositRequest, DepositResponse, PiggyStatsResponse};
use crate::services::PiggyService;
use crate::utils::AppResult;

pub async fn deposit(
    db: web::Data<Database>,
    body: web::Json<DepositRequest>,
) -> AppResult<HttpResponse> {
    let piggy = PiggyService::deposit(&db, &body.user_id, body.amount).await?;

    Ok(HttpResponse::Ok().json(DepositResponse {
        success: true,
        message: format!(
            "Deposit successful! 🐷 Streak: {} days",
            piggy.current_streak
        ),
        total_saved: piggy.total_saved,
        current_streak: piggy.current_streak,
        longest_streak: piggy.longest_streak,
        reward_points: piggy.reward_points,
    }))
}

pub async fn get_stats(
    db: web::Data<Database>,
    path: web::Path<String>,
) -> AppResult<HttpResponse> {
    let user_id = path.into_inner();
    let piggy = PiggyService::get_stats(&db, &user_id).await?;

    Ok(HttpResponse::Ok().json(PiggyStatsResponse {
        user_id: piggy.user_id,
        total_saved: piggy.total_saved,
        current_streak: piggy.current_streak,
        longest_streak: piggy.longest_streak,
        reward_points: piggy.reward_points,
    }))
}