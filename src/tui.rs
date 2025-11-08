use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

use super::differ::Diff;

/// Render a TUI that displays the three sections of the Diff.
pub fn render_diff(diff: Diff) -> io::Result<()> {
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Event/tick config
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    // Main loop
    loop {
        terminal.draw(|f| {
            draw_ui(f, &diff);
        })?;

        // Event polling
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        let mut _maybe_resize = false;

        if event::poll(timeout)? {
            match event::read()? {
                CEvent::Key(key_event) => {
                    // Quit on 'q' or Esc
                    if matches!(key_event.code, KeyCode::Char('q') | KeyCode::Esc) {
                        break;
                    }
                }
                CEvent::Resize(_, _) => {
                    _maybe_resize = true;
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

/// Draw the entire UI for a given Diff.
fn draw_ui(frame: &mut ratatui::Frame, diff: &Diff) {
    let area = frame.area();

    // Top-level layout: Title bar + body
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // header
                Constraint::Min(0),    // body
            ]
            .as_ref(),
        )
        .split(area);

    draw_header(frame, chunks[0]);
    draw_body(frame, chunks[1], diff);
}

/// Draws a simple header with app name
fn draw_header(frame: &mut ratatui::Frame, area: Rect) {
    let header_block = Block::default()
        .title(Span::styled(
            " riff - environment diff viewer ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL);

    let text = vec![Line::from(vec![
        Span::raw("Press "),
        Span::styled(
            "q",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" or "),
        Span::styled(
            "Esc",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" to exit."),
    ])];

    let paragraph = Paragraph::new(text).block(header_block);
    frame.render_widget(paragraph, area);
}

/// Body layout: three columns (unique A, unique B, modified/diff)
fn draw_body(frame: &mut ratatui::Frame, area: Rect, diff: &Diff) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    draw_unique_section(frame, rows[0], diff);
    draw_diff_table(frame, rows[1], &diff.diff);
}

fn draw_unique_section(frame: &mut ratatui::Frame, area: Rect, diff: &Diff) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    draw_unique_table(
        frame,
        cols[0],
        "Unique A",
        &diff.unique_a,
        Style::default().fg(Color::Cyan),
    );
    draw_unique_table(
        frame,
        cols[1],
        "Unique B",
        &diff.unique_b,
        Style::default().fg(Color::Magenta),
    );
}

/// Draw table for unique key/value maps.
fn draw_unique_table(
    frame: &mut ratatui::Frame,
    area: Rect,
    title: &str,
    map: &std::collections::HashMap<&str, &str>,
    color: Style,
) {
    let header_style = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);

    let header = Row::new(vec![
        Cell::from(Span::styled("Key", header_style)),
        Cell::from(Span::styled("Value", header_style)),
    ]);

    let rows: Vec<Row> = map
        .iter()
        .map(|(k, v)| {
            Row::new(vec![
                Cell::from((*k).to_string()),
                Cell::from((*v).to_string()),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        &[Constraint::Percentage(40), Constraint::Percentage(60)],
    )
    .header(header)
    .block(
        Block::default()
            .title(Span::styled(format!(" {title} ({}) ", map.len()), color))
            .borders(Borders::ALL),
    )
    .column_spacing(1);

    frame.render_widget(table, area);
}

/// Draw table for differing keys (left vs right values).
fn draw_diff_table(
    frame: &mut ratatui::Frame,
    area: Rect,
    diff_map: &std::collections::HashMap<&str, (&str, &str)>,
) {
    let header_style = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);

    let header = Row::new(vec![
        Cell::from(Span::styled("Key", header_style)),
        Cell::from(Span::styled("Left Value", header_style)),
        Cell::from(Span::styled("Right Value", header_style)),
    ]);

    let rows: Vec<Row> = diff_map
        .iter()
        .map(|(k, (lv, rv))| {
            let value_style = if lv != rv {
                Style::default().fg(Color::Red)
            } else {
                Style::default()
            };
            Row::new(vec![
                Cell::from((*k).to_string()),
                Cell::from(Span::styled((*lv).to_string(), value_style)),
                Cell::from(Span::styled((*rv).to_string(), value_style)),
            ])
        })
        .collect();
    let table = Table::new(
        rows,
        &[
            Constraint::Percentage(30),
            Constraint::Percentage(35),
            Constraint::Percentage(35),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(Span::styled(
                format!(" Modified ({}) ", diff_map.len()),
                Style::default().fg(Color::Yellow),
            ))
            .borders(Borders::ALL),
    )
    .column_spacing(1);

    frame.render_widget(table, area);
}
