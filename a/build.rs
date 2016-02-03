fn main() {
    if std::env::var("LIBCLANG_PATH").is_ok() {
        panic!()
    }
}
