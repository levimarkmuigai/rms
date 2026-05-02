use uuid::Uuid;

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
    pub collected: i64,
}
