// members.rs

use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Membership {
    standard,
    premium,
    vip,
}

pub type MemberId = Uuid;

#[derive(Debug, Clone)]
pub struct Member {
    pub member_id: MemberId,
    pub name: String,
    pub email: String,
    pub membership: Membership,
    pub joined_at: chrono::DateTime<Utc>,
}
