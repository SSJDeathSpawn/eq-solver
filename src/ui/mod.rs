use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::application::{App, CurrentlyEditing};

pub fn ui(f: &mut Frame, app: &App) {
    let matrix_size = app.mat_size.parse::<i8>().unwrap_or(0_i8);
    let active_style = Style::default().fg(Color::Black).bg(Color::White);

    let chunks = Layout::default()
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(1),    // Matrix
            Constraint::Length(3), // Footer
        ])
        .split(f.size());

    let title = Paragraph::new(Text::styled(
        "Guass-Jordan Equation Solver",
        Style::default().fg(Color::Green),
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default()),
    );

    f.render_widget(title, chunks[0]);

    let main_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let main_layout = Layout::default()
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(main_block.inner(chunks[1]));

    let mat_size = Paragraph::new(Text::styled(
        format!("Number of variables: {}", app.mat_size),
        Style::default(),
    ))
    .block(Block::default());

    let final_mat_size = if CurrentlyEditing::EquationSize == app.currently_editing {
        mat_size.style(active_style)
    } else {
        mat_size.style(Style::default().fg(Color::White))
    };

    f.render_widget(final_mat_size, main_layout[0]);

    let mut list_items = Vec::<ListItem>::new();

    let var_list: Vec<char> = (0..matrix_size as u8)
        .map(|x| {
            if x < 3 {
                ('x' as u8 + x) as char
            } else {
                ('x' as u8 - x + 2) as char
            }
        })
        .chain(vec!['='])
        .collect();

    if let CurrentlyEditing::Equation { x, y } = app.currently_editing {
        for i in 0..matrix_size {
            let mut string = Vec::<Span>::new();
            let first = (matrix_size + 1) * i;
            let last = (matrix_size + 1) * (i + 1) - 1;
            for j in first..=last {
                let var_n = j - (matrix_size + 1) * i;
                let var = var_list[var_n as usize];

                if j == last {
                    string.push(Span::styled(
                        format!("{:^3}", '='),
                        Style::default().fg(Color::White),
                    ));
                }
                if y * (matrix_size + 1) + x == j {
                    string.push(Span::styled(
                        format!("{:>4}", app.matrix[j as usize]),
                        active_style,
                    ));
                } else {
                    string.push(Span::styled(
                        format!("{:>4}", app.matrix[j as usize]),
                        Style::default().fg(Color::White),
                    ));
                }
                if j != last {
                    string.push(Span::styled(
                        format!("{:<3}", var),
                        Style::default().fg(Color::White),
                    ));
                }
            }

            list_items.push(ListItem::new(Line::from(string)));
        }
    }

    let list = List::new(list_items);

    f.render_widget(list, main_layout[1]);
    f.render_widget(main_block, chunks[1]);

    let key_hints = match app.currently_editing {
        CurrentlyEditing::Equation { .. } => Span::styled("q / Esc - Quit | h j k l / Left Down Up Right - Move Around Matrix | Tab - Switch fields", Style::default().fg(Color::Yellow)),
        CurrentlyEditing::EquationSize => Span::styled("q - Quit | Tab / Enter - Next field", Style::default().fg(Color::Yellow)),
    };

    let footer =
        Paragraph::new(Line::from(key_hints)).block(Block::default().borders(Borders::ALL));

    f.render_widget(footer, chunks[2]);
}
