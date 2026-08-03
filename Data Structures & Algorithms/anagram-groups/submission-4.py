class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        anagrams = defaultdict(list)
        for s in strs:
            # count anagrams
            hashmap = [0] * 26
            for c in s:
                hashmap[ord(c) - ord('a')] += 1
            anagrams[tuple(hashmap)].append(s)
        return list(anagrams.values())