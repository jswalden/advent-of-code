fn parse_trees() -> Vec<Vec<i8>> {
    let mut trees = vec![];

    let mut row_width = None;

    include_str!("../input").lines().for_each(|line| {
        match row_width {
            None => {
                row_width = Some(line.len());
            }
            Some(row_width) => {
                assert!(row_width == line.len(), "all rows must be same length");
            }
        }

        let mut tree_row = vec![];
        for c in line.chars() {
            tree_row.push(c.to_digit(10).expect("digit") as i8 + 1);
        }
        trees.push(tree_row);
    });

    trees
}

fn count_visible_trees(trees: &mut Vec<Vec<i8>>, row_count: usize, col_count: usize) {
    // Mark any visible tree along a row or column, in positive or negative
    // direction, as negative.
    let row_iter = 0..row_count;
    let rev_row_iter = row_iter.clone().rev();
    let col_iter = 0..col_count;
    let rev_col_iter = col_iter.clone().rev();

    for i in row_iter.clone() {
        let mut current_tree_height = 0;
        for j in col_iter.clone() {
            let tree = &mut trees[i][j];
            if tree.abs() > current_tree_height {
                current_tree_height = tree.abs();
                *tree = -tree.abs();
            }
        }

        let mut current_tree_height = -1;
        for j in rev_col_iter.clone() {
            let tree = &mut trees[i][j];
            if tree.abs() > current_tree_height {
                current_tree_height = tree.abs();
                *tree = -tree.abs();
            }
        }
    }

    for j in col_iter.clone() {
        let mut current_tree_height = -1;
        for i in row_iter.clone() {
            let tree = &mut trees[i][j];
            if tree.abs() > current_tree_height {
                current_tree_height = tree.abs();
                *tree = -tree.abs();
            }
        }

        let mut current_tree_height = -1;
        for i in rev_row_iter.clone() {
            let tree = &mut trees[i][j];
            if tree.abs() > current_tree_height {
                current_tree_height = tree.abs();
                *tree = -tree.abs();
            }
        }
    }

    let visible_tree_count = trees
        .iter()
        .map(|row| row.iter().fold(0, |acc, v| acc + (*v < 0) as usize))
        .fold(0, |acc, v| acc + v);

    for row in trees {
        for tree in row {
            *tree = tree.abs();
        }
    }

    println!("Visible tree count: {}", visible_tree_count);
}

#[derive(Copy, Clone)]
struct Partial {
    lr: usize,
    rl: usize,
    tb: usize,
    bt: usize,
}

impl Partial {
    fn zero() -> Partial {
        Partial {
            lr: 0,
            rl: 0,
            tb: 0,
            bt: 0,
        }
    }
}

fn find_best_scenic_score(trees: &Vec<Vec<i8>>, row_count: usize, col_count: usize) {
    let mut partials = vec![vec![Partial::zero(); col_count]; row_count];

    for i in 0..row_count {
        let elem = |j| trees[i][j];

        let mut stack = vec![];
        for j in 0..col_count {
            let tree = elem(j);

            loop {
                match stack.last() {
                    None => {
                        stack.push(j);
                        partials[i][j].lr = j;
                        break;
                    }
                    Some(least) => {
                        let least = *least;
                        let low_tree = elem(least);
                        if low_tree >= tree {
                            if low_tree == tree {
                                stack.pop();
                            }
                            stack.push(j);
                            partials[i][j].lr = j - least;
                            break;
                        } else {
                            stack.pop();
                        }
                    }
                }
            }
        }

        let mut stack = vec![];
        for j in (0..col_count).rev() {
            let tree = elem(j);

            loop {
                match stack.last() {
                    None => {
                        stack.push(j);
                        partials[i][j].rl = col_count - 1 - j;
                        break;
                    }
                    Some(least) => {
                        let least = *least;
                        let low_tree = elem(least);
                        if low_tree >= tree {
                            if low_tree == tree {
                                stack.pop();
                            }
                            stack.push(j);
                            partials[i][j].rl = least - j;
                            break;
                        } else {
                            stack.pop();
                        }
                    }
                }
            }
        }
    }

    for j in 0..col_count {
        let elem = |i: usize| trees[i][j];

        let mut stack = vec![];
        for i in 0..row_count {
            let tree = elem(i);

            loop {
                match stack.last() {
                    None => {
                        stack.push(i);
                        partials[i][j].tb = i;
                        break;
                    }
                    Some(least) => {
                        let least = *least;
                        let low_tree = elem(least);
                        if low_tree >= tree {
                            if low_tree == tree {
                                stack.pop();
                            }
                            stack.push(i);
                            partials[i][j].tb = i - least;
                            break;
                        } else {
                            stack.pop();
                        }
                    }
                }
            }
        }

        let mut stack = vec![];
        for i in (0..row_count).rev() {
            let tree = elem(i);

            loop {
                match stack.last() {
                    None => {
                        stack.push(i);
                        partials[i][j].bt = row_count - 1 - i;
                        break;
                    }
                    Some(least) => {
                        let least = *least;
                        let low_tree = elem(least);
                        if low_tree >= tree {
                            if low_tree == tree {
                                stack.pop();
                            }
                            stack.push(i);
                            partials[i][j].bt = least - i;
                            break;
                        } else {
                            stack.pop();
                        }
                    }
                }
            }
        }
    }

    let partials = &partials;
    let scenic_score = |p: &Partial| p.lr * p.rl * p.bt * p.tb;

    let ((i, j), best_score) = itertools::Itertools::cartesian_product(0..row_count, 0..col_count)
        .map(|(i, j)| ((i, j), scenic_score(&partials[i][j])))
        .fold(
            ((usize::MAX, usize::MAX), 0),
            |best_score, (coords, score)| {
                if score >= best_score.1 {
                    (coords, score)
                } else {
                    best_score
                }
            },
        );
    println!(
        "Best possible scenic score: {} at ({}, {})",
        best_score, i, j
    );
}

fn main() {
    let mut trees = parse_trees();

    let row_count = trees.len();
    let col_count = trees[0].len();

    count_visible_trees(&mut trees, row_count, col_count);

    find_best_scenic_score(&trees, row_count, col_count);
}
