fn main() {
    println!("{}",is_even(7));
}

fn is_even(num: i32) -> i32 {
    if num ==0 || num ==1 {
        return num;
    }
    return is_even(num-1) + is_even(num-2);
}
