fn main() {
    let v:Vec<u8> = (0..=10).collect();
    let v_pair:Vec<u8> = v.iter().cloned().filter(|&x| x % 2 == 0).collect();
    println!("Vecteur : {:?}", v);
    println!("Vecteur pair : {:?}", v_pair)
}