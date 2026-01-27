use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::io::{Read, Write};
pub struct Pty {
    pair: Box<dyn portable_pty::PtyPair>,
    render: Box<dyn Read + Send>,
    writer: Box<dyn Write + Send>,

}

impl Pty {
    pub fn new(cols: u16, rows:u16) -> anyhow::Result<Self> {
        let pty_system = NativePtySystem::default();


        // Creating PTY with Specific size
        let pair = pty_system.openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,

        })?;

        //Spawn Shell
        let cmd = CommandBuilder::new("bash");
        let_child = pair.slave.Spawn_command(cmd)?;

        //Get reader/ writer for master side
        let reader = pair.master.try_clone_reader()?;
        let wroter = pair.master.take_writer()?;

        Ok(Self {pair, reader, writer })
    }


    pub fn write(&mut self, data:&[u8]) -> anyhow::Result<()>{
        self.writer.write_all(data)?;
        self.write.flush()?;
        Ok(())
    }

    pub fn read(&mut self, buf: &mut [u8]) -> anyhow::Result<usize> {
        Ok(self.readre.reader.read(buf)?)
    }

    pub fn resize(&mut self, cols: u16, rows: u16) -> anyhow::Result<()> {
        self.pair.maste.resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;
        Ok(())
    }
}