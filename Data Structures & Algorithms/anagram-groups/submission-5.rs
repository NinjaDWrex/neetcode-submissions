use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // this is easy. store a dictionary of arrays the value of the dict will be of type Vec<String>. The first dictionary is for the word, and its value will be a dictionary for each character so it is its "characters signature" the key for first dict must be the characters signature though
        //steps:
        // turn each word into its char array and work out how many letters of each are on it
        //init dict
        let mut groups : HashMap<[i32;26], Vec<String>> = HashMap::new();
        for s in strs.into_iter() {
            let mut signature : [i32; 26]=[0;26]; 
            for c in s.bytes(){
                signature[(c - b'a') as usize] += 1;
            }
            // once we have done that, give ownership of signature to the dictionary
            if let Some(words) = groups.get_mut(&signature) {
                //case1 it exists and then we can just add it
                words.push(s);
            }
            else {
                //otherwise we need to insert the value and key into the dictionary
                groups.insert(signature, vec![s]);
            }
        }

        groups.into_values().collect()
        // now loop through the dict at the end, getting the values of each thingy and appending it to an answer
    }
}
