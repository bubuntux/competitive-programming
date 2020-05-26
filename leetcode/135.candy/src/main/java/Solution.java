public class Solution {
    public int candy(int[] ratings) {
        int[] l2r = new int[ratings.length];
        for (int i = 1; i < ratings.length; i++) {
            if (ratings[i] > ratings[i - 1]) {
                l2r[i] = l2r[i - 1] + 1;
            }
        }

        int[] r2l = new int[ratings.length];
        for (int i = ratings.length - 2; i >= 0; i--) {
            if (ratings[i] > ratings[i + 1]) {
                r2l[i] = r2l[i + 1] + 1;
            }
        }

        int candies = ratings.length;
        for (int i = 0; i < ratings.length; i++) {
            candies += Math.max(l2r[i], r2l[i]);
        }
        return candies;
    }
}