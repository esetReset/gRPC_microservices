use std::io::prelude::*;
use std::fs::File;

fn main()  -> std::io::Result<()>{
    let mut buffer = File::create("foo.txt")?;

    // Writes some prefix of the byte string, not necessarily all of it.
    buffer.write(b"some bytes")?;
    Ok(())
}

fn write_text_to_file(file : String, text:String){

}