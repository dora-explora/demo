use std::fs::File;
use std::io::{Result, stdout, BufReader};
use rodio::{OutputStreamBuilder, play};
use crossterm::{cursor, terminal, ExecutableCommand, style::{self, Stylize, Color}};

fn main() -> Result<()> {
	let stream_handle = OutputStreamBuilder::open_default_stream()
	    .expect("Could not open default audio stream");
	let file = BufReader::new(File::open("song.mp3").unwrap());
	let sink = play(&stream_handle.mixer(), file).unwrap();

	let mut stdout = stdout();
    // stdout.execute(terminal::EnterAlternateScreen)?;
    // terminal::enable_raw_mode()?;
    // stdout.execute(cursor::Hide)?;
    // stdout.execute(terminal::DisableLineWrap)?;
    // stdout.execute(terminal::Clear(terminal::ClearType::All))?;

	for i in 0..85 {
        stdout.execute(style::PrintStyledContent(' '.on(Color::Rgb { r: 255, g: i * 3, b: 0 })))?;
    }
    // stdout.execute(cursor::MoveToNextLine(1))?;
	for i in 0..85 {
        stdout.execute(style::PrintStyledContent(' '.on(Color::Rgb { r: 255 - i * 3, g: 255, b: 0 })))?;
    }
    // stdout.execute(cursor::MoveToNextLine(1))?;
	for i in 0..85 {
        stdout.execute(style::PrintStyledContent(' '.on(Color::Rgb { r: 0, g: 255, b: i * 3 })))?;
    }
    // stdout.execute(cursor::MoveToNextLine(1))?;
	for i in 0..85 {
        stdout.execute(style::PrintStyledContent(' '.on(Color::Rgb { r: 0, g: 255 - i * 3, b: 255 })))?;
    }
    // stdout.execute(cursor::MoveToNextLine(1))?;
	for i in 0..85 {
        stdout.execute(style::PrintStyledContent(' '.on(Color::Rgb { r: i * 3, g: 0, b: 255 })))?;
    }
    // stdout.execute(cursor::MoveToNextLine(1))?;
	for i in 0..85 {
        stdout.execute(style::PrintStyledContent(' '.on(Color::Rgb { r: 255, g: 0, b: 255 - i * 3 })))?;
    }
    // stdout.execute(cursor::MoveToNextLine(1))?;

    // sink.sleep_until_end();

    // terminal::disable_raw_mode()?;
    println!("bye bye");
    return Ok(());
}
