impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut message = String::new();
        let mut encodings = String::new();
        for s in strs.into_iter() {
            encodings.push_str(&format!("{},", s.len()));
            message.push_str(&s);
        }
        encodings.pop(); // remove trailing comma
        format!("{}#{}", encodings, message)
    }

    pub fn decode(s: String) -> Vec<String> {
        let parts: Vec<&str> = s.splitn(2, '#').collect();
        let encode = parts[0];
        let message = parts[1];

        let mut upto: usize = 0;
        let mut result: Vec<String> = Vec::new();

        if encode.is_empty() {
            return result;
        }

        for len_str in encode.split(',') {
            let len: usize = len_str.parse().unwrap();
            result.push(message[upto..upto + len].to_string());
            upto += len;
        }

        result
    }
}