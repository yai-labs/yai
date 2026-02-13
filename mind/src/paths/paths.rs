use std::path::PathBuf;

use super::runtime_layout::RuntimeLayout;

/// Facade “Paths” per compat e per evitare import sparsi.
/// (in futuro ci metti dentro anche config root, datasets root, ecc.)
#[derive(Clone, Debug)]
pub struct Paths {
    pub runtime: RuntimeLayout,
}

impl Paths {
    pub fn new(runtime_root: PathBuf) -> Self {
        Self {
            runtime: RuntimeLayout::new(runtime_root),
        }
    }

    /// Fonte unica del runtime root: env (per ora), poi config injected.
    pub fn from_env() -> Self {
        let root = std::env::var("YAI_RUNTIME_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."));
        Self::new(root)
    }
}
