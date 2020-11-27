fn main() {
    let mut s = String::from("hello");

    let len = calculate_length(&s);

    change(&mut s);

    let len2 = calculate_length(&s);

    println!("'{}'의 길이는 {}->{}입니다.", s, len, len2);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}