/// An image file interface.
///
/// With the `std` feature enabled, all types that implement
/// [`std::io::Read`] also implement [`InputDevice`].
pub trait ImageFile {
    type Error;
    /// If the return value of this method is [`Ok(n)`], then implementations must
    /// guarantee that `0 <= n <= buf.len()`. A nonzero `n` value indicates
    /// that the buffer `buf` has been filled in with `n` bytes of data from this
    /// source. If `n` is `0`, then it can indicate one of two scenarios:
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;
}

impl<T: std::io::Read> ImageFile for T {
    type Error = std::io::Error;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.read(buf)
    }
}
