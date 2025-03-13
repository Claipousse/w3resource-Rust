fn apply_closure<F> (vec:&mut Vec<i32>, mut closure:F)
where F: FnMut(&mut i32)
{
    for num in vec.iter_mut() {
        closure(num);
    }
}

fn main() {
    let mut vec:Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("Vector : {:?}", vec);
    let square_closure = |x:&mut i32| *x *= *x;
    apply_closure(&mut vec, square_closure);
    println!("Square vector : {:?}", vec);
}