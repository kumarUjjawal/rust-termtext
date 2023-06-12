fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Please provide file name as argument");
        std::process::exit(0);
    }

    if !std::path::Path::new(&args[1]).exists() {
        println!("File does not exist");
        std::process::exit(0);
    }

    println!("{}", termion::cursor::Show);
    let mut viewer = TextViewer::init(&args[1]);
    viewer.show_document();
    viewer.run();
}
