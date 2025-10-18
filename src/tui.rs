use std::{sync::mpsc};

use color_eyre::Result;
use ratatui::{
    Frame, Terminal, TerminalOptions, Viewport, crossterm::event, prelude::Backend, text::Line,
    widgets::Block,
};

use super::differ::Diff;

//        Remy
//        .  ,
//       (\;/)
//      oo   \//,        _
//    ,/_;~      \,     / '
//    "'    (  (   \    !
//         //  \   |__.'
//        '~  '~----''

enum Event {
    Input(event::KeyEvent),
    Resize,
    Tick,
}

pub fn render_diff(diff: Diff) -> Result<()> {
    let mut terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(20),
    });

    let (tx, rx) = mpsc::channel();

    let app_result = run(&mut terminal, &diff, rx);

    app_result
}

fn run<'a>(
    terminal: &mut Terminal<impl Backend>,
    diff: &Diff<'a>,
    rx: mpsc::Receiver<Event>,
) -> Result<()> {
    let mut redraw = true;
    loop {
        if redraw {
            terminal.draw(|frame| draw(frame, diff))?;
        }
        redraw = true;

        match rx.recv()? {
            Event::Input(event) => {
                if event.code == event::KeyCode::Char('q') {
                    break;
                }
            }
            Event::Resize => {
                terminal.autoresize()?;
            }
            Event::Tick => {}
        };
    }

    Ok(())
}

fn draw<'a>(frame: &mut Frame, diff: &Diff<'a>) {
    let area = frame.area();

    let block = Block::new().title(Line::from("Diff").centered());
    frame.render_widget(block, area);
}
