use crate::models::{FirewallPolicy, FirewallRule, FirewallStatus};
use async_trait::async_trait;
use std::fmt;

#[derive(Debug)]
pub struct EngineError(pub String);

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for EngineError {}

pub type EngineResult<T> = Result<T, EngineError>;

#[async_trait]
pub trait FirewallEngine: Send + Sync {
    /// Returns the current OS firewall status & profile states
    async fn get_status(&self) -> EngineResult<FirewallStatus>;

    /// Enables or disables the OS firewall across all profiles
    async fn set_enabled(&self, enabled: bool) -> EngineResult<()>;

    /// Enables or disables outbound airplane/isolation mode shield
    async fn set_outbound_block(&self, blocked: bool) -> EngineResult<()>;

    /// Lists active OS firewall rules
    async fn list_rules(&self) -> EngineResult<Vec<FirewallRule>>;

    /// Applies a declarative policy configuration
    async fn apply_policy(&self, policy: &FirewallPolicy) -> EngineResult<()>;
}
