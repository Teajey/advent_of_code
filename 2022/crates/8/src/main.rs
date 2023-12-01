mod matrix;

use std::ops::Deref;

use common::{e, get_input, Failure, Result};

use matrix::Matrix;

fn trees_visible_across<T: PartialOrd>(trees: &[T]) -> Vec<u32> {
    let mut mask = vec![0; trees.len()];

    for i in 0..trees.len() {
        for j in i + 1..trees.len() {
            mask[i] += 1;
            if trees[i] <= trees[j] {
                break;
            }
        }
    }

    mask
}

fn scenic_score_map(mut forest: Matrix<u8>) -> Matrix<u32> {
    let mut scenic_score_map: Matrix<u32> = forest.clone().into();

    for row in scenic_score_map.iter_mut() {
        let across_score = trees_visible_across(row.deref());
        *row = across_score.into_boxed_slice();
    }

    for _ in 0..3 {
        forest = forest.rotate();
        scenic_score_map = scenic_score_map.rotate();
        for (row, vis_row) in forest.iter().zip(scenic_score_map.iter_mut()) {
            let mut row_mask = trees_visible_across(row.deref());
            for (tree, mask_tree) in vis_row.iter_mut().zip(&mut row_mask) {
                *tree *= *mask_tree;
            }
        }
    }

    scenic_score_map
}

fn main() -> Result<()> {
    let forest: Matrix<u8> = get_input()?.try_into()?;

    let ssm = scenic_score_map(forest);

    let max_scenic_score = ssm
        .iter()
        .map(|row| row.iter().max())
        .max()
        .flatten()
        .ok_or_else(|| e!("Scenic Score Map was empty"))?;

    println!("{max_scenic_score}");

    Ok(())
}

#[cfg(test)]
mod test {
    use std::ops::Deref;

    use super::{Matrix, Result};

    fn test_matrix() -> Result<Matrix<u8>> {
        let data = r#"30373
25512
65332
33549
35390"#
            .to_owned();

        Matrix::try_from(data)
    }

    impl<T: Clone + Copy> Matrix<T> {
        pub fn slice_rows(&self) -> Box<[&[T]]> {
            self.iter().map(|row| row.deref()).collect::<Box<[_]>>()
        }
    }

    #[test]
    fn trees_visible_across() {
        let mask = super::trees_visible_across(&[3, 0, 3, 7, 3]);

        assert_eq!(mask, &[2, 1, 1, 1, 0]);
    }

    #[test]
    fn scenic_score_map() -> Result<()> {
        let forest = test_matrix()?;

        let scenic_score_map = super::scenic_score_map(forest);

        let expected = [
            [0, 0, 0, 0, 0],
            [0, 1, 4, 1, 0],
            [0, 6, 1, 2, 0],
            [0, 1, 8, 3, 0],
            [0, 0, 0, 0, 0],
        ];

        let scenic_score_map = scenic_score_map.rotate();

        assert_eq!(scenic_score_map.slice_rows().deref(), expected);

        Ok(())
    }
}
