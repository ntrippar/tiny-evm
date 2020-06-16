//  Count Allocation
#[cfg(feature = "alloc_counter")]
use alloc_counter::AllocCounterSystem;

// replace the system allocator for the alloc_counter one
#[cfg_attr(feature = "alloc_counter", global_allocator)]
static A: AllocCounterSystem = AllocCounterSystem;

mod bytecode;
mod context;
mod evm;
mod execution_error;
mod i256;
mod memory;
mod opcode_handlers;
mod opcodes;
mod stack;
mod vm;

pub use crate::context::{BlockContext, CallContext};
pub use bytecode::Bytecode;
pub use evm::run;
