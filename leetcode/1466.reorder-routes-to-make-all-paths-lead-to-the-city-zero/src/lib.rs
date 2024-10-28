impl Solution {
    pub fn min_reorder(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let edges = {
            let mut ans = vec![Vec::new(); edges.len() + 1];
            for edge in edges {
                ans[edge[0] as usize].push((edge[1] as usize, true));
                ans[edge[1] as usize].push((edge[0] as usize, false));
            }
            ans
        };

        Self::dfs(0, n as usize, &edges)
    }

    fn dfs(node: usize, parent: usize, edges: &Vec<Vec<(usize, bool)>>) -> i32 {
        let mut ans = 0;

        for &(neighbor, dir) in edges[node].iter() {
            if neighbor != parent {
                if dir {
                    ans += 1
                }
                ans += Self::dfs(neighbor, node, edges);
            }
        }

        ans
    }
}

// https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::min_reorder(
                6,
                vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]]
            ),
            3
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::min_reorder(5, vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]]),
            2
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::min_reorder(3, vec![vec![1, 0], vec![2, 0]]), 0);
    }

    #[test]
    fn example_65() {
        assert_eq!(
            Solution::min_reorder(
                6,
                vec![vec![4, 5], vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0]]
            ),
            3
        );
    }
}
