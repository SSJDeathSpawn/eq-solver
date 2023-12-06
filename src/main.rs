extern crate crossterm;

mod app;
mod matrix;
mod ui;
use app as application;
use application::CurrentlyEditing;

use std::{cmp, error::Error, io};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;

use crate::application::App;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Ok(did_finish) = res {
        if did_finish {
            app.print_result()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}")
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;
        let matrix_size = app.mat_size.parse::<i8>().unwrap_or(0);
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match key.code {
                KeyCode::Tab => {
                    app.toggle_editing(true);
                }
                KeyCode::BackTab => {
                    app.toggle_editing(false);
                }
                KeyCode::Backspace => match app.currently_editing {
                    CurrentlyEditing::EquationSize => {
                        app.mat_size.pop();
                    }
                    CurrentlyEditing::Equation { x, y } => {
                        app.matrix[(y * (matrix_size + 1) + x) as usize].pop();
                    }
                },
                KeyCode::Left | KeyCode::Char('h') => {
                    if let CurrentlyEditing::Equation { x, y } = app.currently_editing {
                        if x > 0 || y > 0 {
                            app.currently_editing = CurrentlyEditing::Equation {
                                x: (x - 1) % (matrix_size + 1),
                                y: y + (x - 1) / (matrix_size + 1),
                            };
                        }
                    }
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    if let CurrentlyEditing::Equation { x, y } = app.currently_editing {
                        if x < matrix_size + 1 || y < matrix_size {
                            app.currently_editing = CurrentlyEditing::Equation {
                                x: (x + 1) % (matrix_size + 1),
                                y: y + (x + 1) / (matrix_size + 1),
                            };
                        }
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if let CurrentlyEditing::Equation { x, y } = app.currently_editing {
                        if y < matrix_size {
                            app.currently_editing = CurrentlyEditing::Equation {
                                x,
                                y: cmp::min(matrix_size - 1, y - 1),
                            };
                        }
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if let CurrentlyEditing::Equation { x, y } = app.currently_editing {
                        if y < matrix_size {
                            app.currently_editing = CurrentlyEditing::Equation {
                                x,
                                y: cmp::min(matrix_size - 1, y + 1),
                            };
                        }
                    }
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    return Ok(false);
                }
                KeyCode::Enter => {
                    if let CurrentlyEditing::EquationSize = app.currently_editing {
                        app.toggle_editing(true);
                    } else {
                        return Ok(true);
                    }
                }
                KeyCode::Char(ch) => {
                    if ch.is_ascii_digit()
                        || ((app.currently_editing == CurrentlyEditing::EquationSize)
                            != (ch == '.' || ch == '-'))
                    {
                        match app.currently_editing {
                            CurrentlyEditing::EquationSize => {
                                app.mat_size.push(ch);
                            }
                            CurrentlyEditing::Equation { x, y } => {
                                app.matrix[(y * (matrix_size + 1) + x) as usize].push(ch);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
