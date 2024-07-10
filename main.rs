use std::time::Instant;
use rayon::prelude::*;
use std::sync::Mutex;

pub struct Matrix {
    rows: i32,
    columns: i32,
    values: Vec<f32>,
}

impl Matrix {
    pub fn new (rows: i32, columns: i32) -> Matrix {
        let size: usize = (rows * columns) as usize;
        Matrix {
            rows,
            columns,
            values: vec![0.0; size],
        }
    }

    pub fn get_val (&self, row_ind: i32, col_ind: i32) -> Option<f32> {
        if row_ind > self.rows || col_ind > self.columns || row_ind < 0 || col_ind < 0 {
            None
        } else {
            let index: usize = (row_ind * col_ind + col_ind) as usize;
            Some(self.values[index])
        }
    }

    pub fn print_val (&self, row_ind: i32, col_ind: i32) {
        match self.get_val(row_ind, col_ind) {
            Some(val) => println!("{}", val),
            None => println!("Index out of range"),
        }
    }

    pub fn print_all_val (&self) {
        for ind in 0..self.values.len() {
            print!("{} ", self.values[ind]);
            if ((ind) as i32 + 1) % self.columns == 0 && ind > 0 {
                println!("\n");
            }
        }
    }
    
    pub fn set_val (&mut self, row_ind: i32, col_ind: i32, value: f32) -> Result<(), &'static str> {
        if row_ind > self.rows || col_ind > self.columns || row_ind < 0 || col_ind < 0 {
            Err("Index out of bounds")
        } else {
            let index: usize = (row_ind * col_ind + col_ind) as usize;
            self.values[index] = value;
            Ok(())
        }
    }

    pub fn set_all (&mut self, values: &Vec<f32>) -> Result<(), &'static str> {
        if values.len() != (self.columns * self.rows)  as usize {
            Err("Number of values mismatch")
        } else {
            self.values = values.clone();
            Ok(())
        }
    }
    // No optimizations
    /* pub fn dot (&self, other: &Matrix) -> Matrix {
        assert!(self.columns == other.rows);
        let mut result: Matrix = Matrix::new(self.rows, other.columns);
        for row in 0..self.rows {
            for col in 0..other.columns{
                let mut sum: f32 = 0.0;
                for ind in 0..self.columns {
                    let index_self = (row * self.columns + ind) as usize;
                    let index_other = (ind * other.columns + col) as usize;
                    sum += self.values[index_self] * other.values[index_other];
                }
                let index_result = (row * other.columns + col) as usize;
                result.values[index_result] = sum
            }
        }
        result

    } */

    // Cache friendly accessing
    /* pub fn dot (&self, other: &Matrix) -> Matrix {
        assert!(self.columns == other.rows);
        let mut result: Matrix = Matrix::new(self.rows, other.columns);
        for row in 0..self.rows {
            for ind in 0..self.columns {
                for col in 0..other.columns{
                    let index_self = (row * self.columns + ind) as usize;
                    let index_other = (ind * other.columns + col) as usize;
                    result.values[(row * other.columns + col) as usize] += self.values[index_self] * other.values[index_other];

                }
                
            }
        }
        result

    } */
     
    // Column tiling

    /* pub fn dot (&self, other: &Matrix) -> Matrix {
        assert!(self.columns == other.rows);
        let mut result: Matrix = Matrix::new(self.rows, other.columns);
        let tileSize = 16;
        let mut col_tiles_start = 0;
        while col_tiles_start < self.columns {
            let col_tiles_end = col_tiles_start + tileSize;
            for row in 0..self.rows {
                for ind in col_tiles_start..col_tiles_end {
                    for col in 0..other.columns{
                        let index_self = (row * self.columns + ind) as usize;
                        let index_other = (ind * other.columns + col) as usize;
                        result.values[(row * other.columns + col) as usize] += self.values[index_self] * other.values[index_other];

                    }
                    
                }
            }
            col_tiles_start += tileSize;
        }   
        result

    } */

    pub fn dot (&self, other: &Matrix) -> Matrix {
        assert!(self.columns == other.rows);
        let mut result: Matrix = Matrix::new(self.rows, other.columns);
        let result_values_mutex = Mutex::new(result.values);
        let tileSize: i32 = 256;
        (0..self.rows).into_par_iter().step_by(tileSize as usize).for_each( |row_tiles_start| {
            let mut partial_res = vec![0.0; (self.rows * other.columns) as usize];
            for col_tiles_start in (0..other.columns).step_by(tileSize as usize) {

                for inner_tiles_start in (0..self.columns).step_by(tileSize as usize) {

                    for row in row_tiles_start..(row_tiles_start + tileSize) {

                        for inner in inner_tiles_start..(inner_tiles_start + tileSize) {

                            for col in col_tiles_start..(col_tiles_start + tileSize){
                                let index_self = (row * self.columns + inner) as usize;
                                let index_other = (inner * other.columns + col) as usize;
                                partial_res[(row * other.columns + col) as usize] += self.values[index_self] * other.values[index_other];

                            }
                        }
                    }
                }
            }
            let mut result_values = result_values_mutex.lock().unwrap();
            for (i, &value) in partial_res.iter().enumerate() {
                result_values[i] += value;
            }

        });
        {
            let computed_values = result_values_mutex.lock().unwrap();
            result.set_all(&computed_values);
        }

        result
        

    }

    fn dot(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.columns, other.rows);
        let mut result = Matrix::new(self.rows, other.columns);
        let result_values_mutex = Mutex::new(vec![0.0; (self.rows * other.columns) as usize]);
        let tileSize: i32 = 256;

        (0..self.rows).into_par_iter().step_by(tileSize as usize).for_each(|row_tiles_start| {
            let mut partial_res = vec![0.0; (self.rows * other.columns) as usize];
            for col_tiles_start in (0..other.columns).step_by(tileSize as usize) {
                for inner_tiles_start in (0..self.columns).step_by(tileSize as usize) {
                    for row in row_tiles_start..(row_tiles_start + tileSize) {

                        for inner in inner_tiles_start..(inner_tiles_start + tileSize) {

                            for col in col_tiles_start..(col_tiles_start + tileSize){
                                let index_self = (row * self.columns + inner) as usize;
                                let index_other = (inner * other.columns + col) as usize;
                                partial_res[(row * other.columns + col) as usize] += self.values[index_self] * other.values[index_other];
                            }
                        }
                    }
                }
            }
            let mut result_values = result_values_mutex.lock().unwrap();
            for (i, &value) in partial_res.iter().enumerate() {
                result_values[i] += value;
            }
        });

        {
            let computed_values = result_values_mutex.lock().unwrap();
            result.set_all(&computed_values);
        }

        result
    }
    
}


fn main() {
    let mut my_mat = Matrix::new(1024,1024);
    let mut my_mat2 = Matrix::new(1024,1024);

    // let values = vec![1.0, 3.0, 4.0, 5.0, 6.0, 7.0];

    // my_mat.set_all(&values);
    // my_mat2.set_all(&values);

    let now = Instant::now();
    let my_mat3 = my_mat.dot(&my_mat2);
    let elapsed = now.elapsed();

    println!("Printing matrix");
    my_mat3.print_all_val();

    println!("Elapsed: {:.2?}", elapsed);

    // No optimizations takes: 1.42s
    // Reordering loops takes: 93.28ms
    // Column tiling takes: 80.87ms

}
