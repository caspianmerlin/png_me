mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if let Ok(c_a) = args::process_args(&args) {
        match c_a {
            
        }
    }



    Ok(())
}
