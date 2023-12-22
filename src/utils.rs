use std::ffi::CString;

impl WithLength for CString {
    fn with_length(length: usize) -> Self {
        unsafe { CString::from_vec_unchecked(Vec::from_iter(std::iter::repeat(b' ').take(length))) }
    }
}

pub trait WithLength {
    fn with_length(length: usize) -> Self;
}
