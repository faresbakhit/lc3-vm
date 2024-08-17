use core::slice;

/// An image file interface.
#[cfg_attr(
    feature = "std",
    doc = "

With the `std` feature enabled, all types that implement
[`std::io::Read`] also implement [`ImageFile`]."
)]
pub trait ImageFile {
    type Error;

    /// Pull some bytes from this source into the specified buffer, returning
    /// how many bytes were read.
    ///
    /// If the return value of this method is [`Ok(n)`], then implementations must
    /// guarantee that `0 <= n <= buf.len()`. A nonzero `n` value indicates
    /// that the buffer `buf` has been filled in with `n` bytes of data from this
    /// source. If `n` is `0`, then it can indicate one of two scenarios:
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;

    /// Load image into `memory`.
    fn load_image_into(&mut self, memory: &mut [u16]) -> Result<(), Self::Error> {
        let mut origin: [u8; 2] = [0; 2];
        self.read(&mut origin)?;
        let origin = u16::from_be_bytes(origin) as usize;

        // Memory at `origin` as a slice of `u8` bytes
        let slice = {
            let data = &mut memory[origin] as *mut u16 as *mut u8;
            let len = (memory.len() - origin) * 2;
            unsafe { slice::from_raw_parts_mut(data, len) }
        };

        let end /* exclusive */ = origin + self.read(slice)? / 2;

        //
        // Proof that `end <= memory.len`
        //
        // [`ImageFile::read`] guarantees `0 <= n <= slice.len()`,
        // where `n` is the return value of [`ImageFile::read`].
        //
        // Definitions:
        // - `origin`: an arbitrary number.
        // - `end`: `origin+n/2`.
        // - `slice.len`: `2(memory.len-origin)`.
        //
        // 0      <=           n    <=               slice.len
        // 0      <=           n/2  <=               slice.len/2
        // origin <= (origin + n/2) <=               slice.len/2
        // origin <= (origin + n/2) <= origin +      slice.len/2
        // origin <= (origin + n/2) <= origin + (memory.len - origin)
        // origin <=       end      <= memory.len
        //
        // From this it follows that: `end <= memory.len`. Q.E.D.
        //

        unsafe {
            memory
                .get_unchecked_mut(origin..end)
                .iter_mut()
                .for_each(|x| *x = u16::from_be(*x))
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
impl<T: std::io::Read> ImageFile for T {
    type Error = std::io::Error;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        std::io::Read::read(self, buf)
    }
}
