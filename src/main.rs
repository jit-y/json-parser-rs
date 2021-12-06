use json_parser_rs;

fn main() {
    let arg = std::env::args().nth(1).unwrap();

    json_parser_rs::print_json_ast(arg.as_str()).unwrap();
}
