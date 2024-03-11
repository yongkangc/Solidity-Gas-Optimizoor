use optimizoor_parser::{parse, Parser};

fn main() {
    let source = include_str!("../examples/unoptimized_contracts/struct_packing.sol");

    //     let tokens = tokenize(struct_contract_source);
    //     let output = generate_output(tokens);
    //     println!("{}", output);
    // }

    // parse the source code
    let mut ast = parse(source).unwrap();
    println!("{:#?}", ast.body());
}
