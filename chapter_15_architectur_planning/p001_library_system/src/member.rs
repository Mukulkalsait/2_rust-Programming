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

impl Member {
    pub fn new(name: String, email: String, membership: Membership) -> Member {
        Member { member_id: Uuid::new_v4(), name, email, membership, joined_at: Utc::now() }
    }
}
