use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize , Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub amount : i64,
    pub currency: Currency,
    pub status : TransactionStatus,
    pub created_at: chrono::DateTime<Utc>,
    pub update_at : chrono::DateTime<Utc>
}

#[derive(Debug, Serialize , Deserialize)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Refunded
}

#[derive(Debug , Serialize , Deserialize)]
pub enum Currency {
    USD,
    EURO,
    INR   
}

impl Default for Transaction {
    fn default() -> Self {
        Self { 
            id: Uuid::new_v4(), 
            amount: 0, 
            currency: Currency::USD,
            status: TransactionStatus::Pending, 
            created_at: Utc::now(), 
            update_at: Utc::now()
        }
    }
}