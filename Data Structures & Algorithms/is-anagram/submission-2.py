class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        if len(s) != len(t):
            return False

        
        chars_values_s, chars_values_t = {},{}

        for i in range(len(s)):
            chars_values_s[s[i]] = 1 + chars_values_s.get(s[i],0)
            chars_values_t[t[i]] = 1 + chars_values_t.get(t[i],0)

        return chars_values_s == chars_values_t
        