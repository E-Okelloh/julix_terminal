mod pty;
mod terminal;

use anyhow::Result;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color as CTColor, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
};
use pty::Pty;
use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use terminal::Terminal;

fn main() -> Result<()> {
    // Get terminal size
    let (cols, rows) = size()?;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, Hide, Clear(ClearType::All))?;

    // Create PTY and terminal emulator
    let mut pty = Pty::new(cols, rows)?;
    let mut terminal = Terminal::new(cols as usize, rows as usize);

    // Spawn thread to read from PTY
    let (tx, rx) = mpsc::channel();
    
    // Clone PTY for reading thread
    thread::spawn({
        let mut pty_reader = Pty::new(cols, rows).expect("Failed to create PTY reader");
        move || {
            let mut buf = [0u8; 4096];
            loop {
                match pty_reader.read(&mut buf) {
                    Ok(n) if n > 0 => {
                        if tx.send(buf[..n].to_vec()).is_err() {
                            break;
                        }
                    }
                    Ok(_) => {
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(_) => break,
                }
            }
        }
    });

    // Main loop
    'main_loop: loop {
        // Read from PTY
        while let Ok(bytes) = rx.try_recv() {
            terminal.feed_bytes(&bytes);
        }

        // Render terminal
        render(&mut stdout, &terminal)?;

        // Handle input
        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(KeyEvent { 
                    code, 
                    modifiers,
                    ..
                }) => {
                    // Quit on Ctrl+Q
                    if code == KeyCode::Char('q') && modifiers.contains(KeyModifiers::CONTROL) {
                        break 'main_loop;
                    }

                    match code {
                        KeyCode::Char(c) => {
                            if modifiers.contains(KeyModifiers::CONTROL) {
                                // Send control characters
                                let ctrl_char = (c.to_ascii_lowercase() as u8 & 0x1f) as u8;
                                pty.write(&[ctrl_char])?;
                            } else {
                                pty.write(&[c as u8])?;
                            }
                        }
                        KeyCode::Enter => pty.write(b"\r")?,
                        KeyCode::Backspace => pty.write(b"\x7f")?,
                        KeyCode::Tab => pty.write(b"\t")?,
                        KeyCode::Up => pty.write(b"\x1b[A")?,
                        KeyCode::Down => pty.write(b"\x1b[B")?,
                        KeyCode::Right => pty.write(b"\x1b[C")?,
                        KeyCode::Left => pty.write(b"\x1b[D")?,
                        KeyCode::Home => pty.write(b"\x1b[H")?,
                        KeyCode::End => pty.write(b"\x1b[F")?,
                        KeyCode::PageUp => pty.write(b"\x1b[5~")?,
                        KeyCode::PageDown => pty.write(b"\x1b[6~")?,
                        KeyCode::Delete => pty.write(b"\x1b[3~")?,
                        KeyCode::Insert => pty.write(b"\x1b[2~")?,
                        _ => {}
                    }
                }
                Event::Resize(new_cols, new_rows) => {
                    terminal.resize(new_cols as usize, new_rows as usize);
                    pty.resize(new_cols, new_rows)?;
                }
                _ => {}
            }
        }
    }

    // Cleanup
    execute!(stdout, Show, Clear(ClearType::All))?;
    disable_raw_mode()?;
    
    println!("\nJulix terminal exited. Thanks for using!");
    
    Ok(())
}

fn render(stdout: &mut io::Stdout, terminal: &Terminal) -> Result<()> {
    execute!(stdout, MoveTo(0, 0))?;
    
    for (y, row) in terminal.grid.iter().enumerate() {
        execute!(stdout, MoveTo(0, y as u16))?;
        
        for cell in row {
            execute!(
                stdout,
                SetForegroundColor(CTColor::Rgb { 
                    r: cell.fg.r, 
                    g: cell.fg.g, 
                    b: cell.fg.b 
                }),
                SetBackgroundColor(CTColor::Rgb { 
                    r: cell.bg.r, 
                    g: cell.bg.g, 
                    b: cell.bg.b 
                }),
                Print(cell.c)
            )?;
        }
    }
    
    // Draw cursor
    execute!(
        stdout,
        MoveTo(terminal.cursor.x as u16, terminal.cursor.y as u16)
    )?;
    
    stdout.flush()?;
    Ok(())
}