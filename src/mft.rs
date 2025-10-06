use std::io::{Read, Seek};
use crate::Ntfs;

// source : https://stackoverflow.com/questions/16421033/lazy-sequence-generation-in-rust

/// Range-based MFT implementation.
/// If 'ntfs.file(&mut ntfs_image, pos);' returns 'InvalidFileSignature', 
/// entry at 'pos' is most likely not allocated yet (but corruption possibility must still be considered)
pub struct Mft<'a>
{
    pub ntfs : &'a Ntfs,

    // Default : 0
    pub start_rc : u64,

    pub end_rc : u64
}

impl<'a> Mft<'a>
{
    pub fn new<T>(ntfs : &'a Ntfs, fs : &mut T, ignore_dos : bool) -> crate::error::Result<Self>
    where
        T : Read + Seek
    {
        let start_rc = if ignore_dos { 24u64 } else { 0u64 };
        let total_record_count = ntfs.mft_entry_count(fs)?;
        Ok(Self {
            ntfs : ntfs,
            start_rc : start_rc, // default
            end_rc : total_record_count
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = u64> + '_ {
        self.start_rc..self.end_rc
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = u64> + '_ {
        self.start_rc..self.end_rc
    }

}

impl Iterator for Mft<'_> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start_rc == self.end_rc {
            return None;
        }
        let result = Some(self.start_rc);
        self.start_rc += 1;
        result
    }
}
