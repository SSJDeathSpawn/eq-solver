
pub enum RowOperation {
    Scale { row: u8, factor: f32 },
    Add { alter: u8, with: u8, scale: f32 },
}

pub struct Matrix {
    pub values: Vec<Vec<f32>>,
    pub order: [u8; 2],
}

impl Matrix {
    #[allow(dead_code)]
    pub fn new(vars: u8) -> Self {
        Matrix {
            values: vec![vec![0_f32; (vars + 1) as usize]; vars as usize],
            order: [vars, vars + 1],
        }
    }

    pub fn with_values(values: Vec<Vec<f32>>) -> Self{
        let order: [u8;2] = [values.len() as u8, values[0].len() as u8];
        Matrix {
            values,
            order
        }
    }

    pub fn row_operation(&mut self, op: RowOperation) {
        match op {
            RowOperation::Scale { row, factor } => self.scale_row(row, factor),
            RowOperation::Add { alter, with, scale } => self.add_rows(alter, with, scale),
        }
    }

    fn scale_row(&mut self, row: u8, factor: f32) {
        self.values[row as usize] = self.values[row as usize]
            .iter()
            .map(|x| x * factor)
            .collect();
    }

    fn add_rows(&mut self, initial: u8, with: u8, scale: f32) {
        for i in 0_u8..self.order[1] {
            self.values[initial as usize][i as usize] = self.values[initial as usize][i as usize]
                - scale * self.values[with as usize][i as usize];
        }
    }

    pub fn solve_equation(&mut self) {
        for i in 0_u8..self.order[0] - 1 {
            let scale = self.values[i as usize][i as usize];

            self.row_operation(RowOperation::Scale {
                row: i,
                factor: 1_f32 / scale,
            });

            
            for j in i+1..self.order[0] {
                let scale_2 = self.values[(j) as usize][i as usize];

                self.row_operation(RowOperation::Add {
                    alter: j,
                    with: i,
                    scale: scale_2,
                });
            }
            
        }

        let scale = self.values[(self.order[0] - 1) as usize][(self.order[0] - 1) as usize];
        
        self.row_operation(RowOperation::Scale {
            row: self.order[0] - 1,
            factor: 1_f32 / scale,
        });


        for i in (1..self.order[0]).rev() {
            let scale = self.values[i as usize][i as usize];

            self.row_operation(RowOperation::Scale {
                row: i,
                factor: 1_f32 / scale,
            });
            
            for j in (0..i).rev() {
                let scale = self.values[j as usize][i as usize];

                self.row_operation(RowOperation::Add {
                    alter: j,
                    with: i,
                    scale,
                })
            }
        }
    }
}
