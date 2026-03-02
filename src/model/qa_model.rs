
use burn::module::Module;
use burn::tensor::backend::Backend;
use burn::config::Config;
use burn::tensor::{Tensor, Int};
use std::marker::PhantomData;
use super::encoder::{Encoder, EncoderConfig};  // Now works because mod.rs exports them

#[derive(Module, Debug)]
pub struct QAModel<B: Backend> {
    encoder: Encoder<B>,
    _phantom: PhantomData<B>,
}

#[derive(Config, Debug)]
pub struct QAModelConfig {
    pub encoder: EncoderConfig,
    #[config(default = "64")] pub d_model: usize,
}

impl QAModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> QAModel<B> {
        QAModel {
            encoder: self.encoder.init(device),
            _phantom: PhantomData,
        }
    }
}

impl<B: Backend> QAModel<B> {
    pub fn forward(&self, ids: Tensor<B, 2, Int>) -> (Tensor<B, 2>, Tensor<B, 2>) {
        
        let _encoded = self.encoder.forward(ids);
        unimplemented!("Stub - production would compute start/end span logits")
    }
}