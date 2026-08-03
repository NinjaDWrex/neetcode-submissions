class Solution:

    def encode(self, strs: List[str]) -> str:
        if not strs:
            return ""
        #map each string to its length in word lengths to get however many words we have
        word_lens = list(map(lambda s: len(s), strs))
        #now we structure the output
        lens = ",".join([str(length) for length in word_lens])
        stream = lens + "#" + "".join(strs)
        return stream
        
    def decode(self, s: str) -> List[str]:
        if not s:
            return []
        #go backwards
        #cut the first part before the # off the string
        strs = []
        lens = s.split("#")[0]
        s = s[len(lens)+1:]
        word_lens = lens.split(",")
        for n in word_lens:

            strs.append(s[:int(n)])
            s = s[int(n):]
        return strs

