
use axum::async_trait;

/// A trait for validating authentication tokens.
#[async_trait]
pub trait TokenValidator: Send + Sync {
    async fn validate(&self, token: &str) -> bool;
}

/// A default token validator that accepts any token.
#[derive(Clone)]
pub struct AllowAllValidator;

#[async_trait]
impl TokenValidator for AllowAllValidator {
    async fn validate(&self, _token: &str) -> bool {
        true
    }
}

/// A closure-based token validator.
pub struct ClosureValidator {
    validate_fn: Box<dyn Fn(String) -> bool + Send + Sync>,
}

impl ClosureValidator {
    pub fn new<F>(validate_fn: F) -> Self
    where
        F: Fn(String) -> bool + Send + Sync + 'static,
    {
        Self { validate_fn: Box::new(validate_fn) }
    }
}

#[async_trait]
impl TokenValidator for ClosureValidator {
    async fn validate(&self, token: &str) -> bool {
        (self.validate_fn)(token.to_string())
    }
}
