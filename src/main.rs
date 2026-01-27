use std::io::{self, Write};

fn main() ->anyhow::Result<()> {
    let mut pty = Pty::new(80, 24)?;

    //Send command to shell
    pty.write(b"echo 'Hello from Julix terminal!'\n")?;

    //Read Outpust
    let mut buf = [0u8; 4096];
    loop {
        match pty.read(&mut buf) {
            Ok(n) if n > 0 => {
                io::stdout().write_all(&buf[..n])?;
                io::stdout().flush()?;
            }
            _ => break,
        }
    }

    Ok(())
}