use crossterm::{
    event::{self, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    Terminal,
};

use rpc_request::{get_path::get_path, get_root::get_root, send_proof::send_proof};
use ui::{render_home, render_reqs};

mod rpc_request;
mod ui;
mod utils;

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Home,
    Request,
}

enum InputMode {
    Normal,
    Edit,
}

pub struct App {
    input: String,
    input_mode: InputMode,
    //
    leaf: u8,
    leaf_idx: u8,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            //
            leaf: 0,
            leaf_idx: 0,
        }
    }
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Request => 1,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ReqItem {
    GetPath,
    SendProof,
}

impl From<ReqItem> for usize {
    fn from(input: ReqItem) -> usize {
        match input {
            ReqItem::GetPath => 0,
            ReqItem::SendProof => 2,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;

    tui_logger::init_logger(log::LevelFilter::Trace).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Info);

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::default();

    let menu_titles = vec!["Home", "Requests"];
    let mut active_menu_item = MenuItem::Home;
    let mut selected_req_item = ReqItem::GetPath;

    let mut path_data: Vec<Vec<u8>> = Vec::new();
    let mut root_data: Vec<u8> = vec![];

    loop {
        terminal.draw(|rect| {
            let size = rect.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let footer = Paragraph::new("2022 KEEPER Tech Report, Client TUI")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("")
                        .border_type(BorderType::Plain),
                );

            let menu = menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();

            let tabs = Tabs::new(menu)
                .select(active_menu_item.into())
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw("|"));

            rect.render_widget(tabs, chunks[0]);

            match active_menu_item {
                MenuItem::Home => rect.render_widget(render_home(), chunks[1]),
                MenuItem::Request => {
                    let req_layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [
                                Constraint::Percentage(20),
                                Constraint::Percentage(20),
                                Constraint::Percentage(60),
                            ]
                            .as_ref(),
                        )
                        .split(chunks[1]);

                    render_reqs(
                        //
                        rect,
                        req_layout,
                        &selected_req_item,
                        &app,
                    );
                }
            }
            rect.render_widget(footer, chunks[2]);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => match app.input_mode {
                    InputMode::Normal => {
                        disable_raw_mode()?;
                        terminal.show_cursor()?;
                        break;
                    }
                    InputMode::Edit => {
                        app.input.push('q');
                    }
                },

                KeyCode::Char('h') => match app.input_mode {
                    InputMode::Normal => {
                        active_menu_item = MenuItem::Home;
                    }
                    InputMode::Edit => {
                        app.input.push('h');
                    }
                },

                KeyCode::Char('r') => match app.input_mode {
                    InputMode::Normal => {
                        active_menu_item = MenuItem::Request;
                    }
                    InputMode::Edit => {
                        app.input.push('r');
                    }
                },

                KeyCode::Char('1') => match app.input_mode {
                    InputMode::Normal => {
                        selected_req_item = ReqItem::GetPath;
                        log::info!(" Request `Get Tree`");
                    }
                    InputMode::Edit => {
                        app.input.push('1');
                    }
                },

                KeyCode::Char('2') => match app.input_mode {
                    InputMode::Normal => {
                        selected_req_item = ReqItem::SendProof;
                        log::info!(" Request `Is Member`");
                    }
                    InputMode::Edit => {
                        app.input.push('2');
                    }
                },

                KeyCode::Char(q) => match app.input_mode {
                    InputMode::Normal => {}
                    InputMode::Edit => {
                        app.input.push(q);
                    }
                },

                KeyCode::Backspace => match app.input_mode {
                    InputMode::Normal => {}
                    InputMode::Edit => {
                        if app.input.len() > 0 {
                            app.input.pop();
                        }
                    }
                },

                KeyCode::Enter => match app.input_mode {
                    InputMode::Normal => match selected_req_item {
                        ReqItem::GetPath => {
                            app.input_mode = InputMode::Edit;
                        }

                        ReqItem::SendProof => {
                            app.input_mode = InputMode::Edit;
                        }
                    },
                    InputMode::Edit => match selected_req_item {
                        ReqItem::GetPath => {
                            let name = app.input.clone();

                            let url = String::from("http://127.0.0.1:8080/get_path");
                            let url = url.parse::<hyper::Uri>().unwrap();
                            path_data = get_path(url, format!("{}_path", name.clone()))
                                .await
                                .unwrap();

                            let url = String::from("http://127.0.0.1:8080/get_root");
                            let url = url.parse::<hyper::Uri>().unwrap();
                            root_data = get_root(url, format!("{}_root", name.clone()))
                                .await
                                .unwrap();

                            app.input_mode = InputMode::Normal;
                            app.input.clear();
                        }
                        ReqItem::SendProof => {
                            let value: Vec<u8> = app
                                .input
                                .split(' ')
                                .map(|v| match v.parse::<u8>() {
                                    Ok(n) => n,
                                    Err(_e) => {
                                        log::error!("Error, non-integer value found, return 0");
                                        0
                                    }
                                })
                                .collect();

                            app.leaf = value[0];
                            app.leaf_idx = value[1];

                            log::info!("leaf value: {:?}", app.leaf);
                            log::info!("leaf index: {:?}", app.leaf_idx);

                            {
                                log::warn!("[!] generating proof...");

                                let url = String::from("http://127.0.0.1:8080/send_proof");
                                let url = url.parse::<hyper::Uri>().unwrap();
                                match send_proof(
                                    url,
                                    &path_data,
                                    &root_data,
                                    app.leaf,
                                    app.leaf_idx,
                                )
                                .await
                                {
                                    Ok(_) => {
                                        log::warn!("gen proof done!");
                                    }
                                    Err(e) => {
                                        log::error!("Error: {:?}", e);
                                    }
                                };
                            }

                            app.input_mode = InputMode::Normal;
                            app.input.clear();
                        }
                    },
                },
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
