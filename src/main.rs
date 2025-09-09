use std::fs::File;
use std::io::{Result, Stdout, stdout, BufReader};
use rodio::{OutputStreamBuilder, play};
use crossterm::{cursor, terminal, ExecutableCommand};

fn main() -> Result<()> {
	let stream_handle = OutputStreamBuilder::open_default_stream()
	    .expect("Could not open default audio stream");
	let file = BufReader::new(File::open("song.mp3").unwrap());
	let sink = play(&stream_handle.mixer(), file).unwrap();

	let mut stdout = stdout();
    stdout.execute(terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    stdout.execute(cursor::Hide)?;
    stdout.execute(terminal::DisableLineWrap)?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

	sink.sleep_until_end();

    terminal::disable_raw_mode()?;
    println!("bye bye");
    return Ok(());
}
