import java.util.LinkedHashMap;

class LRUCache {

    private final LinkedHashMap<Integer, Integer> map;
    private final int cap;

    public LRUCache(int capacity) {
        map = new LinkedHashMap<>(capacity);
        cap = capacity;
    }

    public int get(int key) {
        Integer value = map.remove(key);
        if (value == null) {
            return -1;
        }
        map.put(key, value);
        return value;
    }

    public void put(int key, int value) {
        map.remove(key);
        map.put(key, value);
        if (map.size() > cap) {
            Integer toRemove = map.keySet().iterator().next();
            map.remove(toRemove);
        }
    }

}