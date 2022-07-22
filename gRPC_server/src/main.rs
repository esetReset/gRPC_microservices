use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main()  {
   let file_path = Path::new("./target/foo.txt");
   let text = String::from("test bytes");
   write_text_to_file(file_path, text).expect("Could not write to file {file_path}");
}

fn write_text_to_file(file: &Path, text:String) -> std::io::Result<()>{
   let mut buffer = File::create(file.as_os_str())?;

   // Writes some prefix of the byte string, not necessarily all of it.
   buffer.write( text.as_bytes())?;
   Ok(())
}