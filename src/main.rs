use std::{fs::OpenOptions, io::Read};
use libc::size_t;

#[link(name = "printer")]
extern "C" {
    fn print_file(
        file_contents: *const [u8],
        file_size: size_t
    );
}

fn main() {
    let mut content_buf: String = String::new();

    let mut file = OpenOptions::new().read(true).open("content.txt").unwrap();
    file.read_to_string(&mut content_buf).unwrap();

    let file_size: size_t = content_buf.len();

    let content_bytes = content_buf.as_bytes() as *const [u8];
    unsafe {
        print_file(content_bytes, file_size);
    }
}
