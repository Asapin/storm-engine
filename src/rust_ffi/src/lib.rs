use std::{fs::File, io::Write};

#[no_mangle]
pub extern "C" fn hello_world() {
    let result = write_file("string");
    match result {
        Ok(()) => {},
        Err(e) => println!("Couldn't write file: {}", e)
    };
}

fn write_file(string: &str) -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    file.write_all(string.as_bytes())?;
    Ok(())
}