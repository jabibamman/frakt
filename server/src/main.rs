mod services {
    pub mod connect;
    pub mod write;
}

use services::connect::connect;
use services::write::write;

fn main() -> std::io::Result<()> {
    let stream = connect("localhost:8787")?;

    write(stream, "Hello Worlds !")?;

    Ok(())
}