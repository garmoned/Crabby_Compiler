#[no_mangle]
pub extern "C" fn print_int(int: i16) {
    println!("{}", int);
    return;
}

// Adding the functions above to a global array,
// so Rust compiler won't remove them.
#[used]
static EXTERNAL_FNS: [extern "C" fn(i16); 1] = [print_int];
