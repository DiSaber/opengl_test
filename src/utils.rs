use std::ffi::CString;

pub fn cstring_with_length(length: usize) -> CString {
    unsafe { CString::from_vec_unchecked(Vec::from_iter(std::iter::repeat(b' ').take(length))) }
}
