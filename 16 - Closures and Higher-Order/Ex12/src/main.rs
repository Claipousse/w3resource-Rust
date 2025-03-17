fn apply_closure<F>(chars:Vec<char>, closure: F) -> Vec<char>
where
    F: Fn(char) -> char,
{
    chars.into_iter().map(closure).collect()

}


fn main() {
    let chars:Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let closure = |x:char| x.to_ascii_uppercase();
    let uppercase_chars = apply_closure(chars.clone(), closure);
    println!("lowercase : {:?}", chars);
    println!("Uppercase : {:?}", uppercase_chars)
}