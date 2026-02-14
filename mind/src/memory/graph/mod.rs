// src/memory/graph/mod.rs
pub mod backend;
pub mod backend_rpc;
pub mod facade;
pub mod ids;
pub mod domains;
pub mod stores; // Solo se vuoi tenere il modulo, ma svuotalo dai file _local

// Esponi la Facade per il resto del Mind
pub use facade::GraphFacade;