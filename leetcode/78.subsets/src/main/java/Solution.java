import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.stream.Collectors;

class Solution {
    public List<List<Integer>> subsets(int... nums) {
        var out = new ArrayList<List<Integer>>();
        out.add(Collections.emptyList());
        Arrays.stream(nums).mapToObj(n -> out.stream().map(ArrayList::new).peek(sub -> sub.add(n)).collect(Collectors.toList())).forEach(out::addAll);
        return out;
    }
}