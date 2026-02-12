use yai_mind::transport::bridge::shm::VaultBridge;

#[test]
fn test_vault_connection() {
    // Cerchiamo di connetterci alla SHM del Kernel
    // Nota: Il Kernel C deve essere attivo o aver creato il file in /dev/shm
    let bridge = VaultBridge::new("/yai_vault_arch_dev_session");

    match bridge {
        Ok(b) => {
            let state = b.read_live();
            println!("Vault Letto: {}", state.vault_name);
            println!("Energy: {}/{}", state.energy_consumed, state.energy_quota);
            assert!(state.energy_consumed <= state.energy_quota);
        }
        Err(e) => panic!("Fallimento connessione SHM: {}", e),
    }
}
