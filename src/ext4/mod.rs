use crate::constants::*;
use crate::ext4_defs::*;
use crate::prelude::*;
use crate::return_error;

mod alloc;
mod dir;
mod extent;
mod high_level;
mod journal;
mod link;
mod low_level;
mod rw;

#[derive(Debug)]
pub struct Ext4 {
    block_device: Arc<dyn BlockDevice>,
}

impl Ext4 {
    /// Opens and loads an Ext4 from the `block_device`.
    pub fn load(block_device: Arc<dyn BlockDevice>) -> Result<Self> {
        // Load the superblock
        // TODO: if the main superblock is corrupted, should we load the backup?
        let block = block_device.read_block(0);
        let sb = block.read_offset_as::<SuperBlock>(BASE_OFFSET);
        // Check magic number
        if !sb.check_magic() {
            return_error!(ErrCode::EINVAL, "invalid magic number");
        }
        // Create Ext4 instance
        Ok(Self { block_device })
    }
    /// Initializes the root directory.
    pub fn init(&mut self) -> Result<()> {
        // Create root directory
        self.create_root_inode().map(|_| ())
    }
}
