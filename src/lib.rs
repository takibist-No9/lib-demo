mod generator;
pub fn print_radom_number() {
    let n = generator::gen_ran();
    println!("Random u8: {}", n);
}
