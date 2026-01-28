use anyhow::Result;
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem, PtyPair};
use std::io::{Read, Write};

pub struct Pty {
    pair: PtyPair,
    reader: Box<dyn Read + Send>,
    writer: Box<dyn Write + Send>,
}

impl Pty {
    pub fn new(cols: u16, rows: u16) -> Result<Self> {
        let pty_system = NativePtySystem::default();
        
        let pair = pty_system.openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        // Spawn shell (bash on Unix, cmd on Windows)
        let shell = if cfg!(windows) {
            "cmd.exe".to_string()
        } else {
            std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string())
        };
        
        let cmd = CommandBuilder::new(shell);
        let _child = pair.slave.spawn_command(cmd)?;

        let reader = pair.master.try_clone_reader()?;
        let writer = pair.master.take_writer()?;

        Ok(Self { pair, reader, writer })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<()> {
        self.writer.write_all(data)?;
        self.writer.flush()?;
        Ok(())
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        Ok(self.reader.read(buf)?)
    }

    pub fn resize(&mut self, cols: u16, rows: u16) -> Result<()> {
        self.pair.master.resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;
        Ok(())
    }
}