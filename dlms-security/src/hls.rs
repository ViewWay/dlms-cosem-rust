//! HLS-ISM (High Level Security for Smart Metering) authentication

use crate::{SecurityError, SecuritySuite};

/// HLS authentication step
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HlsStep {
    Idle,
    RequestSent,
    ChallengeReceived,
    Authenticated,
    Failed,
}

/// HLS authentication result
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HlsAuthResult {
    pub success: bool,
    pub server_challenge: Option<[u8; 8]>,
    pub client_challenge: Option<[u8; 8]>,
}

/// HLS-ISM context for authentication
pub struct HlsContext {
    suite: SecuritySuite,
    step: HlsStep,
    key: Option<[u8; 16]>,
    client_challenge: Option<[u8; 8]>,
    server_challenge: Option<[u8; 8]>,
}

impl HlsContext {
    pub fn new(suite: SecuritySuite) -> Self {
        Self {
            suite,
            step: HlsStep::Idle,
            key: None,
            client_challenge: None,
            server_challenge: None,
        }
    }

    pub fn suite(&self) -> SecuritySuite {
        self.suite
    }

    pub fn step(&self) -> HlsStep {
        self.step
    }

    pub fn set_key(&mut self, key: [u8; 16]) {
        self.key = Some(key);
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    /// Start HLS authentication - generates client challenge
    pub fn start(&mut self) -> Result<[u8; 8], SecurityError> {
        let challenge = [0x42u8; 8]; // Simplified: use fixed challenge
        self.client_challenge = Some(challenge);
        self.step = HlsStep::RequestSent;
        Ok(challenge)
    }

    /// Process server challenge response
    pub fn process_server_challenge(&mut self, server_challenge: &[u8; 8]) -> Result<HlsAuthResult, SecurityError> {
        self.server_challenge = Some(*server_challenge);

        if let Some(key) = self.key {
            // Simplified: just verify we have a key
            let _ = key;
            self.step = HlsStep::Authenticated;
            Ok(HlsAuthResult {
                success: true,
                server_challenge: Some(*server_challenge),
                client_challenge: self.client_challenge,
            })
        } else {
            self.step = HlsStep::Failed;
            Ok(HlsAuthResult {
                success: false,
                server_challenge: Some(*server_challenge),
                client_challenge: self.client_challenge,
            })
        }
    }
}
