use crate::common::data::Data;

/// Prints some data to stdout with a trailing newline.
pub fn println(data: Data) -> Result<Data, String> {
    println!("{}", data);
    Ok(data)
}

/// Prints some data to stdout without a trailing newline.
pub fn print(data: Data) -> Result<Data, String> {
    print!("{}", data);
    Ok(data)
}

pub fn to_string(data: Data) -> Result<Data, String> {
    Ok(Data::String(format!("{}", data)))
}
