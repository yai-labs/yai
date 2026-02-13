use std::collections::HashMap;

pub struct VectorIndex {
    dim: usize,
    vectors: HashMap<String, Vec<f32>>,
}

impl VectorIndex {
    pub fn new(dim: usize) -> Self {
        Self {
            dim,
            vectors: HashMap::new(),
        }
    }

    pub fn build(&mut self, items: &[(String, Vec<f32>)]) {
        self.vectors.clear();
        for (id, vec) in items.iter() {
            if vec.len() != self.dim {
                continue;
            }
            self.vectors.insert(id.clone(), vec.clone());
        }
    }

    pub fn search(&self, query: &[f32], k: usize) -> Vec<(String, f32)> {
        let mut scored: Vec<(String, f32)> = self
            .vectors
            .iter()
            .map(|(id, v)| (id.clone(), cosine(query, v)))
            .collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(k);
        scored
    }
}

fn cosine(a: &[f32], b: &[f32]) -> f32 {
    let mut dot = 0.0f32;
    let mut na = 0.0f32;
    let mut nb = 0.0f32;
    for (x, y) in a.iter().zip(b.iter()) {
        dot += x * y;
        na += x * x;
        nb += y * y;
    }
    if na == 0.0 || nb == 0.0 {
        return 0.0;
    }
    dot / (na.sqrt() * nb.sqrt())
}
