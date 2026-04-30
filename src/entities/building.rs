use uuid::Uuid;

pub struct Building {
    pub id: Uuid,
    pub landlord_id: Uuid,
    pub name: String,
}
