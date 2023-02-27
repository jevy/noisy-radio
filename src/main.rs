use std::io;

use tui::{
    backend::Backend,
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal
};
use tui::widgets::{Paragraph};
use tui::style::{Color, Modifier, Style};
use tui::layout::{Alignment};
use crossterm::{
    event::{self, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crossterm::event::Event::Key;

mod radio;


fn main() -> Result<(), io::Error> {
    
    enable_raw_mode()?;
    execute!(
        std::io::stdout(),
        EnterAlternateScreen,
        EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        println!("{}", e.to_string());
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), std::io::Error> {

    // TODO: Adjust volumes iterating over and modifying the channels (mutable iterators?)
    // TODO: stream_handle moved into Radio.
    let (_stream, stream_handle) = match rodio::OutputStream::try_default() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Couldn't open audio, with error {e}");
            std::process::exit(1);
        }
    };


    let mut radio = radio::Radio::new(stream_handle);
    radio.add_radio_channel("src/nature.mp3".to_owned(), 25);
    radio.add_radio_channel("src/radio-static.mp3".to_owned(), 0);
  
    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // let sink = Sink::try_new(&stream_handle).unwrap();
    // let filename = "src/nature.mp3";
    // let file = BufReader::new(File::open(filename).unwrap());
    // let file_source = Decoder::new(file).unwrap();
    // sink.append(file_source);

    loop {

        terminal.draw(|f| {
            let size = f.size();
            let freq_text = Paragraph::new(radio.current_freq.to_string())
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Center);
            f.render_widget(freq_text, size);
        });

        if let Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('+') => {
                    // println!("Tune up");
                    radio.tune_up();
                }
                KeyCode::Char('-') => {
                    radio.tune_down();
                }
                _ => {}
            // println!("{:?}", event),
            }
        }

        //     match key.code {
        //         KeyCode::Char('+') => {
        //             radio.tune_up();
        //         }
        //         KeyCode::Char('-') => {
        //             // Decrease tuning
        //         }
        //         _ => {}
        //     }
        // }
        // sink.sleep_until_end();
    }

}