mod matrix;

use std::ops::Deref;

use common::{get_input, Result};

use matrix::Matrix;

fn visible_trees_mask(trees: &[u8]) -> Vec<u8> {
    let mut mask = vec![1; trees.len()];

    for i in 0..trees.len() {
        for j in i + 1..trees.len() {
            if trees[i] <= trees[j] {
                mask[i] = 0;
                break;
            }
        }
    }

    mask
}

fn visible_tree_map(mut forest: Matrix) -> Matrix {
    let mut visible_trees = forest.clone();

    for row in visible_trees.iter_mut() {
        let row_mask = visible_trees_mask(row.deref());
        *row = row_mask.into_boxed_slice();
    }

    for _ in 0..3 {
        forest = forest.rotate();
        visible_trees = visible_trees.rotate();
        for (row, vis_row) in forest.iter().zip(visible_trees.iter_mut()) {
            let mut row_mask = visible_trees_mask(row.deref());
            for (tree, mask_tree) in vis_row.iter_mut().zip(&mut row_mask) {
                *tree |= *mask_tree;
            }
        }
    }

    visible_trees
}

fn main() -> Result<()> {
    let forest: Matrix = get_input()?.try_into()?;

    let visible_trees = visible_tree_map(forest);

    let total_visible_trees = visible_trees
        .iter()
        .map(|row| row.iter().map(|n| *n as u32).sum::<u32>())
        .sum::<u32>();

    println!("{}", total_visible_trees);

    Ok(())
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

    impl Matrix {
        pub fn slice_rows(&self) -> Box<[&[u8]]> {
            self.iter().map(|row| row.deref()).collect::<Box<[_]>>()
        }
    }

    #[test]
    fn visible_trees_mask() {
        let mask = super::visible_trees_mask(&[3, 0, 3, 7, 3]);

        assert_eq!(mask, &[0, 0, 0, 1, 1]);
    }

    #[test]
    fn visible_tree_map() -> Result<()> {
        let forest = test_matrix()?;

        let visible_trees = super::visible_tree_map(forest);

        let expected = [
            [1, 1, 1, 1, 1],
            [1, 1, 1, 0, 1],
            [1, 1, 0, 1, 1],
            [1, 0, 1, 0, 1],
            [1, 1, 1, 1, 1],
        ];

        assert_eq!(visible_trees.slice_rows().deref(), expected);

        Ok(())
    }
}
