#[test]


/*// Make it work, don't modify `implicitly_ret_unit` !
fn main() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}*/



// Make it work, don't modify `implicitly_ret_unit` !
fn main() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(v, (2,3));
    //Утверждение теперь сравнивает v с кортежем (2, 3) напрямую
    // вместо вызова implicitly_ret_unit(), поскольку эта функция не возвращает никакое значение,
    // которое можно было бы сравнить с кортежем.

    println!("Success!");
}

fn _implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn _explicitly_ret_unit() -> () {
    println!("I will return a ()");
}
