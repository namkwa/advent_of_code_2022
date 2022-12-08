use std::fs;
fn main() {
    let input = fs::read_to_string("day_8/src/input.txt").unwrap();

    let input: Vec<&str> = input.lines().collect();
    star1(input);
}

fn star1(input: Vec<&str>) {
    let mut visible_trees = 0;
    let mut max_scenic_view = 0;
    let forest: Vec<Vec<u32>> = input
        .iter()
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap()).collect())
        .collect();
    for (index_row, line) in forest.iter().enumerate() {
        for (index_col, tree) in line.iter().enumerate() {
            let result = is_visible(*tree, index_row, index_col, &forest);
            if result.0 {
                visible_trees += 1;
            }
            if max_scenic_view < result.1 {
                max_scenic_view = result.1;
            }
        }
    }
    println!("{} {}", visible_trees, max_scenic_view);
}

fn is_visible(
    tree: u32,
    index_row: usize,
    index_col: usize,
    forest: &Vec<Vec<u32>>,
) -> (bool, i32) {
    let top = contains_taller_tree_in_col(tree, (0..index_row).rev().collect(), index_col, forest);
    let bot = contains_taller_tree_in_col(
        tree,
        ((index_row + 1)..forest.len()).collect(),
        index_col,
        forest,
    );
    let left = contains_taller_tree_in_row(tree, (0..index_col).rev().collect(), index_row, forest);
    let right = contains_taller_tree_in_row(
        tree,
        ((index_col + 1)..forest.len()).collect(),
        index_row,
        forest,
    );
    return (
        top.0 || bot.0 || left.0 || right.0,
        top.1 * bot.1 * left.1 * right.1,
    );
}

fn contains_taller_tree_in_row(
    tree: u32,
    range: Vec<usize>,
    index_row: usize,
    forest: &Vec<Vec<u32>>,
) -> (bool, i32) {
    let mut scenic_view = 0;
    for i in &range {
        if *forest.get(index_row).unwrap().get(*i).unwrap() >= tree {
            return (false, scenic_view + 1);
        }
        if range.len() != 1 {
            scenic_view += 1;
        }
    }
    return (true, scenic_view);
}

fn contains_taller_tree_in_col(
    tree: u32,
    range: Vec<usize>,
    index_col: usize,
    forest: &Vec<Vec<u32>>,
) -> (bool, i32) {
    let mut scenic_view = 0;
    for i in &range {
        if *forest.get(*i).unwrap().get(index_col).unwrap() >= tree {
            return (false, scenic_view + 1);
        }
        if range.len() != 1 {
            scenic_view += 1;
        }
    }
    return (true, scenic_view);
}
