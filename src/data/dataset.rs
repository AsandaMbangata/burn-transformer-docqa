
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QAItem {
    pub input_ids: Vec<i64>,
    pub start_pos: usize,
    pub end_pos: usize,
}

pub struct QADataset {
    pub items: Vec<QAItem>,
}

impl QADataset {
    pub fn new(items: Vec<QAItem>) -> Self {
        Self { items }
    }
    pub fn len(&self) -> usize { self.items.len() }
    pub fn get(&self, idx: usize) -> Option<&QAItem> { self.items.get(idx) }
}

pub struct QABatch {
    pub input_ids: Vec<Vec<i64>>,
    pub starts: Vec<i64>,
    pub ends: Vec<i64>,
}

pub fn collate(items: Vec<QAItem>) -> QABatch {
    QABatch {
        input_ids: items.iter().map(|i| i.input_ids.clone()).collect(),
        starts: items.iter().map(|i| i.start_pos as i64).collect(),
        ends: items.iter().map(|i| i.end_pos as i64).collect(),
    }
}