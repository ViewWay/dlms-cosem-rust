//! Block transfer support for DLMS/COSEM (Green Book 9.4/9.5/9.6)
//!
//! This module implements block transfer for GET, NEXT, ACTION, and data transmission.
//!
//! Block transfer allows transmitting large data that exceeds the maximum PDU size
//! by splitting it into multiple blocks with sequence numbers.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use core::fmt;

#[cfg(feature = "std")]
use std::time::{Duration, Instant};

/// Block transfer state machine states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockTransferState {
    /// No active transfer
    Idle,
    /// Transfer in progress
    Transmitting,
    /// Last block has been received
    LastBlockReceived,
    /// Block received successfully
    BlockReceived,
    /// Waiting for next block
    WaitingForNextBlock,
    /// Transfer failed with error
    Error,
    /// Transfer timed out
    Timeout,
    /// Transfer completed successfully
    Complete,
}

impl Default for BlockTransferState {
    fn default() -> Self {
        BlockTransferState::Idle
    }
}

impl fmt::Display for BlockTransferState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockTransferState::Idle => write!(f, "IDLE"),
            BlockTransferState::Transmitting => write!(f, "TRANSMITTING"),
            BlockTransferState::LastBlockReceived => write!(f, "LAST_BLOCK_RECEIVED"),
            BlockTransferState::BlockReceived => write!(f, "BLOCK_RECEIVED"),
            BlockTransferState::WaitingForNextBlock => write!(f, "WAITING_FOR_NEXT_BLOCK"),
            BlockTransferState::Error => write!(f, "ERROR"),
            BlockTransferState::Timeout => write!(f, "TIMEOUT"),
            BlockTransferState::Complete => write!(f, "COMPLETE"),
        }
    }
}

/// A single data block in block transfer
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    /// Block number (0-based)
    pub block_number: u32,
    /// Block data
    pub data: Vec<u8>,
    /// Is this the last block?
    pub is_last_block: bool,
}

impl Block {
    /// Create a new block
    pub fn new(block_number: u32, data: Vec<u8>, is_last_block: bool) -> Self {
        Block {
            block_number,
            data,
            is_last_block,
        }
    }
}

/// Block transfer error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockTransferError {
    /// Invalid state for the requested operation
    InvalidState(BlockTransferState),
    /// Block sequence number mismatch
    SequenceMismatch { expected: u32, got: u32 },
    /// Transfer not complete
    TransferIncomplete,
    /// Transfer timed out
    Timeout,
    /// Custom error message
    Custom(String),
}

impl fmt::Display for BlockTransferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockTransferError::InvalidState(state) => {
                write!(f, "Invalid state for operation: {}", state)
            }
            BlockTransferError::SequenceMismatch { expected, got } => {
                write!(f, "Block sequence mismatch: expected {}, got {}", expected, got)
            }
            BlockTransferError::TransferIncomplete => {
                write!(f, "Transfer not complete")
            }
            BlockTransferError::Timeout => {
                write!(f, "Transfer timed out")
            }
            BlockTransferError::Custom(msg) => {
                write!(f, "Block transfer error: {}", msg)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BlockTransferError {}

/// Block transfer manager for DLMS/COSEM
#[derive(Debug, Clone)]
pub struct BlockTransfer {
    /// Current state
    state: BlockTransferState,
    /// Current block number (next expected)
    current_block_number: u32,
    /// Received blocks
    received_blocks: Vec<Block>,
    /// Is transfer complete?
    is_complete: bool,
    /// Total expected length (optional)
    total_expected_length: Option<usize>,
    /// Timeout in milliseconds
    timeout_ms: u32,
    #[cfg(feature = "std")]
    /// Start time for timeout tracking
    start_time: Option<Instant>,
}

impl Default for BlockTransfer {
    fn default() -> Self {
        BlockTransfer {
            state: BlockTransferState::Idle,
            current_block_number: 0,
            received_blocks: Vec::new(),
            is_complete: false,
            total_expected_length: None,
            timeout_ms: 5000,
            #[cfg(feature = "std")]
            start_time: None,
        }
    }
}

impl BlockTransfer {
    /// Create a new block transfer manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Set timeout in milliseconds
    pub fn with_timeout(mut self, timeout_ms: u32) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    /// Reset transfer state
    pub fn reset(&mut self) {
        self.state = BlockTransferState::Idle;
        self.current_block_number = 0;
        self.received_blocks.clear();
        self.is_complete = false;
        self.total_expected_length = None;
        #[cfg(feature = "std")]
        {
            self.start_time = None;
        }
    }

    /// Start a new block transfer
    pub fn start_transfer(&mut self, expected_length: Option<usize>) -> Result<(), BlockTransferError> {
        if self.state != BlockTransferState::Idle {
            return Err(BlockTransferError::InvalidState(self.state));
        }

        self.state = BlockTransferState::Transmitting;
        self.current_block_number = 0;
        self.is_complete = false;
        self.total_expected_length = expected_length;
        #[cfg(feature = "std")]
        {
            self.start_time = Some(Instant::now());
        }

        Ok(())
    }

    /// Add a received block
    pub fn add_block(&mut self, block: Block) -> Result<(), BlockTransferError> {
        if self.state != BlockTransferState::Transmitting
            && self.state != BlockTransferState::WaitingForNextBlock
        {
            return Err(BlockTransferError::InvalidState(self.state));
        }

        if block.block_number != self.current_block_number {
            return Err(BlockTransferError::SequenceMismatch {
                expected: self.current_block_number,
                got: block.block_number,
            });
        }

        let is_last_block = block.is_last_block;
        self.received_blocks.push(block);
        self.current_block_number += 1;

        if is_last_block {
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

        let total_size: usize = self.received_blocks.iter().map(|b| b.data.len()).sum();
        let mut data = Vec::with_capacity(total_size);

        for block in &self.received_blocks {
            data.extend_from_slice(&block.data);
        }

        Some(data)
    }

    /// Get a specific block by block number
    pub fn get_block(&self, block_number: u32) -> Option<&Block> {
        self.received_blocks
            .iter()
            .find(|b| b.block_number == block_number)
    }

    /// Reassemble all received blocks into complete data
    pub fn reassemble(&self) -> Result<Vec<u8>, BlockTransferError> {
        if !self.is_complete {
            return Err(BlockTransferError::TransferIncomplete);
        }

        self.get_data().ok_or(BlockTransferError::TransferIncomplete)
    }

    /// Get total number of transmitted blocks
    pub fn get_total_transmitted(&self) -> usize {
        self.received_blocks.len()
    }

    /// Get transfer progress
    pub fn get_progress(&self) -> BlockTransferProgress {
        BlockTransferProgress {
            state: self.state,
            current_block: self.current_block_number,
            total_blocks: self.received_blocks.len(),
            is_complete: self.is_complete,
        }
    }

    /// Validate that block sequence numbers are correct
    pub fn validate_sequence_numbers(&self) -> Result<(), BlockTransferError> {
        for (index, block) in self.received_blocks.iter().enumerate() {
            if block.block_number != index as u32 {
                return Err(BlockTransferError::SequenceMismatch {
                    expected: index as u32,
                    got: block.block_number,
                });
            }
        }
        Ok(())
    }

    /// Check if transfer has timed out (requires std feature)
    #[cfg(feature = "std")]
    pub fn check_timeout(&self) -> bool {
        if let Some(start_time) = self.start_time {
            if !self.is_complete {
                let elapsed = start_time.elapsed();
                return elapsed > Duration::from_millis(self.timeout_ms as u64);
            }
        }
        false
    }

    /// Get current state
    pub fn state(&self) -> BlockTransferState {
        self.state
    }

    /// Check if transfer is complete
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }
}

/// Block transfer progress information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockTransferProgress {
    /// Current state
    pub state: BlockTransferState,
    /// Current block number
    pub current_block: u32,
    /// Total blocks received
    pub total_blocks: usize,
    /// Is transfer complete?
    pub is_complete: bool,
}

/// Block splitter for sending large data in chunks
#[derive(Debug, Clone)]
pub struct BlockSplitter {
    /// Maximum block size
    max_block_size: usize,
    /// Current block number
    current_block_number: u32,
    /// Data to split
    data: Vec<u8>,
    /// Current position in data
    position: usize,
}

impl BlockSplitter {
    /// Create a new block splitter
    pub fn new(data: Vec<u8>, max_block_size: usize) -> Self {
        BlockSplitter {
            max_block_size,
            current_block_number: 0,
            data,
            position: 0,
        }
    }

    /// Get the next block to send
    pub fn next_block(&mut self) -> Option<Block> {
        if self.position >= self.data.len() {
            return None;
        }

        let end = core::cmp::min(self.position + self.max_block_size, self.data.len());
        let block_data = self.data[self.position..end].to_vec();
        let is_last_block = end >= self.data.len();

        let block = Block::new(self.current_block_number, block_data, is_last_block);

        self.position = end;
        self.current_block_number += 1;

        Some(block)
    }

    /// Check if there are more blocks to send
    pub fn has_more(&self) -> bool {
        self.position < self.data.len()
    }

    /// Get total number of blocks
    pub fn total_blocks(&self) -> usize {
        let data_len = self.data.len();
        if data_len == 0 {
            return 0;
        }
        (data_len + self.max_block_size - 1) / self.max_block_size
    }

    /// Reset splitter to start
    pub fn reset(&mut self) {
        self.current_block_number = 0;
        self.position = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let block = Block::new(0, Vec::from(&[1u8, 2, 3][..]), false);
        assert_eq!(block.block_number, 0);
        assert_eq!(block.data, Vec::from(&[1u8, 2, 3][..]));
        assert_eq!(block.is_last_block, false);
    }

    #[test]
    fn test_block_transfer_new() {
        let bt = BlockTransfer::new();
        assert_eq!(bt.state(), BlockTransferState::Idle);
        assert_eq!(bt.is_complete(), false);
    }

    #[test]
    fn test_block_transfer_start() {
        let mut bt = BlockTransfer::new();
        bt.start_transfer(None).unwrap();
        assert_eq!(bt.state(), BlockTransferState::Transmitting);
    }

    #[test]
    fn test_block_transfer_add_blocks() {
        let mut bt = BlockTransfer::new();
        bt.start_transfer(None).unwrap();

        // Add first block
        let block1 = Block::new(0, Vec::from(&[1u8, 2, 3][..]), false);
        bt.add_block(block1).unwrap();
        assert_eq!(bt.state(), BlockTransferState::WaitingForNextBlock);

        // Add second block (last)
        let block2 = Block::new(1, Vec::from(&[4u8, 5, 6][..]), true);
        bt.add_block(block2).unwrap();
        assert_eq!(bt.state(), BlockTransferState::Complete);
        assert_eq!(bt.is_complete(), true);
    }

    #[test]
    fn test_block_transfer_sequence_mismatch() {
        let mut bt = BlockTransfer::new();
        bt.start_transfer(None).unwrap();

        // Add block with wrong sequence number
        let block = Block::new(5, Vec::from(&[1u8, 2, 3][..]), false);
        let result = bt.add_block(block);
        assert!(matches!(
            result,
            Err(BlockTransferError::SequenceMismatch {
                expected: 0,
                got: 5
            })
        ));
    }

    #[test]
    fn test_block_transfer_reassemble() {
        let mut bt = BlockTransfer::new();
        bt.start_transfer(None).unwrap();

        let block1 = Block::new(0, Vec::from(&[1u8, 2, 3][..]), false);
        let block2 = Block::new(1, Vec::from(&[4u8, 5, 6][..]), true);

        bt.add_block(block1).unwrap();
        bt.add_block(block2).unwrap();

        let data = bt.reassemble().unwrap();
        assert_eq!(data, Vec::from(&[1u8, 2, 3, 4, 5, 6][..]));
    }

    #[test]
    fn test_block_transfer_get_data() {
        let mut bt = BlockTransfer::new();
        bt.start_transfer(None).unwrap();

        // Not complete yet
        assert_eq!(bt.get_data(), None);

        // Add blocks
        let block1 = Block::new(0, Vec::from(&[1u8, 2, 3][..]), false);
        let block2 = Block::new(1, Vec::from(&[4u8, 5, 6][..]), true);
        bt.add_block(block1).unwrap();
        bt.add_block(block2).unwrap();

        // Now complete
        let data = bt.get_data().unwrap();
        assert_eq!(data, Vec::from(&[1u8, 2, 3, 4, 5, 6][..]));
    }

    #[test]
    fn test_block_splitter() {
        let data = Vec::from(&[1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10][..]);
        let mut splitter = BlockSplitter::new(data, 3);

        assert_eq!(splitter.total_blocks(), 4);

        // Get first block
        let block1 = splitter.next_block().unwrap();
        assert_eq!(block1.block_number, 0);
        assert_eq!(block1.data, Vec::from(&[1u8, 2, 3][..]));
        assert_eq!(block1.is_last_block, false);

        // Get second block
        let block2 = splitter.next_block().unwrap();
        assert_eq!(block2.block_number, 1);
        assert_eq!(block2.data, Vec::from(&[4u8, 5, 6][..]));
        assert_eq!(block2.is_last_block, false);

        // Get third block
        let block3 = splitter.next_block().unwrap();
        assert_eq!(block3.block_number, 2);
        assert_eq!(block3.data, Vec::from(&[7u8, 8, 9][..]));
        assert_eq!(block3.is_last_block, false);

        // Get last block
        let block4 = splitter.next_block().unwrap();
        assert_eq!(block4.block_number, 3);
        assert_eq!(block4.data, Vec::from(&[10u8][..]));
        assert_eq!(block4.is_last_block, true);

        // No more blocks
        assert_eq!(splitter.next_block(), None);
        assert_eq!(splitter.has_more(), false);
    }

    #[test]
    fn test_block_transfer_progress() {
        let mut bt = BlockTransfer::new();
        bt.start_transfer(None).unwrap();

        let block1 = Block::new(0, Vec::from(&[1u8, 2, 3][..]), false);
        let block2 = Block::new(1, Vec::from(&[4u8, 5, 6][..]), true);

        bt.add_block(block1).unwrap();
        let progress = bt.get_progress();
        assert_eq!(progress.current_block, 1);
        assert_eq!(progress.total_blocks, 1);
        assert_eq!(progress.is_complete, false);

        bt.add_block(block2).unwrap();
        let progress = bt.get_progress();
        assert_eq!(progress.current_block, 2);
        assert_eq!(progress.total_blocks, 2);
        assert_eq!(progress.is_complete, true);
    }

    #[test]
    fn test_block_transfer_reset() {
        let mut bt = BlockTransfer::new();
        bt.start_transfer(None).unwrap();

        let block = Block::new(0, Vec::from(&[1u8, 2, 3][..]), true);
        bt.add_block(block).unwrap();
        assert_eq!(bt.is_complete(), true);

        bt.reset();
        assert_eq!(bt.state(), BlockTransferState::Idle);
        assert_eq!(bt.is_complete(), false);
        assert_eq!(bt.get_total_transmitted(), 0);
    }
}
