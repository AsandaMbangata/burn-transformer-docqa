use tokenizers::Tokenizer;
use anyhow::{Result, anyhow};

pub struct QATokenizer {
    tokenizer: Tokenizer,
    max_length: usize,
}

impl QATokenizer {
    pub fn new(path: &str, max_length: usize) -> Result<Self> {
        let tokenizer = Tokenizer::from_file(path)
            .map_err(|e| anyhow!("Tokenizer error: {}", e))?;
        Ok(Self { tokenizer, max_length })
    }

    pub fn encode(&self, text: &str) -> Result<Vec<i64>> {
        let encoding = self.tokenizer.encode(text, true)
            .map_err(|e| anyhow!("Encode failed: {}", e))?;
        let mut ids: Vec<i64> = encoding.get_ids().iter().map(|&x| x as i64).collect();
        if ids.len() > self.max_length {
            ids.truncate(self.max_length);
        } else {
            ids.resize(self.max_length, 0);
        }
        Ok(ids)
    }

    pub fn decode(&self, ids: &[i64]) -> Result<String> {
        let ids: Vec<u32> = ids.iter().map(|&x| x as u32).collect();
        self.tokenizer.decode(&ids, true)
            .map_err(|e| anyhow!("Decode failed: {}", e))
    }
}