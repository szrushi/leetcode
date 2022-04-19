struct Solution;
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let (mut f, mut res) = (1, 0);
        for (i,c) in  s.trim().chars().enumerate() {
            if i == 0 && c == '-' {
                f = -1;
            } else if i == 0 && c == '+' {
                f = 1;
            } else if ('0'..='9').contains(&c) {
                //判断越界问题 Integer.MAX_VALUE - digit
                if  res>( i32::MAX - (c as i32 - '0' as i32))/ 10  {
                    if f == 1 {
                        return i32::MAX;
                    } else {
                        return i32::MIN;
                    } 
                }
                res = res*10+(c as i32 - '0' as i32);
            } else if c == ' ' {
                return res * f;
            }else {
                break;
            }
        }
        res * f
    }
}
fn main() {
    println!("Hello, world!");
    println!("{}", Solution::my_atoi("-42 asdad".to_string()));
    println!("{}", Solution::my_atoi("words and 987".to_string()));
    println!("{}", Solution::my_atoi("-91283472332".to_string()));
    println!("{}", Solution::my_atoi("3.14159".to_string()));
    println!("{}", Solution::my_atoi("2147483648".to_string()));
    println!("{}", Solution::my_atoi("2147483646".to_string()));
    println!("{}", Solution::my_atoi("-2147483647".to_string()));
   
}
