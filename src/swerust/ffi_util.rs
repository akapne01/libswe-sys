use std::ffi::c_char;

/// swephexp.h AS_MAXCH: every `char *serr` must have room for this many bytes.
pub(crate) const AS_MAXCH: usize = 256;
/// Star name buffer. Older docs say 40 chars + NUL; newer headers use 256.
/// 256 is safe either way. Check SE_MAX_STNAME in your vendored swephexp.h.
pub(crate) const SE_MAX_STNAME: usize = 256;

pub(crate) type ErrBuf = [c_char; AS_MAXCH];

pub(crate) fn new_err_buf() -> ErrBuf {
    [0; AS_MAXCH]
}

/// Never reads past the buffer, even if C failed to NUL-terminate. Never panics.
pub(crate) fn buf_to_string(buf: &[c_char]) -> String {
    let bytes: Vec<u8> = buf
        .iter()
        .take_while(|&&c| c != 0)
        .map(|&c| c as u8)
        .collect();
    String::from_utf8_lossy(&bytes).into_owned()
}
