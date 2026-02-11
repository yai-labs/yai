// src/bridge/shm.rs
#![allow(dead_code)]
use std::ptr;
use libc::{shm_open, mmap, O_RDONLY, PROT_READ, MAP_SHARED, close};
use std::ffi::CString;

#[derive(Debug, Clone)]
pub struct VaultState {
    pub status: u32,
    pub energy_quota: u32,
    pub energy_consumed: u32,
    pub vault_name: String,
}

pub struct VaultBridge {
    name: String,
    ptr: *mut libc::c_void,
    size: usize,
}

impl VaultBridge {
    pub fn new(vault_name: &str) -> Result<Self, String> {
        let c_name = CString::new(vault_name).map_err(|_| "Invalid name")?;
        
        unsafe {
            // 1. Apri la memoria condivisa (shm_open)
            let fd = shm_open(c_name.as_ptr(), O_RDONLY, 0);
            if fd < 0 {
                return Err(format!("[BRIDGE-ERROR] Impossibile aprire SHM: {}", vault_name));
            }

            // 2. Mappa la memoria nell'indirizzo dello spazio del processo
            let size = 12; // 3 * uint32 (4 bytes ciascuno)
            let ptr = mmap(ptr::null_mut(), size, PROT_READ, MAP_SHARED, fd, 0);
            
            // Chiudiamo il descrittore del file, la mappatura resta attiva
            close(fd);

            if ptr == libc::MAP_FAILED {
                return Err("Mmap failed".to_string());
            }

            Ok(Self {
                name: vault_name.to_string(),
                ptr,
                size,
            })
        }
    }

    pub fn read_live(&self) -> VaultState {
        unsafe {
            // Leggiamo i dati direttamente tramite pointer offset (Zero-copy)
            let status = *(self.ptr as *const u32);
            let energy_quota = *(self.ptr.add(4) as *const u32);
            let energy_consumed = *(self.ptr.add(8) as *const u32);

            VaultState {
                status,
                energy_quota,
                energy_consumed,
                vault_name: self.name.clone(),
            }
        }
    }
}

// Implementiamo il Drop per pulire la memoria quando l'oggetto viene distrutto
impl Drop for VaultBridge {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.ptr, self.size);
        }
    }
}
