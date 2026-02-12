use crate::shared::constants::{
    MAX_ERR_MSG, MAX_TRACE_ID, MAX_WS_ID, RESPONSE_BUFFER_LEN, SHM_VAULT_PREFIX,
};
use libc::{
    c_void, close, fstat, mmap, shm_open, MAP_FAILED, MAP_SHARED, O_RDWR, PROT_READ, PROT_WRITE,
};
use std::ffi::CString;
use std::mem::{align_of, size_of};
use std::ptr;

#[repr(C)]
pub struct Vault {
    pub status: u32,
    pub energy_quota: u32,
    pub energy_consumed: u32,
    pub workspace_id: [u8; MAX_WS_ID],
    pub trace_id: [u8; MAX_TRACE_ID],
    pub authority_lock: u8,
    pub _pad0: [u8; 3],
    pub last_command_id: u32,
    pub command_seq: u32,
    pub last_processed_seq: u32,
    pub last_result: u32,
    pub response_buffer: [u8; RESPONSE_BUFFER_LEN],
    pub last_error: [u8; MAX_ERR_MSG],
    pub logical_clock: u64,
}

pub struct VaultBridge {
    name: String,
    ptr: *mut Vault,
    size: usize,
}

impl VaultBridge {
    pub fn attach(workspace_id: &str) -> Result<Self, String> {
        let shm_name = format!("{}{}", SHM_VAULT_PREFIX, workspace_id);
        let c_name = CString::new(shm_name.clone()).map_err(|_| "Invalid SHM name")?;
        unsafe {
            let fd = shm_open(c_name.as_ptr(), O_RDWR, 0);
            if fd < 0 {
                return Err(format!(
                    "shm_open failed for {} (vault not found?)",
                    shm_name
                ));
            }
            let size = size_of::<Vault>();
            let mut st: libc::stat = std::mem::zeroed();
            if fstat(fd, &mut st) != 0 {
                close(fd);
                return Err("fstat failed".to_string());
            }
            if (st.st_size as usize) < size {
                close(fd);
                return Err(format!(
                    "vault size mismatch (expected >= {}, got {})",
                    size, st.st_size
                ));
            }

            let ptr = mmap(
                ptr::null_mut(),
                size,
                PROT_READ | PROT_WRITE,
                MAP_SHARED,
                fd,
                0,
            );
            close(fd);
            if ptr == MAP_FAILED {
                return Err("mmap failed".to_string());
            }

            Ok(Self {
                name: shm_name,
                ptr: ptr as *mut Vault,
                size,
            })
        }
    }

    pub fn as_mut(&self) -> &mut Vault {
        unsafe { &mut *self.ptr }
    }

    pub fn read_response(&self) -> String {
        let vault = self.as_mut();
        let nul = vault
            .response_buffer
            .iter()
            .position(|b| *b == 0)
            .unwrap_or(RESPONSE_BUFFER_LEN);
        String::from_utf8_lossy(&vault.response_buffer[..nul]).to_string()
    }

    pub fn clear_response(&self) {
        let vault = self.as_mut();
        vault.response_buffer[0] = 0;
        vault.last_result = 0;
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Drop for VaultBridge {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.ptr as *mut c_void, self.size);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::constants::RESPONSE_BUFFER_LEN;
    use memoffset::offset_of;

    #[test]
    fn vault_layout_matches_expected() {
        let expected_size = 1448;
        assert_eq!(size_of::<Vault>(), expected_size);
        assert_eq!(align_of::<Vault>(), 8);

        assert_eq!(offset_of!(Vault, status), 0);
        assert_eq!(offset_of!(Vault, last_command_id), 144);
        assert_eq!(offset_of!(Vault, command_seq), 148);
        assert_eq!(offset_of!(Vault, last_processed_seq), 152);
        assert_eq!(offset_of!(Vault, response_buffer), 160);
        assert_eq!(offset_of!(Vault, logical_clock), 1440);
    }
}
