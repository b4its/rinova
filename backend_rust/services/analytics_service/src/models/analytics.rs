use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Page view event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageViewEvent {
    pub id: Uuid,
    pub project_id: Uuid,
    pub path: String,
    pub visitor_id: String,
    pub session_id: String,
    pub referrer: Option<String>,
    pub user_agent: Option<String>,
    pub country: Option<String>,
    pub device_type: DeviceType,
    pub timestamp: DateTime<Utc>,
    pub duration_seconds: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "device_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    Desktop,
    Tablet,
    Mobile,
    Unknown,
}

/// Analytics metrics for a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetrics {
    pub project_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_page_views: u64,
    pub unique_visitors: u64,
    pub avg_session_duration_seconds: f64,
    pub bounce_rate: f64,
    pub top_pages: Vec<PageMetric>,
    pub traffic_sources: Vec<TrafficSource>,
    pub device_breakdown: DeviceBreakdown,
    pub daily_views: Vec<DailyView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageMetric {
    pub path: String,
    pub views: u64,
    pub avg_duration_seconds: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficSource {
    pub source: String,
    pub visits: u64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceBreakdown {
    pub desktop: u64,
    pub tablet: u64,
    pub mobile: u64,
    pub unknown: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyView {
    pub date: DateTime<Utc>,
    pub page_views: u64,
    pub visitors: u64,
}

/// Traffic alert for significant changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficAlert {
    pub id: Uuid,
    pub project_id: Uuid,
    pub alert_type: AlertType,
    pub message: String,
    pub change_percentage: f64,
    pub triggered_at: DateTime<Utc>,
    pub is_resolved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertType {
    TrafficSpike,
    TrafficDrop,
    BounceRateHigh,
    ErrorRateHigh,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_view_event_creation() {
        let event = PageViewEvent {
            id: Uuid::new_v4(),
            project_id: Uuid::new_v4(),
            path: "/".to_string(),
            visitor_id: "visitor-123".to_string(),
            session_id: "session-456".to_string(),
            referrer: None,
            user_agent: None,
            country: None,
            device_type: DeviceType::Desktop,
            timestamp: Utc::now(),
            duration_seconds: Some(30),
        };

        assert_eq!(event.path, "/");
        assert_eq!(event.device_type, DeviceType::Desktop);
    }

    #[test]
    fn test_project_metrics_defaults() {
        let metrics = ProjectMetrics {
            project_id: Uuid::new_v4(),
            period_start: Utc::now(),
            period_end: Utc::now(),
            total_page_views: 0,
            unique_visitors: 0,
            avg_session_duration_seconds: 0.0,
            bounce_rate: 0.0,
            top_pages: vec![],
            traffic_sources: vec![],
            device_breakdown: DeviceBreakdown {
                desktop: 0,
                tablet: 0,
                mobile: 0,
                unknown: 0,
            },
            daily_views: vec![],
        };

        assert_eq!(metrics.total_page_views, 0);
        assert!(metrics.top_pages.is_empty());
    }
}
