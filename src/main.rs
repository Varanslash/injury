use std::env::args;
use std::fs;
use std::collections::HashMap;

fn main() {
    let argv: Vec<String> = args().collect();
    let code = fs::read_to_string(argv[1].clone()).expect("Couldn't read code");
    let outputpath = argv[1].clone() + ".ij";

    let macro_apply = preproc_macro(code.clone());
    let main = preproc_main(code);
    if main == "Error Code 00000001" {
        return;
    }
    let constcode = preproc_construct(main, macro_apply);
    let machcode: Vec<u8> = constcode.into_bytes();
    let _ = fs::write(outputpath, machcode);
}

fn preproc_macro(code: String) -> HashMap<String, String> {
    let mut macros = HashMap::new();
    for lines in code.lines() {
        let lsplit: Vec<_> = lines.split_whitespace().collect();
        match lsplit[0] {
            "macro" => {
                macros.insert(lsplit[1].to_string(), lsplit[2].to_string());
            }
            _ => { continue; }
        }
    }
    return macros;
}

fn preproc_main(code: String) -> String {
    for lines in code.lines() {
        let lsplit: Vec<_> = lines.split_whitespace().collect();
        match lsplit[0] {
            "main" => {
                return lsplit[1].to_string();
            }
            _ => { continue; }
        }
    }
    println!("No main code found");
    return "Error Code 00000001".to_string();
}

fn preproc_construct(code: String, macro_a: HashMap<String, String>) -> String {
    if macro_a.is_empty() {
        return code;
    }

    let mut new_code: String = code.to_string();

    for (key, val) in macro_a {
        new_code = new_code.replace(&format!("${}$", key), &val);
    }

    return new_code;
}