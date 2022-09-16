pub mod block;
pub use crate::block::Block;
pub mod hashable;
pub use crate::hashable::Hashable;

type BlockHash = Vec<u8>;
