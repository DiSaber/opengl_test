use std::ffi::CString;

impl WithLength for CString {
    fn with_length(length: usize) -> CString {
        unsafe { CString::from_vec_unchecked(Vec::from_iter(std::iter::repeat(b' ').take(length))) }
    }
}

pub trait WithLength {
    fn with_length(length: usize) -> Self;
}

impl<T, U> FlattenToVec<T> for Vec<U>
where
    U: FlattenToVec<T>,
{
    fn flatten(&self) -> Vec<T> {
        self.iter().flat_map(|v| v.flatten()).collect()
    }
}

pub trait FlattenToVec<T> {
    fn flatten(&self) -> Vec<T>;
}
