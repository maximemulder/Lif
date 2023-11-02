use crate::parser;
use crate::parser::Code;
use crate::walker::nodes::AProgram;
use crate::runtime::bis::engine::{Engine, Io};

use std::ffi::OsStr;
use std::fs;
use std::io::empty;
use std::path::PathBuf;

struct Test {
    name: String,
    code: String,
    out: String,
    err: String,
}

#[test]
fn test() {
    let mut paths = Vec::new();
    fill_paths(PathBuf::from("./src/tests/samples/"), &mut paths);
    for path in paths {
        let test = make_test(&path);
        let grammar = parser::grammar();
        let mut r#in  = empty();
        let mut out = Vec::new();
        let mut err  = Vec::new();
        let io = Io::new(&mut r#in, &mut out, &mut err);

        {
            let mut engine = Engine::new(io, &grammar);
            let code = Code::new::<AProgram>(engine.grammar, engine.grammar.program, Some(&test.name), test.code.clone().into_boxed_str());
            engine.run(code);
        }

        compare_results(test, out, err);
    }
}

fn fill_paths(path: PathBuf, paths: &mut Vec<PathBuf>) {
    if path.extension() == Some(OsStr::new("lif")) {
        paths.push(path);
    } else if path.is_dir() {
        for dir in fs::read_dir(path).unwrap() {
            fill_paths(dir.unwrap().path(), paths);
        }
    }
}

fn make_test(path: &PathBuf) -> Test {
    let name = path.file_name().unwrap().to_str().unwrap().to_string();
    let code = fs::read_to_string(path).ok().unwrap();
    let out = clean_string(&fs::read_to_string(path.with_extension("out")).ok().unwrap_or(String::from("")));
    let err = clean_string(&fs::read_to_string(path.with_extension("err")).ok().unwrap_or(String::from("")));
    Test { name, code, out, err }
}

fn compare_results(test: Test, out: Vec<u8>, err: Vec<u8>) {
    let out = clean_string(&String::from_utf8(out).unwrap());
    let err = clean_string(&String::from_utf8(err).unwrap());
    assert_eq!(out, test.out, "\nTEST FAIL: file `{}`\nEXPECTED OUTPUT:\n{}FOUND OUTPUT:\n{}", test.name, test.out, out);
    assert_eq!(err, test.err, "\nTEST FAIL: file `{}`\nEXPECTED ERROR:\n{}FOUND ERROR:\n{}", test.name, test.err, err);
}

fn clean_string(input: &String) -> String {
    input.replace("\r\n", "\n")
}
