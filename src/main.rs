extern crate libc;
use libc::{c_int, size_t};

#[link(name = "snappy")]
extern {
    fn snappy_compress(input: *const u8,
                       input_length: size_t,
                       compressed: *mut u8,
                       compressed_length: *mut size_t) -> c_int;

    fn snappy_uncompress(compressed: *const u8,
                         compressed_length: size_t,
                         uncompressed: *mut u8,
                         uncompressed_length: *mut size_t) -> c_int;

    fn snappy_max_compressed_length(source_length: size_t) -> size_t;

    fn snappy_uncompressed_length(compressed: *const u8,
                                  compressed_length: size_t,
                                  result: *mut size_t) -> c_int;

    fn snappy_validate_compressed_buffer(compressed: *const u8,
                                         compressed_length: size_t) -> c_int;
}

pub fn validate_compressed_buffer(src: &[u8]) -> bool {
    unsafe {
        snappy_validate_compressed_buffer(src.as_ptr(), src.len() as size_t) == 0
    }
}

pub fn compress(src: &[u8]) -> Vec<u8> {
    unsafe {
        let src_len = src.len() as size_t;
        let src_ptr = src.as_ptr();

        let mut dst_len = snappy_max_compressed_length(src_len);
        let mut dst = Vec::with_capacity(dst_len as usize);
        let dst_ptr = dst.as_mut_ptr();

        snappy_compress(src_ptr, src_len, dst_ptr, &mut dst_len);
        dst.set_len(dst_len as usize);
        dst
    }
}

pub fn uncompress(src: &[u8]) -> Option<Vec<u8>> {
    unsafe {
        let src_len = src.len() as size_t;
        let src_ptr = src.as_ptr();

        let mut dst_len: size_t = 0;
        snappy_uncompressed_length(src_ptr, src_len, &mut dst_len);

        let mut dst = Vec::with_capacity(dst_len as usize);
        let dst_ptr = dst.as_mut_ptr();

        if snappy_uncompress(src_ptr, src_len, dst_ptr, &mut dst_len) == 0 {
            dst.set_len(dst_len as usize);
            Some(dst)
        } else {
            None
        }
    }
}

fn main() {
    let x = unsafe { snappy_max_compressed_length(100) };
    println!("max compressed length of a 100 byte buffer: {}", x);
}
