fn main() {
    dbg!(is_palindrome(121));
}

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false
    } else {
        let mut reversed = 0;
        let mut aux = x;
        while aux > 0 {
            reversed = reversed * 10 + aux % 10;
            aux /= 10;
        }
        return reversed == x
    }
} 