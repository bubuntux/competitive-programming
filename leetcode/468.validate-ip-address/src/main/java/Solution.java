public class Solution {

    public String validIPAddress(String ip) {
        if (ip == null || ip.isEmpty()) return "Neither";
        var length = ip.length();
        if (length >= 7 && length <= 15 && isValidIpv4(ip)) return "IPv4";
        if (length >= 15 && length <= 39 && isValidIpv6(ip)) return "IPv6";
        return "Neither";
    }

    private boolean isValidIpv4(String ip) {
        var value = 0;
        var hasValue = false;
        var dotCounter = 0;

        for (var c : ip.toCharArray()) {
            if (c == '.') {
                if (!hasValue || ++dotCounter > 3) return false;
                value = 0;
                hasValue = false;
                continue;
            }
            if (c < '0' || c > '9') return false;
            if (hasValue && value == 0) return false;
            value = (value * 10) + Character.getNumericValue(c);
            if (value > 255) return false;
            hasValue = true;
        }
        return hasValue && dotCounter == 3;
    }

    private boolean isValidIpv6(String ip) {
        var value = 0;
        var hasValue = false;
        var prevIndex = 0;
        var colonCounter = 0;
        var chars = ip.toCharArray();

        for (int i = 0; i < chars.length; i++) {
            var c = chars[i];
            if (c == ':') {
                if (!hasValue || (i - prevIndex) > 4 || ++colonCounter > 7) return false;
                value = 0;
                prevIndex = i + 1;
                hasValue = false;
                continue;
            }
            if ((c < '0' || c > '9') && (c < 'a' || c > 'f') && (c < 'A' || c > 'F')) return false;
            value = (value * 10) + Integer.parseInt(String.valueOf(c), 16);
            if (value > 0xFFFF) return false;
            hasValue = true;
        }
        return hasValue && (chars.length - prevIndex) <= 4 && colonCounter == 7;
    }
}
