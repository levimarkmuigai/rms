use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Building {
    pub id: Uuid,
    pub landlord_id: Uuid,
    pub name: String,
}

pub struct BuildingSummaryRow {
    pub id: Uuid,
    pub name: String,
    pub total_units: i64,
    pub occupied: i64,
    pub collected: i32,
}

pub struct BuildingOverviewRow {
    pub name: String,
    pub caretaker_id: Option<Uuid>,
    pub caretaker_name: Option<String>,
    pub caretaker_number: Option<String>,
    pub requests: Option<i64>,
}
