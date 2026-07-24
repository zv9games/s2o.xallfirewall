use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfileType {
    Private,
    Public,
    Domain,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleAction {
    Allow,
    Block,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleDirection {
    Inbound,
    Outbound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallStatus {
    pub enabled: bool,
    pub outbound_blocked: bool,
    pub defender_active: bool,
    pub profile_private: bool,
    pub profile_public: bool,
    pub profile_domain: bool,
    pub platform: String,
    pub backend_driver: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallRule {
    pub name: String,
    pub enabled: bool,
    pub action: RuleAction,
    pub direction: RuleDirection,
    pub profile: ProfileType,
    pub application: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallPolicy {
    pub name: String,
    pub version: String,
    pub rules: Vec<FirewallRule>,
}
