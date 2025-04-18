#[allow(dead_code)]
impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let start = (entrance[0], entrance[1]);
        let mut visited = vec![vec![false; maze[0].len()]; maze.len()];
        bfs(maze, &mut visited, vec![start], 1).map_or(-1, |steps| steps as i32)
    }
}

type Point = (i32, i32);

fn bfs(
    maze: Vec<Vec<char>>,
    visited: &mut [Vec<bool>],
    paths: Vec<Point>,
    steps: u32,
) -> Option<u32> {
    let mut next_paths = vec![];

    for path in paths {
        visited[path.0 as usize][path.1 as usize] = true;

        // up
        if (path.0 - 1) >= 0 {
            let y = (path.0 - 1) as usize;
            let x = path.1 as usize;
            if !visited[y][x] && maze[y][x] == '.' {
                if y == 0 || x == 0 || x == (maze[y].len() - 1) {
                    return Some(steps);
                }
                next_paths.push((y as i32, x as i32));
            }
        }
        // down
        if (path.0 + 1) < maze.len() as i32 {
            let y = (path.0 + 1) as usize;
            let x = path.1 as usize;
            if !visited[y][x] && maze[y][x] == '.' {
                if y == (maze.len() - 1) || x == 0 || x == (maze[y].len() - 1) {
                    return Some(steps);
                }
                next_paths.push((y as i32, x as i32));
            }
        }
        // left
        if (path.1 - 1) >= 0 {
            let y = path.0 as usize;
            let x = (path.1 - 1) as usize;
            if !visited[y][x] && maze[y][x] == '.' {
                if x == 0 || y == 0 || y == (maze.len() - 1) {
                    return Some(steps);
                }
                next_paths.push((y as i32, x as i32));
            }
        }
        // right
        if (path.1 + 1) < maze[0].len() as i32 {
            let y = path.0 as usize;
            let x = (path.1 + 1) as usize;
            if !visited[y][x] && maze[y][x] == '.' {
                if x == (maze[y].len() - 1) || y == 0 || y == (maze.len() - 1) {
                    return Some(steps);
                }
                next_paths.push((y as i32, x as i32));
            }
        }
    }
    if next_paths.is_empty() {
        return None;
    }

    bfs(maze, visited, next_paths, steps + 1)
}
// https://leetcode.com/problems/nearest-exit-from-entrance-in-maze

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::nearest_exit(
                vec![
                    vec!['+', '+', '.', '+'],
                    vec!['.', '.', '.', '+'],
                    vec!['+', '+', '+', '.']
                ],
                vec![1, 2]
            ),
            1
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::nearest_exit(
                vec![
                    vec!['+', '+', '+'],
                    vec!['.', '.', '.'],
                    vec!['+', '+', '+'],
                ],
                vec![1, 0]
            ),
            2
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::nearest_exit(vec![vec!['.', '+']], vec![0, 0]), -1);
    }

    #[test]
    fn example_4() {
        assert_eq!(
            Solution::nearest_exit(vec![vec!['+'], vec!['.']], vec![1, 0]),
            -1
        );
    }
}
