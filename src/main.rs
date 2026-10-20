use c2c::cli::driver;
fn main() {
    match driver() {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e)
        }
    }
}
