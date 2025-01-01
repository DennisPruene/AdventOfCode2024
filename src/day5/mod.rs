use crate::DynResult;
use crate::day4::matrix::{Matrix, MutMatrix, bool::BoolMatrix};
use crate::parsing::parse_integers;
use std::path::Path;

pub fn solve_day5<P: AsRef<Path>>(path: P) -> DynResult<()> {
    println!(
        "Solving Day 5 on the input {}",
        path.as_ref().to_str().unwrap()
    );
    let input = parse_integers(path)?;
    let node_count = (*input.iter().flatten().max().unwrap() + 1) as usize;
    let mut split_input = input.split(Vec::is_empty);
    let page_ordering_rules: Vec<(usize, usize)> = split_input
        .next()
        .unwrap()
        .iter()
        .cloned()
        .map(|l| (l[0] as usize, l[1] as usize))
        .collect();
    let updates: Vec<Vec<usize>> = split_input
        .next()
        .unwrap()
        .iter()
        .cloned()
        .map(|l| l.into_iter().map(|i| i as usize).collect())
        .collect();
    let mut page_ordering_graph = BoolMatrix::new(node_count, node_count);
    for (source, dest) in page_ordering_rules.iter() {
        page_ordering_graph.set(*dest, *source, true);
    }
    let first_result = updates
        .iter()
        .filter(|u| is_update_correctly_ordered(*u, &page_ordering_graph))
        .fold(0usize, |acc, u| acc + u[u.len() / 2]);
    println!("The solution to part 1 is: {first_result}");

    let mut updates = updates;
    let second_result = updates
        .iter_mut()
        .filter(|u| !is_update_correctly_ordered(*u, &page_ordering_graph))
        .fold(0usize, |acc, u| {
            sort_update(u, &page_ordering_graph);
            acc + u[u.len() / 2]
        });
    println!("The solution to part 2 is: {second_result}");
    Ok(())
}

fn is_update_correctly_ordered(update: &[usize], page_ordering_graph: &BoolMatrix) -> bool {
    for (i, current_page) in update.iter().enumerate() {
        for following_page in update[i + 1..].iter() {
            if page_ordering_graph
                .get(*current_page, *following_page)
                .unwrap()
            {
                return false;
            }
        }
    }
    true
}

fn sort_update(update: &mut Vec<usize>, page_ordering_graph: &BoolMatrix) {
    let n = update.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if page_ordering_graph.get(update[i], update[j]).unwrap() {
                update.swap(i, j);
                sort_update(update, page_ordering_graph);
            }
        }
    }
}

fn is_anti_reflexive(matrix: &BoolMatrix) -> bool {
    let n = matrix.row_count();
    for i in 0..n {
        if matrix.get(i, i).unwrap() {
            return false;
        }
    }
    true
}

fn is_transitive(matrix: &BoolMatrix) -> bool {
    let n = matrix.row_count();
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if matrix.get(j, i).unwrap()
                    && matrix.get(k, j).unwrap()
                    && !matrix.get(k, i).unwrap()
                {
                    return false;
                }
            }
        }
    }
    true
}

fn is_complete(matrix: &BoolMatrix) -> bool {
    let n = matrix.row_count();
    for i in 0..n {
        for j in 0..n {
            if !matrix.get(j, i).unwrap() && !matrix.get(i, j).unwrap() {
                return false;
            }
        }
    }
    true
}

fn is_linear_ordering(matrix: &BoolMatrix) -> bool {
    is_anti_reflexive(matrix) && is_transitive(matrix) && is_complete(matrix)
}
