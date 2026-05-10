use core::fmt;
use std::{str::FromStr, time::SystemTime};

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MaintenanceRequest {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub tenant_id: Uuid,
    pub status: RequestStatus,
    pub desc: String,
}

#[derive(Debug, Clone)]
pub enum RequestStatus {
    Pending,
    InProgress,
    Resolved,
}

impl fmt::Display for RequestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RequestStatus::Pending => "pending",
            RequestStatus::InProgress => "in_progress",
            RequestStatus::Resolved => "resolved",
        };
        write!(f, "{s}")
    }
}

impl FromStr for RequestStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(Self::Pending),
            "in_progress" => Ok(Self::InProgress),
            "resolved" => Ok(Self::Resolved),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RequestWithLabel {
    pub category: String,
    pub unit_label: String,
    pub status: String,
    pub age_days: i32,
}

#[derive(Debug, Clone)]
pub struct RequestPanelRow {
    pub id: Uuid,
    pub desc: String,
    pub unit: String,
    pub status: String,
    pub created_at: SystemTime,
}

#[derive(Debug, Clone)]
pub struct ViewRequest {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub description: String,
    pub status: String,
    pub submitted_at: SystemTime,
}
