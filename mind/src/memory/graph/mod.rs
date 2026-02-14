pub mod backend;       // Factory (store_for_scope)
pub mod backend_rpc;   // Implementazione RPC (quella che abbiamo scritto)
pub mod facade;        // Punto d'ingresso (Logic)
pub mod domains;       // Solo logica di dominio (senza store fisici)
pub mod ids;           // Generazione ID