use std::io::prelude::*;
use std::io::SeekFrom;
use std::error::Error;
use std::fs::File;
use std::fs::Metadata;
use std::str;
use std::path::Path;

// Define a buffer size (user settable)
// CHeck the file exits, size
// If file size < buffer size -> buffer size = file size
// if search is backwards => read buffer from End
// readjust buffer and seek position to last \n
// parse buffer in lines
// search for input - if you hit end of buffer -- reload and reiterate

struct SearchFile <'a> {
    path: &'a Path,  // This is an unsized type -> always be used behind a pointer like & or Box.
    file_handle: File,
    reverse: bool,
    buffer_size: usize,
}

// impl Default for SearchFile {
//     fn default() -> SearchFile {
//         SearchFile { path: Path::new(filename) }
//     }
// }

fn open_file(filename: &str, reverse: bool, buffer_size: usize) -> SearchFile {
    let path = Path::new(filename);
    let metadata = match std::fs::metadata(&path) {
        Ok(metadata) => metadata,
        Err(why) => panic!("couldn't obtain metadata {}: {}", path.display(),
                                                              Error::description(&why))
    };
    let file_size = metadata.len() as usize;
    let buffer_size = if buffer_size < file_size {
        buffer_size
    } else {
        file_size
    };
    let file_handle = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", path.display(), Error::description(&why)),
        Ok(file) => file
    };
    SearchFile { path: &path, file_handle: file_handle, reverse: reverse, buffer_size: buffer_size }
}

fn main() {
    let mut search_file = open_file("./foo.txt", true, 1000);
    let pattern = "tg";
    let dspl = search_file.path.display();
    println!("The path is: {}", dspl);
    let mut buffer = vec![0; search_file.buffer_size].into_boxed_slice();
    let rev_pos = search_file.buffer_size as i64;
    // seek position in file
    let seek_pos = if search_file.reverse {
        SeekFrom::End(-rev_pos)
    } else {
        SeekFrom::Start(0)
    };
    match search_file.file_handle.seek(seek_pos) {
        Err(why) => panic!("couldn't seek {}: {}", dspl, Error::description(&why)),
        Ok(_) => match search_file.file_handle.read_exact(&mut buffer) {
            Err(why) => panic!("couldn't read {}: {}", dspl, Error::description(&why)),
            Ok(_) => {},
        }
    };
    let mut lines = str::from_utf8(&buffer).unwrap().lines().rev();
    // println!("Last line would be: {:?}", lines.next())
    let cur_line = lines.next().unwrap();
    let v: Vec<_> = cur_line.rmatch_indices(pattern).collect();
    println!("Occurences are: {:?}", v)

}
