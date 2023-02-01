#![allow(non_snake_case)]
#![allow(unused_imports)]

use std;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::Read;
use std::fs;
use std::io;
use std::io::Error;
use std::path::Path;

fn main() {
    let mut input_file_data_buf: [u8; 1024] = [0; 1024];
    let mut output_file_data_str = String::new();

    // Get application call arguments
    // syntax:  Bin2C [-ascii] input_filename output_filename [c_variable_name]
    // example: Bin2C -ascii file.bin file.c c_array
    // Full arguments example:
    // args.len() = 5
    // args[0] = "Bin2C"
    // args[1] = "-ascii"
    // args[2] = "file.bin"
    // args[3] = "file.c"
    // args[4] = "c_array"
    let args: Vec<String> = env::args().collect();

    // Analyze arguments, check if they are valid and load local variables accordingly
    let mut arguments_valid:    bool  = false;
    let mut arg_use_ascii:      bool  = false;
    let mut arg_num:            usize = 1;
    let mut input_filename    = String::new();
    let mut output_filename   = String::new();
    let mut c_variable_name   = String::from("c_array");

    // Check arguments
    if args.len() > 1 && args[1] == String::from("-ascii") {
        arg_use_ascii = true;
        arg_num = 2;
    }

    match args.len() - arg_num {
        0 => { 
            println!("Missing input and output file name arguments!")
        },
        1 => { 
            println!("Missing output file name argument!")
        },
        2 => { 
            arguments_valid = true; 
            input_filename.push_str(&args[arg_num]); 
            output_filename.push_str(&args[arg_num + 1]); 
        },
        3 => { 
            arguments_valid = true; 
            input_filename.push_str(&args[arg_num]); 
            output_filename.push_str(&args[arg_num + 1]); 
            c_variable_name.clear(); 
            c_variable_name.push_str(&args[arg_num + 2]); 
        },
        _ => { 
            println!("Invalid munber of arguments!") 
        },
    }

    if arguments_valid == false {
        println!("Syntax: Bin2C [-ascii] input_filename output_filename [c_variable_name]");
        return;
    }

#[cfg(debug_assertions)]
  /* Debug arguments */ 
    if arg_use_ascii {
        println!("-ascii option was provided");
    }
    println!("Input file name is {}",      &input_filename);
    println!("Output file name is {}",     &output_filename);
    println!("C variable name used is {}", &c_variable_name);

    // Open input file
    let mut input_file = match File::open(&input_filename) {
        Ok(file)  => file,
        Err(_err) => { println!("Input file not found!"); return } ,
    };

    // Get input file metadata
    let input_file_metadata = match input_file.metadata() {
        Ok(metadata) => metadata,
        Err(_err) => { println!("Invalid input file metadata!"); return } ,
    };

    // Get size of input file
    let input_file_size = input_file_metadata.len() as usize;
    if  input_file_size == 0 {
        println!("Input file is empty!");
    } else {
        println!("Input file size is {} bytes.", input_file_size);
    };

    // Create output file
    let mut output_file = match File::create(&output_filename) {
        Ok(file)  => file,
        Err(_err) => { println!("Output file creation failed!"); return } ,
    };

    // Construct header for output file
    output_file_data_str.push_str("const uint8_t ");
    output_file_data_str.push_str(&format!("{}[{}] = {{\r\n  ", c_variable_name, input_file_size));

    // Costruct data content for output file
    let mut total_read_size = 0;
    while   total_read_size != input_file_size {
        // Read up to 1024 bytes of data from input file into array
        let read_size = match input_file.read(&mut input_file_data_buf) {
            Ok(read_num_of_bytes) => read_num_of_bytes,
            Err(_err) => { println!("Could not read input file content!"); return } ,
        };

        for i in 0..read_size {
            if arg_use_ascii && input_file_data_buf[i] >= 32 && input_file_data_buf[i] <= 126 {
                output_file_data_str.push_str(&format!(" '{}'", input_file_data_buf[i] as char));
            } else {
                output_file_data_str.push_str(&format!("{:#04X}", input_file_data_buf[i]));
            }
            if (total_read_size + i) != (input_file_size - 1) {
                output_file_data_str.push_str(", ");
                if ((i + 1) % 16) == 0 {
                    output_file_data_str.push_str("\r\n  ");
                };
            };
        };
            
        total_read_size += read_size;
    }

    // Construct footer for output file
    output_file_data_str.push_str("\r\n};");

    // Write constructed data to output file
    let output_data_size = match output_file.write(output_file_data_str.as_bytes()) {
        Ok(num_of_bytes_written) => num_of_bytes_written,
        Err(_err) => { println!("Could not write output file content!"); return } ,
    };

    println!("Output file size is {} bytes.", output_data_size);

    println!("C variable name is \"{}\".", c_variable_name);
} 
