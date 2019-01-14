// Temporary silence until diesel-1.4.
// See <https://github.com/diesel-rs/diesel/issues/1785#issuecomment-422579609>.
#![allow(proc_macro_derive_resolution_fallback)]

//use crate::schema::stones;
use chrono::NaiveDateTime;
use diesel::Queryable;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Stone {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
