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
        /*
         * https://ntfs.com/ntfs-system-files.htm
         * But i am not entirely sure that this source is up-to-date
         * According to data-set i am parsing :
                Entry 24 Name : $Quota
                Entry 25 Name : $ObjId
                Entry 26 Name : $Reparse
                Entry 27 Name : $RmMetadata
                Entry 28 Name : $Repair
                Entry 29 Name : $Deleted
                Entry 30 Name : $TxfLog
                Entry 31 Name : $Txf
                Entry 32 Name : $Tops
                Entry 33 Name : $TxfLog.blf
                Entry 34 Name : $TxfLogContainer00000000000000000001
                Entry 35 Name : $TxfLogContainer00000000000000000002
                Entry 36 Name : System Volume Information
                Entry 37 Name : ClientRecoveryPasswordRotation
                Entry 38 Name : AadRecoveryPasswordDelete
                Entry 39 Name : FveDecryptedVolumeFolder
                Entry 40 Name : WPSettings.dat
         * the first 40 MFT entries are reserved for system files
         */
        let start_rc = if ignore_dos { 26u64 } else { 0u64 };
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
