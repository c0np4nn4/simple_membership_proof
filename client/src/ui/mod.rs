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

use crate::{App, InputMode, ReqItem};

pub fn render_home<'a>(//
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
    app: &App,
) {
    let req_get_path = Block::default()
        .borders(Borders::ALL)
        .style(match selected_req_item {
            ReqItem::GetPath => Style::default().fg(Color::Cyan),
            ReqItem::SendProof => Style::default().fg(Color::White),
        })
        .title("[1] GET TREE")
        .border_type(BorderType::Plain);

    let req_is_member = Block::default()
        .borders(Borders::ALL)
        .style(match selected_req_item {
            ReqItem::GetPath => Style::default().fg(Color::White),
            ReqItem::SendProof => Style::default().fg(Color::Cyan),
        })
        .title("[2] SEND PROOF")
        .border_type(BorderType::Thick);

    let selected_style = Style::default()
        .bg(Color::Yellow)
        .fg(Color::LightBlue)
        .add_modifier(Modifier::BOLD);

    let default_style = Style::default()
        .bg(Color::Yellow)
        .fg(Color::Black)
        .add_modifier(Modifier::BOLD);

    let descript_gp = vec![ListItem::new(
        "Get necessary data from the Server.\nPaths, Root value, etc.",
    )];
    let descript_im = vec![ListItem::new(
        "Press `Enter` to input `Leaf` and `Leaf Index`",
    )];

    let list_gp =
        List::new(descript_gp)
            .block(req_get_path)
            .highlight_style(match selected_req_item {
                ReqItem::GetPath => selected_style,
                ReqItem::SendProof => default_style,
            });

    let list_im =
        List::new(descript_im)
            .block(req_is_member)
            .highlight_style(match selected_req_item {
                ReqItem::GetPath => default_style,
                ReqItem::SendProof => selected_style,
            });

    let upper_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                //
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(req_layout[0]);

    rect.render_widget(list_gp, upper_layout[0]);
    rect.render_widget(list_im, upper_layout[1]);

    let input_member_id = render_input_member_id(app);
    rect.render_widget(input_member_id, req_layout[1]);

    let req_result = render_log();
    rect.render_widget(req_result, req_layout[2]);
}

pub fn render_input_member_id<'a>(app: &'a App) -> Paragraph<'a> {
    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Edit => Style::default().fg(Color::Yellow),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("  Input Box | [1]: String | [2]: u8<space>u8  "),
        );
    input
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
