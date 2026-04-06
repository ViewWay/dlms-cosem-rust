//! Block transfer support for DLMS/COSEM (Green Book 9.4.6/9.5/9.6)
//!
//! Implements block transfer for GET, NEXT, ACTION, and data transmission using blocks.

use alloc::vec::Vec;
use core::fmt;

/// Block transfer state machine states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockTransferState {
    /// No active block transfer
    Idle,
    /// Currently transmitting blocks
    Transmitting,
    /// Last block received
    LastBlockReceived,
    /// Block received
    BlockReceived,
    /// Waiting for next block
    WaitingForNextBlock,
    /// Error occurred
    Error,
    /// Timeout occurred
    Timeout,
    /// Transfer complete
    Complete,
}

/// A single data block
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    /// Block number (0-based)
    pub block_number: u32,
    /// Block data
    pub data: Vec<u8>,
    /// Is this the last block?
    pub last_block: bool,
}

impl Block {
    /// Create a new block
    pub fn new(block_number: u32, data: Vec<u8>, last_block: bool) -> Self {
        Self {
            block_number,
            data,
            last_block,
        }
    }
}

/// Block transfer error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockTransferError {
    /// Cannot perform operation in current state
    InvalidState { state: BlockTransferState },
    /// Block number mismatch
    BlockNumberMismatch { expected: u32, got: u32 },
    /// Cannot reassemble incomplete transfer
    IncompleteTransfer,
    /// Block sequence error
    SequenceError { expected: u32, got: u32 },
}

impl fmt::Display for BlockTransferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidState { state } => write!(f, "Invalid state: {:?}", state),
            Self::BlockNumberMismatch { expected, got } => {
                write!(f, "Expected block {}, got {}", expected, got)
            }
            Self::IncompleteTransfer => write!(f, "Cannot reassemble incomplete transfer"),
            Self::SequenceError { expected, got } => {
                write!(f, "Sequence error: expected {}, got {}", expected, got)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BlockTransferError {}

/// Block transfer manager
#[derive(Debug)]
pub struct BlockTransfer {
    state: BlockTransferState,
    current_block_number: u32,
    received_blocks: Vec<Block>,
    is_complete: bool,
    total_expected_length: Option<u32>,
    timeout_ms: u32,
}

impl Default for BlockTransfer {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockTransfer {
    /// Create a new block transfer manager
    pub fn new() -> Self {
        Self {
            state: BlockTransferState::Idle,
            current_block_number: 0,
            received_blocks: Vec::new(),
            is_complete: false,
            total_expected_length: None,
            timeout_ms: 5000, // 5 second default
        }
    }

    /// Reset the transfer state
    pub fn reset(&mut self) {
        self.state = BlockTransferState::Idle;
        self.current_block_number = 0;
        self.received_blocks.clear();
        self.is_complete = false;
        self.total_expected_length = None;
    }

    /// Start a new block transfer
    pub fn start_transfer(&mut self, expected_length: Option<u32>) -> Result<(), BlockTransferError> {
        if self.state != BlockTransferState::Idle {
            return Err(BlockTransferError::InvalidState {
                state: self.state,
            });
        }

        self.state = BlockTransferState::Transmitting;
        self.current_block_number = 0;
        self.is_complete = false;
        self.total_expected_length = expected_length;

        Ok(())
    }

    /// Add a received block
    pub fn add_block(&mut self, block: Block) -> Result<(), BlockTransferError> {
        if self.state != BlockTransferState::Transmitting
            && self.state != BlockTransferState::WaitingForNextBlock
        {
            return Err(BlockTransferError::InvalidState {
                state: self.state,
            });
        }

        if block.block_number != self.current_block_number {
            return Err(BlockTransferError::BlockNumberMismatch {
                expected: self.current_block_number,
                got: block.block_number,
            });
        }

        let last_block = block.last_block;
        self.received_blocks.push(block);
        self.current_block_number += 1;

        if last_block {
            self.state = BlockTransferState::Complete;
            self.is_complete = true;
        } else {
            self.state = BlockTransferState::WaitingForNextBlock;
        }

        Ok(())
    }

    /// Get reassembled data if transfer is complete
    pub fn get_data(&self) -> Option<Vec<u8>> {
        if !self.is_complete {
            return None;
        }

        let mut data = Vec::new();
        for block in &self.received_blocks {
            data.extend_from_slice(&block.data);
        }

        Some(data)
    }

    /// Get a specific block by block number
    pub fn get_block(&self, block_number: u32) -> Option<&Block> {
        for block in &self.received_blocks {
            if block.block_number == block_number {
                return Some(block);
            }
        }
        None
    }

    /// Reassemble all received blocks into complete data
    pub fn reassemble(&self) -> Result<Vec<u8>, BlockTransferError> {
        if !self.is_complete {
            return Err(BlockTransferError::IncompleteTransfer);
        }

        let mut data = Vec::new();
        for block in &self.received_blocks {
            data.extend_from_slice(&block.data);
        }

        Ok(data)
    }

    /// Get total number of transmitted blocks
    pub fn get_total_transmitted(&self) -> usize {
        self.received_blocks.len()
    }

    /// Get transfer progress
    pub fn get_progress(&self) -> TransferProgress {
        TransferProgress {
            state: self.state,
            current_block: self.current_block_number,
            total_blocks: self.received_blocks.len() as u32,
            is_complete: self.is_complete,
        }
    }

    /// Validate that block sequence numbers are correct
    pub fn validate_sequence_numbers(&self) -> Result<(), BlockTransferError> {
        let mut expected_number: u32 = 0;
        for block in &self.received_blocks {
            if block.block_number != expected_number {
                return Err(BlockTransferError::SequenceError {
                    expected: expected_number,
                    got: block.block_number,
                });
            }
            expected_number += 1;
        }

        Ok(())
    }

    /// Set the timeout in milliseconds
    pub fn set_timeout(&mut self, timeout_ms: u32) {
        self.timeout_ms = timeout_ms;
    }

    /// Get the timeout in milliseconds
    pub fn get_timeout(&self) -> u32 {
        self.timeout_ms
    }

    /// Get the current state
    pub fn get_state(&self) -> BlockTransferState {
        self.state
    }

    /// Check if transfer is complete
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }
}

/// Transfer progress information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransferProgress {
    /// Current state
    pub state: BlockTransferState,
    /// Current block number
    pub current_block: u32,
    /// Total blocks received
    pub total_blocks: u32,
    /// Is transfer complete?
    pub is_complete: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_block_transfer() {
        let bt = BlockTransfer::new();
        assert_eq!(bt.state, BlockTransferState::Idle);
        assert_eq!(bt.current_block_number, 0);
        assert_eq!(bt.received_blocks.len(), 0);
        assert!(!bt.is_complete);
    }

    #[test]
    fn test_start_transfer() {
        let mut bt = BlockTransfer::new();
        bt.start_transfer(Some(1000)).unwrap();

        assert_eq!(bt.state, BlockTransferState::Transmitting);
        assert_eq!(bt.current_block_number, 0);
        assert_eq!(bt.total_expected_length, Some(1000));
    }

    #[test]
    fn test_start_transfer_twice_fails() {
        let mut bt = BlockTransfer::new();
        bt.start_transfer(None).unwrap();

        let result = bt.start_transfer(None);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_single_block() {
        let mut bt = BlockTransfer::new();
        bt.start_transfer(None).unwrap();

        let block = Block::new(0, b"hello".to_vec(), true);
        bt.add_block(block).unwrap();

        assert_eq!(bt.state, BlockTransferState::Complete);
        assert!(bt.is_complete);
        assert_eq!(bt.received_blocks.len(), 1);
    }

    #[test]
    fn test_add_multiple_blocks() {
        let mut bt = BlockTransfer::new();
        bt.start_transfer(None).unwrap();

        let block1 = Block::new(0, b"part1".to_vec(), false);
        bt.add_block(block1).unwrap();

        assert_eq!(bt.state, BlockTransferState::WaitingForNextBlock);
        assert_eq!(bt.current_block_number, 1);

        let block2 = Block::new(1, b"part2".to_vec(), true);
        bt.add_block(block2).unwrap();

        assert_eq!(bt.state, BlockTransferState::Complete);
        assert!(bt.is_complete);
        assert_eq!(bt.received_blocks.len(), 2);
    }

    #[test]
    fn test_add_block_wrong_number_fails() {
        let mut bt = BlockTransfer::new();
        bt.start_transfer(None).unwrap();

        let block = Block::new(5, b"test".to_vec(), false);
        let result = bt.add_block(block);

        assert!(result.is_err());
    }

    #[test]
    fn test_reassemble_complete() {
        let mut bt = BlockTransfer::new();
        bt.start_transfer(None).unwrap();

        bt.add_block(Block::new(0, b"Hello ".to_vec(), false)).unwrap();
        bt.add_block(Block::new(1, b"World".to_vec(), true)).unwrap();

        let data = bt.reassemble().unwrap();
        assert_eq!(data, b"Hello World");
    }

    #[test]
    fn test_reassemble_incomplete_fails() {
        let mut bt = BlockTransfer::new();
        bt.start_transfer(None).unwrap();

        bt.add_block(Block::new(0, b"part".to_vec(), false)).unwrap();

        let result = bt.reassemble();
        assert!(result.is_err());
    }

    #[test]
    fn test_get_data_complete() {
        let mut bt = BlockTransfer::new();
        bt.start_transfer(None).unwrap();

        bt.add_block(Block::new(0, b"data1".to_vec(), false)).unwrap();
        bt.add_block(Block::new(1, b"data2".to_vec(), true)).unwrap();

        let data = bt.get_data();
        assert_eq!(data, Some(b"data1data2".to_vec()));
    }

    #[test]
    fn test_get_data_incomplete() {
        let mut bt = BlockTransfer::new();
        bt.start_transfer(None).unwrap();

        bt.add_block(Block::new(0, b"part".to_vec(), false)).unwrap();

        let data = bt.get_data();
        assert_eq!(data, None);
    }

    #[test]
    fn test_reset() {
        let mut bt = BlockTransfer::new();
        bt.start_transfer(None).unwrap();
        bt.add_block(Block::new(0, b"test".to_vec(), false)).unwrap();

        bt.reset();

        assert_eq!(bt.state, BlockTransferState::Idle);
        assert_eq!(bt.current_block_number, 0);
        assert_eq!(bt.received_blocks.len(), 0);
        assert!(!bt.is_complete);
    }

    #[test]
    fn test_validate_sequence_numbers_correct() {
        let mut bt = BlockTransfer::new();
        bt.start_transfer(None).unwrap();

        bt.add_block(Block::new(0, b"a".to_vec(), false)).unwrap();
        bt.add_block(Block::new(1, b"b".to_vec(), false)).unwrap();
        bt.add_block(Block::new(2, b"c".to_vec(), true)).unwrap();

        assert!(bt.validate_sequence_numbers().is_ok());
    }

    #[test]
    fn test_validate_sequence_numbers_incorrect() {
        let mut bt = BlockTransfer::new();
        bt.start_transfer(None).unwrap();

        bt.received_blocks.push(Block::new(0, b"a".to_vec(), false));
        // Skip block 1
        bt.received_blocks.push(Block::new(2, b"c".to_vec(), true));

        let result = bt.validate_sequence_numbers();
        assert!(result.is_err());
    }

    #[test]
    fn test_large_data_split() {
        let mut bt = BlockTransfer::new();
        bt.start_transfer(Some(300)).unwrap();

        let block1 = Block::new(0, vec![b'x'; 100], false);
        let block2 = Block::new(1, vec![b'y'; 100], false);
        let block3 = Block::new(2, vec![b'z'; 100], true);

        bt.add_block(block1).unwrap();
        bt.add_block(block2).unwrap();
        bt.add_block(block3).unwrap();

        assert!(bt.is_complete);
        let data = bt.reassemble().unwrap();
        assert_eq!(data.len(), 300);
    }
}
