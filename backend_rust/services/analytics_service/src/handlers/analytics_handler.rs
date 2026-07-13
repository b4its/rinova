use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::models::*;

/// GET /api/v1/analytics/:project_id - Get analytics for a project
pub async fn get_analytics(
    path: web::Path<Uuid>,
    query: web::Query<AnalyticsQuery>,
) -> HttpResponse {
    let project_id = path.into_inner();
    
    // Mock response - in production, fetch from database
    let metrics = ProjectMetrics {
        project_id,
        period_start: query.start_date.unwrap_or_else(|| Utc::now() - chrono::Duration::days(30)),
        period_end: query.end_date.unwrap_or_else(Utc::now),
        total_page_views: 15234,
        unique_visitors: 4521,
        avg_session_duration_seconds: 185.5,
        bounce_rate: 42.3,
        top_pages: vec![
            PageMetric {
                path: "/".to_string(),
                views: 5234,
                avg_duration_seconds: 120.5,
            },
            PageMetric {
                path: "/about".to_string(),
                views: 2341,
                avg_duration_seconds: 95.3,
            },
            PageMetric {
                path: "/contact".to_string(),
                views: 1892,
                avg_duration_seconds: 45.2,
            },
        ],
        traffic_sources: vec![
            TrafficSource {
                source: "direct".to_string(),
                visits: 5000,
                percentage: 32.8,
            },
            TrafficSource {
                source: "google".to_string(),
                visits: 4500,
                percentage: 29.5,
            },
            TrafficSource {
                source: "social".to_string(),
                visits: 2000,
                percentage: 13.1,
            },
        ],
        device_breakdown: DeviceBreakdown {
            desktop: 8500,
            tablet: 2500,
            mobile: 4000,
            unknown: 234,
        },
        daily_views: generate_daily_views(),
    };

    HttpResponse::Ok().json(metrics)
}

/// POST /api/v1/analytics/:project_id/events - Track a page view event
pub async fn track_event(
    path: web::Path<Uuid>,
    body: web::Json<TrackEventRequest>,
) -> HttpResponse {
    let project_id = path.into_inner();
    
    // Mock response - in production, store in database
    HttpResponse::Ok().json(TrackEventResponse {
        success: true,
        event_id: Uuid::new_v4(),
    })
}

/// GET /api/v1/analytics/:project_id/alerts - Get traffic alerts
pub async fn get_alerts(
    path: web::Path<Uuid>,
) -> HttpResponse {
    let project_id = path.into_inner();
    
    HttpResponse::Ok().json(vec![
        TrafficAlert {
            id: Uuid::new_v4(),
            project_id,
            alert_type: AlertType::TrafficSpike,
            message: "Traffic increased by 250% compared to last week".to_string(),
            change_percentage: 250.0,
            triggered_at: Utc::now(),
            is_resolved: false,
        },
    ])
}

// Helper function to generate mock daily views
fn generate_daily_views() -> Vec<DailyView> {
    let mut views = Vec::new();
    for i in 0..7 {
        views.push(DailyView {
            date: Utc::now() - chrono::Duration::days(i),
            page_views: 2000 + (i * 100) as u64,
            visitors: 500 + (i * 25) as u64,
        });
    }
    views.reverse();
    views
}

// Request/Response types
#[derive(Debug, Deserialize)]
pub struct AnalyticsQuery {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub granularity: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TrackEventRequest {
    pub path: String,
    pub visitor_id: String,
    pub session_id: String,
    pub referrer: Option<String>,
    pub user_agent: Option<String>,
    pub device_type: DeviceType,
    pub duration_seconds: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct TrackEventResponse {
    pub success: bool,
    pub event_id: Uuid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analytics_query_defaults() {
        let query = AnalyticsQuery {
            start_date: None,
            end_date: None,
            granularity: None,
        };

        assert!(query.start_date.is_none());
        assert!(query.granularity.is_none());
    }

    #[test]
    fn test_daily_views_generation() {
        let views = generate_daily_views();
        assert_eq!(views.len(), 7);
    }
}
