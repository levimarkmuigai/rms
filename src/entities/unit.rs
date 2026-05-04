use core::fmt;
use std::str::FromStr;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Unit {
    pub id: Uuid,
    pub building_id: Uuid,
    pub unit_number: String,
    pub tenant_id: Option<Uuid>,
    pub rent_amount: i32,
    pub status: UnitStatus,
}

#[derive(Debug, Clone)]
pub enum UnitStatus {
    Vacant,
    Occupied,
}

impl FromStr for UnitStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "vacant" => Ok(UnitStatus::Vacant),
            "occupied" => Ok(UnitStatus::Occupied),
            _ => Err(()),
        }
    }
}

impl fmt::Display for UnitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UnitStatus::Vacant => "vacant",
            UnitStatus::Occupied => "occupied",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone)]
pub struct UnitPortfolioSummary {
    pub id: Uuid,
    pub number: String,
    pub status: String,
    pub rent_amount: i64,
}
