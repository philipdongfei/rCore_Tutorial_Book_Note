use backtrace::Backtrace;


fn main() {
    let bt = Backtrace::new();

    println!("Hello, world!");
    let a = 10u8;
    println!("a = {}", a);

    println!("{:?}", bt);

    println!("\nbacktrace::trace:\n");
    backtrace::trace(|frame| {
        let ip = frame.ip();
        let symbol_address = frame.symbol_address();

        println!("ip: {:?}", ip);
        println!("symbol_address: {:?}", symbol_address);
        // Resolve this instruction pointer to a symbol name
        backtrace::resolve_frame(frame, |symbol| {
            if let Some(name) = symbol.name(){
                println!("name: {:?}", name);
            }
            if let Some(filename) = symbol.filename() {
                println!("filename: {:?}", filename);
            }
        });
        true
    });
}
