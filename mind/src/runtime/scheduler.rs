use crate::runtime::protocol::{CommandId, EngineStatus};
use crate::shared::constants::YAI_STATE_READY;
use crate::transport::bridge::vault::VaultBridge;
use std::time::{Duration, Instant};

pub struct Scheduler {
    vault: VaultBridge,
}

impl Scheduler {
    pub fn new(vault: VaultBridge) -> Self {
        Self { vault }
    }

    pub fn send_command_and_wait(
        &self,
        command: CommandId,
        timeout: Duration,
    ) -> Result<String, String> {
        let vault = self.vault.as_mut();
        vault.command_seq = vault.command_seq.wrapping_add(1);
        let seq = vault.command_seq;
        vault.last_command_id = command as u32;
        vault.last_result = 0;
        vault.response_buffer[0] = 0;

        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            if vault.last_processed_seq == seq
                && vault.last_result != 0
                && vault.status == YAI_STATE_READY
            {
                return Ok(self.vault.read_response());
            }
            std::thread::sleep(Duration::from_millis(20));
        }

        Err("timeout waiting for engine response".to_string())
    }

    pub fn engine_status(&self) -> EngineStatus {
        EngineStatus::from_raw(self.vault.as_mut().status)
    }

    pub fn vault_name(&self) -> &str {
        self.vault.name()
    }
}
