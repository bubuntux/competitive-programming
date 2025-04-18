use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let graph = build_graph(equations, values);
        queries
            .into_iter()
            .map(|query| dfs(&graph, query, &mut HashSet::new(), None).map_or(-1.0, |v| v))
            .collect()
    }
}

type Graph = HashMap<String, HashMap<String, f64>>;

fn build_graph(equations: Vec<Vec<String>>, values: Vec<f64>) -> Graph {
    equations
        .into_iter()
        .zip(values)
        .fold(HashMap::new(), |mut acc, (eq, val)| {
            let entry = acc.entry(eq[0].clone()).or_default();
            entry.insert(eq[1].clone(), val);

            let entry = acc.entry(eq[1].clone()).or_default();
            entry.insert(eq[0].clone(), 1.0 / val);

            acc
        })
}

fn dfs(
    graph: &Graph,
    query: Vec<String>,
    visited: &mut HashSet<String>,
    current_sum: Option<f64>,
) -> Option<f64> {
    visited.insert(query[0].clone());

    // Check the the node exists in the graph
    let entry = graph.get(&query[0])?;

    // a / a = 1.0
    if query[0] == query[1] {
        return Some(1.0);
    }

    // If our current node has a direct route to our target, return it
    if let Some(answer) = entry.get(&query[1]) {
        return current_sum.map_or(Some(*answer), |s| Some(s * *answer));
    };

    // Otherwise perform a DFS of connected nodes (that we haven't already visited)
    for path in entry.keys() {
        if !visited.contains(path) {
            if let Some(val) = entry.get(path) {
                let result = dfs(
                    graph,
                    vec![path.clone(), query[1].clone()],
                    visited,
                    current_sum.map_or(Some(*val), |s| Some(s * *val)),
                );
                if result.is_some() {
                    return result;
                }
            }
        }
    }
    None
}

// https://leetcode.com/problems/evaluate-division

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::calc_equation(
                vec![
                    vec!["a".to_string(), "b".to_string()],
                    vec!["b".to_string(), "c".to_string()]
                ],
                vec![2.0, 3.0],
                vec![
                    vec!["a".to_string(), "c".to_string()],
                    vec!["b".to_string(), "a".to_string()],
                    vec!["a".to_string(), "e".to_string()],
                    vec!["a".to_string(), "a".to_string()],
                    vec!["x".to_string(), "x".to_string()]
                ]
            ),
            vec![6.0, 0.5, -1.0, 1.0, -1.0]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::calc_equation(
                vec![
                    vec!["a".to_string(), "b".to_string()],
                    vec!["b".to_string(), "c".to_string()],
                    vec!["bc".to_string(), "cd".to_string()]
                ],
                vec![1.5, 2.5, 5.0],
                vec![
                    vec!["a".to_string(), "c".to_string()],
                    vec!["c".to_string(), "b".to_string()],
                    vec!["bc".to_string(), "cd".to_string()],
                    vec!["cd".to_string(), "bc".to_string()],
                ]
            ),
            vec![3.75, 0.4, 5.0, 0.2]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::calc_equation(
                vec![vec!["a".to_string(), "b".to_string()]],
                vec![0.5],
                vec![
                    vec!["a".to_string(), "b".to_string()],
                    vec!["b".to_string(), "a".to_string()],
                    vec!["a".to_string(), "c".to_string()],
                    vec!["x".to_string(), "y".to_string()]
                ]
            ),
            vec![0.5, 2.0, -1.0, -1.0]
        );
    }
}
