// file:/src/errors/payment_err.rs

use crate::errors::err_prelud::*;

#[derive(Debug, Deserialize, Serialize, Error)]
pub enum PaymentError {
    #[error("Payment Method not supported.")]
    UnsupportedPaymentMethod,

    #[error("Insufficent Client Funds.")]
    InsufficientClientFunds,

    #[error("Timeout to proced payment, Plase try again.")]
    PaymentTimeout,
    #[error("Subcribtino Not Avialable on your account Please Check our plains.")]
    SubscriptionNotFound,
    #[error("Payment is allready in processing. Please Wait!")]
    PaymentAlreadyProcessed,
    #[error("Transition Not Found Try Again after successfull transitions.")]
    TransactionNotFound,
    #[error("Security Alert Triggered.")]
    SignatureVerificationFailed,

    #[error("Failed to process the payment: {reason}")]
    PaymentFailed { reason: String },
    #[error("Payment Expired.{reason}")]
    PaymentExpired { reason: String },
    #[error("Invalid Amount Intered : {amount}. Please inter valid amount.")]
    InvalidAmount { amount: usize },
    #[error("Refund Faild:{reason}")]
    RefundFailed { reason: String },
    #[error("Unsupported Currency : {currency}")]
    CurrencyNotSupported { currency: String },
    #[error("GateWay Error: {gateway}, {message_code}")]
    GatewayError { gateway: String, message_code: String },
}

impl IntoResponse for PaymentError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            PaymentError::UnsupportedPaymentMethod => (StatusCode::BAD_REQUEST, self.to_string()),
            PaymentError::InsufficientClientFunds => (StatusCode::PAYMENT_REQUIRED, self.to_string()),
            PaymentError::PaymentTimeout => (StatusCode::REQUEST_TIMEOUT, self.to_string()),
            PaymentError::SubscriptionNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            PaymentError::PaymentAlreadyProcessed => (StatusCode::CONFLICT, self.to_string()),
            PaymentError::TransactionNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            PaymentError::SignatureVerificationFailed => (StatusCode::BAD_REQUEST, self.to_string()),
            //

            // Y: Previously used like this.
            //
            // PaymentError::PaymentFailed { reason } => (StatusCode::PAYMENT_REQUIRED, format!("Payment-Failed deu to : {}", reason)),
            // PaymentError::PaymentExpired { reason } => (StatusCode::PAYMENT_REQUIRED, format!("Payment Expired : {}", reason)),
            // PaymentError::InvalidAmount { amount } => (StatusCode::BAD_REQUEST, format!("Invalid Amount {} input", amount)),
            // PaymentError::RefundFailed { reason } => (StatusCode::PAYMENT_REQUIRED, format!("Payment Processing Error: {}", reason)),
            // PaymentError::CurrencyNotSupported { currency } => (StatusCode::BAD_REQUEST, format!("Currency Not Supported: {}", currency)),
            // PaymentError::GatewayError { gateway, message_code } => (StatusCode::BAD_GATEWAY, format!("Bad GateWay: {}, Message: {}", gateway, message_code)),
            PaymentError::PaymentFailed { .. } => (StatusCode::PAYMENT_REQUIRED, self.to_string()),
            PaymentError::PaymentExpired { .. } => (StatusCode::PAYMENT_REQUIRED, self.to_string()),
            PaymentError::InvalidAmount { .. } => (StatusCode::BAD_REQUEST, self.to_string()),
            PaymentError::RefundFailed { .. } => (StatusCode::PAYMENT_REQUIRED, self.to_string()),
            PaymentError::CurrencyNotSupported { .. } => (StatusCode::BAD_REQUEST, self.to_string()),
            PaymentError::GatewayError { .. } => (StatusCode::BAD_GATEWAY, self.to_string()),
        };
        (status, Json(ErrorResponse { status: "Fail".to_string(), message, code: status.as_u16() })).into_response()
    }
}
