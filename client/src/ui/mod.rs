use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerWidget};

use crate::ReqItem;

// use crate::utils::{read_db, Request};

pub fn render_home<'a>(//
    // root: &mut Vec<u8>,
    // path: &Vec<Vec<u8>>,
) -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(" ")]),
        Spans::from(vec![Span::raw("  ███████╗██╗  ██╗██████╗ ")]),
        Spans::from(vec![Span::raw("  ╚══███╔╝██║ ██╔╝██╔══██╗")]),
        Spans::from(vec![Span::raw("    ███╔╝ █████╔╝ ██████╔╝")]),
        Spans::from(vec![Span::raw("   ███╔╝  ██╔═██╗ ██╔═══╝ ")]),
        Spans::from(vec![Span::raw("  ███████╗██║  ██╗██║     ")]),
        Spans::from(vec![Span::raw("  ╚══════╝╚═╝  ╚═╝╚═╝     ")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "ZKP Client",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![
            Span::raw("(Default server) "),
            Span::styled("localhost", Style::default().fg(Color::Green)),
            Span::raw(":"),
            Span::styled("8080", Style::default().fg(Color::Red)),
        ]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Press 'r' to access Request page")]),
        // Spans::from(vec![Span::raw("")]),
        // Spans::from(vec![Span::raw("")]),
        // Spans::from(vec![Span::raw(format!("path: {:?}", path))]),
        // Spans::from(vec![Span::raw("")]),
        // Spans::from(vec![Span::raw(format!("root: {:?}", root))]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    home
}

pub fn render_reqs<'a>(
    rect: &mut Frame<CrosstermBackend<Stdout>>,
    req_layout: Vec<Rect>,
    selected_req_item: &ReqItem,
) {
    let req_get_tree = Block::default()
        .borders(Borders::ALL)
        .style(match selected_req_item {
            ReqItem::GET_TREE => Style::default().fg(Color::Cyan),
            // ReqItem::GET_HASH_PARAM => Style::default().fg(Color::White),
            ReqItem::IS_MEMBER => Style::default().fg(Color::White),
        })
        .title("[1] GET TREE")
        .border_type(BorderType::Plain);

    let req_get_hash_param = Block::default()
        .borders(Borders::ALL)
        .style(match selected_req_item {
            ReqItem::GET_TREE => Style::default().fg(Color::White),
            // ReqItem::GET_HASH_PARAM => Style::default().fg(Color::Cyan),
            ReqItem::IS_MEMBER => Style::default().fg(Color::White),
        })
        .title("[2] GET HASH PARAM")
        .border_type(BorderType::Plain);

    let req_is_member = Block::default()
        .borders(Borders::ALL)
        .style(match selected_req_item {
            ReqItem::GET_TREE => Style::default().fg(Color::White),
            // ReqItem::GET_HASH_PARAM => Style::default().fg(Color::White),
            ReqItem::IS_MEMBER => Style::default().fg(Color::Cyan),
        })
        // .title("[3] IS MEMBER")
        .title("[2] IS MEMBER")
        .border_type(BorderType::Thick);

    let selected_style = Style::default()
        .bg(Color::Yellow)
        .fg(Color::LightBlue)
        .add_modifier(Modifier::BOLD);

    let default_style = Style::default()
        .bg(Color::Yellow)
        .fg(Color::Black)
        .add_modifier(Modifier::BOLD);

    let descript_GT = vec![ListItem::new("This is Get Tree Request")];
    // let descript_GP = vec![ListItem::new("This is Get Hash Param Request")];
    let descript_IM = vec![ListItem::new("This is Is Member Request")];

    let list_GT =
        List::new(descript_GT)
            .block(req_get_tree)
            .highlight_style(match selected_req_item {
                ReqItem::GET_TREE => selected_style,
                // ReqItem::GET_HASH_PARAM => default_style,
                ReqItem::IS_MEMBER => default_style,
            });

    // let list_GP = List::new(descript_GP)
    //     .block(req_get_hash_param)
    //     .highlight_style(match selected_req_item {
    //         ReqItem::GET_TREE => default_style,
    //         ReqItem::GET_HASH_PARAM => selected_style,
    //         ReqItem::IS_MEMBER => default_style,
    //     });

    let list_IM =
        List::new(descript_IM)
            .block(req_is_member)
            .highlight_style(match selected_req_item {
                ReqItem::GET_TREE => default_style,
                // ReqItem::GET_HASH_PARAM => default_style,
                ReqItem::IS_MEMBER => selected_style,
            });

    let upper_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                // Constraint::Percentage(33),
                // Constraint::Percentage(33),
                // Constraint::Percentage(33),
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(req_layout[0]);

    rect.render_widget(list_GT, upper_layout[0]);
    // rect.render_widget(list_GP, upper_layout[1]);
    // rect.render_widget(list_IM, upper_layout[2]);
    rect.render_widget(list_IM, upper_layout[1]);

    let req_result = render_log();

    rect.render_widget(req_result, req_layout[1]);
}

pub fn render_log<'a>() -> TuiLoggerWidget<'a> {
    TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_info(Style::default().fg(Color::White))
        .style_trace(Style::default().fg(Color::Cyan))
        .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
        .block(
            Block::default()
                .title("Logs")
                .border_style(Style::default().fg(Color::White).bg(Color::Black))
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
}
