static MUH_GLOBAL: i32 = 1_000;

fn steal<'aaaaaargh>(x: &'aaaaaargh mut [i32]) {
    println!("{:?}", x);
    x.sort();
}

fn my_extend(vec: &mut Vec<i64>, slice: &[i64]) {
    for el in slice {
        vec.push(*el);
    }
}

fn main() {
    let mut super_var = vec![1, 90, 2, 1234, 123, 2, 45];
    let strings = vec!["hello".to_string(), "yes!".to_string(), "nope".to_string()];
    //steal(&mut super_var);
    //let t = &super_var; 
    //println!("t[0] = {}", t[0]);
    //let x = super_var;
    //my_extend(&mut super_var, &super_var);
    for rs in &strings {
        println!("{:?} @ {:p}", *rs, rs);
    }
}
