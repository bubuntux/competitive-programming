use std::collections::HashSet;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited: HashSet<usize> = HashSet::new();
        Self::dfs(&rooms, 0, &mut visited);
        visited.len() == rooms.len()
    }

    fn dfs(rooms: &Vec<Vec<i32>>, room: usize, visited: &mut HashSet<usize>) {
        if !visited.contains(&room) {
            visited.insert(room);
            for key in &rooms[room] {
                Self::dfs(rooms, *key as usize, visited)
            }
        }
    }
}

// https://leetcode.com/problems/keys-and-rooms/

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example_1() {
        assert!(Solution::can_visit_all_rooms(vec![
            vec![1],
            vec![2],
            vec![3],
            vec![]
        ]));
    }

    #[test]
    fn example_2() {
        assert!(!Solution::can_visit_all_rooms(vec![
            vec![1, 3],
            vec![3, 0, 1],
            vec![2],
            vec![0]
        ]));
    }

    #[test]
    fn example_56() {
        assert!(Solution::can_visit_all_rooms(vec![
            vec![2],
            vec![],
            vec![1]
        ]));
    }

    #[test]
    fn example_50() {
        assert!(!Solution::can_visit_all_rooms(vec![
            vec![4],       // 0
            vec![3],       // 1
            vec![],        // 2
            vec![2, 5, 7], // 3
            vec![1],       // 4
            vec![],        // 5
            vec![8, 9],    // 6
            vec![],        // 7
            vec![],        // 8
            vec![6]        // 9
        ]));
    }
}
