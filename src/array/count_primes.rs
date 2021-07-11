// <计数质数>

// Count the number of prime numbers less than a non-negative number, n.

// Example 1:
// Input: n = 10
// Output: 4
// Explanation: There are 4 prime numbers less than 10, they are 2, 3, 5, 7.

// Example 2:
// Input: n = 0
// Output: 0

// Example 3:
// Input: n = 1
// Output: 0

// Constraints:
// 0 <= n <= 5 * 106

struct Solution;
impl Solution {
    /*
     * 排除法 计算0~n之间不符合质数的值
     */
    pub fn count_primes(n: i32) -> i32 {
        let mut res = 0;
        let n = n as usize;
        let mut is_primes = vec![true; n];
        for i in 2..n {
            if is_primes[i] {
                res += 1;
                let mut j = i;
                // NOTE 计算当前值之后所有值的倍数
                while i * j < n {
                    is_primes[i * j] = false;
                    j += 1;
                }
            }
        }
        return res;
    }

    /*
     * 暴力法，计算质数只需要校验2-sqrt(n)之间的数
     * 时间复杂度O(n * sqrt(n))
     */

    pub fn count_primes_sqrt(n: i32) -> i32 {
        let mut res = 0;
        for i in 2..n {
            if Self::is_prime(i) {
                res += 1;
            }
        }
        return res;
    }
    fn is_prime(n: i32) -> bool {
        let sqrt_n = (n as f32).sqrt().ceil() as i32;
        for i in 2..=sqrt_n {
            if n != sqrt_n && n % i == 0 {
                return false;
            }
        }
        return true;
    }
}

#[test]
fn run() {
    assert_eq!(Solution::count_primes(10), 4);
    assert_eq!(Solution::count_primes(0), 0);
    assert_eq!(Solution::count_primes(1), 0);
}
