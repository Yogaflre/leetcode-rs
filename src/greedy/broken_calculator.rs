// <坏了的计算器>
// There is a broken calculator that has the integer startValue on its display initially. In one operation, you can:
//     multiply the number on display by 2, or
//     subtract 1 from the number on display.
// Given two integers startValue and target, return the minimum number of operations needed to display target on the calculator.

// Example 1:
// Input: startValue = 2, target = 3
// Output: 2
// Explanation: Use double operation and then decrement operation {2 -> 4 -> 3}.

// Example 2:
// Input: startValue = 5, target = 8
// Output: 2
// Explanation: Use decrement and then double {5 -> 4 -> 8}.

// Example 3:
// Input: startValue = 3, target = 10
// Output: 3
// Explanation: Use double, decrement and double {3 -> 6 -> 5 -> 10}.

// Constraints:
//     1 <= x, y <= 109

struct Solution;
impl Solution {
    /*
     * ❌ 正向思维：要先判断-1更临近1/8..1/4..1/2..x，再判断需要*几次2
     * ✅ 逆向思维：先判断targe能否变为1/2，再判断需要+几次1
     * 贪心的原则，优先取最优解
     */
    pub fn broken_calc(start_value: i32, target: i32) -> i32 {
        if start_value >= target {
            return start_value - target;
        } else {
            if target % 2 == 0 {
                return 1 + Self::broken_calc(start_value, target / 2);
            } else {
                return 1 + Self::broken_calc(start_value, target + 1);
            }
        }
    }
}
