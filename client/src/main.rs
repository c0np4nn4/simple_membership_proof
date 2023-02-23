use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
// use payment::ledger::Parameters;
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
// use utils::make_tree;

// mod payment;
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
    GET_PATH,
    // GET_HASH_PARAM,
    SEND_PROOF,
}

impl From<ReqItem> for usize {
    fn from(input: ReqItem) -> usize {
        match input {
            ReqItem::GET_PATH => 0,
            // ReqItem::GET_HASH_PARAM => 1,
            ReqItem::SEND_PROOF => 2,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_BACKTRACE", "1");

    enable_raw_mode().expect("can run in raw mode");
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

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let menu_titles = vec!["Home", "Requests"];
    let mut active_menu_item = MenuItem::Home;
    let mut selected_req_item = ReqItem::GET_PATH;

    let mut path_data: Vec<Vec<u8>> = Vec::new();
    // let mut root_data: Vec<u8> = Vec::default();
    let mut root_data: Vec<u8> = vec![1, 2, 3, 4];

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
                MenuItem::Home => {
                    rect.render_widget(
                        render_home(
                            // for check
                            //
                            // &mut root_data,
                            // &path_data,
                        ),
                        chunks[1],
                    )
                }
                MenuItem::Request => {
                    let req_layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[1]);

                    render_reqs(rect, req_layout, &selected_req_item);
                }
            }
            rect.render_widget(footer, chunks[2]);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char('h') => active_menu_item = MenuItem::Home,
                KeyCode::Char('r') => active_menu_item = MenuItem::Request,

                KeyCode::Char('1') => {
                    selected_req_item = ReqItem::GET_PATH;
                    log::info!(" Request `Get Tree`");
                }

                // KeyCode::Char('2') => {
                //     selected_req_item = ReqItem::GET_HASH_PARAM;
                //     log::info!(" Request `Get Hash Param`");
                // }
                // KeyCode::Char('3') => {
                //     selected_req_item = ReqItem::IS_MEMBER;
                //     log::info!(" Request `Is Member`");
                // }
                KeyCode::Char('2') => {
                    selected_req_item = ReqItem::SEND_PROOF;
                    log::info!(" Request `Is Member`");
                }

                KeyCode::Enter => match selected_req_item {
                    ReqItem::GET_PATH => {
                        let url = String::from("http://127.0.0.1:8080/get_path");
                        let url = url.parse::<hyper::Uri>().unwrap();
                        path_data = get_path(url).await.unwrap();

                        let url = String::from("http://127.0.0.1:8080/get_root");
                        let url = url.parse::<hyper::Uri>().unwrap();
                        root_data = get_root(url).await.unwrap();
                    }

                    ReqItem::SEND_PROOF => {
                        log::warn!("[!] generating proof...");

                        let mut url = String::from("http://127.0.0.1:8080/send_proof");

                        let url = url.parse::<hyper::Uri>().unwrap();

                        match send_proof(url, &path_data, &root_data).await {
                            Ok(res) => {
                                log::warn!("gen proof done!");
                            }
                            Err(e) => {
                                log::error!("Error: {:?}", e);
                            }
                        };
                    }
                },
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
