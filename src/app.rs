use anyhow::Result;

use crate::matrix::Matrix;

#[derive(PartialEq)]
pub enum CurrentlyEditing {
    EquationSize,
    Equation { x: i8, y: i8 },
}

pub struct App {
    pub mat_size: String,
    pub matrix: Vec<String>,
    pub currently_editing: CurrentlyEditing,
}

impl App {
    pub fn new() -> Self {
        App {
            mat_size: String::new(),
            matrix: vec![String::new()],
            currently_editing: CurrentlyEditing::EquationSize,
        }
    }

    pub fn toggle_editing(&mut self, forwards: bool) {
        let matrix_size = self.mat_size.parse::<i8>().unwrap();
        match self.currently_editing {
            CurrentlyEditing::EquationSize => {
                if forwards {
                    let matrix_size = self.mat_size.parse::<usize>().unwrap();
                    if self.matrix.len() != (matrix_size + 1) * (matrix_size) {
                        self.matrix = vec!["".to_string(); (matrix_size + 1) * (matrix_size)];
                    }
                    self.currently_editing = CurrentlyEditing::Equation { x: 0, y: 0 }
                }
            }
            CurrentlyEditing::Equation { x, y } => {
                if forwards {
                    self.currently_editing = CurrentlyEditing::Equation {
                        x: (x + 1) % (matrix_size + 1),
                        y: (y + (x + 1) / (matrix_size + 1)) % matrix_size,
                    }
                } else {
                    print!("{x}, {y}");
                    if x == y && y == 0_i8 {
                        self.currently_editing = CurrentlyEditing::EquationSize;
                        return
                    }
                    self.currently_editing = CurrentlyEditing::Equation {
                        x: (x - 1) % (matrix_size + 1),
                        y: (y + (x - 1) / (matrix_size + 1)) % matrix_size,
                    }
                }
            }
        }
    }

    pub fn print_result(&self) -> Result<()> {
        let mut iter = self
            .matrix
            .clone()
            .into_iter()
            .map(|x| x.parse::<f32>().unwrap_or(1.0));

        let matrix_size = self.mat_size.parse::<u8>().unwrap();
        let values: Vec<Vec<f32>> = (0..matrix_size)
            .map(|_| iter.by_ref().take((matrix_size + 1_u8) as usize).collect())
            .collect();
        let mut mat: Matrix = Matrix::with_values(values);
        mat.solve_equation();
        for i in 0..std::cmp::min(3_u8, matrix_size) {
            println!(
                "{} = {}",
                ('x' as u8 + i) as char,
                mat.values[i as usize][(matrix_size) as usize]
            );
        }
        for i in 3_u8..matrix_size {
            println!(
                "{} = {}",
                ('x' as u8 - i + 2) as char,
                mat.values[i as usize][(matrix_size) as usize]
            );
        }
        Ok(())
    }
}
