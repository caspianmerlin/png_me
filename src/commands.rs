use std::{fmt::{Display, Formatter}, error::Error, path::{Path, PathBuf}, io::{BufReader, Read, Write}, fs::{File, OpenOptions}, str::FromStr, process::Command};

use crate::{args::{PngMeArgs, EncodeArgs, DecodeArgs, RemoveArgs, PrintArgs}, png::Png, chunk::Chunk, chunk_type::ChunkType};


#[derive(Debug)]
pub enum CommandError {
    SyntaxError,
    FileError,
    PngFormatError,
    ChunkTypeError,
    ChunkNotFoundError,
    OtherError,
}
impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            CommandError::SyntaxError => write!(f, "Syntax Error"),
            CommandError::FileError => write!(f, "Error opening file"),
            CommandError::PngFormatError => write!(f, "The input file is not a valid PNG file"),
            CommandError::ChunkTypeError => write!(f, "Invalid chunk format"),
            CommandError::ChunkNotFoundError => write!(f, "Chunk not found"),
            CommandError::OtherError => write!(f, "Error: other"),
        }
    }
}
impl std::error::Error for CommandError {}


pub fn process_command(command_args: PngMeArgs) -> Result<(), Box<dyn Error>> {
    match command_args {
        PngMeArgs::Encode(args) => {
            return match encode(args) {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(e)),
            }
        },
        PngMeArgs::Decode(args) => {
            match decode(args) {
                Ok(message_chunk) => {
                    match message_chunk {
                        Some(m) => println!("Hidden message: {}", m),
                        None => println!("No message found"),
                    }
                    return Ok(());
                },
                Err(e) => return Err(Box::new(e)),
            }
        },
        PngMeArgs::Remove(args) => {
            return match remove(args) {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(e)),
            }
        },
        PngMeArgs::Print(args) => {
            return match print(args) {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(e)),
            }
        }
    };
    Ok(())
}

fn encode(args: EncodeArgs) -> Result<(), CommandError> {
    let opened_file =  match load_bytes_from_file(&args.path) {
        Ok(b) => b,
        Err(_) => return Err(CommandError::FileError),
    };
    let mut png = match Png::try_from(&opened_file[..]) {
        Ok(p) => p,
        Err(_) => return Err(CommandError::PngFormatError),
    };
    let new_chunktype = match ChunkType::from_str(&args.chunk_type) {
        Ok(c) => c,
        Err(_) => return Err(CommandError::ChunkTypeError),
    };
    let new_chunk = Chunk::new(new_chunktype, args.message.into_bytes());
    png.append_chunk(new_chunk);
    let destination_path = match &args.output_file {
        Some(p) => p,
        None => &args.path,
    };
    match write_to_file(destination_path, &png.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err(CommandError::FileError),
    }
}

fn decode(args: DecodeArgs) -> Result<Option<String>, CommandError> {
    let opened_file =  match load_bytes_from_file(&args.path) {
        Ok(b) => b,
        Err(_) => return Err(CommandError::FileError),
    };
    let png = match Png::try_from(&opened_file[..]) {
        Ok(p) => p,
        Err(_) => return Err(CommandError::PngFormatError),
    };
    match png.chunk_by_type(&args.chunk_type) {
        Some(c) => Ok(Some(c.data_as_string().unwrap())),
        None => Ok(None),
    }
}

fn remove(args: RemoveArgs) -> Result<(), CommandError> {
    let opened_file =  match load_bytes_from_file(&args.path) {
        Ok(b) => b,
        Err(_) => return Err(CommandError::FileError),
    };
    let mut png = match Png::try_from(&opened_file[..]) {
        Ok(p) => p,
        Err(_) => return Err(CommandError::PngFormatError),
    };

    if let Err(_) =  png.remove_chunk(&args.chunk_type) {
        return Err(CommandError::ChunkNotFoundError);
    }

    match write_to_file(&args.path, &png.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err(CommandError::FileError),
    }
}

fn print(args: PrintArgs) -> Result<(), CommandError> {
    let opened_file =  match load_bytes_from_file(&args.path) {
        Ok(b) => b,
        Err(_) => return Err(CommandError::FileError),
    };
    let png = match Png::try_from(&opened_file[..]) {
        Ok(p) => p,
        Err(_) => return Err(CommandError::PngFormatError),
    };

    println!("{}", png);
    Ok(())
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
    let mut temp_file = OpenOptions::new().read(true).write(true).create(true).open(&temp_path)?;
    temp_file.write_all(bytes)?;
    std::fs::rename(&temp_path, &write_path)?;
    Ok(())
}