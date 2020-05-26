import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;


class SolutionTest {

    @Test
    void test() {
        Solution solution = new Solution();
        assertEquals("IPv4", solution.validIPAddress("172.16.254.255"));
        assertEquals("IPv4", solution.validIPAddress("172.10.250.255"));
        assertEquals("Neither", solution.validIPAddress("12.12.12"));
        assertEquals("Neither", solution.validIPAddress("12.12.12.12.12"));

        assertEquals("IPv4", solution.validIPAddress("172.16.254.1"));
        assertEquals("Neither", solution.validIPAddress("172.16.254.01"));
        assertEquals("Neither", solution.validIPAddress("2001:0db8:85a3:0:0:8A2E:0370:7334:"));

        assertEquals("IPv6", solution.validIPAddress("2001:0db8:85a3:0000:0:8A2E:0370:733a"));


        assertEquals("Neither", solution.validIPAddress("256.256.256.256"));

        assertEquals("Neither", solution.validIPAddress("2001:0db8:85a3:00000:0:8A2E:0370:7334"));

        assertEquals("Neither", solution.validIPAddress("02001:0db8:85a3:0000:0000:8a2e:0370:7334"));
        assertEquals("IPv6", solution.validIPAddress("2001:0db8:85a3:0:0:8A2E:0370:7334"));
    }
}