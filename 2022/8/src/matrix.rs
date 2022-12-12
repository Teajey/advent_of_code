use std::{
    f32::consts::FRAC_PI_2,
    fmt::Debug,
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
    vec,
};

use common::{e, Failure, Result};

#[derive(Clone)]
pub struct Matrix(Box<[Box<[u8]>]>, usize);

impl Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in self.iter() {
            for cell in row.iter() {
                write!(f, "{:>3} ", cell)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl TryFrom<String> for Matrix {
    type Error = Failure;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        let matrix = string
            .split('\n')
            .map(|ln| {
                ln.chars()
                    .map(|c| {
                        c.to_digit(10)
                            .map(|n| n as u8)
                            .ok_or_else(|| e!("Non-digit in input!"))
                    })
                    .collect::<Result<Vec<_>>>()
                    .map(|v| v.into_boxed_slice())
            })
            .collect::<Result<Vec<_>>>()?
            .into_boxed_slice();

        let length = matrix.len();

        if matrix.iter().any(|row| row.len() != length) {
            return Err(e!("Matrix is not square: {matrix:?}"));
        }

        Ok(Self(matrix, length))
    }
}

impl Index<usize> for Matrix {
    type Output = Box<[u8]>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

fn quarter_turn_matrix_index((x, y): (usize, usize), length: f32) -> (usize, usize) {
    let x = x as f32;
    let y = y as f32;

    let dx = x * FRAC_PI_2.cos() - y * FRAC_PI_2.sin();
    let dy = x * FRAC_PI_2.sin() + y * FRAC_PI_2.cos();

    let x = dx.round();
    let y = dy.round();

    let x = x + length;

    let x = x as usize;
    let y = y as usize;

    (x, y)
}

impl Matrix {
    pub fn debug_try_new(vec2d: Vec<Vec<u8>>) -> Result<Self> {
        let boxed_slice = vec2d
            .clone()
            .into_iter()
            .map(|row| row.into_boxed_slice())
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let length = boxed_slice.len();

        if boxed_slice.iter().any(|row| row.len() != length) {
            return Err(e!("vec2d is not square: {vec2d:?}"));
        }

        Ok(Self(boxed_slice, length))
    }

    pub fn rotate(self) -> Self {
        let mut new = self.clone();

        for (x, i) in self.iter().enumerate() {
            for (y, j) in i.iter().enumerate() {
                let (x, y) = quarter_turn_matrix_index((x, y), (self.1 - 1) as f32);
                new[x][y] = *j;
            }
        }

        new
    }

    pub fn iter(&self) -> Iter<'_, Box<[u8]>> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Box<[u8]>> {
        self.0.iter_mut()
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;

    use super::{Matrix, Result};

    fn test_matrix() -> Result<Matrix> {
        let data = r#"30373
25512
65332
33549
35390"#
            .to_owned();

        Matrix::try_from(data)
    }

    #[test]
    fn string_parse() -> Result<()> {
        let mat = test_matrix()?;

        let expected = [
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ];

        assert_eq!(mat.slice_rows().deref(), expected);

        Ok(())
    }

    #[test]
    fn quarter_turn_matrix_index() {
        let x_y = super::quarter_turn_matrix_index((4, 3), 4.);
        assert_eq!(x_y, (1, 4));

        let x_y = super::quarter_turn_matrix_index((0, 0), 4.);
        assert_eq!(x_y, (4, 0));
        let x_y = super::quarter_turn_matrix_index(x_y, 4.);
        assert_eq!(x_y, (4, 4));
        let x_y = super::quarter_turn_matrix_index(x_y, 4.);
        assert_eq!(x_y, (0, 4));
        let x_y = super::quarter_turn_matrix_index(x_y, 4.);
        assert_eq!(x_y, (0, 0));
    }

    #[test]
    fn rotate() -> Result<()> {
        let mat = test_matrix()?;

        let mat = mat.rotate();

        let expected = [
            [3, 2, 2, 9, 0],
            [7, 1, 3, 4, 9],
            [3, 5, 3, 5, 3],
            [0, 5, 5, 3, 5],
            [3, 2, 6, 3, 3],
        ];

        assert_eq!(mat.slice_rows().deref(), expected);

        Ok(())
    }
}
