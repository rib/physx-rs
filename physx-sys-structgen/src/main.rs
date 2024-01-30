extern "C" {
    pub fn structgen_main();

}

#[allow(unsafe_code)]
fn main() {
    let docs_dir = dirs::document_dir().unwrap();
    let _ = std::env::set_current_dir(&docs_dir);
    println!("Output Dir = {:?}", std::env::current_dir().unwrap());
    unsafe { structgen_main() };
}
