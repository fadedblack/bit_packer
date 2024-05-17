use std::{fs::File, io::{Error, Read, ErrorKind}};
use byteorder::{LittleEndian, ReadBytesExt};

struct BitPacker;

impl BitPacker {

    pub fn open_file(path: &str) -> Result<Vec<u32>, Error> {
        let mut file = File::open(path)?;
        let mut buffer : Vec<u8> = Vec::new();
        let no_bytes = file.read_to_end(&mut buffer)?;
        if no_bytes == file.metadata()?.len() as usize {
            let contents : Vec<u32> = buffer
            .chunks(4)
            .map(|mut chunk| chunk.read_u32::<LittleEndian>().unwrap())
            .collect();
            Ok(contents)
        } else {
            Err(Error::new(ErrorKind::Other, "All bytes couldn't be read"))
        }
    }
}

fn main() -> Result<(), Error> {
    let path: &'static str = "../assest/binaryfile.bin";
    match BitPacker::open_file( &path) {
        Ok(words) => {
            println!("No of bytes written: {}", words.len());
            for word in words{
                println!("{word} ");
            }
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }

    Ok(())
}