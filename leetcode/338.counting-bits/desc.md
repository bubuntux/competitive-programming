Given an integer n, return an array ans of length n + 1 such that for each i (0 <= i <= n), ans[i] is the number of 1's in the binary representation of i.

Example 1:

Input: n = 2
Output: [0,1,1]
Explanation:
0 --> 0
1 --> 1
2 --> 10

Example 2:

Input: n = 5
Output: [0,1,1,2,1,2]
Explanation:
0 --> 0 0
1 --> 1 1
2 --> 10 1
3 --> 11 2
4 --> 100 1
5 --> 101 2

6 --> 110 , 2
7--> 111 , 3
8 --> 1000, 1
9-> 1001, 2
10-> 1010, 2
11 1011, 3
12 1100, 2
13 1101, 3
14 1110 3
15 1111 4

Constraints:

    0 <= n <= 105

Follow up:

    It is very easy to come up with a solution with a runtime of O(n log n). Can you do it in linear time O(n) and possibly in a single pass?
    Can you do it without using any built-in function (i.e., like __builtin_popcount in C++)?
