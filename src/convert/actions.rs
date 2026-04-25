use crate::{
    convert_byte::to_base_string,
    encryption::{decript, encript},
    option::OptionAction,
};
use std::fs;

pub fn file_to_bytes(argv: &mut Vec<String>) {
    let options = OptionAction::parse(argv.as_ref());
    if options.is_error {
        return;
    }
    // write file name in first position of text file.
    let file_name = options.get_filename();
    let mut byte_string: String = file_name.clone();
    // save base type to text file.
    byte_string += format!(" {}", options.base_type).as_str();
    let mut content = fs::read(options.file_path).expect("error read file");
    if let Some(v) = options.key {
        content = encript(&content, &v);
    }
    byte_string += to_base_string(&content, options.base_type).as_str();
    fs::write(format!("./{}.txt", file_name), byte_string).expect("cannot write file");

    // report status
    println!("\nConvert file to base {} successfuly.", options.base_type);
    println!("output ./{}.txt", file_name);
}

pub fn bytes_to_file(argv: &mut Vec<String>) {
    let mut options = OptionAction::parse(argv.as_ref());

    let content = fs::read_to_string(options.file_path).expect("error read file");
    let mut content_split = content.split(" ").collect::<Vec<&str>>();
    let file_name = *content_split.get(0).expect("Error read file name");
    options.base_type = content_split
        .get(1)
        .expect("Error read base type")
        .parse::<u8>()
        .expect("Error read base type");
    content_split.remove(1); // remove base type
    content_split.remove(0); // remove file name

    let mut content_byte = content_split
        .iter()
        .map(|f| {
            u8::from_str_radix(f.trim(), options.base_type as u32).expect("Error convert string to number")
        })
        .collect::<Vec<u8>>();
    if let Some(v) = options.key {
        if let Ok(byte) = decript(&content_byte, &v) {
            content_byte = byte;
        }
    }

    fs::write(file_name, content_byte).expect("cannot write file");
    println!("\nConvert base {} to file successfuly.", options.base_type);
    println!("output ./{}", file_name);
}
