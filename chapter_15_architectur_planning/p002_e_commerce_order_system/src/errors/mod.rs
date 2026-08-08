// errors/mod.rs

#[derive(Debug, Clone, PartialEq, Eq)]
/// Enum: Product Specific Errors.
pub enum GeneralErrors {
    FaildToUpdate,
    InvalidRequest { mess: String },
    InsufficientData { mess: String },
    OutOfScope { mess: String },
}

impl std::fmt::Display for GeneralErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeneralErrors::FaildToUpdate => write!(f, "Request Faild Unexpectedly"),
            GeneralErrors::InvalidRequest { mess } => write!(f, "Invalid Request: {}", mess),
            GeneralErrors::InsufficientData { mess } => write!(f, "Insufficient Data : {}", mess),
            GeneralErrors::OutOfScope { mess } => write!(f, "Out of scope: {}", mess),
        }
    }
}
impl std::error::Error for GeneralErrors {}
