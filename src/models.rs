use schema::stones;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Stone {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
