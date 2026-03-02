use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    pub num_epochs: usize,
    pub batch_size: usize,
    pub learning_rate: f64,
    pub max_seq_length: usize,
    pub num_layers: usize,
    pub d_model: usize,
    pub n_heads: usize,
    pub checkpoint_dir: String,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            num_epochs: 3,
            batch_size: 2,
            learning_rate: 5e-5,
            max_seq_length: 128,
            num_layers: 6,  // Meets assignment minimum
            d_model: 64,
            n_heads: 2,
            checkpoint_dir: "./out/checkpoints".into(),
        }
    }
}