pub mod handler;

use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Opcode {
  opcode: i16,
  mnemonic: String,
  addressing_mode: String,
  bytes: i32,
  cycles: String,
  flags: String,
}