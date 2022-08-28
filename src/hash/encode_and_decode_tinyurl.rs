// <TinyURL 的加密与解密>

// TinyURL is a URL shortening service where you enter a URL such as https://leetcode.com/problems/design-tinyurl and it returns a short URL such as http://tinyurl.com/4e9iAk. Design a class to encode a URL and decode a tiny URL.
// There is no restriction on how your encode/decode algorithm should work. You just need to ensure that a URL can be encoded to a tiny URL and the tiny URL can be decoded to the original URL.
// Implement the Solution class:
// Solution() Initializes the object of the system.
// String encode(String longUrl) Returns a tiny URL for the given longUrl.
// String decode(String shortUrl) Returns the original long URL for the given shortUrl. It is guaranteed that the given shortUrl was encoded by the same object.

// Example 1:
// Input: url = "https://leetcode.com/problems/design-tinyurl"
// Output: "https://leetcode.com/problems/design-tinyurl"

// Explanation:
// Solution obj = new Solution();
// string tiny = obj.encode(url); // returns the encoded tiny url.
// string ans = obj.decode(tiny); // returns the original url after deconding it.

// Constraints:
// 1 <= url.length <= 104
// url is guranteed to be a valid URL.

use std::collections::HashMap;

struct Codec {
    counter: usize,
    map: HashMap<usize, String>,
}

/*
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {
            counter: 0,
            map: HashMap::new(),
        }
    }

    // Encodes a URL to a shortened URL.
    fn encode(&mut self, longURL: String) -> String {
        self.map.insert(self.counter, longURL);
        let suffix = encode(self.counter);
        self.counter += 1;
        return format!("{}{}", PREFIX, suffix);
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, mut shortURL: String) -> String {
        let num = decode(shortURL.split_off(19));
        return self.map.get(&num).unwrap().clone();
    }
}

const PREFIX: &str = "http://tinyurl.com/";
const SCALE: usize = 62;
const CHARS: [u8; SCALE] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
    b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V',
    b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l',
    b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
];

fn encode(mut num: usize) -> String {
    let mut res: Vec<u8> = vec![];
    while num > SCALE {
        res.push(CHARS[num % SCALE]);
        num /= SCALE;
    }
    res.push(CHARS[num]);
    res.reverse();
    return String::from_utf8(res).expect("Base62 encode error");
}

fn decode(string: String) -> usize {
    let mut res = 0;
    for (i, b) in string.bytes().rev().enumerate() {
        println!("byte:{}", b as char);
        let tmp = CHARS.binary_search(&b).expect("Base62 decode error");
        res += SCALE.pow(i as u32) * tmp;
    }
    return res;
}
