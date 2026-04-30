use std::str::FromStr;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MaintenanceRequest {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub category: String,
    pub status: RequestStatus,
    pub priority: RequestPriority,
    pub age_days: i32,
}

#[derive(Debug, Clone)]
pub enum RequestStatus {
    Pending,
    InProgress,
    Resolved,
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
pub enum RequestPriority {
    Medium,
    High,
}

impl FromStr for RequestPriority {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            _ => Err(()),
        }
    }
}
