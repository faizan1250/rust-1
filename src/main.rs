fn main() {
    println!("{}",is_even(31));
}

fn is_even(num: i32) -> bool {
    if num% 2 ==0 {
        return true;
    }
    return false;
}
