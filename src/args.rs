use std::{fmt::{Display, Formatter}};
use crate::{Error, Result};

pub fn process_args(args: &[String]) -> Result<PngMeArgs> {
    if args.len() < 2 { 
        return Err(Box::new(ArgsError::NotEnoughArgs(args.len())));
    }
    if let Some(command) = args.get(1) {
        let command = command.to_lowercase();
        match command.as_str() {
            "encode" => {
                let encode_args = EncodeArgs::new(&args[1..]);
                match encode_args {
                    Err(e) => return Err(e),
                    Ok(o) => return Ok(PngMeArgs::Encode(o)),
                }
            },
            "decode" => {
                let decode_args = DecodeArgs::new(&args[1..]);
                match decode_args {
                    Err(e) => return Err(e),
                    Ok(o) => return Ok(PngMeArgs::Decode(o)),
                }
            },
            "remove" => {
                let remove_args = RemoveArgs::new(&args[1..]);
                match remove_args {
                    Err(e) => return Err(e),
                    Ok(o) => return Ok(PngMeArgs::Remove(o)),
                }
            },
            "print" => {
                let print_args = PrintArgs::new(&args[1..]);
                match print_args {
                    Err(e) => return Err(e),
                    Ok(o) => return Ok(PngMeArgs::Print(o)),
                }
            },
            _ => return Err(Box::new(ArgsError::InvalidCommand(command))),
        };

    }
    return Err(Box::new(ArgsError::NoCommand));
}

#[derive(Debug)]
pub enum ArgsError {
    NoCommand,
    InvalidCommand(String),
    NotEnoughArgs(usize),
    TooManyArgs(usize),
}
impl Display for ArgsError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ArgsError::NoCommand => write!(f, "You did not provide a command."),
            ArgsError::InvalidCommand(c) => write!(f, "{} is not a valid command.", c),
            ArgsError::NotEnoughArgs(n) => write!(f, "You only provided {} arguments, which is not enough.", n),
            ArgsError::TooManyArgs(n) => write!(f, "You provided {} arguments, which is too many.", n),
        }
    }
}
impl std::error::Error for ArgsError {}

pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

pub struct EncodeArgs {
    pub path: String,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<String>,
}
impl EncodeArgs {
    pub fn new(args: &[String]) -> Result<Self> {
        let args_length = args.len();
        if args_length < 3 {
            return Err(Box::new(ArgsError::NotEnoughArgs(args_length)));
        } else if args_length > 5 {
            return Err(Box::new(ArgsError::TooManyArgs(args_length)));
        } 
        let output_file = if args_length == 3 { None } else { Some(args[3].clone()) };

        Ok(Self {
            path: args[1].clone(),
            chunk_type: args[2].clone(),
            message: args[3].clone(),
            output_file: output_file,
        })
    }
}

pub struct DecodeArgs {
    path: String,
    chunk_type: String,
}
impl DecodeArgs {
    pub fn new(args: &[String]) -> Result<Self> {
        let args_length = args.len();
        if args_length < 3 {
            return Err(Box::new(ArgsError::NotEnoughArgs(args_length)));
        } else if args_length > 3 {
            return Err(Box::new(ArgsError::TooManyArgs(args_length)));
        }
        
        Ok(Self {
            path: args[1].clone(),
            chunk_type: args[2].clone(),
        })
    }
}

pub struct RemoveArgs {
    path: String,
    chunk_type: String,
}
impl RemoveArgs {
    pub fn new(args: &[String]) -> Result<Self> {
        let args_length = args.len();
        if args_length < 3 {
            return Err(Box::new(ArgsError::NotEnoughArgs(args_length)));
        } else if args_length > 3 {
            return Err(Box::new(ArgsError::TooManyArgs(args_length)));
        }
        
        Ok(Self {
            path: args[1].clone(),
            chunk_type: args[2].clone(),
        })
    }
}

pub struct PrintArgs {
    path: String,
}
impl PrintArgs {
    pub fn new(args: &[String]) -> Result<Self> {
        let args_length = args.len();
        if args_length < 2 {
            return Err(Box::new(ArgsError::NotEnoughArgs(args_length)));
        } else if args_length > 2 {
            return Err(Box::new(ArgsError::TooManyArgs(args_length)));
        }
        
        Ok(Self {
            path: args[1].clone(),
        })
    }
}