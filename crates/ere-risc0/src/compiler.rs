use risc0_zkvm::Digest;
use serde::{Deserialize, Serialize};

mod rust_rv32ima;
mod rust_rv32ima_customized;

pub use rust_rv32ima::RustRv32ima;
pub use rust_rv32ima_customized::RustRv32imaCustomized;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Risc0Program {
    pub(crate) elf: Vec<u8>,
    pub(crate) image_id: Digest,
}


impl Risc0Program {
    pub fn new(elf: Vec<u8>, image_id: Digest) -> Self {
        Self { elf, image_id }
    }
}
