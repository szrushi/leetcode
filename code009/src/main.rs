/**
 * z 字排序
 *
 * PAYPALISHIRING
 * P   A   H   N          
 * A P L S I I G              
 * Y   I   R
 *
 * rows = 3
 * 1 -> 1.1
 * 2 -> 2.2
 *
 *
 *
 * 1          1.5        1.9
 * 2.2  2.4   2.6   2.8  2.10
 * 3.3        3.7
 *
 *
 *
 * 计算周期数量 即算出横向数量
 */
struct Solution {}
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        s
       // _rows.join("")
    }
}
fn main() {
    println!("Hello, world!");
    println!("{}", Solution::convert("PAYPALISHIRING".to_string(), 3));
    //print!("{});
}
