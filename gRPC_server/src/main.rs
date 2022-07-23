use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::env;


fn main() -> std::io::Result<()>{
   let mut file_path = env::current_dir()?;
   file_path.push("foo.txt");
   println!("File path {}", file_path.display());

   let text = "test bytes\n";

   write_text_to_file(file_path.as_path(), text)
   .expect("Could not write to file");

Ok(())
}

fn write_text_to_file(file_path: &Path, text:&str) -> std::io::Result<()>{
   let mut file_option = File::options()
      .append(true)
      .write(true)
      .open(file_path)?;


      file_option.write(text.as_bytes())?;
      Ok(())
}