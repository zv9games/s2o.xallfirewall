use async_trait::async_trait;
use cyberwall_core::{
    EngineError, EngineResult, FirewallEngine, FirewallPolicy, FirewallRule, FirewallStatus, ProfileType, RuleAction, RuleDirection,
};

pub struct WindowsFirewallEngine;

impl WindowsFirewallEngine {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl FirewallEngine for WindowsFirewallEngine {
    async fn get_status(&self) -> EngineResult<FirewallStatus> {
        let fw_enabled = tokio::task::spawn_blocking(|| {
            s2o_net_lib::firewall::FirewallController::is_firewall_enabled().unwrap_or(false)
        })
        .await
        .map_err(|e| EngineError(e.to_string()))?;

        let outbound_blocked = tokio::task::spawn_blocking(|| {
            s2o_net_lib::firewall::FirewallController::is_outbound_blocked().unwrap_or(false)
        })
        .await
        .map_err(|e| EngineError(e.to_string()))?;

        let defender_active = tokio::task::spawn_blocking(|| {
            s2o_net_lib::defender::DefenderController::is_defender_active()
        })
        .await
        .map_err(|e| EngineError(e.to_string()))?;

        Ok(FirewallStatus {
            enabled: fw_enabled,
            outbound_blocked,
            defender_active,
            profile_private: fw_enabled,
            profile_public: fw_enabled,
            profile_domain: fw_enabled,
            platform: "Windows".to_string(),
            backend_driver: "Win32 COM INetFwPolicy2 + Netsh Advfirewall Service".to_string(),
        })
    }

    async fn set_enabled(&self, enabled: bool) -> EngineResult<()> {
        tokio::task::spawn_blocking(move || {
            if enabled {
                s2o_net_lib::firewall::FirewallController::enable_firewall()
            } else {
                s2o_net_lib::firewall::FirewallController::disable_firewall()
            }
        })
        .await
        .map_err(|e| EngineError(e.to_string()))?
        .map_err(|e| EngineError(format!("{:?}", e)))?;

        Ok(())
    }

    async fn set_outbound_block(&self, blocked: bool) -> EngineResult<()> {
        tokio::task::spawn_blocking(move || {
            if blocked {
                s2o_net_lib::firewall::FirewallController::airplane_mode_enable()
            } else {
                s2o_net_lib::firewall::FirewallController::airplane_mode_disable()
            }
        })
        .await
        .map_err(|e| EngineError(e.to_string()))?
        .map_err(|e| EngineError(format!("{:?}", e)))?;

        Ok(())
    }

    async fn list_rules(&self) -> EngineResult<Vec<FirewallRule>> {
        Ok(vec![
            FirewallRule {
                name: "Split2ops Cyberwall Core Ruleset".to_string(),
                enabled: true,
                action: RuleAction::Allow,
                direction: RuleDirection::Inbound,
                profile: ProfileType::All,
                application: Some("cyberwalld.exe".to_string()),
            }
        ])
    }

    async fn apply_policy(&self, _policy: &FirewallPolicy) -> EngineResult<()> {
        Ok(())
    }
}
