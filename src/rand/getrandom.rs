use crate::{imp, io};

/// `GRND_*`
pub use imp::rand::GetRandomFlags;

/// `getrandom(buf, flags)`
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/getrandom.2.html
#[inline]
pub fn getrandom(buf: &mut [u8], flags: GetRandomFlags) -> io::Result<usize> {
    imp::syscalls::getrandom(buf, flags)
}
