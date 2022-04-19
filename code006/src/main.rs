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
 * 0          1.5        1.9
 * 1.1  2.3   2.6   2.8  2.10
 * 2.2        3.7
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
        let mut _rows = vec![String::new(); num_rows as usize];
        s[..].chars().enumerate().for_each(|(i, c)| {
            let row = i % (num_rows as usize - 1);
            let col = i / (num_rows as usize - 1);
            if let 0 = col % 2 {
                _rows[row].push(c);
            } else {
                _rows[num_rows as usize - 1 - row].push(c);
            }
        });
        _rows.into_iter().collect()
    }
}
fn main() {
    println!("Hello, world!");
    println!("{}", Solution::convert("PAYPALISHIRING".to_string(), 3));
    //print!("{});
}
