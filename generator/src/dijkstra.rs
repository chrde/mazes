use crate::common::*;

const MAX_DISTANCE: usize = 9999;

pub fn shortest_path(from: usize, to: usize, maze: &Maze, distances: &[usize]) -> Vec<usize> {
    let mut breadcrumbs = vec![to];
    let mut current = to;
    while let Some(next) = maze.linked_neighbors(current).iter().min_by_key(|n| distances[n.idx]) {
        current = next.idx;
        breadcrumbs.push(next.idx);
        if next.idx == from {
            break;
        }
    }

    breadcrumbs
}

pub fn longest_path(maze: &Maze) -> Vec<usize> {
    let distances = flood(0, maze);
    let (from, _) = distances.iter().enumerate().max_by_key(|(_, dis)| *dis).unwrap();
    let new_distances = flood(from, maze);
    let (to, _) = new_distances.iter().enumerate().max_by_key(|(_, dis)| *dis).unwrap();
    shortest_path(from, to, maze, &new_distances)
}

pub fn flood(from: usize, maze: &Maze) -> Vec<usize> {
    assert!(maze.cells().len() < 9999);
    let mut distances = vec![MAX_DISTANCE; maze.cells().len()];
    let mut pending = vec![(from, 1)];
    distances[from] = 0;
    while let Some((current, distance)) = pending.pop() {
        for neighbor in maze.linked_neighbors(current).iter() {
            if distances[neighbor.idx] == MAX_DISTANCE || distances[neighbor.idx] > distance {
                distances[neighbor.idx] = distance;
                pending.push((neighbor.idx, distance + 1));
            }
        }
    }
    distances
}
