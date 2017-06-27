use std::error::Error;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut argument_spaces = "2".to_string();
    if args.len() == 2 {
        argument_spaces = args[1].clone().to_string();
    }

    let mut argument_path = (".").to_string();
    if args.len() == 3 {
        argument_spaces = args[1].clone().to_string();
        argument_path = args[2].clone().to_string();
    }

    let file_name = "/.editorconfig";
    let full_path = argument_path + &file_name;

    let save_path = Path::new(&full_path);
    let save_path_display = save_path.display();

    let part_1 = String::from("# editorconfig.org\n\
\n\
root = true\n\
\n\
[*]\n\
end_of_line = lf\n\
charset = utf-8\n\
trim_trailing_whitespace = true\n\
insert_final_newline = true\n\
indent_style = space\n\
indent_size = ");

let part_2 = String::from("\n\
\n\
[*.md]\n\
trim_trailing_whitespace = false\n");

    let contents = part_1 + &argument_spaces + &part_2;

    let mut output_file = match File::create(&save_path) {
        Err(error) => panic!("Could not create {}: {}", save_path_display, error.description()),
        Ok(file) => file,
    };

    output_file.write_all(contents.as_bytes()).unwrap();
}
