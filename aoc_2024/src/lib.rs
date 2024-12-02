pub fn get_single_path_as_arg() -> std::path::PathBuf {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Expected exactly one argument.")
    }
    std::path::PathBuf::from(args.get(1).expect("Failed to get first argument"))
        .canonicalize()
        .expect("Failed to parse path")
}
