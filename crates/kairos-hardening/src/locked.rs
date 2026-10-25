//! Modelled after: libsodium secure allocation and region's fallible page locks.
//! <https://github.com/jedisct1/libsodium/blob/1.0.20/src/libsodium/sodium/utils.c>
//! <https://docs.rs/region/3.0.2/region/>.

use zeroize::Zeroize;

use crate::HardeningError;

/// An exclusive, page-isolated allocation with a fixed usable capacity.
///
/// Construction fails if locking fails. No `Debug`, `Clone`, string conversion
/// or growing API is provided. Borrowed slices may still be copied by callers.
/// Windows uses VirtualAlloc/VirtualLock via region; page locks do not exclude
/// dumps or hibernation. Call [`Self::close`] to observe VirtualUnlock errors.
pub struct LockedSecret {
    memory: region::Allocation,
    capacity: usize,
    len: usize,
    locked: bool,
}

impl LockedSecret {
    pub fn new(capacity: usize) -> Result<Self, HardeningError> {
        if !(1..=65536).contains(&capacity) {
            return Err(HardeningError::InvalidCapacity);
        }
        let memory = region::alloc(capacity, region::Protection::READ_WRITE)
            .map_err(HardeningError::Memory)?;
        let guard =
            region::lock(memory.as_ptr::<u8>(), memory.len()).map_err(HardeningError::Memory)?;
        // Unlock explicitly so errors can be returned by close(), without the
        // region guard's debug-build panic or a second unlock on drop.
        std::mem::forget(guard);
        Ok(Self {
            memory,
            capacity,
            len: 0,
            locked: true,
        })
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Borrows only initialized, currently used bytes.
    pub fn expose(&self) -> &[u8] {
        // SAFETY: region owns committed readable pages for this entire borrow.
        // len never exceeds capacity, and replace/extend initialize those bytes.
        unsafe { std::slice::from_raw_parts(self.memory.as_ptr(), self.len) }
    }

    /// Extends without reallocating. Overflow leaves the previous value intact.
    pub fn extend_from_slice(&mut self, bytes: &[u8]) -> Result<(), HardeningError> {
        let end = self
            .len
            .checked_add(bytes.len())
            .filter(|n| *n <= self.capacity)
            .ok_or(HardeningError::CapacityExceeded)?;
        let start = self.len;
        self.all_bytes()[start..end].copy_from_slice(bytes);
        self.len = end;
        Ok(())
    }

    /// Overwrites the complete allocation before replacing the logical value.
    pub fn replace(&mut self, bytes: &[u8]) -> Result<(), HardeningError> {
        if bytes.len() > self.capacity {
            return Err(HardeningError::CapacityExceeded);
        }
        self.clear();
        self.extend_from_slice(bytes)
    }

    pub fn clear(&mut self) {
        self.all_bytes().zeroize();
        self.len = 0;
    }

    /// Scrubs before unlocking and freeing. Drop also scrubs, but cannot return
    /// an OS failure. If unlock fails, region still releases the zeroed pages.
    pub fn close(mut self) -> Result<(), HardeningError> {
        self.scrub_and_unlock()
    }

    fn all_bytes(&mut self) -> &mut [u8] {
        let len = self.memory.len();
        // SAFETY: exclusive owner of dedicated committed READ_WRITE pages.
        // Anonymous pages are initialized by the OS, including page padding.
        unsafe { std::slice::from_raw_parts_mut(self.memory.as_mut_ptr(), len) }
    }

    fn scrub_and_unlock(&mut self) -> Result<(), HardeningError> {
        self.clear();
        if self.locked {
            self.locked = false;
            region::unlock(self.memory.as_ptr::<u8>(), self.memory.len())
                .map_err(HardeningError::Memory)?;
        }
        Ok(())
    }
}

impl Drop for LockedSecret {
    fn drop(&mut self) {
        let _ = self.scrub_and_unlock();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_capacity_move_replace_clear_and_close() {
        let mut secret = LockedSecret::new(64).unwrap();
        let address = secret.memory.as_ptr::<u8>();
        secret.extend_from_slice(b"synthetic-test-secret").unwrap();
        let mut moved = secret;
        assert_eq!(address, moved.memory.as_ptr());
        assert_eq!(moved.expose(), b"synthetic-test-secret");
        assert!(moved.extend_from_slice(&[1; 65]).is_err());
        assert_eq!(moved.expose(), b"synthetic-test-secret");
        moved.replace(b"short").unwrap();
        assert_eq!(moved.expose(), b"short");
        assert!(moved.all_bytes()[5..].iter().all(|b| *b == 0));
        moved.clear();
        assert!(moved.is_empty());
        assert!(moved.all_bytes().iter().all(|b| *b == 0));
        moved.close().unwrap();
    }

    #[test]
    fn independent_allocations_do_not_share_lock_pages() {
        let mut first = LockedSecret::new(1).unwrap();
        let mut second = LockedSecret::new(1).unwrap();
        assert_ne!(first.memory.as_ptr::<u8>(), second.memory.as_ptr());
        first.replace(b"a").unwrap();
        second.replace(b"b").unwrap();
        first.close().unwrap();
        assert_eq!(second.expose(), b"b");
        second.close().unwrap();
    }

    #[test]
    fn invalid_capacities_fail_before_allocating() {
        for capacity in [0, 65537, usize::MAX] {
            assert!(matches!(
                LockedSecret::new(capacity),
                Err(HardeningError::InvalidCapacity)
            ));
        }
    }
}
