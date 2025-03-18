fn apply_closure<T, U, F>(vect: Vec<Option<T>>, mut closure: F) -> Vec<Option<U>>
where
    F: FnMut(T) -> U,
{
    vect.into_iter().map(|opt| opt.map(|x| closure(x))).collect()
}


fn main() {
    let vec = vec![Some(5), None, Some(10), Some(15)];
    println!("Original options : {:?}", vec);
    let modified_temps = apply_closure(vec, |x| x*2);
    println!("Options modifieds : {:?}", modified_temps)
}
