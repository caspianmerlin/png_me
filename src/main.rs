mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args::process_args(&args) {
        Ok(command_args) => {
            match commands::process_command(command_args) {
                Ok(_) => return Ok(()),
                Err(e) => println!("{}", e),
            };
        },
        Err(e) => println!("{}", e),
    }

    Ok(())
}
