/*
* https://leetcode.com/problems/can-place-flowers/
*
You have a long flowerbed in which some of the plots are planted, and some are not. However, flowers cannot be planted in adjacent plots.

Given an integer array flowerbed containing 0's and 1's, where 0 means empty and 1 means not empty, and an integer n, return true if n new flowers can be planted in the flowerbed without violating the no-adjacent-flowers rule and false otherwise.

Example 1:

    Input: flowerbed = [1,0,0,0,1], n = 1
    Output: true

Example 2:

    Input: flowerbed = [1,0,0,0,1], n = 2
    Output: false

Constraints:

    1 <= flowerbed.length <= 2 * 104
    flowerbed[i] is 0 or 1.
    There are no two adjacent flowers in flowerbed.
    0 <= n <= flowerbed.length

*/

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut remain = n;
        let mut i = 0;

        while i < flowerbed.len() && remain > 0 {
            if flowerbed[i] == 0
                && (i == 0 || flowerbed[i - 1] == 0)
                && (i == flowerbed.len() - 1 || flowerbed[i + 1] == 0)
            {
                i = i + 1;
                remain = remain - 1;
            }
            i = i + 1;
        }

        return remain == 0;
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
    }

    #[test]
    fn example_2() {
        assert!(!Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2));
    }

    #[test]
    fn example_3() {
        assert!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1, 0, 0], 2));
    }

    #[test]
    fn example_4() {
        assert!(Solution::can_place_flowers(vec![0, 0, 1, 0, 0, 0, 1], 2));
    }
}
