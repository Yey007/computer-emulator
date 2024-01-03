pub mod readonly;
pub mod readwrite;

#[macro_export]
macro_rules! bits_to_index_length {
    ($n:expr) => {
        (($n - 1).ilog2() + 1) as usize
    };
}
