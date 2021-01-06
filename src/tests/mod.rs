use crate::code::Code;
use crate::nodes::build;
use crate::parser::Parser;
use crate::runtime::engine::Engine;

use std::io::empty;

fn assert_output(text: &str, result: &str) {
    let parser = Parser::new();
    let mut input  = empty();
    let mut output = Vec::new();
    let mut error  = Vec::new();

    {
        let mut engine = Engine::new(&parser, &mut input, &mut output, &mut error);
        let code = Code::from_string(engine.parser, 0, &build::program, text);
        engine.run(code);
    }

    assert!(String::from_utf8(error).unwrap().is_empty());
    assert_eq!(String::from_utf8(output).unwrap(), format!("{}\n", result));
}

#[test]
fn test() {
    assert_output("print(0);", "0");
    assert_output("print(1);", "1");
    assert_output("print(227);", "227");

    assert_output("print(2 == 2);", "true");
    assert_output("print(2 != 2);", "false");
    assert_output("print(2 <= 2);", "true");
    assert_output("print(2 >= 2);", "true");

    assert_output("print(2 < 3);", "true");
    assert_output("print(2 > 3);", "false");
    assert_output("print(2 <= 3);", "true");
    assert_output("print(2 >= 3);", "false");
    assert_output("print(2 == 3);", "false");
    assert_output("print(2 != 3);", "true");

    assert_output("print(3 < 2);", "false");
    assert_output("print(3 > 2);", "true");
    assert_output("print(3 <= 2);", "false");
    assert_output("print(3 >= 2);", "true");
    assert_output("print(3 == 2);", "false");
    assert_output("print(3 != 2);", "true");

    assert_output("print(+0);", "0");
    assert_output("print(-0);", "0");
    assert_output("print(-1);", "-1");
    assert_output("print(--1);", "1");

    assert_output("print(12 + 2);", "14");
    assert_output("print(12 - 2);", "10");
    assert_output("print(12 * 2);", "24");
    assert_output("print(12 / 2);", "6");
    assert_output("print(12 % 2);", "0");

    assert_output("print(~21);", "-22");

    assert_output("print(21 & 7);", "5");
    assert_output("print(21 | 7);", "23");
    assert_output("print(21 ^ 7);", "18");

    assert_output("print(17 >> 1);", "8");
    assert_output("print(17 << 1);", "34");

    assert_output("print(true);", "true");
    assert_output("print(false);", "false");

    assert_output("print(true && true);", "true");
    assert_output("print(true && false);", "false");
    assert_output("print(false && true);", "false");
    assert_output("print(false && false);", "false");

    assert_output("print(true || true);", "true");
    assert_output("print(true || false);", "true");
    assert_output("print(false || true);", "true");
    assert_output("print(false || false);", "false");

    assert_output("print(\"Hello World !\");", "Hello World !");
    assert_output("print(\"1\" + \"2\");", "12");
    assert_output("print(\"1\" + 2);", "12");

    assert_output("print([0, 1, 2]);", "[0, 1, 2]");

    assert_output("print([2, 3, 4][0]);", "2");
    assert_output("print([2, 3, 4][2]);", "4");

    assert_output("let array = [0, 1, 2]; array.append(3); print(array);", "[0, 1, 2, 3]");
    assert_output("let array = [0, 1, 2]; array.prepend(-1); print(array);", "[-1, 0, 1, 2]");
    assert_output("let array = [0, 1, 2]; array.insert(1, 10); print(array);", "[0, 10, 1, 2]");
    assert_output("let array = [0, 1, 2]; array.remove(1); print(array);", "[0, 2]");

    assert_output("print(if true { 1 } else { 0 });", "1");
    assert_output("print(if false { 1 } else { 0 });", "0");
}
