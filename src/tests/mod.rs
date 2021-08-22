use crate::parser;
use crate::parser::Code;
use crate::runtime::engine::Engine;
use crate::walker::build;

use std::fs;
use std::io::empty;

#[test]
fn test() {
    let files: Vec<Box<str>> = std::fs::read_dir("./src/tests/samples/")
        .unwrap()
        .filter_map(|directory| {
            let path = directory.unwrap().path();
            let name = path.as_os_str().to_str().unwrap();
            name.ends_with(".lif").then(|| Box::from(&name[.. name.len() - 4]))
        })
        .collect();

    for file in files {
        let strings = read_content(&file);
        let grammar = parser::grammar();
        let mut input  = empty();
        let mut output = Vec::new();
        let mut error  = Vec::new();

        {
            let mut engine = Engine::new(&grammar, &mut input, &mut output, &mut error);
            let code = Code::from_string(engine.grammar, engine.grammar.program, build::traitify(&build::program), &strings.0);
            engine.run(code);
        }

        assert!(String::from_utf8(error).unwrap().is_empty());
        assert_eq!(String::from_utf8(output).unwrap(), format!("{}\n", strings.1));
    }
}

fn read_content(name: &str) -> (String, String) {
    let code = format!("{}.lif", name);
    let output = format!("{}.out", name);
    (fs::read_to_string(code).ok().unwrap(), clean_string(&fs::read_to_string(output).ok().unwrap()))
}

fn clean_string(input: &String) -> String {
    let mut output = input.replace("\r\n", "\n");
    output.pop();
    output
}
