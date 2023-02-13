use tui::{
    layout::{Alignment, Constraint},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table},
};

use crate::utils::read_db;

pub fn render_home<'a>() -> Paragraph<'a> {
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

pub fn render_pets<'a>(pet_list_state: &ListState) -> (List<'a>, Table<'a>) {
    let pets = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Info")
        .border_type(BorderType::Plain);

    let pet_list = read_db().expect("can fetch pet list");
    let items: Vec<_> = pet_list
        .iter()
        .map(|pet| {
            ListItem::new(Spans::from(vec![Span::styled(
                pet.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let selected_pet = pet_list
        .get(
            pet_list_state
                .selected()
                .expect("there is always a selected pet"),
        )
        .expect("exists")
        .clone();

    let list = List::new(items).block(pets).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let pet_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_pet.id.to_string())),
        Cell::from(Span::raw(selected_pet.name)),
        Cell::from(Span::raw(selected_pet.category)),
        Cell::from(Span::raw(selected_pet.age.to_string())),
        // Cell::from(Span::raw(selected_pet.created_at.to_string())),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "AccId",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Root",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Balance",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Detail")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(10),
        Constraint::Percentage(15),
        Constraint::Percentage(35),
        Constraint::Percentage(35),
        // Constraint::Percentage(20),
    ]);

    (list, pet_detail)
}
