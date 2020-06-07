// Working with text files
// To run: cargo run --bin text_files

use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Lines, Write, SeekFrom};
use std::io::prelude::*;

fn main() {
    println!(">>> Working with text files");

    let path = "./foo.txt";
    println!("Writing some data to '{}'", path);
    write_file(path, "Hello World!!!\n").expect("Failed to write to file");

    // ---------------------------------------------

    // read the entire file as string
    let content = read_file(path).expect("Failed to read file");
    println!("The file {} contains: ", path);
    println!("{}", content);

    // ------------------------------------------------

    // overwrite the file
    println!("Writing new data to '{}'", path);
    write_file(path, "New content\n").expect("Failed to write to file");
    let content = read_file(path).expect("Failed to read file");
    println!("The file '{}' now contains: ", path);
    println!("{}", content);

    // --------------------------------------------

    // append data to the file
    println!("Appending data to '{}'", path);
    append_file(path, "Some more content\n").expect("Failed to append to file");
    println!("The file '{}' now contains:", path);
    
    // read file line by line as an iterator
    let lines = read_file_iterator(path).expect("Failed to read file");
    for line in lines {
        println!("{}", line.expect("Failed to read line"));
    }

    // ---------------------------------------

    append_and_read(path, "Last line in the file").expect("Failed to read and write file");
}

fn write_file(path: &str, content: &str) -> io::Result<()> {
    // create() open a file with the standard options to create, write and truncate a file
    let file = File::create(path)?;

    // wrap the file in a BufReader to read in an efficient way
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(content.as_bytes())?;
    Ok(())
}

fn read_file(path: &str) -> io::Result<String> {
    // open() opens the file in read-only mode
    let file = File::open(path)?;

    // wrap the file in a BufReader to read in an efficient way
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}

fn append_file(path: &str, content: &str) -> io::Result<()> {
    // OpenOptions lets you set all options individually
    let file = OpenOptions::new().append(true).open(path)?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(content.as_bytes())?;
    Ok(())
}

fn read_file_iterator(path: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);

    // lines() returns an iterator over lines
    Ok(buf_reader.lines())
}

// reading and writing on the same handle
fn append_and_read(path: &str, content: &str) -> io::Result<()> {
    let file = OpenOptions::new().read(true).append(true).open(path)?;

    // passing a reference of the file will not move it
    // allowing you to create both a reader and a writer
    let mut buf_reader = BufReader::new(&file); // passed by reference
    let mut buf_writer = BufWriter::new(&file);

    let mut file_content = String::new();

    buf_reader.read_to_string(&mut file_content)?;
    println!("File before appending:\n{}", file_content);

    // appending will shift the positional pointer. So you
    // have to save and restore it
    let pos = buf_reader.seek(SeekFrom::Current(0))?;
    buf_writer.write_all(content.as_bytes())?;

    // flushing forces the write to happen right now
    buf_writer.flush()?;
    buf_reader.seek(SeekFrom::Start(pos))?;

    buf_reader.read_to_string(&mut file_content)?;
    println!("File after appending: \n{}", file_content);

    Ok(())
}