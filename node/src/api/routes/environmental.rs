use crate::api::error::{ApiError, ApiResult};
use crate::api::types::{
    EnvironmentalImpact, EnergyUsage, CarbonFootprint, EnvironmentalSettings,
    ResourceUtilization,
};
use crate::environmental::EnvironmentalMonitor;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use std::sync::Arc;

/// Configure environmental API routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/environmental")
            .route("/impact", web::get().to(get_environmental_impact))
            .route("/energy", web::get().to(get_energy_usage))
            .route("/carbon", web::get().to(get_carbon_footprint))
            .route("/resources", web::get().to(get_resource_utilization))
            .route("/settings", web::get().to(get_environmental_settings))
            .route("/settings", web::put().to(update_environmental_settings)),
    );
}

/// Get overall environmental impact data
///
/// Returns comprehensive data about the node's environmental impact.
#[utoipa::path(
    get,
    path = "/api/v1/environmental/impact",
    params(
        GetEnvironmentalImpactParams
    ),
    responses(
        (status = 200, description = "Environmental impact data retrieved successfully", body = EnvironmentalImpact),
        (status = 400, description = "Invalid request parameters", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
#[derive(Debug, Deserialize, IntoParams)]
struct GetEnvironmentalImpactParams {
    /// Time period in seconds for which to retrieve data (default: 86400 - 1 day)
    #[param(default = "86400")]
    period: Option<u64>,
    
    /// Level of detail for the report (default: "standard")
    #[param(default = "standard")]
    detail: Option<String>,
}

async fn get_environmental_impact(
    params: web::Query<GetEnvironmentalImpactParams>,
    monitor: web::Data<Arc<EnvironmentalMonitor>>,
) -> ApiResult<EnvironmentalImpact> {
    let period = params.period.unwrap_or(86400);
    let detail = params.detail.clone().unwrap_or_else(|| "standard".to_string());
    
    // TODO: Implement real environmental impact retrieval
    let impact = monitor.get_environmental_impact(period, &detail)?;
    
    Ok(HttpResponse::Ok().json(impact))
}

/// Get energy usage data
///
/// Returns detailed information about the node's energy consumption.
#[utoipa::path(
    get,
    path = "/api/v1/environmental/energy",
    params(
        GetEnergyUsageParams
    ),
    responses(
        (status = 200, description = "Energy usage data retrieved successfully", body = EnergyUsage),
        (status = 400, description = "Invalid request parameters", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
#[derive(Debug, Deserialize, IntoParams)]
struct GetEnergyUsageParams {
    /// Time period in seconds for which to retrieve data (default: 3600 - 1 hour)
    #[param(default = "3600")]
    period: Option<u64>,
    
    /// Whether to include historical data (default: false)
    #[param(default = "false")]
    include_history: Option<bool>,
}

async fn get_energy_usage(
    params: web::Query<GetEnergyUsageParams>,
    monitor: web::Data<Arc<EnvironmentalMonitor>>,
) -> ApiResult<EnergyUsage> {
    let period = params.period.unwrap_or(3600);
    let include_history = params.include_history.unwrap_or(false);
    
    // TODO: Implement real energy usage retrieval
    let energy_data = monitor.get_energy_usage(period, include_history)?;
    
    Ok(HttpResponse::Ok().json(energy_data))
}

/// Get carbon footprint data
///
/// Returns information about the node's carbon emissions.
#[utoipa::path(
    get,
    path = "/api/v1/environmental/carbon",
    params(
        GetCarbonFootprintParams
    ),
    responses(
        (status = 200, description = "Carbon footprint data retrieved successfully", body = CarbonFootprint),
        (status = 400, description = "Invalid request parameters", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
#[derive(Debug, Deserialize, IntoParams)]
struct GetCarbonFootprintParams {
    /// Time period in seconds for which to retrieve data (default: 86400 - 1 day)
    #[param(default = "86400")]
    period: Option<u64>,
    
    /// Whether to include offset information (default: true)
    #[param(default = "true")]
    include_offsets: Option<bool>,
}

async fn get_carbon_footprint(
    params: web::Query<GetCarbonFootprintParams>,
    monitor: web::Data<Arc<EnvironmentalMonitor>>,
) -> ApiResult<CarbonFootprint> {
    let period = params.period.unwrap_or(86400);
    let include_offsets = params.include_offsets.unwrap_or(true);
    
    // TODO: Implement real carbon footprint retrieval
    let carbon_data = monitor.get_carbon_footprint(period, include_offsets)?;
    
    Ok(HttpResponse::Ok().json(carbon_data))
}

/// Get resource utilization data
///
/// Returns information about the node's hardware resource utilization.
#[utoipa::path(
    get,
    path = "/api/v1/environmental/resources",
    params(
        GetResourceUtilizationParams
    ),
    responses(
        (status = 200, description = "Resource utilization data retrieved successfully", body = ResourceUtilization),
        (status = 400, description = "Invalid request parameters", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
#[derive(Debug, Deserialize, IntoParams)]
struct GetResourceUtilizationParams {
    /// Time period in seconds for which to retrieve data (default: 300 - 5 minutes)
    #[param(default = "300")]
    period: Option<u64>,
}

async fn get_resource_utilization(
    params: web::Query<GetResourceUtilizationParams>,
    monitor: web::Data<Arc<EnvironmentalMonitor>>,
) -> ApiResult<ResourceUtilization> {
    let period = params.period.unwrap_or(300);
    
    // TODO: Implement real resource utilization retrieval
    let resource_data = monitor.get_resource_utilization(period)?;
    
    Ok(HttpResponse::Ok().json(resource_data))
}

/// Get environmental monitoring settings
///
/// Returns the current environmental monitoring and optimization settings.
#[utoipa::path(
    get,
    path = "/api/v1/environmental/settings",
    responses(
        (status = 200, description = "Environmental settings retrieved successfully", body = EnvironmentalSettings),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
async fn get_environmental_settings(
    monitor: web::Data<Arc<EnvironmentalMonitor>>,
) -> ApiResult<EnvironmentalSettings> {
    // TODO: Implement real environmental settings retrieval
    let settings = monitor.get_settings()?;
    
    Ok(HttpResponse::Ok().json(settings))
}

/// Update environmental monitoring settings
///
/// Updates the environmental monitoring and optimization settings.
#[utoipa::path(
    put,
    path = "/api/v1/environmental/settings",
    request_body = EnvironmentalSettings,
    responses(
        (status = 200, description = "Environmental settings updated successfully", body = EnvironmentalSettings),
        (status = 400, description = "Invalid settings", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    )
)]
async fn update_environmental_settings(
    request: web::Json<EnvironmentalSettings>,
    monitor: web::Data<Arc<EnvironmentalMonitor>>,
) -> ApiResult<EnvironmentalSettings> {
    // TODO: Implement real environmental settings update
    let updated_settings = monitor.update_settings(request.0)?;
    
    Ok(HttpResponse::Ok().json(updated_settings))
} 