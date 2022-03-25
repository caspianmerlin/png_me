use std::{fmt::{Display, Formatter}, error::Error, path::{Path, PathBuf}, io::{BufReader, Read, Write}, fs::{File, OpenOptions}, str::FromStr};

use crate::{args::{PngMeArgs, EncodeArgs, DecodeArgs, RemoveArgs, PrintArgs}, png::Png, chunk::Chunk, chunk_type::ChunkType};


#[derive(Debug)]
pub enum CommandError {
    SyntaxError,
    FileError,
    PngFormatError,
    ChunkTypeError,
    OtherError,
}
impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            CommandError::SyntaxError => write!(f, "Syntax Error"),
            CommandError::FileError => write!(f, "Error opening file"),
            CommandError::PngFormatError => write!(f, "The input file is not a valid PNG file"),
            CommandError::ChunkTypeError => write!(f, "Invalid chunk format"),
            CommandError::OtherError => write!(f, "Error: other"),
        }
    }
}
impl std::error::Error for CommandError {}


fn process_command(command_args: PngMeArgs) -> Result<(), Box<dyn Error>> {
    match command_args {
        PngMeArgs::Encode(args) => encode(args),
        PngMeArgs::Decode(args) => decode(args),
        PngMeArgs::Remove(args) => remove(args),
        PngMeArgs::Print(args) => print(args),
    };


    Ok(())
}

fn encode(args: EncodeArgs) -> Result<(), CommandError> {
    let opened_file =  match load_bytes_from_file(&args.path) {
        Ok(b) => b,
        Err(_) => return Err(CommandError::FileError),
    };
    let png = match Png::try_from(&opened_file[..]) {
        Ok(p) => p,
        Err(_) => return Err(CommandError::PngFormatError),
    };
    let new_chunktype = match ChunkType::from_str(&args.chunk_type) {
        Ok(c) => c,
        Err(_) => return Err(CommandError::ChunkTypeError),
    };
    let new_chunk = Chunk::new(new_chunktype, chunk_data)
}
fn decode(args: DecodeArgs) -> Result<(), CommandError> {

}
fn remove(args: RemoveArgs) -> Result<(), CommandError> {

}
fn print(args: PrintArgs) -> Result<(), CommandError> {

}



fn load_bytes_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Box <dyn Error>>  {
    let file = File::open(path)?;
    let mut buffer = vec![];
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut buffer);
    Ok(buffer)
}

fn write_to_file<P: AsRef<Path>>(path: P, bytes: &[u8]) -> Result<(), Box <dyn Error>>  {
    //Set up file path and temp file path 
    let write_path = Path::new(path.as_ref());
    let write_path = PathBuf::from(write_path);
    let temp_path = write_path.clone().with_extension("tmp");

    //Create temp file and write contents to it
    let temp_file = OpenOptions::new().read(true).write(true).create(true).open(&temp_path)?;
    temp_file.write_all(bytes)?;
    std::fs::rename(&temp_path, &write_path)?;
    Ok(())
}