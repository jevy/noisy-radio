use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::Source;
use rodio::dynamic_mixer::{self, DynamicMixer};

use std::{io, thread, time::Duration};
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

struct Radio {
    tuned_channel: i8,
}

impl Radio {
    fn new() -> Self {
        Radio {
            tuned_channel: 50,
        }
    }

    pub fn tune_up(&mut self) {
        self.tuned_channel += (1 as i8);
    }

    pub fn tune_down(&mut self) {
        self.tuned_channel -= (1 as i8);
    }
}

struct Channel {
    freq: i8,
}

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

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();

    // Load a sound from a file, using a path relative to Cargo.toml
    let nature_file = BufReader::new(File::open("src/nature.mp3").unwrap());
    let static_file = BufReader::new(File::open("src/radio-static.mp3").unwrap());
    // Decode that sound file into a source
    let nature_source = Decoder::new(nature_file).unwrap();
    let static_source = Decoder::new(static_file).unwrap();
    
    let (tx, mut rx) = dynamic_mixer::mixer(1, 48000);

    tx.add(static_source);
    tx.add(nature_source);
    sink.append(rx);

    let mut radio = Radio::new();
  
    loop {

        terminal.draw(|f| {
            let size = f.size();
            let freq_text = Paragraph::new(radio.tuned_channel.to_string())
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Center);
            f.render_widget(freq_text, size);
        })?;

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