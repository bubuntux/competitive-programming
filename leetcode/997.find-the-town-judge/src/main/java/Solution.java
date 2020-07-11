class Solution {

    public int findJudge(int N, int[][] trust) {
        var voted = new boolean[N];
        var votes = new int[N];

        for (var t : trust) {
            int a = t[0] - 1;
            int b = t[1] - 1;
            voted[a] = true;
            votes[b]++;
        }

        var target = N - 1;
        for (int i = 0; i < N; i++) {
            if (!voted[i] && votes[i] == target) {
                return i + 1;
            }
        }

        return -1;
    }

}