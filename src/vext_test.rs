use std::io::{self,Write};
use std::path::Path;
use std::process::Command;
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const BLUE: &str = "\x1b[34m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
struct InvalidCharacter;
struct Vim;
struct Create;
struct Edit;
struct Compile;
struct Remove;
struct Move;
struct Copy;
struct Input;
struct Error;
struct Checker;
struct Nano;
struct OpenEditor;
struct Configuration;
struct Neovim;
struct Ls;
struct Rename;
pub struct Vexa;
trait Renames {
    fn rename_languages(language: &str,extension: &str) -> std::io::Result<()>;
}
trait Lss {
    fn ls() -> std::io::Result<()>;
}
trait Neovims {
    fn open_neovim(file: &str) -> std::io::Result<()>;
}
trait Configurations {
    fn configuration() -> std::io::Result<()>;
}
trait InvalidCharacters {
    fn invalid_characters(something: &str,what: &str) -> bool;
}
trait Nanos {
    fn open_nano(file: &str) -> std::io::Result<()>;
}
trait Vims {
    fn open_vim(file: &str) -> std::io::Result<()>;
}
trait OpenEditors {
    fn open_editor(file: &str) -> std::io::Result<()>;
}
trait Inputs {
    fn input() -> std::io::Result<String>;
}
trait Removes {
    fn remove_languages(language: &str,extension: &str) -> std::io::Result<()>;
    fn remove_cpp() -> std::io::Result<()>;
    fn remove_file() -> std::io::Result<()>;
    fn remove_dir() -> std::io::Result<()>;
    fn remove() -> std::io::Result<()>;
}
trait Copys {
    fn copy_languages(language: &str,extension: &str) -> std::io::Result<()>;
    fn copy_cpp() -> std::io::Result<()>;
    fn copy_file() -> std::io::Result<()>;
}
trait Moves {
    fn move_languages(language: &str,extension: &str) -> std::io::Result<()>;
    fn move_cpp() -> std::io::Result<()>;
    fn move_file() -> std::io::Result<()>;
    fn move_dir() -> std::io::Result<()>;
    fn moves() -> std::io::Result<()>;
}
trait Creates {
    fn create_languages(language: &str,extension: &str) -> std::io::Result<()>;
    fn create_cpp() -> std::io::Result<()>;
    fn create_file() -> std::io::Result<()>;
    fn create_dir() -> std::io::Result<()>;
    fn create() -> std::io::Result<()>;
}
trait Compiles {
    fn compile_rust(file: &str) -> std::io::Result<()>;
    fn compile_cpp(file: &str) -> std::io::Result<()>;
    fn compile_c(file: &str) -> std::io::Result<()>;
    fn compile_python(file: &str) -> std::io::Result<()>;
    fn compile_lua(file: &str) -> std::io::Result<()>;
    fn compile_js(file: &str) -> std::io::Result<()>;
    fn compile(language: &str,file: &str) -> std::io::Result<()>;
    fn compile_alone(compiler: &str,extension: &str,language: &str) -> std::io::Result<()>;
    fn rustc_explain(error: &str) -> std::io::Result<()>;
}
trait Errors {
    fn not_installed(not_installed: &str,what: &str);
    fn not_exist(what: &str,something: &str);
    fn invalid_extension(operation: &str,valid: &str,language: &str);
    fn spaces(something: &str,what: &str);
    fn invalid_characters(something: &str,what: &str);
    fn empty(expected: &str);
    fn operation_already_completed(operation: &str,something: &str,father_folder: &str,what: &str);
    fn ambiguities(file: &str,dir: &str);
    fn warning_remove(what: &str,something: &str);
    fn new_name_exist(name: &str,what: &str);
}
trait Checkers {
    fn checker(check: &str,something: &str) -> bool;
    fn check(what: &str,extension: &str) -> std::io::Result<()>;
    fn check_cpp() -> std::io::Result<()>;
    fn check_dir() -> std::io::Result<()>;
    fn check_program(what: &str) -> std::io::Result<()>;
    fn checker_program() -> std::io::Result<()>;
}
trait Edits {
    fn edit_languages(language: &str,extension: &str) -> std::io::Result<()>;
    fn edit_cpp() -> std::io::Result<()>;
    fn edit_file() -> std::io::Result<()>;
}
pub trait Vexas {
    fn vexa() -> std::io::Result<bool>;
}
impl Lss for Ls {
    fn ls() -> std::io::Result<()> {
    let entries = std::fs::read_dir(".")?;
    println!("Current dir: {:?}", std::env::current_dir());
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            println!(
                "{}{}{}{}",
                BOLD,
                BLUE,
                entry.file_name().to_string_lossy(),
                RESET
            );
        } else {
            println!("{}", entry.file_name().to_string_lossy());
        }
    }
    Ok(())
}
}
impl InvalidCharacters for InvalidCharacter {
    fn invalid_characters(something: &str,what: &str) -> bool {
        match what.trim() {
            "file" => {
                something.contains('@') ||
                something.contains('#') ||
                something.contains('$') ||
                something.contains('&') ||
                something.contains('-') ||
                something.contains('+') ||
                something.contains('(') ||
                something.contains(')') ||
                something.contains('/') ||
                something.contains('\\') ||
                something.contains('*') ||
                something.contains('"') ||
                something.contains(':') ||
                something.contains(';') ||
                something.contains('!') ||
                something.contains('?') ||
                something.contains('~') ||
                something.contains('`') ||
                something.contains('|') ||
                something.contains('•') ||
                something.contains('√') ||
                something.contains('π') ||
                something.contains('÷') ||
                something.contains('×') ||
                something.contains('§') ||
                something.contains('∆') ||
                something.contains('£') ||
                something.contains('¢') ||
                something.contains('€') ||
                something.contains('¥') ||
                something.contains('^') ||
                something.contains('°') ||
                something.contains('=') ||
                something.contains('{') ||
                something.contains('}') ||
                something.contains('%') ||
                something.contains('©') ||
                something.contains('®') ||
                something.contains('™') ||
                something.contains('✓') ||
                something.contains('[') ||
                something.contains(']') ||
                something.contains('←') ||
                something.contains('→') ||
                something.contains('↑') ||
                something.contains('↓') ||
                something.contains('<') ||
                something.contains('>')
            }
            "dir" | "folder" => {
                something.contains('.') ||
                something.contains('@') ||
                something.contains('#') ||                                                   
                something.contains('$') ||
                something.contains('&') ||                                                   
                something.contains('-') ||
                something.contains('+') ||                                                  
                something.contains('(') ||
                something.contains(')') ||
                something.contains('/') ||
                something.contains('\\') ||
                something.contains('*') ||
                something.contains('"') ||
                something.contains(':') ||
                something.contains(';') ||
                something.contains('!') ||
                something.contains('?') ||
                something.contains('~') ||
                something.contains('`') ||
                something.contains('|') ||
                something.contains('•') ||
                something.contains('√') ||
                something.contains('π') ||
                something.contains('÷') ||
                something.contains('×') ||
                something.contains('§') ||
                something.contains('∆') ||
                something.contains('£') ||
                something.contains('¢') ||
                something.contains('€') ||
                something.contains('¥') ||
                something.contains('^') ||
                something.contains('°') ||
                something.contains('=') ||
                something.contains('{') ||
                something.contains('}') ||
                something.contains('%') ||
                something.contains('©') ||
                something.contains('®') ||
                something.contains('™') ||
                something.contains('✓') ||
                something.contains('[') ||
                something.contains(']') ||
                something.contains('←') ||
                something.contains('→') ||
                something.contains('↑') ||
                something.contains('↓') ||
                something.contains('<') ||
                something.contains('>')
            }
            "arg" => {
                something.contains('.') || 
                something.contains('@') ||
                something.contains('#') ||
                something.contains('$') ||
                something.contains('&') ||
                something.contains('-') ||
                something.contains('(') ||
                something.contains(')') ||
                something.contains('/') ||
                something.contains('\\') ||
                something.contains('*') ||
                something.contains('"') ||
                something.contains(':') ||
                something.contains(';') ||
                something.contains('!') ||
                something.contains('?') ||
                something.contains('~') ||
                something.contains('`') ||
                something.contains('|') ||
                something.contains('•') ||
                something.contains('√') ||
                something.contains('π') ||
                something.contains('÷') ||
                something.contains('×') ||
                something.contains('§') ||
                something.contains('∆') ||
                something.contains('£') ||
                something.contains('¢') ||
                something.contains('€') ||
                something.contains('¥') ||
                something.contains('^') ||
                something.contains('°') ||
                something.contains('=') ||
                something.contains('{') ||
                something.contains('}') ||
                something.contains('%') ||
                something.contains('©') ||
                something.contains('®') ||
                something.contains('™') ||
                something.contains('✓') ||
                something.contains('[') ||
                something.contains(']') ||
                something.contains('←') ||
                something.contains('→') ||
                something.contains('↑') ||
                something.contains('↓') ||
                something.contains('<') ||
                something.contains('>') ||
                something.contains('∅') ||
                something.contains('‰') ||
                something.contains('℅') ||
                something.contains('∞') 
            }
            _=> {
                println!("invalid object");
                false
            }
        }
    }
}
impl Inputs for Input {
    fn input() -> std::io::Result<String> {
        print!("{BLUE}~{RESET} V ");
        io::stdout().flush()?;
        let mut text = String::new();
        io::stdin().read_line(&mut text)?;
        let text = text.trim().to_string();
        return Ok(text);
    }
}
impl Errors for Error {
    fn ambiguities(file: &str,dir: &str) {
        println!(r#"

        {BOLD}{RED}error{RESET}{BOLD}: The file "{file}" already exists on "{dir}"

        "#);
    }
    fn not_installed(not_installed: &str,what: &str) {
        println!(r#"

       {BOLD}{RED}error{RESET}{BOLD}: {not_installed} not installed{RESET}

       {BOLD}cannot open the non-existent editor code{RESET}
       {BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}

       {GREEN}~+ pkg install {what}{RESET}

       "#);
    }
    fn not_exist(what: &str,something: &str) {
        println!(r#"

        {BOLD}{RED}error{RESET}{BOLD}: Non-existent {what}{RESET}

        {BOLD}Cannot do any operation with a file that does not exist "{something}"{RESET}

        "#);
    }
    fn invalid_extension(operation: &str,valid: &str,language: &str) {
        match operation.trim() {
            "rename" => {
                println!(r#"

                {BOLD}{RED}error{RESET}{BOLD}: Invalid extension for operation{RESET}

        {BOLD}The extension entered and the operation entered they don't match.{RESET}               {BOLD}This operation only supports {language} files ({valid}){RESET}
        {BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}

        {BOLD}If you wanted to change name but entered invalid extension you must switch the extension{RESET}

        {BOLD}{YELLOW}Example{RESET}{BOLD}:{RESET}

        {BLUE}~{RESET} V {GREEN}+ Rename Rust file{RESET}

        Type the filename:
        {RED}-{RESET} file.txt
        {GREEN}+{RESET} file.rs

        {BOLD}But if you entered the wrong extension for operation:{RESET}

        {BOLD}try this command:{RESET}

        extensions --supported

        {BOLD}If the extension that you want is there run this command:{RESET}

        {BLUE}~{RESET} V {GREEN}+ {operation}{RESET}{YELLOW}#EXTENSION THAT YOU WANT{RESET}
        "#);
            }
            _=> {
            println!(r#"

        {BOLD}{RED}error{RESET}{BOLD}: Invalid extension for operation{RESET}

        {BOLD}The extension entered and the operation entered they don't match.{RESET}
        {BOLD}This operation only supports {language} files ({valid}){RESET}
        {BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}

        {BOLD}try this command:{RESET}

        extensions --supported

        {BOLD}If the extension that you want is there run this command:{RESET}

        {BLUE}~{RESET} V {GREEN}+ {operation}{RESET}{YELLOW}#EXTENSION THAT YOU WANT{RESET}

        "#);
            }
        }
    }
    fn spaces(something: &str,what: &str) {
        println!(r#"

        {BOLD}{RED}error{RESET}{BOLD}: {what} contains invalid spaces{RESET}


        {BOLD}The {what} contains invalid spaces "{something}", with these spaces the {what} It's no longer just one thing{BOLD}

        {BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}

        {BOLD}Remove these spaces{RESET}
        {BOLD}{YELLOW}Example{RESET}{BOLD}:{RESET}

        "#);
        match what.trim() {
            "file" => {
                println!(r#"
                {RED}-{RESET} hello world.rs
                {GREEN}+{RESET} hello_world.rs
                "#);
            }
            "dir" | "folder" => {
                println!(r#"
                {RED}-{RESET} Projects C
                {GREEN}+{RESET} Projects_C
                "#);
            }
            _=> {
                println!("not a valid object");
            }
        }
    }
    fn invalid_characters(something: &str,what: &str) {
        println!(r#"

        {BOLD}{RED}error{RESET}{BOLD}: {what} contains invalid characters{RESET}

        {BOLD}The {what} contains invalid characters "{something}"{RESET}
        {BOLD}Invalid characters can make it difficult to perform operations involving files or folders{RESET}

        {BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}

        {BOLD}Remove these invalid characters{RESET}
        {BOLD}{YELLOW}Example{RESET}{BOLD}:{RESET}

        "#);
        match what.trim() {
            "folder" | "dir" => {
                println!(r#"
                {RED}-{RESET} Projects@&#+$($)$()
                {GREEN}+{RESET} Projects
                "#);
            }
            "file" => {
                println!(r#"
                {RED}-{RESET} helloW@r$d#&$()*;$.rs
                {GREEN}+{RESET} helloWorld.rs
                "#);
            }
            _=> {
                println!("not a object");
            }
        }
    }
    fn empty(expected: &str) {
        println!(r#"

        {BOLD}{RED}error{RESET}{BOLD}: Empty arguments{RESET}

        {BOLD}Cannot do any operation with empty arguments{RESET}
        {BOLD}{YELLOW}Expected{RESET}{BOLD}:{RESET}

        {BLUE}~{RESET} V {GREEN}~+ {expected}{RESET}

        "#);
    }
    fn operation_already_completed(operation: &str,something: &str,father_folder: &str,what: &str) {
        println!(r#"

        {BOLD}{RED}error{RESET}{BOLD}: Operation already completed{RESET}

        "#);
        match operation.trim() {
            "move" | "mv" => {
                println!(r#"
                {BOLD}Already exists a {what} on {father_folder} with this name \"{something}\", Cannot move the {what} the operation was already completed{RESET}
                "#);
            }
            "rename" => {
                println!(r#"
                {BOLD}The {what} It is already called "{something}"
                {BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}
                {BOLD}Enter a different name to "{something}"{RESET}
                {YELLOW}{BOLD}Example{RESET}{BOLD}:{RESET}
                
                Type another name to "hello_world.rs"


                {BLUE}~{RESET} V {RED}-{RESET} hello_world.rs
                {BLUE}~{RESET} V {GREEN}+{RESET} hello_new_world.rs
                "#);
            }
            _=> {
                println!("Invalid object");
            }
        }
    }
    fn warning_remove(what: &str,something: &str) {
        println!(r#"

        {BOLD}{YELLOW}warning{RESET}{BOLD}: After confirming the {what} removal, it will not be possible to recover it{RESET}

        {BOLD}are you sure do you want to delete the {what} "{something}"? [y/n]

        "#);
    }
    fn new_name_exist(name: &str,what: &str) {
        println!(r#"

        {BOLD}{RED}error{RESET}{BOLD}: The name "{name}" already exists

        {BOLD}Already exists a {what} with this name "{name}", after the change of name, The {what} may be at risk of being overwritten{RESET}

        {BOLD}{YELLOW}Suggestion{RESET}{BOLD}:{RESET}

        {BOLD}check if any {what} uses the new name{RESET}

        {BOLD}{YELLOW}Example{RESET}{BOLD}:{RESET}

        {BOLD}{RED}Wrong{RESET}{BOLD} Way:{RESET}
        {BLUE}~{RESET} V {GREEN}+ ls{RESET}

        Projects_C
        file.rs
        test.rs

        {BLUE}~{RESET} V {GREEN}+ rnm rs file{RESET}

        vexa: type the file name
        {BLUE}~{RESET} V {GREEN}+ file.rs{RESET}

        vexa: type another name to "file.rs"
        {BLUE}~{RESET} V {GREEN}+ test.rs{RESET}

        {BOLD}{RED}#Error the name "test.rs" already exists{RESET}


        {BOLD}{GREEN}Right way:{RESET}
        {BLUE}~{RESET} V {GREEN}+ ls{RESET}

        Projects_C
        file.rs
        test.rs

        {BLUE}~{RESET} V {GREEN}+ rnm rs file{RESET}

        vaxa: type the file name
        {BLUE}~{RESET} V {GREEN}+ file.rs{RESET}

        vexa: type another name to "file.rs"
        {BLUE}~{RESET} V {GREEN}+ new_name.rs{RESET}

        {BOLD}{GREEN}#The file has been successfully renamed{RESET}

        "#);
    }
}
impl Checkers for Checker {
    fn checker(check: &str,something: &str) -> bool {
        match something.trim() {
            "file" => Path::new(&check).is_file(),
            "dir" | "folder"  => Path::new(&check).is_dir(),
            "exists" => Path::new(&check).exists(),
            _=> {
                println!("invalid object");
                return false;
            }
        }
    }
    fn check(what: &str,extension: &str) -> std::io::Result<()> {
        match what.trim() {
            "file" => {
                println!("\nType the file name");
                let file = Input::input()?;
                let expected = format!("file{extension}");
                let file_type = format!("{file}{extension}");
                if file.is_empty() {
                    Error::empty(&expected);
                    return Ok(());
                }
                else if file.contains(' ') && InvalidCharacter::invalid_characters(&file,"file") {
                    Error::spaces("file",&file);
                    Error::invalid_characters("file",&file);
                    println!("{BOLD}2 Errors detected{RESET}\n");
                    return Ok(());
                }
                else if InvalidCharacter::invalid_characters(&file,"file") {
                    Error::invalid_characters("file",&file);
                    return Ok(());
                }
                else if file.contains(' ') {
                    Error::spaces("file",&file);
                    return Ok(());
                }
                else if file.ends_with(extension) && Checker::checker(&file,"file") {
                    println!("\nFile \"{file}\" exist? = {BOLD}{BLUE}True{RESET}\n");
                    return Ok(());
                }
                else if file.ends_with(extension) && !Checker::checker(&file,"file") {
                    println!("\nFile \"{file}\" exist? = {BOLD}{BLUE}False{RESET}\n");
                    return Ok(());
                }
                else if Checker::checker(&file_type,"file") {
                    println!("\nFile \"{file}\" exist? = {BOLD}{BLUE}True{RESET}\n");
                    return Ok(());
                }
                else if !Checker::checker(&file_type,"file") {
                    println!("\nFile \"{file}\" exist? = {BOLD}{BLUE}False{RESET}\n");
                    return Ok(());
                }
                return Ok(());
            }
            _=> {
                println!("Invalid object");
                return Ok(());
            }
        }
    }
    fn check_cpp() -> std::io::Result<()> {
        println!("type the file name");
        let file = Input::input()?;
        let space_invalid_characters = InvalidCharacter::invalid_characters(&file,"file") && file.contains(' ');
        let is_cpp = file.ends_with(".cpp") || file.ends_with(".cxx") || file.ends_with(".c++") || file.ends_with(".cc") || file.ends_with(".cp");
        if file.is_empty() {
            Error::empty("file.cpp");
            return Ok(());
        }
        else if space_invalid_characters {
            Error::spaces(&file,"file");
            Error::invalid_characters(&file,"file");
            println!(r#"
            {BOLD}2 Errors detected{RESET}
            "#);
            return Ok(());
        }
        else if InvalidCharacter::invalid_characters(&file,"file") {
            Error::invalid_characters(&file,"file");
            return Ok(());
        }
        else if file.contains(' ') {
            Error::spaces(&file,"file");
            return Ok(());
        }
        else if is_cpp && Checker::checker(&file,"file") {
            println!("File \"{file}\" exist? = {BOLD}{BLUE}True{RESET}");
            return Ok(());
        }
        else if is_cpp && !Checker::checker(&file,"file") {
            println!("File \"{file}\" exist? = {BOLD}{BLUE}False{RESET}");
            return Ok(());
        }
        else if file.contains('.') {
            Error::invalid_extension("checker",".cpp .cxx .c++ .cc .cp","C++");
            return Ok(());
        }
        else {
            println!(r#"

            Which of these extensions?

            [1] {file}.cpp
            [2] {file}.cxx
            [3] {file}.c++
            [4] {file}.cc
            [5] {file}.cp

            "#);
            let option = Input::input()?;
            let file_cpp = format!("{file}.cpp");
            let file_cxx = format!("{file}.cxx");
            let file_cppp = format!("{file}.c++");
            let file_cc = format!("{file}.cc");
            let file_cp = format!("{file}.cp");
            match option.trim() {
                "1" => {
                    if Checker::checker(&file_cpp,"file") {
                        println!("File \"{file_cpp}\" exist? = {BOLD}{BLUE}True{RESET}");
                        return Ok(());
                    }
                    else if !Checker::checker(&file_cpp,"file") {
                        println!("File \"{file_cpp}\" exist? = {BOLD}{BLUE}False{RESET}");
                        return Ok(());
                    }
                    return Ok(());
                }
                "2" => {
                    if Checker::checker(&file_cxx,"file") {
                        println!("File \"{file_cxx}\" exist? = {BOLD}{BLUE}True{RESET}");
                        return Ok(());
                    }
                    else if !Checker::checker(&file_cxx,"file") {
                        println!("File \"{file_cxx}\" exist? = {BOLD}{BLUE}False{RESET}");
                        return Ok(());
                    }
                    return Ok(());
                }
                "3" => {
                    if Checker::checker(&file_cppp,"file") {
                        println!("File \"{file_cppp}\" exist? = {BOLD}{BLUE}True{RESET}");
                        return Ok(());
                    }
                    else if !Checker::checker(&file_cppp,"file") {
                        println!("File \"{file_cppp}\" exist? = {BOLD}{BLUE}False{RESET}");
                        return Ok(());
                    }
                    return Ok(());
                }
                "4" => {
                    if Checker::checker(&file_cc,"file") {
                        println!("File \"{file_cc}\" exist? = {BOLD}{BLUE}True{RESET}");
                        return Ok(());
                    }
                    else if !Checker::checker(&file_cc,"file") {
                        println!("File \"{file_cc}\" exist? = {BOLD}{BLUE}False{RESET}");
                        return Ok(());
                    }
                    return Ok(());
                }
                "5" => {
                    if Checker::checker(&file_cp,"file") {
                        println!("File \"{file_cp}\" exist? = {BOLD}{BLUE}True{RESET}");
                        return Ok(());
                    }
                    else if !Checker::checker(&file_cp,"file") {
                        println!("File \"{file_cp}\" exist? = {BOLD}{BLUE}False{RESET}");
                        return Ok(());
                    }
                    return Ok(());
                }
                _=> {
                    println!("{BOLD}{RED}error{RESET}{BOLD}: not a valid option{RESET}");
                    return Ok(());
                }
            }
        }
    }
    fn check_dir() -> std::io::Result<()> {
        println!("type the folder name");
        let folder = Input::input()?;
        let space_invalid_characters = InvalidCharacter::invalid_characters(&folder,"dir") && folder.contains(' ');
        if folder.is_empty() {
            Error::empty("Projects");
            return Ok(());
        }
        else if space_invalid_characters {
            Error::spaces(&folder,"folder");
            Error::invalid_characters(&folder,"folder");
            println!("{BOLD}2 Errors detected{RESET}\n");
            return Ok(());
        }
        else if InvalidCharacter::invalid_characters(&folder,"dir") {
            Error::invalid_characters(&folder,"folder");
            return Ok(());
        }
        else if folder.contains(' ') {
            Error::spaces(&folder,"folder");
            return Ok(());
        }
        else if Checker::checker(&folder,"dir") {
            println!("\nFolder \"{folder}\" exist? = {BOLD}{BLUE}True{RESET}");
            return Ok(());
        }
        else if !Checker::checker(&folder,"dir") {
            println!("\nFolder \"{folder}\" exist? = {BOLD}{BLUE}False{RESET}");
            return Ok(());
        }
        return Ok(());
    }
    fn check_program(what: &str) -> std::io::Result<()> {
        match Command::new(&what)
            .arg("--version")
            .status()
            {
                Ok(_) => println!("\nProgram \"{what}\" installed? = {BOLD}{BLUE}True{RESET}\n"),
                Err(_) => println!("\nProgram \"{what}\" installed? = {BOLD}{BLUE}False{RESET}\n"),
            }
        Ok(())
    }
    fn checker_program() -> std::io::Result<()> {
        println!("type the program name");
        let program = Input::input()?;
        let space_invalid_characters = program.contains(' ') && InvalidCharacter::invalid_characters(&program,"arg");
        if program.is_empty() {
            Error::empty("clang");
            return Ok(());
        }
        else if space_invalid_characters {
            Error::spaces(&program,"file");
            Error::invalid_characters(&program,"file");
            println!("\n{BOLD}2 Errors detected{RESET}");
            return Ok(());
        }
        else if program.contains(' ') {
            Error::spaces(&program,"file");
            return Ok(());
        }
        else if InvalidCharacter::invalid_characters(&program,"arg") {
            Error::invalid_characters(&program,"file");
            return Ok(());
        }
        else {
            match program.trim() {
                "rustc" => {
                    Checker::check_program("rustc")?;
                    return Ok(());
                }
                "clang" => {
                    Checker::check_program("clang")?;
                    return Ok(());
                }
                "clang++" => {
                    Checker::check_program("clang++")?;
                    return Ok(());
                }
                "lua" => {
                    Checker::check_program("lua")?;
                    return Ok(());
                }
                "node" => {
                    Checker::check_program("node")?;
                    return Ok(());
                }
                "python" => {
                    Checker::check_program("python")?;
                    return Ok(());
                }
                "vim" => {
                    Checker::check_program("vim")?;
                    return Ok(());
                }
                "nvim" => {
                    Checker::check_program("nvim")?;
                    return Ok(());
                }
                "nano" => {
                    Checker::check_program("nano")?;
                    return Ok(());
                }
                "cargo" => {
                    Checker::check_program("cargo")?;
                    return Ok(());
                }
                _=> {
                    println!("{BOLD}{RED}error{RESET}: The terminal dosen't support this program{RESET}\n");
                    return Ok(());
                }
            }
        }
    }
}
impl Nanos for Nano {
    fn open_nano(file: &str) -> std::io::Result<()> {
        match Command::new("nano")
            .arg(file)
            .status()
            {
                Ok(_) => {}
                Err(_) => Error::not_installed("Edito code","nano"),
            }
        Ok(())
    }
}
impl Vims for Vim {
    fn open_vim(file: &str) -> std::io::Result<()> {
        match Command::new("vim")
            .arg(file)
            .status()
            {
                Ok(_) => {}
                Err(_) => Error::not_installed("Editor code","vim"),
            }
        Ok(())
    }
}
impl Neovims for Neovim {
    fn open_neovim(file: &str) -> std::io::Result<()> {
        match Command::new("nvim")
            .arg(file)
            .status()
            {
                Ok(_) => {}
                Err(_) => Error::not_installed("Editor code","nvim"),
            }
        Ok(())
    }
}
impl OpenEditors for OpenEditor {
    fn open_editor(file: &str) -> std::io::Result<()> {
        if !Checker::checker("vexa.lock","file") {
            let code = format!(r#"
            CODEEDITOR="NANO"
            "#);
            std::fs::write("vexa.lock",code)?;
            return Ok(());
        }
        else if Checker::checker("vexa.lock","file") {
            let editor = std::fs::read_to_string("vexa.lock")?;
            if editor.contains("CODEEDITOR=\"NANO\"") {
                Nano::open_nano(&file)?;
                return Ok(());
            }
            else if editor.contains("CODEEDITOR=\"VIM\"") {
                Vim::open_vim(&file)?;
                return Ok(());
            }
            else if editor.contains("CODEEDITOR=\"NEOVIM\"") {
                Neovim::open_neovim(&file)?;
                return Ok(());
            }
            return Ok(());
        }
        return Ok(());
    }
}
impl Compiles for Compile {
    fn compile_rust(file: &str) -> std::io::Result<()> {
    let executable = file.strip_suffix(".rs").unwrap_or(file);
    match Command::new("rustc")
        .arg(file)
        .arg("-o")
        .arg(&executable)
        .status()
    {
        Ok(status) => {
            if status.success() {
                let executable = file.strip_suffix(".rs").unwrap_or(file);
                Command::new(format!("./{}", executable)).status()?;
            }
        }
        Err(_) => {
            Error::not_installed("Compiler", "rustc");
        }
    }
    Ok(())
}
fn compile_c(file: &str) -> std::io::Result<()> {
    let executable = file.strip_suffix(".c").unwrap_or(file);
    match Command::new("clang")
        .arg(file)
        .arg("-o")
        .arg(&executable)
        .status()
    {
        Ok(status) => {
            if status.success() {
                Command::new(format!("./{}",executable)).status()?;
            }
        }
        Err(_) => {
            Error::not_installed("Compiler", "clang");
        }
    }
    Ok(())
}
fn compile_cpp(file: &str) -> std::io::Result<()> {
    let path = Path::new(file);
    let executable = path.with_extension("");
    match Command::new("clang++")
        .arg(file)
        .arg("-o")
        .arg(&executable)
        .status()
    {
        Err(_) => {
            Error::not_installed("Compiler", "clang");
        }
        Ok(status) => {
            if status.success() {
                Command::new(format!("./{}", executable.display())).status()?;
            }
        }
    }
    Ok(())
}
fn compile_js(file: &str) -> std::io::Result<()> {
    match Command::new("node").arg(file).status() {
        Ok(_) => {}
        Err(_) => {
            Error::not_installed("Interpreter", "node");
        }
    }
    Ok(())
}
fn compile_python(file: &str) -> std::io::Result<()> {
    match Command::new("python").arg(file).status() {
        Ok(_) => {}
        Err(_) => {
            Error::not_installed("Interpreter", "python");
        }
    }
    Ok(())
}
fn compile_lua(file: &str) -> std::io::Result<()> {
    match Command::new("lua")
        .arg(file)
        .status() {
            Ok(_) => {}
            Err(_) => {
                Error::not_installed("Interpreter","lua");
            }
        }
    Ok(())
}
fn compile(language: &str,file: &str) -> std::io::Result<()> {
    match language.trim() {
        "rust" | "Rust" => {
            Compile::compile_rust(&file)?;
            return Ok(());
        }
        "cpp" | "C++" => {
            Compile::compile_cpp(&file)?;
            return Ok(());
        }
        "c" | "C" => {
            Compile::compile_c(&file)?;
            return Ok(());
        }
        "js" | "JavaScript" => {
            Compile::compile_js(&file)?;
            return Ok(());
        }
        "py" | "Python" => {
            Compile::compile_python(&file)?;
            return Ok(());
        }
        "lua" | "Lua" => {
            Compile::compile_lua(&file)?;
            return Ok(());
        }
        _=> {
            println!("The language \"{language}\" does not exist");
            return Ok(());
        }
    }
}
fn compile_alone(compiler: &str,extension: &str,language: &str) -> std::io::Result<()> {
    println!("\nType the file name");
    let file = Input::input()?;
    let file_type = format!("{file}{extension}");
    if file.is_empty() {
        match compiler.trim() {
            "rustc" => {
                Error::empty("file.rs");
                return Ok(());
            }
            "clang" => {
                Error::empty("file.c");
                return Ok(());
            }
            "clang++" => {
                Error::empty("file.cpp");
                return Ok(());
            }
            "nodejs" => {
                Error::empty("file.js");
                return Ok(());
            }
            "python" => {
                Error::empty("file.py");
                return Ok(());
            }
            "lua" => {
                Error::empty("file.lua");
                return Ok(());
            }
            _=> {
                println!("Invalid Compiler");
                return Ok(());
            }
        }
    }
    else if InvalidCharacter::invalid_characters(&file,"file") && file.contains(' ') {
        Error::spaces(&file,"file");
        Error::invalid_characters(&file,"file");
        println!("{BOLD}2 Errors Detected{RESET}\n");
        return Ok(());
    }
    else if InvalidCharacter::invalid_characters(&file,"file") {
        Error::invalid_characters(&file,"file");
        return Ok(());
    }
    else if file.contains(' ') {
        Error::invalid_characters(&file,"file");
        return Ok(());
    }
    else if file.ends_with(extension) && Checker::checker(&file,"file") {
        Compile::compile(&compiler,&file)?;
        return Ok(());
    }
    else if file.ends_with(extension) && !Checker::checker(&file,"file") {
        Error::not_exist(&file,"file");
        return Ok(());
    }
    else if file.contains('.') {
        Error::invalid_extension(&compiler,&extension,&language);
        return Ok(());
    }
    else if Checker::checker(&file_type,"file") {
        Compile::compile(&compiler,&file_type)?;
        return Ok(());
    }
    else if !Checker::checker(&file_type,"file") {
        Error::not_exist(&file_type,"file");
        return Ok(());
    }
    return Ok(());
}
fn rustc_explain(error: &str) -> std::io::Result<()> {
    match Command::new("rustc")
        .args(["--explain",error])
        .status()
        {
            Ok(_) => {}
            Err(_) => Error::not_installed("Compiler","rustc"),
        }
    Ok(())
}
}
impl Configurations for Configuration {
    fn configuration() -> std::io::Result<()> {
        println!(r#"

        {BOLD}• change code editor{RESET}

        {BLUE}• -> equipped code editor{RESET}
        {YELLOW}• -> code editor not equipped{RESET}

        "#);
        if !Checker::checker("vexa.lock","file") {
            let code = format!(r#"
            CODEEDITOR="NANO"
            "#);
            std::fs::write("vexa.lock",&code)?;
        }
        let editor = std::fs::read_to_string("vexa.lock")?;
        if editor.contains("CODEEDITOR=\"NANO\"") {
            println!("{BLUE}• Nano{RESET}");
            println!("{YELLOW}• Vim{RESET}");
            println!("{YELLOW}• Neovim{RESET}\n");
            println!("type in a code editor to equipped\n");
            let change = Input::input()?;
            match change.trim() {
                "vim" => {
                    let code = format!(r#"
                    CODEEDITOR="VIM"
                    "#);
                    std::fs::write("vexa.lock",&code)?;
                    return Ok(());
                }
                "neovim" | "nvim" => {
                    let code = format!(r#"
                    CODEEDITOR="NEOVIM"
                    "#);
                    std::fs::write("vexa.lock",&code)?;
                    return Ok(());
                }
                "nano" => {
                    println!("{BOLD}{RED}error{RESET}{BOLD}: code editor already equipped try another{RESET}");
                    return Ok(());
                }
                _=> {
                    println!("{BOLD}{RED}error{RESET}{BOLD}: editor code not found{RESET}");
                    return Ok(());
                }
            }
        }
        else if editor.contains("CODEEDITOR=\"VIM\"") {
            println!("{BLUE}• Vim{RESET}");
            println!("{YELLOW}• Neovim{RESET}");
            println!("{YELLOW}• Nano{RESET}\n");
            println!("type in a code editor to equipped\n");
            let change = Input::input()?;
            match change.trim() {
                "nano" => {
                    let code = format!(r#"
                    CODEEDITOR="NANO"
                    "#);
                    std::fs::write("vexa.lock",&code)?;
                    return Ok(());
                }
                "neovim" => {
                    let code = format!(r#"
                    CODEEDITOR="NEOVIM"
                    "#);
                    std::fs::write("vexa.lock",&code)?;
                    return Ok(());
                }
                "vim" => {
                    println!("{BOLD}{RED}error{RESET}{BOLD}: code editor already equipped try another{RESET}");
                    return Ok(());
                }
                _=> {
                    println!("{BOLD}{RED}error{RESET}{BOLD}: editor code not found{RESET}");
                    return Ok(());
                }
            }
        }
        else if editor.contains("CODEEDITOR=\"NEOVIM\"") {
            println!("{BLUE}• Neovim{RESET}");
            println!("{YELLOW}• Nano{RESET}");
            println!("{YELLOW}• Vim{RESET}");
            println!("type in a code editor to equipped\n");
            let change = Input::input()?;
            match change.trim() {
                "nano" => {
                    let code = format!(r#"
                    CODEEDITOR="NANO"
                    "#);
                    std::fs::write("vexa.lock",&code)?;
                    return Ok(());
                }
                "vim" => {
                    let code = format!(r#"
                    CODEEDITOR="VIM"
                    "#);
                    std::fs::write("vexa.lock",&code)?;
                    return Ok(());
                }
                "neovim" => {
                    println!("{BOLD}{RED}error{RESET}{BOLD}: code editor already equipped try another{RESET}");
                    return Ok(());
                }
                _=> {
                    println!("{BOLD}{RED}error{RESET}{BOLD}: code editor already equipped try another{RESET}");
                    return Ok(());
                }
            }
        }
        return Ok(());
    }
}
impl Creates for Create {
    fn create_languages(language: &str,extension: &str) -> std::io::Result<()> {
        println!("\nType the file name");
        let file = Input::input()?;
        let space_invalid_characters = file.contains(' ') && InvalidCharacter::invalid_characters(&file,"file");
        let file_type = format!("{file}{extension}");
        if file.is_empty() {
            match language.trim() {
                "rust" | "Rust"=> {
                    Error::empty("file.rs");
                    return Ok(());
                }
                "c" | "C" => {
                    Error::empty("file.c");
                    return Ok(());
                }
                "javascript" | "JavaScript"=> {
                    Error::empty("file.js");
                    return Ok(());
                }
                "python" | "Python"=> {
                    Error::empty("file.py");
                    return Ok(());
                }
                "lua" | "Lua" => {
                    Error::empty("file.lua");
                    return Ok(());
                }
                _=> {
                    println!("error");
                    return Ok(());
                }
            }
        }
        else if space_invalid_characters {
            Error::spaces(&file,"file");
            Error::invalid_characters(&file,"file");
            println!("{BOLD}2 Errors detected{RESET}\n");
            return Ok(());
        }
        else if InvalidCharacter::invalid_characters(&file,"file") {
            Error::invalid_characters(&file,"file");
            return Ok(());
        }
        else if file.contains(' ') {
            Error::spaces(&file,"file");
            return Ok(());
        }
        else if file.ends_with(extension) && Checker::checker(&file,"file") {
            println!("The file \"{file}\" already exists want to overwrite? [Y/N]");
            let option = Input::input()?;
            match option.trim() {
                "y" | "yes" => {
                    std::fs::write(&file,"")?;
                    OpenEditor::open_editor(&file)?;
                    Compile::compile(&language,&file)?;
                }
                _=> {
                    println!("\n");
                }
            }
            return Ok(());
        }
        else if file.ends_with(extension) && !Checker::checker(&file,"file") {
            OpenEditor::open_editor(&file)?;
            Compile::compile(&language,&file)?;
            return Ok(());
        }
        else if file.contains('.') {
            Error::invalid_extension("create file",&extension,&language);
            return Ok(());
        }
        else if Checker::checker(&file_type,"file") {
            println!("The file \"{file_type}\" already exists want to overwrite? [Y/N]");
            let option = Input::input()?;
            match option.trim() {
                "y" | "yes" => {
                    std::fs::write(&file_type,"")?;
                    OpenEditor::open_editor(&file)?;
                    Compile::compile(&language,&file_type)?;
                }
                _=> {
                    println!("\n");
                }
            }
            return Ok(());
        }
        else if !Checker::checker(&file_type,"file") {
            OpenEditor::open_editor(&file_type)?;
            Compile::compile(&language,&file_type)?;
            return Ok(());
        }
        return Ok(());
    }
    fn create_cpp() -> std::io::Result<()> {
        println!("\nType the file name");
        let file = Input::input()?;
        let is_cpp = file.ends_with(".cpp") || file.ends_with(".cxx") || file.ends_with(".c++") || file.ends_with(".cc") || file.ends_with(".cp");
        let space_invalid_characters = file.contains(' ') && InvalidCharacter::invalid_characters(&file,"file");
        if file.is_empty() {
            Error::empty("file.cpp");
            return Ok(());
        }
        else if space_invalid_characters {
            Error::spaces(&file,"file");
            Error::invalid_characters(&file,"file");
            println!("{BOLD}2 Errors detected{RESET}\n");
            return Ok(());
        }
        else if InvalidCharacter::invalid_characters(&file,"file") {
            Error::invalid_characters(&file,"file");
            return Ok(());
        }
        else if file.contains(' ') {
            Error::spaces(&file,"file");
            return Ok(());
        }
        else if is_cpp && Checker::checker(&file,"file") {
            println!("\nThe file \"{file}\" already exists, want to overwrite? [y/n]");
            let option = Input::input()?;
            match option.trim() {
                "y" | "yes" => {
                    std::fs::write(&file,"")?;
                    OpenEditor::open_editor(&file)?;
                    Compile::compile("cpp",&file)?;
                }
                _=> {
                    println!("\n");
                }
            }
            return Ok(());
        }
        else if is_cpp && !Checker::checker(&file,"file") {
            OpenEditor::open_editor(&file)?;
            Compile::compile("cpp",&file)?;
            return Ok(());
        }
        else if file.contains('.') {
            Error::invalid_extension("create file",".cpp .cxx .c++ .cc .cp","C++");
            return Ok(());
        }
        else {
            println!(r#"

            which of these extensions?

            [1] {file}.cpp
            [2] {file}.cxx
            [3] {file}.c++
            [4] {file}.cc
            [5] {file}.cp

            "#);
            let option = Input::input()?;
            let file_cpp = format!("{file}.cpp");
            let file_cxx = format!("{file}.cxx");
            let file_cppp = format!("{file}.c++");
            let file_cc = format!("{file}.cc");
            let file_cp = format!("{file}.cp");
            match option.trim() {
                "1" => {
                    if Checker::checker(&file_cpp,"file") {
                        println!("The file \"{file_cpp}\" already exists, want to overwrite? [y/n]");
                        let option = Input::input()?;
                        match option.trim() {
                            "yes" | "y" => {
                                std::fs::write(&file_cpp,"")?;
                                OpenEditor::open_editor(&file_cpp)?;
                                Compile::compile("cpp",&file_cpp)?;
                            }
                            _=> {
                                println!("\n");
                            }
                        }
                        return Ok(());
                    }
                    else if !Checker::checker(&file_cpp,"file") {
                        OpenEditor::open_editor(&file_cpp)?;
                        Compile::compile("cpp",&file_cpp)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                "2" => {
                    if Checker::checker(&file_cxx,"file") {
                        println!("The file \"{file_cxx}\" already exists, want to overwrite? [y/n]");
                        let option = Input::input()?;
                        match option.trim() {
                            "yes" | "y" => {
                                std::fs::write(&file_cxx,"")?;
                                OpenEditor::open_editor(&file_cxx)?;
                                Compile::compile("cpp",&file_cxx)?;
                            }
                            _=> {
                                println!("\n");
                            }
                        }
                        return Ok(());
                    }
                    else if !Checker::checker(&file_cxx,"file") {
                        OpenEditor::open_editor(&file_cxx)?;
                        Compile::compile("cpp",&file_cxx)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                "3" => {
                    if Checker::checker(&file_cppp,"file") {
                        println!("The file \"{file_cppp}\" already exists, want to overwrite? [y/n]");
                        let option = Input::input()?;
                        match option.trim() {
                            "yes" | "y" => {
                                std::fs::write(&file_cppp,"")?;
                                OpenEditor::open_editor(&file_cppp)?;
                                Compile::compile("cpp",&file_cppp)?;
                            }
                            _=> {
                                println!("\n");
                            }
                        }
                        return Ok(());
                    }
                    else if !Checker::checker(&file_cppp,"file") {
                        OpenEditor::open_editor(&file_cppp)?;
                        Compile::compile("cpp",&file_cppp)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                "4" => {
                    if Checker::checker(&file_cc,"file") {
                        println!("The file \"{file_cc}\" already exists, want to overwrite? [y/n]");
                        let option = Input::input()?;
                        match option.trim() {
                            "yes" | "y" => {
                                std::fs::write(&file_cc,"")?;
                                OpenEditor::open_editor(&file_cc)?;
                                Compile::compile("cpp",&file_cc)?;
                            }
                            _=> {
                                println!("\n");
                            }
                        }
                        return Ok(());
                    }
                    else if !Checker::checker(&file_cc,"file") {
                        OpenEditor::open_editor(&file_cc)?;
                        Compile::compile("cpp",&file_cc)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                "5" => {
                    if Checker::checker(&file_cp,"file") {
                        println!("The file \"{file_cp}\" already exists, want to overwrite? [y/n]");
                        let option = Input::input()?;
                        match option.trim() {
                            "yes" | "y" => {
                                std::fs::write(&file_cp,"")?;
                                OpenEditor::open_editor(&file_cp)?;
                                Compile::compile("cpp",&file_cp)?;
                            }
                            _=> {
                                println!("\n");
                            }
                        }
                        return Ok(());
                    }
                    else if !Checker::checker(&file_cp,"file") {
                        OpenEditor::open_editor(&file_cp)?;
                        Compile::compile("cpp",&file_cp)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                _=> {
                    println!("{BOLD}{RED}error{RESET}{BOLD}: not a valid option{RESET}\n");
                    return Ok(());
                }
            }
        }
    }
    fn create_file() -> std::io::Result<()> {
        let languages = ["[1] Rust (.rs)","[2] C++ (.cpp) (.cxx) (.c++) (.cc) (.cp)","[3] C (.c)","[4] JavaScript (.js)","[5] Python (.py)","[6] Lua (.lua)"];
        for language in languages {
            println!("{language}");
        }
        let option = Input::input()?;
        match option.trim() {
            "1" => {
                Create::create_languages("Rust",".rs")?;
                return Ok(());
            }
            "2" => {
                Create::create_cpp()?;
                return Ok(());
            }
            "3" => {
                Create::create_languages("C",".c")?;
                return Ok(());
            }
            "4" => {
                Create::create_languages("JavaScript",".js")?;
                return Ok(());
            }
            "5" => {
                Create::create_languages("Python",".py")?;
                return Ok(());
            }
            "6" => {
                Create::create_languages("Lua",".lua")?;
                return Ok(());
            }
            _=> {
                println!("\n{BOLD}{RED}error{RESET}{BOLD}: not a valid option{RESET}\n");
                return Ok(());
            }
        }
    }
    fn create_dir() -> std::io::Result<()> {
        println!("\nType the folder name");
        let folder = Input::input()?;
        let space_invalid_characters = folder.contains(' ') && InvalidCharacter::invalid_characters(&folder,"dir");
        if folder.is_empty() {
            Error::empty("Projects");
            return Ok(());
        }
        else if space_invalid_characters {
            Error::spaces(&folder,"folder");
            Error::invalid_characters(&folder,"folder");
            println!("{BOLD}2 Errors detected{RESET}\n");
            return Ok(());
        }
        else if InvalidCharacter::invalid_characters(&folder,"dir") {
            Error::invalid_characters(&folder,"folder");
            return Ok(());
        }
        else if folder.contains(' ') {
            Error::spaces(&folder,"folder");
            return Ok(());
        }
        else if Checker::checker(&folder,"folder") {
            println!(r#"

            {BOLD}{YELLOW}warning{RESET}{BOLD}: Already exists a folder with this name{RESET}

            {BOLD}I'ts not possible to overwrite a folder{RESET}

            "#);
            return Ok(());
        }
        else if !Checker::checker(&folder,"folder") {
            std::fs::create_dir_all(&folder)?;
            return Ok(());
        }
        return Ok(());
    }
    fn create() -> std::io::Result<()> {
        println!("\nCreate What?\n");
        println!("[1] File");
        println!("[2] Folder\n");
        let option = Input::input()?;
        match option.trim() {
            "1" => {
                Create::create_file()?;
                return Ok(());
            }
            "2" => {
                Create::create_dir()?;
                return Ok(());
            }
            _=> {
                println!("{BOLD}{RED}error{RESET}{BOLD}: Not a valid option{RESET}");
                return Ok(());
            }
        }
    }
}
impl Moves for Move {
    fn move_languages(language: &str,extension: &str) -> std::io::Result<()> {
        println!("\nType the folder name");
        let folder = Input::input()?;
        let space_invalid_characters_folder = folder.contains(' ') && InvalidCharacter::invalid_characters(&folder,"dir");
        if folder.is_empty() {
            Error::empty("Projects");
            return Ok(());
        }
        else if space_invalid_characters_folder {
            Error::spaces(&folder,"folder");
            Error::invalid_characters(&folder,"folder");
            println!("{BOLD}2 Errors detected{RESET}\n");
            return Ok(());
        }
        else if InvalidCharacter::invalid_characters(&folder,"dir") {
            Error::invalid_characters(&folder,"folder");
            return Ok(());
        }
        else if folder.contains(' ') {
            Error::spaces(&folder,"folder");
            return Ok(());
        }
        else if !Checker::checker(&folder,"folder") {
            Error::not_exist("folder",&folder);
            return Ok(());
        }
        else if Checker::checker(&folder,"folder") {
        println!("\nType the file name");
        let file = Input::input()?;
        let path_new = format!("{folder}/{file}");
        let space_invalid_characters = file.contains(' ') && InvalidCharacter::invalid_characters(&file,"file");
        let expected = format!("file{extension}");
        let file_type = format!("{file}{extension}");
        let path_new_type = format!("{folder}/{file_type}");
        if file.is_empty() {
            Error::empty(&expected);
            return Ok(());
        }
        else if space_invalid_characters {
            Error::spaces(&file,"file");
            Error::invalid_characters(&file,"file");
            println!("{BOLD}2 Errors detected{RESET}");
            return Ok(());
        }
        else if InvalidCharacter::invalid_characters(&file,"file") {
            Error::invalid_characters(&file,"file");
            return Ok(());
        }
        else if file.contains(' ') {
            Error::spaces(&file,"file");
            return Ok(());
        }
        else if file.ends_with(extension) && Checker::checker(&file,"file")  && !Checker::checker(&path_new,"exists") {
            std::fs::rename(&file,&path_new)?;
            if extension == ".c" || extension == ".rs" {
                let executable_rs = file.replace(".rs","");
                let executable_c = file.replace(".c","");
                let path_new_executable_rs = format!("{folder}/{executable_rs}");
                let path_new_executable_c = format!("{folder}/{executable_c}");
                if file.ends_with(".rs") && Checker::checker(&executable_rs,"file") {
                    std::fs::rename(&executable_rs,&path_new_executable_rs)?;
                    return Ok(());
                }
                else if file.ends_with(".c") && Checker::checker(&executable_c,"file") {
                    std::fs::rename(&executable_c,&path_new_executable_c)?;
                    return Ok(());
                }
                return Ok(());
            }
            return Ok(());
        }
        else if file.ends_with(extension) && !Checker::checker(&file,"file") && Checker::checker(&path_new,"exists") {
            Error::operation_already_completed("move",&file,&folder,"file");
            return Ok(());
        }
        else if file.ends_with(extension) && !Checker::checker(&file,"file") && !Checker::checker(&path_new,"exists") {
            Error::not_exist("file",&file);
            return Ok(());
        }
        else if file.ends_with(extension) && Checker::checker(&file,"file") && Checker::checker(&path_new,"exists") {
            Error::ambiguities(&file,&folder);
            return Ok(());
        }
        else if file.contains('.') {
            Error::invalid_extension("move",&extension,&language);
            return Ok(());
        }
        else if Checker::checker(&file_type,"file") && !Checker::checker(&path_new_type,"exists") {
            std::fs::rename(&file_type,&path_new_type)?;
            if extension == ".rs" || extension == ".c" {
                let path_new_executable = format!("{folder}/{file}");
                if Checker::checker(&file,"file") {
                    std::fs::rename(&file,&path_new_executable)?;
                    return Ok(());
                }
                return Ok(());
            }
            return Ok(());
        }
        else if !Checker::checker(&file_type,"file") && Checker::checker(&path_new_type,"exists") {
            Error::operation_already_completed("move",&file_type,&folder,"file");
            return Ok(());
        }
        else if !Checker::checker(&file_type,"file") && !Checker::checker(&path_new_type,"exists") {
            Error::not_exist("file",&file_type);
            return Ok(());
        }
        else if Checker::checker(&file_type,"file") && Checker::checker(&path_new_type,"exists") {
            Error::ambiguities(&file_type,&folder);
            return Ok(());
        }
        return Ok(());
        }
        return Ok(());
    }
    fn move_cpp() -> std::io::Result<()> {
        println!("\nType the folder name");
        let folder = Input::input()?;
        let space_invalid_characters = folder.contains(' ') && InvalidCharacter::invalid_characters(&folder,"dir");
        if folder.is_empty() {
            Error::empty("Projects");
            return Ok(());
        }
        else if space_invalid_characters {
            Error::spaces(&folder,"folder");
            Error::invalid_characters(&folder,"folder");
            println!("{BOLD}2 Errors detected{RESET}\n");
            return Ok(());
        }
        else if InvalidCharacter::invalid_characters(&folder,"dir") {
            Error::invalid_characters(&folder,"folder");
            return Ok(());
        }
        else if folder.contains(' ') {
            Error::spaces(&folder,"folder");
            return Ok(());
        }
        else if !Checker::checker(&folder,"folder") {
            Error::not_exist("folder",&folder);
            return Ok(());
        }
        else if Checker::checker(&folder,"folder") {
            println!("\nType the file name");
            let file = Input::input()?;
            let is_cpp = file.ends_with(".cpp") || file.ends_with(".cxx") || file.ends_with(".c++") || file.ends_with(".cc") || file.ends_with(".cp");
            let space_invalid_characters_file = file.contains(' ') && InvalidCharacter::invalid_characters(&file,"file");
            let path_new = format!("{folder}/{file}");
            if file.is_empty() {
                Error::empty("file.cpp");
                return Ok(());
            }
            else if space_invalid_characters_file {
                Error::spaces(&file,"file");
                Error::invalid_characters(&file,"file");
                println!("{BOLD}2 Errors detected{RESET}\n");
                return Ok(());
            }
            else if InvalidCharacter::invalid_characters(&file,"file") {
                Error::invalid_characters(&file,"file");
                return Ok(());
            }
            else if file.contains(' ') {
                Error::spaces(&file,"file");
                return Ok(());
            }
            else if is_cpp && Checker::checker(&file,"file") && !Checker::checker(&path_new,"exists") {
                std::fs::rename(&file,&path_new)?;
                let executable_cpp = file.replace(".cpp","");
                let executable_cxx = file.replace(".cxx","");
                let executable_cppp = file.replace(".c++","");
                let executable_cc = file.replace(".cc","");
                let executable_cp = file.replace(".cp","");
                let path_new_executable_cpp = format!("{folder}/{executable_cpp}");
                let path_new_executable_cxx = format!("{folder}/{executable_cxx}");
                let path_new_executable_cppp = format!("{folder}/{executable_cppp}");
                let path_new_executable_cc = format!("{folder}/{executable_cc}");
                let path_new_executable_cp = format!("{folder}/{executable_cp}");
                if file.ends_with(".cpp") && Checker::checker(&executable_cpp,"file") {
                    std::fs::rename(&executable_cpp,&path_new_executable_cpp)?;
                    return Ok(());
                }
                else if file.ends_with(".cxx") && Checker::checker(&executable_cxx,"file") {
                    std::fs::rename(&executable_cxx,&path_new_executable_cxx)?;
                    return Ok(());
                }
                else if file.ends_with(".c++") && Checker::checker(&executable_cppp,"file") {
                    std::fs::rename(&executable_cppp,&path_new_executable_cppp)?;
                    return Ok(());
                }
                else if file.ends_with(".cc") && Checker::checker(&executable_cc,"file") {
                    std::fs::rename(&executable_cc,&path_new_executable_cc)?;
                    return Ok(());
                }
                else if file.ends_with(".cp") && Checker::checker(&executable_cp,"file") {
                    std::fs::rename(&executable_cp,&path_new_executable_cp)?;
                    return Ok(());
                }
                return Ok(());
            }
            else if is_cpp && !Checker::checker(&file,"file") && Checker::checker(&path_new,"exists") {
                Error::operation_already_completed("move",&file,&folder,"file");
                return Ok(());
            }
            else if is_cpp && !Checker::checker(&file,"file") && !Checker::checker(&path_new,"exists") {
                Error::not_exist("file",&file);
                return Ok(());
            }
            else if is_cpp && Checker::checker(&file,"file") && Checker::checker(&path_new,"exists") {
                Error::ambiguities(&file,&folder);
                return Ok(());
            }
            else if file.contains('.') {
                Error::invalid_extension("move",".cpp .cxx .c++ .cc .cp","C++");
                return Ok(());
            }
            else {
                println!(r#"

                Which of these extensions?

                [1] {file}.cpp
                [2] {file}.cxx
                [3] {file}.c++
                [4] {file}.cc
                [5] {file}.cp

                "#);
                let option = Input::input()?;
                let file_cpp = format!("{file}.cpp");
                let file_cxx = format!("{file}.cxx");
                let file_cppp = format!("{file}.c++");
                let file_cc = format!("{file}.cc");
                let file_cp = format!("{file}.cp");
                let path_new_cpp = format!("{folder}/{file_cpp}");
                let path_new_cxx = format!("{folder}/{file_cxx}");
                let path_new_cppp = format!("{folder}/{file_cppp}");
                let path_new_cc = format!("{folder}/{file_cc}");
                let path_new_cp = format!("{folder}/{file_cp}");
                let path_new_executable = format!("{folder}/{file}");
                match option.trim() {
                    "1" => {
                        if Checker::checker(&file_cpp,"file") && !Checker::checker(&path_new_cpp,"exists") {
                            std::fs::rename(&file_cpp,&path_new_cpp)?;
                            if Checker::checker(&file,"file") {
                                std::fs::rename(&file,&path_new_executable)?;
                                return Ok(());
                            }
                            return Ok(());
                        }
                        else if !Checker::checker(&file_cpp,"file") && Checker::checker(&path_new_cpp,"exists") {
                            Error::operation_already_completed("move",&file_cpp,&folder,"file");
                            return Ok(());
                        }
                        else if !Checker::checker(&file_cpp,"file") && !Checker::checker(&path_new_cpp,"exists") {
                            Error::not_exist("file",&file_cpp);
                            return Ok(());
                        }
                        else if Checker::checker(&file_cpp,"file") && Checker::checker(&path_new_cpp,"exists") {
                            Error::ambiguities(&file_cpp,&folder);
                            return Ok(());
                        }
                        return Ok(());
                    }
                    "2" => {
                        if Checker::checker(&file_cxx,"file") && !Checker::checker(&path_new_cxx,"exists") {
                            std::fs::rename(&file_cxx,&path_new_cxx)?;
                            if Checker::checker(&file,"file") {
                                std::fs::rename(&file,&path_new_executable)?;
                                return Ok(());
                            }
                            return Ok(());
                        }
                        else if !Checker::checker(&file_cxx,"file") && Checker::checker(&path_new_cxx,"exists") {
                            Error::operation_already_completed("move",&file_cxx,&folder,"file");
                            return Ok(());
                        }
                        else if !Checker::checker(&file_cxx,"file") && !Checker::checker(&path_new_cxx,"exists") {
                            Error::not_exist("file",&file_cxx);
                            return Ok(());
                        }
                        else if Checker::checker(&file_cxx,"file") && Checker::checker(&path_new_cxx,"exists") {
                            Error::ambiguities(&file_cxx,&folder);
                            return Ok(());
                        }
                        return Ok(());
                    }
                    "3" => {
                        if Checker::checker(&file_cppp,"file") && !Checker::checker(&path_new_cppp,"exists") {
                            std::fs::rename(&file_cppp,&path_new_cppp)?;
                            if Checker::checker(&file,"file") {
                                std::fs::rename(&file,&path_new_executable)?;
                                return Ok(());
                            }
                            return Ok(());
                        }
                        else if !Checker::checker(&file_cppp,"file") && Checker::checker(&path_new_cppp,"exists") {
                            Error::operation_already_completed("move",&file_cppp,&folder,"file");
                            return Ok(());
                        }
                        else if !Checker::checker(&file_cppp,"file") && !Checker::checker(&path_new_cppp,"exists") {
                            Error::not_exist("file",&file_cppp);
                            return Ok(());
                        }
                        else if Checker::checker(&file_cppp,"file") && Checker::checker(&path_new_cppp,"exists") {
                            Error::ambiguities(&file_cppp,&folder);
                            return Ok(());
                        }
                        return Ok(());
                    }
                    "4" => {
                        if Checker::checker(&file_cc,"file") && !Checker::checker(&path_new_cc,"exists") {
                            std::fs::rename(&file_cc,&path_new_cc)?;
                            if Checker::checker(&file,"file") {
                                std::fs::rename(&file,&path_new_executable)?;
                                return Ok(());
                            }
                            return Ok(());
                        }
                        else if !Checker::checker(&file_cc,"file") && Checker::checker(&path_new_cc,"exists") {
                            Error::operation_already_completed("move",&file_cc,&folder,"file");
                            return Ok(());
                        }
                        else if !Checker::checker(&file_cc,"file") && !Checker::checker(&path_new_cc,"exists") {
                            Error::not_exist("file",&file_cc);
                            return Ok(());
                        }
                        else if Checker::checker(&file_cc,"file") && Checker::checker(&path_new_cc,"exists") {
                            Error::ambiguities(&file_cc,&folder);
                            return Ok(());
                        }
                        return Ok(());
                    }
                    "5" => {
                        if Checker::checker(&file_cp,"file") && !Checker::checker(&path_new_cp,"exists") {
                            std::fs::rename(&file_cp,&path_new_cp)?;
                            if Checker::checker(&file,"file") {
                                std::fs::rename(&file,&path_new_executable)?;
                                return Ok(());
                            }
                            return Ok(());
                        }
                        else if !Checker::checker(&file_cp,"file") && Checker::checker(&path_new_cp,"exists") {
                            Error::operation_already_completed("move",&file_cp,&folder,"file");
                            return Ok(());
                        }
                        else if !Checker::checker(&file_cp,"file") && !Checker::checker(&path_new_cp,"exists") {
                            Error::not_exist("file",&file_cp);
                            return Ok(());
                        }
                        else if Checker::checker(&file_cp,"file") && Checker::checker(&path_new_cp,"exists") {
                            Error::ambiguities(&file_cp,&folder);
                            return Ok(());
                        }
                        return Ok(());
                    }
                    _=> {
                        println!("\n{BOLD}{RED}error{RESET}{BOLD}: Not a valid option{RESET}\n");
                        return Ok(());
                    }
                }
            }
        }
        return Ok(());
    }
    fn move_file() -> std::io::Result<()> {
        let languages = ["[1] Rust (.rs)","[2] C++ (.cpp) (.cxx) (.c++) (.cc) (.cp)","[3] C (.c)","[4] JavaScript (.js)","[5] Python (.py)","[6] Lua (.lua)"];
        for language in languages {
            println!("{language}");
        }
        let option = Input::input()?;
        match option.trim() {
            "1" => {
                Move::move_languages("rust",".rs")?;
                return Ok(());
            }
            "2" => {
                Move::move_cpp()?;
                return Ok(());
            }
            "3" => {
                Move::move_languages("c",".c")?;
                return Ok(());
            }
            "4" => {
                Move::move_languages("javascript",".js")?;
                return Ok(());
            }
            "5" => {
                Move::move_languages("python",".py")?;
                return Ok(());
            }
            "6" => {
                Move::move_languages("lua",".lua")?;
                return Ok(());
            }
            _=> {
                println!("\n{BOLD}{RED}error{RESET}{BOLD}: not a valid option{RESET}\n");
                return Ok(());
            }
        }
    }
    fn move_dir() -> std::io::Result<()> {
        println!("\ntype the parent folder");
        let father = Input::input()?;
        let space_invalid_characters = InvalidCharacter::invalid_characters(&father,"dir") && father.contains(' ');
        if father.is_empty() {
            Error::empty("Projects");
            return Ok(());
        }
        else if space_invalid_characters {
            Error::spaces(&father,"folder");
            Error::invalid_characters(&father,"folder");
            println!("{BOLD}2 Errors detected{RESET}\n");
            return Ok(());
        }
        else if InvalidCharacter::invalid_characters(&father,"dir") {
            Error::invalid_characters(&father,"folder");
            return Ok(());
        }
        else if father.contains(' ') {
            Error::spaces(&father,"folder");
            return Ok(());
        }
        else if !Checker::checker(&father,"dir") {
            Error::not_exist(&father,"folder");
            return Ok(());
        }
        else if Checker::checker(&father,"dir") {
            println!("\ntype the child folder");
            let child = Input::input()?;
            let space_invalid_characters_child = InvalidCharacter::invalid_characters(&child,"dir") && child.contains(' ');
            let path_new = format!("{father}/{child}");
            if child.is_empty() {
                Error::empty("components");
                return Ok(());
            }
            else if space_invalid_characters_child {
                Error::spaces(&child,"folder");
                Error::invalid_characters(&child,"folder");
                println!("{BOLD}2 Errors detected{RESET}\n");
                return Ok(());
            }
            else if InvalidCharacter::invalid_characters(&child,"dir") {
                Error::invalid_characters(&child,"folder");
                return Ok(());
            }
            else if child.contains(' ') {
                Error::spaces(&child,"folder");
                return Ok(());
            }
            else if child == father {
                println!("\n{BOLD}{RED}error{RESET}{BOLD}:  It is not possible to move a folder to itself{RESET}\n");
                return Ok(());
            }
            else if Checker::checker(&child,"dir") && !Checker::checker(&path_new,"exists") {
                std::fs::rename(&child,&path_new)?;
                return Ok(());
            }
            else if !Checker::checker(&child,"dir") && Checker::checker(&path_new,"exists") {
                Error::operation_already_completed("move",&child,&father,"folder");
                return Ok(());
            }
            else if !Checker::checker(&child,"dir") && !Checker::checker(&path_new,"exists") {
                Error::not_exist(&child,"folder");
                return Ok(());
            }
            else if Checker::checker(&child,"dir") && Checker::checker(&path_new,"exists") {
                Error::ambiguities(&child,&father);
                return Ok(());
            }
            return Ok(());
        }
        return Ok(());
    }
    fn moves() -> std::io::Result<()> {
        println!("\nMove what?");
        println!("[1] File");
        println!("[2] Folder");
        let option = Input::input()?;
        match option.trim() {
            "1" => {
                Move::move_file()?;
                return Ok(());
            }
            "2" => {
                Move::move_dir()?;
                return Ok(());
            }
            _=> {
                println!("\n{BOLD}{RED}error{RESET}{BOLD}: Not a valid option\n");
                return Ok(());
            }
        }
    }
}
impl Removes for Remove {
    fn remove_languages(language: &str,extension: &str) -> std::io::Result<()> {
        match extension.trim() {
            ".rs" => {
                println!("\ntype the file name");
                let file = Input::input()?;
                let file_type = format!("{file}.rs");
                let space_invalid_characters = InvalidCharacter::invalid_characters(&file,"file") && file.contains(' ');
                if file.is_empty() {
                    Error::empty("file.rs");
                    return Ok(());
                }
                else if space_invalid_characters {
                    Error::spaces(&file,"file");
                    Error::invalid_characters(&file,"file");
                    println!("{BOLD}2 Errors detected{RESET}\n");
                    return Ok(());
                }
                else if InvalidCharacter::invalid_characters(&file,"file") {
                    Error::invalid_characters(&file,"file");
                    return Ok(());
                }
                else if file.contains(' ') {
                    Error::spaces(&file,"file");
                    return Ok(());
                }
                else if file == "Cargo.toml" || file == "Cargo.lock" || file == ".gitignore" {
                    println!("{BOLD}{RED}error{RESET}{BOLD}: OPERATIOND BLOCKED{RESET}");
                    println!("\n{BOLD}Cannot remove the file, It is essential for the system to function; without it, the terminal does not work{RESET}");
                    return Ok(());
                }
                else if file.ends_with(".rs") && Checker::checker(&file,"file") {
                    Error::warning_remove("file",&file);
                    let option = Input::input()?;
                    let binary = file.replace(".rs","");
                    match option.trim() {
                        "y" => {
                            std::fs::remove_file(&file)?;
                            if Checker::checker(&binary,"file") {
                                std::fs::remove_file(&binary)?;
                                return Ok(());
                            }
                            return Ok(());
                        }
                        _=> {
                            println!("\n");
                        }
                    }
                    return Ok(());
                }
                else if file.ends_with(".rs") && !Checker::checker(&file,"file") {
                    Error::not_exist(&file,"file");
                    return Ok(());
                }
                else if file.contains('.') {
                    Error::invalid_extension("remove",&extension,&language);
                    return Ok(());
                }
                else if Checker::checker(&file_type,"file") {
                    Error::warning_remove("file",&file);
                    let option = Input::input()?;
                    match option.trim() {
                        "y" => {
                            std::fs::remove_file(&file_type)?;
                            if Checker::checker(&file,"file") {
                                std::fs::remove_file(&file)?;
                                return Ok(());
                            }
                            return Ok(());
                        }
                        _=> {
                            println!("\n");
                        }
                    }
                    return Ok(());
                }
                else if !Checker::checker(&file_type,"file") {
                    Error::not_exist(&file_type,"file");
                    return Ok(());
                }
                return Ok(());
            }
            ".c" => {
                println!("\ntype the file name");
                let file = Input::input()?;
                let space_invalid_characters = InvalidCharacter::invalid_characters(&file,"file") && file.contains(' ');
                let file_type = format!("{file}.c");
                if file.is_empty() {
                    Error::empty("file.c");
                    return Ok(());
                }
                else if space_invalid_characters {
                    Error::spaces(&file,"file");
                    Error::invalid_characters(&file,"file");
                    println!("{BOLD}2 Errors detected{RESET}\n");
                    return Ok(());
                }
                else if InvalidCharacter::invalid_characters(&file,"file") {
                    Error::invalid_characters(&file,"file");
                    return Ok(());
                }
                else if file.contains(' ') {
                    Error::spaces(&file,"file");
                    return Ok(());
                }
                else if file == "Cargo.toml" || file == "Cargo.lock" || file == ".gitignore" {

                    println!("{BOLD}{RED}error{RESET}{BOLD}: OPERATIOND BLOCKED{RESET}");
                    println!("\n{BOLD}Cannot remove the file, It is essential for the system to function; without it, the terminal does not work{RESET}");
                    return Ok(());
                }
                else if file.ends_with(".c") && Checker::checker(&file,"file") {
                    Error::warning_remove("file",&file);
                    let executable = file.replace(".c","");
                    let option = Input::input()?;
                    match option.trim() {
                        "y" => {
                            std::fs::remove_file(&file)?;
                            if Checker::checker(&executable,"file") {
                                std::fs::remove_file(&executable)?;
                                return Ok(());
                            }
                            return Ok(());
                        }
                        _=> {
                            println!("\n");
                        }
                    }
                    return Ok(());
                }
                else if file.ends_with(".c") && !Checker::checker(&file,"file") {
                    Error::not_exist(&file,"file");
                    return Ok(());
                }
                else if file.contains('.') {
                    Error::invalid_extension("remove",&extension,&language);
                    return Ok(());
                }
                else if Checker::checker(&file_type,"file") {
                    Error::warning_remove("file",&file);
                    let option = Input::input()?;
                    match option.trim() {
                        "y" => {
                            std::fs::remove_file(&file_type)?;
                            if Checker::checker(&file,"file") {
                                std::fs::remove_file(&file)?;
                                return Ok(());
                            }
                            return Ok(());
                        }
                        _=> {
                            println!("\n");
                        }
                    }
                    return Ok(());
                }
                else if !Checker::checker(&file_type,"file") {
                    Error::not_exist(&file_type,"file");
                    return Ok(());
                }
                return Ok(());
            }
            _=> {
                println!("\ntype the file name");
                let file = Input::input()?;
                let space_invalid_characters = InvalidCharacter::invalid_characters(&file,"file") && file.contains(' ');
                let file_type = format!("{file}{extension}");
                let expected = format!("file{extension}");
                if file.is_empty() {
                    Error::empty(&expected);
                    return Ok(());
                }
                else if space_invalid_characters {
                    Error::spaces(&file,"file");
                    Error::invalid_characters(&file,"file");
                    println!("{BOLD}2 Errors detected{RESET}\n");
                    return Ok(());
                }
                else if InvalidCharacter::invalid_characters(&file,"file") {
                    Error::invalid_characters(&file,"file");
                    return Ok(());
                }
                else if file.contains(' ') {
                    Error::spaces(&file,"file");
                    return Ok(());
                }
                else if file == "Cargo.toml" || file == "Cargo.lock" || file == ".gitignore" {

                    println!("{BOLD}{RED}error{RESET}{BOLD}: OPERATIOND BLOCKED{RESET}");
                    println!("\n{BOLD}Cannot remove the file, It is essential for the system to function; without it, the terminal does not work{RESET}");
                    return Ok(());
                }
                else if file.ends_with(extension) && Checker::checker(&file,"file") {
                    Error::warning_remove("file",&file);
                    let option = Input::input()?;
                    match option.trim() {
                        "y" => {
                            std::fs::remove_file(&file)?;
                        }
                        _=> {
                            println!("\n");
                        }
                    }
                    return Ok(());
                }
                else if file.ends_with(extension) && !Checker::checker(&file,"file") {
                    Error::not_exist(&file,"file");
                    return Ok(());
                }
                else if file.contains('.') {
                    Error::invalid_extension("remove",&extension,&language);                    return Ok(());
                }
                else if Checker::checker(&file_type,"file") {
                    Error::warning_remove("file",&file_type);
                    let option = Input::input()?;
                    match option.trim() {
                        "y" => {
                            std::fs::remove_file(&file_type)?;
                        }
                        _=> {
                            println!("\n");
                        }
                    }
                    return Ok(());
                }
                else if !Checker::checker(&file_type,"file") {
                    Error::not_exist(&file_type,"file");
                    return Ok(());
                }
                return Ok(());
            }
        }
    }
    fn remove_cpp() -> std::io::Result<()> {
        println!("\ntype the file name");
        let file = Input::input()?;
        let space_invalid_characters = InvalidCharacter::invalid_characters(&file,"file");
        let is_cpp = file.ends_with(".cpp") || file.ends_with(".cxx") || file.ends_with(".c++") || file.ends_with(".cc") || file.ends_with(".cp");
        if file.is_empty() {
            Error::empty("file.cpp");
            return Ok(());
        }
        else if space_invalid_characters {
            Error::spaces(&file,"file");
            Error::invalid_characters(&file,"file");
            println!("{BOLD}2 Errors detected{RESET}\n");
            return Ok(());
        }
        else if InvalidCharacter::invalid_characters(&file,"file") {
            Error::invalid_characters(&file,"file");
            return Ok(());
        }
        else if file.contains(' ') {
            Error::spaces(&file,"file");
            return Ok(());
        }

                else if file == "Cargo.toml" || file == "Cargo.lock" || file == ".gitignore" {

                    println!("{BOLD}{RED}error{RESET}{BOLD}: OPERATIOND BLOCKED{RESET}");
                    println!("\n{BOLD}Cannot remove the file, It is essential for the system to function; without it, the terminal does not work{RESET}");
                    return Ok(());
                }
        else if is_cpp && Checker::checker(&file,"file") {
            Error::warning_remove("file",&file);
            let executable_for_cpp = file.replace(".cpp","");
            let executable_for_cxx = file.replace(".cxx","");
            let executable_for_cppp = file.replace(".c++","");
            let executable_for_cc = file.replace(".cc","");
            let executable_for_cp = file.replace(".cp","");
            let option = Input::input()?;
            match option.trim() {
                "y" => {
                    std::fs::remove_file(&file)?;
                    if file.ends_with(".cpp") {
                        if Checker::checker(&executable_for_cpp,"file") {
                            std::fs::remove_file(&executable_for_cpp)?;
                            return Ok(());
                        }
                        return Ok(());
                    }
                    else if file.ends_with(".cxx") {
                        if Checker::checker(&executable_for_cxx,"file") {
                            std::fs::remove_file(&executable_for_cxx)?;
                            return Ok(());
                        }
                        return Ok(());
                    }
                    else if file.ends_with(".c++") {
                        if Checker::checker(&executable_for_cppp,"file") {
                            std::fs::remove_file(&executable_for_cppp)?;
                            return Ok(());
                        }
                        return Ok(());
                    }
                    else if file.ends_with(".cc") {
                        if Checker::checker(&executable_for_cc,"file") {
                            std::fs::remove_file(&executable_for_cc)?;
                            return Ok(());
                        }
                        return Ok(());
                    }
                    else if file.ends_with(".cp") {
                        if Checker::checker(&executable_for_cp,"file") {
                            std::fs::remove_file(&executable_for_cp)?;
                            return Ok(());
                        }
                        return Ok(());
                    }
                    return Ok(());
                }
                _=> {
                    println!("\n");
                }
            }
            return Ok(());
        }
        else if is_cpp && !Checker::checker(&file,"file") {
            Error::not_exist(&file,"file");
            return Ok(());
        }
        else if file.contains('.') {
            Error::invalid_extension("remove",".cpp .cxx .c++ .cc .cp","C++");
            return Ok(());
        }
        else {
            println!(r#"

            Which of these extensions?

            [1] {file}.cpp
            [2] {file}.cxx
            [3] {file}.c++
            [4] {file}.cc
            [5] {file}.cp

            "#);
            let option = Input::input()?;
            let file_cpp = format!("{file}.cpp");
            let file_cxx = format!("{file}.cxx");
            let file_cppp = format!("{file}.c++");
            let file_cc = format!("{file}.cc");
            let file_cp = format!("{file}.cp");
            match option.trim() {
                "1" => {
                    if Checker::checker(&file_cpp,"file") {
                        Error::warning_remove("file",&file_cpp);
                        let option = Input::input()?;
                        match option.trim() {
                            "y" => {
                                std::fs::remove_file(&file_cpp)?;
                                if Checker::checker(&file,"file") {
                                    std::fs::remove_file(&file)?;
                                    return Ok(());
                                }
                                return Ok(());
                            }
                            _=> {
                                println!("\n");
                            }
                        }
                        return Ok(());
                    }
                    else if !Checker::checker(&file_cpp,"file") {
                        Error::not_exist(&file_cpp,"file");
                        return Ok(());
                    }
                    return Ok(());
                }
                "2" => {
                    if Checker::checker(&file_cxx,"file") {
                        Error::warning_remove("file",&file_cxx);
                        let option = Input::input()?;
                        match option.trim() {
                            "y" => {
                                std::fs::remove_file(&file_cxx)?;
                                if Checker::checker(&file,"file") {
                                    std::fs::remove_file(&file)?;
                                    return Ok(());
                                }
                                return Ok(());
                            }
                            _=> {
                                println!("\n");
                            }
                        }
                        return Ok(());
                    }
                    else if !Checker::checker(&file_cxx,"file") {
                        Error::not_exist(&file_cxx,"file");
                        return Ok(());
                    }
                    return Ok(());
                }
                "3" => {
                    if Checker::checker(&file_cppp,"file") {
                        Error::warning_remove("file",&file_cppp);
                        let option = Input::input()?;
                        match option.trim() {
                            "y" => {
                                std::fs::remove_file(&file_cppp)?;
                                if Checker::checker(&file,"file") {
                                    std::fs::remove_file(&file)?;
                                    return Ok(());
                                }
                                return Ok(());
                            }
                            _=> {
                                println!("\n");
                            }
                        }
                        return Ok(());
                    }
                        else if !Checker::checker(&file_cppp,"file") {
                            Error::not_exist(&file_cppp,"file");
                            return Ok(());
                        }
                        return Ok(());
                    }
                "4" => {
                    if Checker::checker(&file_cc,"file") {
                        Error::warning_remove("file",&file_cc);
                        let option = Input::input()?;
                        match option.trim() {
                            "y" => {
                                std::fs::remove_file(&file_cc)?;
                                if Checker::checker(&file,"file") {
                                    std::fs::remove_file(&file)?;
                                    return Ok(());
                                }
                                return Ok(());
                            }
                            _=> {
                                println!("\n");
                            }
                        }
                        return Ok(());
                    }
                    else if !Checker::checker(&file_cc,"file") {
                        Error::not_exist(&file_cc,"file");
                        return Ok(());
                    }
                    return Ok(());
                }
                "5" => {
                    if Checker::checker(&file_cp,"file") {
                        Error::warning_remove("file",&file_cp);
                        let option = Input::input()?;
                        match option.trim() {
                            "y" => {
                                std::fs::remove_file(&file_cp)?;
                                if Checker::checker(&file,"file") {
                                    std::fs::remove_file(&file)?;
                                    return Ok(());
                                }
                                return Ok(());
                            }
                            _=> {
                                println!("\n");
                            }
                        }
                        return Ok(());
                    }
                    else if !Checker::checker(&file_cp,"file") {
                        Error::not_exist(&file_cp,"file");
                        return Ok(());
                    }
                    return Ok(());
                }
                _=> {
                    println!("{BOLD}{RED}error{RESET}{BOLD}: not a valid option{RESET}");
                    return Ok(());
                }
            }
        }
    }
    fn remove_file() -> std::io::Result<()> {
        let languages = ["[1] Rust (.rs)","[2] C++ (.cpp) (.cxx) (.c++) (.cc) (.cp)","[3] C (.c)","[4] JavaScript","[5] Python (.py)","[6] Lua (.lua)"];
        for language in languages {
            println!("{language}");
        }
        let option = Input::input()?;
        match option.trim() {
            "1" => {
                Remove::remove_languages("Rust",".rs")?;
                return Ok(());
            }
            "2" => {
                Remove::remove_cpp()?;
                return Ok(());
            }
            "3" => {
                Remove::remove_languages("C",".c")?;
                return Ok(());
            }
            "4" => {
                Remove::remove_languages("JavaScript",".js")?;
                return Ok(());
            }
            "5" => {
                Remove::remove_languages("Python",".py")?;
                return Ok(());
            }
            "6" => {
                Remove::remove_languages("Lua",".lua")?;
                return Ok(());
            }
            _=> {
                println!("{BOLD}{RED}error{RESET}{BOLD}: not a valid option{RESET}\n");
                return Ok(());
            }
        }
    }
    fn remove_dir() -> std::io::Result<()> {
        println!("\ntype the folder name");
        let folder = Input::input()?;
        let space_invalid_characters = InvalidCharacter::invalid_characters(&folder,"dir") && folder.contains(' ');
        if folder.is_empty() {
            Error::empty("Projects");
            return Ok(());
        }
        else if space_invalid_characters {
            Error::spaces(&folder,"folder");
            Error::invalid_characters(&folder,"folder");
            println!("{BOLD}2 Errors detected{RESET}\n");
            return Ok(());
        }
        else if InvalidCharacter::invalid_characters(&folder,"dir") {
            Error::invalid_characters(&folder,"folder");
            return Ok(());
        }
        else if folder.contains(' ') {
            Error::spaces(&folder,"folder");
            return Ok(());
        }
        else if folder == "src" || folder == "target" || folder == ".git" {
            println!("{BOLD}{RED}error{RESET}{BOLD}: OPERATIOND BLOCKED{RESET}");
            println!("\n{BOLD}Cannot remove the folder, It is essential for the system to function; without it, the terminal does not work{RESET}");
                    return Ok(());
        }
        else if Checker::checker(&folder,"dir") {
            Error::warning_remove("folder",&folder);
            let option = Input::input()?;
            match option.trim() {
                "y" => {
                    std::fs::remove_dir_all(&folder)?;
                }
                _=> {
                    println!("\n");
                }
            }
            return Ok(());
        }
        else if !Checker::checker(&folder,"dir") {
            Error::not_exist(&folder,"file");
            return Ok(());
        }
        return Ok(());
    }
    fn remove() -> std::io::Result<()> {
        println!("[1] File");
        println!("[2] Folder");
        let option = Input::input()?;
        match option.trim() {
            "1" => {
                Remove::remove_file()?;
                return Ok(());
            }
            "2" => {
                Remove::remove_dir()?;
                return Ok(());
            }
            _=> {
                println!("{BOLD}{RED}error{RESET}{BOLD}: not a valid option{RESET}\n");
                return Ok(());
            }
        }
    }
}
impl Edits for Edit {
    fn edit_languages(language: &str,extension: &str) -> std::io::Result<()> {
        println!("\ntype the file name");
        let file = Input::input()?;
        let space_invalid_characters = InvalidCharacter::invalid_characters(&file,"file") && file.contains(' ');
        let expected = format!("file{extension}");
        let file_type = format!("{file}{extension}");
        if file.is_empty() {
            Error::empty(&expected);
            return Ok(());
        }
        else if space_invalid_characters {
            Error::spaces(&file,"file");
            Error::invalid_characters(&file,"file");
            println!("{BOLD}2 Errors detected{RESET}\n");
            return Ok(());
        }
        else if InvalidCharacter::invalid_characters(&file,"file") {
            Error::invalid_characters(&file,"file");
            return Ok(());
        }
        else if file.contains(' ') {
            Error::spaces(&file,"file");
            return Ok(());
        }
        else if file.ends_with(extension) && Checker::checker(&file,"file") {
            OpenEditor::open_editor(&file)?;
            return Ok(());
        }
        else if file.ends_with(extension) && !Checker::checker(&file,"file") {
            println!("\n{BOLD}{YELLOW}warning{RESET}{BOLD}: the file does not exist, want to edit a empty file? [y/n]");
            let option = Input::input()?;
            match option.trim() {
                "y" => {
                    OpenEditor::open_editor(&file)?;
                }
                _=> {
                    println!("\n");
                }
            }
            return Ok(());
        }
        else if file.contains('.') {
            Error::invalid_extension("edit",&extension,&language);
            return Ok(());
        }
        else if Checker::checker(&file_type,"file") {
            OpenEditor::open_editor(&file_type)?;
            return Ok(());
        }
        else if !Checker::checker(&file_type,"file") {
            println!("\n{BOLD}{YELLOW}warning{RESET}{BOLD}: the file does not exist, want to edit a empty file? [y/n]");
            let option = Input::input()?;
            match option.trim() {
                "y" => {
                    OpenEditor::open_editor(&file_type)?;
                }
                _=> {
                    println!("\n");
                }
            }
            return Ok(());
        }
        return Ok(());
    }
    fn edit_cpp() -> std::io::Result<()> {
        println!("\ntype the file name");
        let file = Input::input()?;
        let is_cpp = file.ends_with(".cpp") || file.ends_with(".cxx") || file.ends_with(".c++") || file.ends_with(".cc") || file.ends_with(".cp");
        let space_invalid_characters = InvalidCharacter::invalid_characters(&file,"file") && file.contains(' ');
        if file.is_empty() {
            Error::empty("file.cpp");
            return Ok(());
        }
        else if space_invalid_characters {
            Error::spaces(&file,"file");
            Error::invalid_characters(&file,"file");
            println!("{BOLD}2 Errors detected{RESET}\n");
            return Ok(());
        }
        else if InvalidCharacter::invalid_characters(&file,"file") {
            Error::invalid_characters(&file,"file");
            return Ok(());
        }
        else if file.contains(' ') {
            Error::spaces(&file,"file");
            return Ok(());
        }
        else if is_cpp && Checker::checker(&file,"file") {
            OpenEditor::open_editor(&file)?;
            return Ok(());
        }
        else if is_cpp && !Checker::checker(&file,"file") {
            println!("\n{BOLD}{YELLOW}warning{RESET}{BOLD}: the file does no     t exist, want to edit a empty file? [y/n]");
            let option = Input::input()?;
            match option.trim() {
                "y" => {
                    OpenEditor::open_editor(&file)?;
                }
                _=> {
                    println!("\n");
                }
            }
            return Ok(());
        }
        else if file.contains('.') {
            Error::invalid_extension("edit",".cpp .cxx .c++ .cc .cp","C++");
            return Ok(());
        }
        else {
            println!(r#"

            Which of these extensions?

            [1] {file}.cpp
            [2] {file}.cxx
            [3] {file}.c++
            [4] {file}.cc
            [5] {file}.cp

            "#);
            let option = Input::input()?;
            let file_cpp = format!("{file}.cpp");
            let file_cxx = format!("{file}.cxx");
            let file_cppp = format!("{file}.c++");
            let file_cc = format!("{file}.cc");
            let file_cp = format!("{file}.cp");
            match option.trim() {
                "1" => {
                    if Checker::checker(&file_cpp,"file") {
                        OpenEditor::open_editor(&file_cpp)?;
                        return Ok(());
                    }
                    else if !Checker::checker(&file_cpp,"file") {
                    println!("\n{BOLD}{YELLOW}warning{RESET}{BOLD}: the file does not exist, want to edit a empty file? [y/n]");
                    let option = Input::input()?;
                    match option.trim() {
                        "y" => {
                            OpenEditor::open_editor(&file_cpp)?;
                        }
                        _=> {
                            println!("\n");
                        }
                    }
                    return Ok(());
                    }
                    return Ok(());
                }
                "2" => {
                    if Checker::checker(&file_cxx,"file") {
                        OpenEditor::open_editor(&file_cxx)?;
                        return Ok(());
                    }
                    else if !Checker::checker(&file_cxx,"file") {
                        println!("\n{BOLD}{YELLOW}warning{RESET}{BOLD}: the file does not exist, want to edit a empty file? [y/n]");
                        let option = Input::input()?;
                        match option.trim() {
                            "y" => {
                                OpenEditor::open_editor(&file_cxx)?;
                            }
                            _=> {
                                println!("\n");
                            }
                        }
                        return Ok(());
                    }
                    return Ok(());
                }
                "3" => {
                    if Checker::checker(&file_cppp,"file") {
                        OpenEditor::open_editor(&file_cppp)?;
                        return Ok(());
                    }
                    else if !Checker::checker(&file_cppp,"file") {
                        println!("\n{BOLD}{YELLOW}warning{RESET}{BOLD}: the file does not exist, want to edit a empty file? [y/n]");
                        let option = Input::input()?;
                        match option.trim() {
                            "y" => {
                                OpenEditor::open_editor(&file_cppp)?;
                            }
                            _=> {
                                println!("\n");
                            }
                        }
                        return Ok(());
                    }
                    return Ok(());
                }
                "4" => {
                    if Checker::checker(&file_cc,"file") {
                        OpenEditor::open_editor(&file_cc)?;
                        return Ok(());
                    }
                    else if !Checker::checker(&file_cc,"file") {
                        println!("\n{BOLD}{YELLOW}warning{RESET}{BOLD}: the file does not exist, want to edit a empty file? [y/n]");
                        let option = Input::input()?;
                        match option.trim() {
                            "y" => {
                                OpenEditor::open_editor(&file_cc)?;
                            }
                            _=> {
                                println!("\n");
                            }
                        }
                        return Ok(());
                    }
                    return Ok(());
                }
                "5" => {
                    if Checker::checker(&file_cp,"file") {
                        OpenEditor::open_editor(&file_cp)?;
                        return Ok(());
                    }
                    else if !Checker::checker(&file_cp,"file") {
                        println!("\n{BOLD}{YELLOW}warning{RESET}{BOLD}: the file does not exist, want to edit a empty file? [y/n]");
                        let option = Input::input()?;
                        match option.trim() {
                            "y" => {
                                OpenEditor::open_editor(&file_cp)?;
                            }
                            _=> {
                                println!("\n");
                            }
                        }
                        return Ok(());
                    }
                    return Ok(());
                }
                _=> {
                    println!("{BOLD}{RED}error{RESET}{BOLD}: Not a valid option{RESET}\n");
                    return Ok(());
                }
            }
        }
    }
    fn edit_file() -> std::io::Result<()> {
        let languages = ["[1] Rust (.rs)","[2] C++ (.cpp) (.cxx) (.c++) (.cc) (.cp)","[3] C (.c)","[4] JavaScript (.js)","[5] Python (.py)","[6] Lua (.lua)"];
        for language in languages {
            println!("{language}");
        }
        let option = Input::input()?;
        match option.trim() {
            "1" => {
                Edit::edit_languages("Rust",".rs")?;
                return Ok(());
            }
            "2" => {
                Edit::edit_cpp()?;
                return Ok(());
            }
            "3" => {
                Edit::edit_languages("C",".c")?;
                return Ok(());
            }
            "4" => {
                Edit::edit_languages("JavaScript",".js")?;
                return Ok(());
            }
            "5" => {
                Edit::edit_languages("Python",".py")?;
                return Ok(());
            }
            "6" => {
                Edit::edit_languages("Lua",".lua")?;
                return Ok(());
            }
            _=> {
                println!("\n{BOLD}{RED}error{RESET}{BOLD}: not a valid option{RESET}\n");
                return Ok(());
            }
        }
    }
}
impl Copys for Copy {
    fn copy_languages(language: &str,extension: &str) -> std::io::Result<()> {
        println!("\ntype the file name");
        let file = Input::input()?;
        let space_invalid_characters = InvalidCharacter::invalid_characters(&file,"file") && file.contains(' ');
        let expected = format!("file{extension}");
        let file_type = format!("{file}{extension}");
        if file.is_empty() {
            Error::empty(&expected);
            return Ok(());
        }
        else if space_invalid_characters {
            Error::spaces(&file,"file");
            Error::invalid_characters(&file,"file");
            println!("{BOLD}2 Errors detected{RESET}\n");
            return Ok(());
        }
        else if InvalidCharacter::invalid_characters(&file,"file") {
            Error::invalid_characters(&file,"file");
            return Ok(());
        }
        else if file.contains(' ') {
            Error::spaces(&file,"file");
            return Ok(());
        }
        else if file.ends_with(extension) && Checker::checker(&file,"file")  {
            println!("\ntype another name to the cloned file");
            let copy_file = Input::input()?;
            let space_invalid_characters_copy = InvalidCharacter::invalid_characters(&copy_file,"file") && copy_file.contains(' ');
            let copy_type = format!("{copy_file}{extension}");
            let expected2 = format!("file2{extension}");
            if copy_file.is_empty() {
                Error::empty(&expected2);
                return Ok(());
            }
            else if space_invalid_characters_copy {
                Error::spaces(&copy_file,"file");
                Error::invalid_characters(&copy_file,"file");
                println!("{BOLD}2 Errors detected{RESET}\n");
                return Ok(());
            }
            else if InvalidCharacter::invalid_characters(&copy_file,"file") {
                Error::invalid_characters(&copy_file,"file");
                return Ok(());
            }
            else if copy_file.contains(' ') {
                Error::spaces(&copy_file,"file");
                return Ok(());
            }
            else if copy_file.ends_with(extension) && Checker::checker(&copy_file,"file") {
                println!("\n{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_file}\"{RESET}");
                println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                println!("{BOLD}Enter a name that does not exist{RESET}");
                return Ok(());
            }
            else if copy_file.ends_with(extension) && !Checker::checker(&copy_file,"file") {
                std::fs::copy(&file,&copy_file)?;
                return Ok(());
            }
            else if copy_file.contains('.') {
                Error::invalid_extension("copy",extension,language);
                return Ok(());
            }
            else if Checker::checker(&copy_type,"file") {
                println!("\n{BOLD}{RED}error{RESET}{BOLD}: already exists a      file with this name \"{copy_file}\"{RESET}");
                println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                println!("{BOLD}Enter a name that does not exist{RESET}");
                return Ok(());
            }
            else if !Checker::checker(&copy_type,"file") {
                std::fs::copy(&file,&copy_type)?;
                return Ok(());
            }
            return Ok(());
        }
        else if file.ends_with(extension) && !Checker::checker(&file,"file") {
            Error::not_exist(&file,"file");
            return Ok(());
        }
        else if file.contains('.') {
            Error::invalid_extension("copy",extension,language);
            return Ok(());
        }
        else if Checker::checker(&file_type,"file") {
            println!("\ntype another name to the cloned file");
            let copy_file = Input::input()?;
            let space_invalid_characters_copy = InvalidCharacter::invalid_characters(&copy_file,"file") && copy_file.contains(' ');
            let copy_type = format!("{copy_file}{extension}");
            let expected2 = format!("file2{extension}");
            if copy_file.is_empty() {
                Error::empty(&expected2);
                return Ok(());
            }
            else if space_invalid_characters_copy {
                Error::spaces(&copy_file,"file");
                Error::invalid_characters(&copy_file,"file");
                println!("{BOLD}2 Errors detected{RESET}\n");
                return Ok(());
            }
            else if InvalidCharacter::invalid_characters(&copy_file,"file") {
                Error::invalid_characters(&copy_file,"file");
                return Ok(());
            }
            else if copy_file.contains(' ') {
                Error::spaces(&copy_file,"file");
                return Ok(());
            }
            else if copy_file.ends_with(extension) && Checker::checker(&copy_file,"file") {
                   println!("\n{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_file}\"{RESET}");
                   println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                   println!("{BOLD}Enter a name that does not exist{RESET}");
                   return Ok(());
            }
            else if copy_file.ends_with(extension) && !Checker::checker(&copy_file,"file") {
                std::fs::copy(&file_type,&copy_file)?;
                return Ok(());
            }
            else if copy_file.contains('.') {
                Error::invalid_extension("copy",extension,language);
                return Ok(());
            }
            else if Checker::checker(&copy_type,"file") {
                println!("\n{BOLD}{RED}error{RESET}{BOLD}: already exists      a file with this name \"{copy_file}\"{RESET}");
                println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                println!("{BOLD}Enter a name that does not exist{RESET}");
                return Ok(());
            }
            else if !Checker::checker(&copy_type,"file") {
                std::fs::copy(&file_type,&copy_type)?;
                return Ok(());
            }
            return Ok(());
        }
        else if !Checker::checker(&file_type,"file") {
            Error::not_exist(&file_type,"file");
            return Ok(());
        }
        return Ok(());
    }
    fn copy_cpp() -> std::io::Result<()> {
        println!("\ntype the file name");
        let file = Input::input()?;
        let space_invalid_characters = InvalidCharacter::invalid_characters(&file,"file") && file.contains(' ');
        let is_cpp = file.ends_with(".cpp") || file.ends_with(".cxx") || file.ends_with(".c++") || file.ends_with(".cc") || file.ends_with(".cp");
        if file.is_empty() {
            Error::empty("file.cpp");
            return Ok(());
        }
        else if space_invalid_characters {
            Error::spaces(&file,"file");
            Error::invalid_characters(&file,"file");
            println!("{BOLD}2 Errors detected{RESET}\n");
            return Ok(());
        }
        else if InvalidCharacter::invalid_characters(&file,"file") {
            Error::invalid_characters(&file,"file");
            return Ok(());
        }
        else if file.contains(' ') {
            Error::spaces(&file,"file");
            return Ok(());
        }
        else if is_cpp && Checker::checker(&file,"file") {
            println!("\ntype another name to the cloned file");
            let copy_file = Input::input()?;
            let space_invalid_characters_copy = InvalidCharacter::invalid_characters(&copy_file,"file") && copy_file.contains(' ');
            let is_cpp_copy = copy_file.ends_with(".cpp") || copy_file.ends_with(".cxx") || copy_file.ends_with(".c++") || copy_file.ends_with(".cc") || copy_file.ends_with(".cp");
            if copy_file.is_empty() {
                Error::empty("file2.cpp");
                return Ok(());
            }
            else if space_invalid_characters_copy {
                Error::spaces(&copy_file,"file");
                Error::invalid_characters(&copy_file,"file");
                println!("{BOLD}2 Errors detected{RESET}\n");
                return Ok(());
            }
            else if InvalidCharacter::invalid_characters(&copy_file,"file") {
                Error::invalid_characters(&copy_file,"file");
                return Ok(());
            }
            else if file.contains(' ') {
                Error::spaces(&copy_file,"file");
                return Ok(());
            }
            else if is_cpp_copy && Checker::checker(&copy_file,"file") {
                println!("\n{BOLD}{RED}error{RESET}{BOLD}: already exists           a file with this name \"{copy_file}\"{RESET}");
                println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                println!("{BOLD}Enter a name that does not exist{RESET}");
                return Ok(());
            }
            else if is_cpp_copy && !Checker::checker(&copy_file,"file") {
                std::fs::copy(&file,&copy_file)?;
                return Ok(());
            }
            else if copy_file.contains('.') {
                Error::invalid_extension("copy",".cpp .cxx .c++ .cc .cp","C++");
                return Ok(());
            }
            else {
                println!(r#"

                Which of these extensions? 

                [1] {copy_file}.cpp
                [2] {copy_file}.cxx
                [3] {copy_file}.c++
                [4] {copy_file}.cc
                [5] {copy_file}.cp

                "#);
                let option = Input::input()?;
                let copy_cpp = format!("{copy_file}.cpp");
                let copy_cxx = format!("{copy_file}.cxx");
                let copy_cppp = format!("{copy_file}.c++");
                let copy_cc = format!("{copy_file}.cc");
                let copy_cp = format!("{copy_file}.cp");
                match option.trim() {
                    "1" => {
                        if Checker::checker(&copy_cpp,"file") {
                            println!("\n{BOLD}{RED}error{RESET}{BOLD}: already exists           a file with this name \"{copy_cpp}\"{RESET}");
                            println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                            println!("{BOLD}Enter a name that does not exist{RESET}");
                            return Ok(());
                        }
                        else if !Checker::checker(&copy_cpp,"file") {
                            std::fs::copy(&file,&copy_cpp)?;
                            return Ok(());
                        }
                        return Ok(());
                    }
                    "2" => {
                        if Checker::checker(&copy_cxx,"file") {
                            println!("\n{BOLD}{RED}error{RESET}{BOLD}: already exists           a file with this name \"{copy_cxx}\"{RESET}");
                            println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                            println!("{BOLD}Enter a name that does not exist{RESET}");
                            return Ok(());
                        }
                        else if !Checker::checker(&copy_cxx,"file") {
                            std::fs::copy(&file,&copy_cxx)?;
                            return Ok(());
                        }
                        return Ok(());
                    }
                    "3" => {
                        if Checker::checker(&copy_cppp,"file") {
                            println!("\n{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_cppp}\"{RESET}");
                            println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                            println!("{BOLD}Enter a name that does not exist{RESET}"); 
                            return Ok(());
                        }
                        else if !Checker::checker(&copy_cppp,"file") {
                            std::fs::copy(&file,&copy_cppp)?;
                            return Ok(());
                        }
                        return Ok(());
                    }
                    "4" => {
                        if Checker::checker(&copy_cc,"file") {
                            println!("\n{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_cc}\"{RESET}");
                            println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                            println!("{BOLD}Enter a name that does not exist{RESET}");
                            return Ok(());
                        }
                        else if !Checker::checker(&copy_cc,"file") {
                            std::fs::copy(&file,&copy_cc)?;
                            return Ok(());
                        }
                        return Ok(());
                    }
                    "5" => {
                        if Checker::checker(&copy_cp,"file") {
                            println!("\n{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_cp}\"{RESET}");
                            println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                            println!("{BOLD}Enter a name that does not exist{RESET}");
                            return Ok(());
                        }
                        else if !Checker::checker(&copy_cp,"file") {
                            std::fs::copy(&file,&copy_cp)?;
                            return Ok(());
                        }
                        return Ok(());
                    }
                    _=> {
                        println!("\n{BOLD}{RED}error{RESET}{BOLD}: Not a valid option{RESET}\n");
                        return Ok(());
                    }
                }
            }
        }
        else if is_cpp && !Checker::checker(&file,"file") {
            Error::not_exist(&file,"file");
            return Ok(());
        }
        else if file.contains('.') {
            Error::invalid_extension("copy",".cpp .cxx .c++ .cc .cp","C++");
            return Ok(());
        }
        else {
            println!(r#"

            Which of these extensions?

            [1] {file}.cpp
            [2] {file}.cxx
            [3] {file}.c++
            [4] {file}.cc
            [5] {file}.cp

            "#);
            let option = Input::input()?;
            let file_cppp = format!("{file}.c++");
            let file_cpp = format!("{file}.cpp");
            let file_cxx = format!("{file}.cxx");
            let file_cc = format!("{file}.cc");
            let file_cp = format!("{file}.cp");
            match option.trim() {
                "1" => {
                    if Checker::checker(&file_cpp,"file") {
                        println!("\ntype another name to the cloned file");
                        let copy_file = Input::input()?;
                        let space_invalid_characters_copy = InvalidCharacter::invalid_characters(&copy_file,"file") && copy_file.contains(' ');
                        let is_cpp_copy = copy_file.ends_with(".cpp") || copy_file.ends_with(".cxx") || copy_file.ends_with(".c++") || copy_file.ends_with(".cc") || copy_file.ends_with(".cp");
                        if copy_file.is_empty() {
    Error::empty("file2.cpp");
    return Ok(());

}
else if space_invalid_characters_copy {
    Error::spaces(&file,"file");
    Error::invalid_characters(&file,"file");
    println!("{BOLD}2 Errors detected{RESET}\n");
    return Ok(());
}
else if InvalidCharacter::invalid_characters(&file,"file") {
    Error::invalid_characters(&file,"file");
    return Ok(());
}
else if copy_file.contains(' ') {
    Error::spaces(&file,"file");
    return Ok(());
}
else if is_cpp_copy && Checker::checker(&copy_file,"file") {
    println!("\n{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_file}\"{RESET}");
    println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
    println!("{BOLD}Enter a name that does not exist{RESET}");
    return Ok(());
}
else if is_cpp_copy && !Checker::checker(&copy_file,"file") {
    std::fs::copy(&file_cpp,&copy_file)?;
    return Ok(());
}
else if copy_file.contains('.') {
    Error::invalid_extension("copy",".cpp .cxx .c++ .cc .cp","C++");
    return Ok(());
}
else {
    println!(r#"

    Which of these extensions?

    [1] {copy_file}.cpp
    [2] {copy_file}.cxx
    [3] {copy_file}.c++
    [4] {copy_file}.cc
    [5] {copy_file}.cp

    "#);
    let option = Input::input()?;
    let copy_cpp = format!("{copy_file}.cpp");
    let copy_cxx = format!("{copy_file}.cxx");
    let copy_cppp = format!("{copy_file}.c++");
    let copy_cc = format!("{copy_file}.cc");
    let copy_cp = format!("{copy_file}.cp");
    match option.trim() {
        "1" => {
            if Checker::checker(&copy_cpp,"file") {
                println!("\n{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_cpp}\"{RESET}");
                println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                println!("{BOLD}Enter a name that does not exist{RESET}");
                return Ok(());
            }
            else if !Checker::checker(&copy_cpp,"file") {
                std::fs::copy(&file_cpp,&copy_cpp)?;
                return Ok(());
            }
            return Ok(());
        }
        "2" => {
            if Checker::checker(&copy_cxx,"file") {
                println!("\n{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_cxx}\"{RESET}");
                println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                println!("{BOLD}Enter a name that does not exist{RESET}");
                return Ok(());
            }
            else if !Checker::checker(&copy_cxx,"file") {
                std::fs::copy(&file_cpp,&copy_cxx)?;
                return Ok(());
            }
            return Ok(());
        }
        "3" => {
            if Checker::checker(&copy_cppp,"file") {
                println!("\n{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_cppp}\"{RESET}");
                println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                println!("{BOLD}Enter the name that does not exist{RESET}");
                return Ok(());
            }
            else if !Checker::checker(&copy_cppp,"file") {
                std::fs::copy(&file_cpp,&copy_cppp)?;
                return Ok(());
            }
            return Ok(());
        }
        "4" => {
            if Checker::checker(&copy_cc,"file") {
                println!("\n{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_cc}\"{RESET}");
                println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                println!("{BOLD}Enter the name that does not exist{RESET}");
                return Ok(());
            }
            else if !Checker::checker(&copy_cc,"file") {
                std::fs::copy(&file_cpp,&copy_cc)?;
                return Ok(());
            }
            return Ok(());
        }
        "5" => {
            if Checker::checker(&copy_cp,"file") {
                println!("\n{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_cp}\"{RESET}");
                println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                println!("{BOLD}Enter the name that does not exist{RESET}");
                return Ok(());
            }
            else if !Checker::checker(&copy_cp,"file") {
                std::fs::copy(&file_cpp,&copy_cp)?;
                return Ok(());
            }
            return Ok(());
        }
        _=> {
            println!("{BOLD}{RED}error{RESET}{BOLD}: Not a valid option{RESET}");
            return Ok(());
        }
    }
}
}
else if !Checker::checker(&file_cpp,"file") {
    Error::not_exist(&file_cpp,"file");
    return Ok(());
}
return Ok(());
}
"2" => {
    if Checker::checker(&file_cxx,"file") {
        println!("\ntype another name to the cloned file");
        let copy_file = Input::input()?;
        let space_invalid_characters_copy = InvalidCharacter::invalid_characters(&copy_file,"file") && copy_file.contains(' ');
        let is_cpp_copy = copy_file.ends_with(".cpp") || copy_file.ends_with(".cxx") || copy_file.ends_with(".c++") || copy_file.ends_with(".cc") || copy_file.ends_with(".cp");
        if copy_file.is_empty() {
            Error::empty("file2.cpp");
            return Ok(());
        }
        else if space_invalid_characters_copy {
            Error::spaces(&copy_file,"file");
            Error::invalid_characters(&copy_file,"file");
            println!("{BOLD}2 Errors detected{RESET}\n");
            return Ok(());
        }
        else if InvalidCharacter::invalid_characters(&copy_file,"file") {
            Error::invalid_characters(&copy_file,"file");
            return Ok(());
        }
        else if copy_file.contains(' ') {
            Error::spaces(&copy_file,"file");
            return Ok(());
        }
        else if is_cpp_copy && Checker::checker(&copy_file,"file") {
            println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_file}\"{RESET}");
            println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
            println!("{BOLD}Enter a name that does not exist{RESET}\n");
            return Ok(());
        }
        else if is_cpp_copy && !Checker::checker(&copy_file,"file") {
            std::fs::copy(&file_cxx,&copy_file)?;
            return Ok(());
        }
        else if copy_file.contains('.') {
            Error::invalid_extension("copy",".cpp .cxx .c++ .cc .cp","C++");
            return Ok(());
        }
        else {
            println!(r#"

            Which of these extensions?

            [1] {copy_file}.cpp
            [2] {copy_file}.cxx
            [3] {copy_file}.c++
            [4] {copy_file}.cc
            [5] {copy_file}.cp

            "#);
            let option = Input::input()?;
            let copy_cpp = format!("{copy_file}.cpp");
            let copy_cxx = format!("{copy_file}.cxx");
            let copy_cppp = format!("{copy_file}.c++");
            let copy_cc = format!("{copy_file}.cc");
            let copy_cp = format!("{copy_file}.cp");
            match option.trim() {
                "1" => {
                    if Checker::checker(&copy_cpp,"file") {
                        println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_cpp}\"{RESET}");
                        println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                        println!("{BOLD}Enter a name that does not exist{RESET}\n");
                        return Ok(());
                    }
                    else if !Checker::checker(&copy_cpp,"file") {
                        std::fs::copy(&file_cxx,&copy_cpp)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                "2" => {
                    if Checker::checker(&copy_cxx,"file") {
                        println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_cxx}\"{RESET}");
                        println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                        println!("{BOLD}Enter a name that does not exist{RESET}\n");
                        return Ok(());
                    }
                    else if !Checker::checker(&copy_cxx,"file") {
                        std::fs::copy(&file_cxx,&copy_cxx)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                "3" => {
                    if Checker::checker(&copy_cppp,"file") {
                        println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_cppp}\"{RESET}");
                        println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                        println!("{BOLD}Enter a name that does not exist{RESET}\n");
                        return Ok(());
                    }
                    else if !Checker::checker(&copy_cppp,"file") {
                        std::fs::copy(&file_cxx,&copy_cppp)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                "4" => {
                    if Checker::checker(&copy_cc,"file") {
                        println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_cc}\"{RESET}");
                        println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                        println!("{BOLD}Enter a name that does not exist{RESET}\n");
                        return Ok(());
                    }
                    else if !Checker::checker(&copy_cc,"file") {
                        std::fs::copy(&file_cxx,&copy_cc)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                "5" => {
                    if Checker::checker(&copy_cp,"file") {
                    println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_cc}\"{RESET}");
                    println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                    println!("{BOLD}Enter a name that does not exist{RESET}\n");
                    return Ok(());
                    }
                    else if !Checker::checker(&copy_cp, "file") {
                        std::fs::copy(&file_cxx,&copy_cp)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                _=> {
                    println!("{BOLD}{RED}error{RESET}{BOLD}: not a valid option{RESET}\n");
                    return Ok(());
                }
            }
        }
    }
    else if !Checker::checker(&file_cxx,"file") {
        Error::not_exist(&file_cxx,"file");
        return Ok(());
    }
    return Ok(());
}
"3" => {
    if Checker::checker(&file_cppp,"file") {
        println!("\ntype another name to the cloned file");
        let copy_file = Input::input()?;
        let is_cpp_copy = copy_file.ends_with(".cpp") || copy_file.ends_with(".cxx") || copy_file.ends_with(".c++") || copy_file.ends_with(".cc") || copy_file.ends_with(".cp");
        let space_invalid_characters_copy = InvalidCharacter::invalid_characters(&copy_file,"file") && copy_file.contains(' ');
        if copy_file.is_empty() {
            Error::empty("file2.cpp");
            return Ok(());
        }
        else if space_invalid_characters_copy {
            Error::spaces(&copy_file,"file");
            Error::invalid_characters(&copy_file,"file");
            println!("{BOLD}2 Errors detected{RESET}\n");
            return Ok(());
        }
        else if InvalidCharacter::invalid_characters(&copy_file,"file") {
            Error::invalid_characters(&copy_file,"file");
            return Ok(());
        }
        else if copy_file.contains(' ') {
            Error::spaces(&copy_file,"file");
            return Ok(());
        }
        else if is_cpp_copy && Checker::checker(&copy_file,"file") {
            println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_file}\"{RESET}");
            println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
            println!("{BOLD}Enter a name that does not exist{RESET}\n");
            return Ok(());
        }
        else if is_cpp_copy && !Checker::checker(&copy_file,"file") {
            std::fs::copy(&file_cppp,&copy_file)?;
            return Ok(());
        }
        else if copy_file.contains('.') {
            Error::invalid_extension("copy",".cpp .cxx .c++ .cc .cp","C++");
            return Ok(());
        }
        else {
            println!(r#"

            Which of these extensions? 

            [1] {copy_file}.cpp
            [2] {copy_file}.cxx
            [3] {copy_file}.c++
            [4] {copy_file}.cc
            [5] {copy_file}.cp 

            "#);
            let option = Input::input()?;
            let copy_cpp = format!("{copy_file}.cpp");
            let copy_cxx = format!("{copy_file}.cxx");
            let copy_cppp = format!("{copy_file}.c++");
            let copy_cc = format!("{copy_file}.cc");
            let copy_cp = format!("{copy_file}.cp");
            match option.trim() {
                "1" => {
                    if Checker::checker(&copy_cpp,"file") {
                    println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_cpp}\"{RESET}");
                    println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                    println!("{BOLD}Enter a name that does not exist{RESET}\n");
                    return Ok(());
                    }
                    else if !Checker::checker(&copy_cpp,"file") {
                        std::fs::copy(&file_cppp,&copy_cpp)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                "2" => {
                    if Checker::checker(&copy_cxx,"file") {
                              println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_file}\"{RESET}");
                              println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                              println!("{BOLD}Enter a name that does not exist{RESET}\n");
                        return Ok(());
                    }
                    else if !Checker::checker(&copy_cxx,"file") {
                        std::fs::copy(&file_cppp,&copy_cxx)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                "3" => {
                    if Checker::checker(&copy_cppp,"file") {
                              println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_file}\"{RESET}");
                              println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                              println!("{BOLD}Enter a name that does not exist{RESET}\n");
                        return Ok(());
                    }
                    else if !Checker::checker(&copy_cppp,"file") {
                        std::fs::copy(&file_cppp,&copy_cppp)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                "4" => {
                    if Checker::checker(&copy_cc,"file") {
                              println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_file}\"{RESET}");
                              println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                              println!("{BOLD}Enter a name that does not exist{RESET}\n");
                        return Ok(());
                    }
                    else if !Checker::checker(&copy_cc,"file") {
                        std::fs::copy(&file_cppp,&copy_cc)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                "5" => {
                    if Checker::checker(&copy_cp,"file") {
                              println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_file}\"{RESET}");
                              println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                              println!("{BOLD}Enter a name that does not exist{RESET}\n");
                        return Ok(());
                    }
                    else if !Checker::checker(&copy_cp,"file") {
                        std::fs::copy(&file_cppp,&copy_cp)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                _=> {
                    println!("{BOLD}{RED}error{RESET}{BOLD}: not a valid option{RESET}\n");
                    return Ok(());
                }
            }
        }
    }
    else if !Checker::checker(&file_cppp,"file") {
        Error::not_exist(&file_cppp,"file");
        return Ok(());
    }
    return Ok(());
}
"4" => {
    if Checker::checker(&file_cc,"file") {
        println!("\ntype another name to the cloned file");
        let copy_file = Input::input()?;
        let is_cpp_copy = copy_file.ends_with(".cpp") || copy_file.ends_with(".cxx") || copy_file.ends_with(".c++") || copy_file.ends_with(".cc") || copy_file.ends_with(".cp");
        let space_invalid_characters_copy = InvalidCharacter::invalid_characters(&copy_file,"file") && copy_file.contains(' ');
        if copy_file.is_empty() {
            Error::empty("file2.cpp");
            return Ok(());
        }
        else if space_invalid_characters_copy {
            Error::spaces(&copy_file,"file");
            Error::invalid_characters(&copy_file,"file");
            println!("{BOLD}2 Errors detected{RESET}\n");
            return Ok(());
        }
        else if InvalidCharacter::invalid_characters(&copy_file,"file") {
            Error::invalid_characters(&copy_file,"file");
            return Ok(());
        }
        else if copy_file.contains(' ') {
            Error::spaces(&copy_file,"file");
            return Ok(());
        }
        else if is_cpp_copy && Checker::checker(&copy_file,"file") {
            println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_file}\"{RESET}");
            println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
            println!("{BOLD}Enter a name that does not exist{RESET}\n");
            return Ok(());
        }
        else if is_cpp_copy && !Checker::checker(&copy_file,"file") {
            std::fs::copy(&file_cc,&copy_file)?;
            return Ok(());
        }
        else if copy_file.contains('.') {
            Error::invalid_extension("copy",".cpp .cxx .c++ .cc .cp","C++");
            return Ok(());
        }
        else {
            println!(r#"

            Which of these extensions?

            [1] {copy_file}.cpp
            [2] {copy_file}.cxx
            [3] {copy_file}.c++
            [4] {copy_file}.cc
            [5] {copy_file}.cp 

            "#);
            let option = Input::input()?;
            let copy_cpp = format!("{copy_file}.cpp");
            let copy_cxx = format!("{copy_file}.cxx");
            let copy_cppp = format!("{copy_file}.c++");
            let copy_cc = format!("{copy_file}.cc");
            let copy_cp = format!("{copy_file}.cp");
            match option.trim() {
                "1" => {
                    if Checker::checker(&copy_cpp,"file") {  
                      println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_cpp}\"{RESET}");
                      println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                      println!("{BOLD}Enter a name that does not exist{RESET}\n");
                      return Ok(());
                    }
                    else if !Checker::checker(&copy_cpp,"file") {
                        std::fs::copy(&file_cc,&copy_cpp)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                "2" => {
                    if Checker::checker(&copy_cxx,"file") {
                        println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_cxx}\"{RESET}");
                        println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                        println!("{BOLD}Enter a name that does not exist{RESET}\n");
                        return Ok(());
                    }
                    else if !Checker::checker(&copy_cxx,"file") {
                        std::fs::copy(&file_cc,&copy_cxx)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                "3" => {
                    if Checker::checker(&copy_cppp,"file") {
                        println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_file}\"{RESET}");
                        println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                        println!("{BOLD}Enter a name that does not exist{RESET}\n");
                        return Ok(());
                    }
                    else if !Checker::checker(&copy_cppp,"file") {
                        std::fs::copy(&file_cc,&copy_cppp)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                "4" => {
                    if Checker::checker(&copy_cc,"file") {
                        println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_file}\"{RESET}");
                        println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                        println!("{BOLD}Enter a name that does not exist{RESET}\n");
                        return Ok(());
                    }
                    else if !Checker::checker(&copy_cc,"file") {
                        std::fs::copy(&file_cc,&copy_cc)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                "5" => {
                    if Checker::checker(&copy_cp,"file") {
                        println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_file}\"{RESET}");
                        println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                        println!("{BOLD}Enter a name that does not exist{RESET}\n");
                        return Ok(());
                    }
                    else if !Checker::checker(&copy_cp,"file") {
                        std::fs::copy(&file_cc,&copy_cp)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                _=> {
                    println!("{BOLD}{RED}error{RESET}{BOLD}: Not a valid option{RESET}\n");
                    return Ok(());
                }
            }
        }
    }
    else if !Checker::checker(&file_cc,"file") {
        Error::not_exist(&file_cc,"file");
        return Ok(());
    }
    return Ok(());
}
"5" => {
    if Checker::checker(&file_cp,"file") {
        println!("\ntype another name to the cloned file");
        let copy_file = Input::input()?;
        let is_cpp_copy = copy_file.ends_with(".cpp") || copy_file.ends_with(".cxx") || copy_file.ends_with(".c++") || copy_file.ends_with(".cc") || copy_file.ends_with(".cp");
        let space_invalid_characters_copy = InvalidCharacter::invalid_characters(&copy_file,"file");
        if copy_file.is_empty() {
            Error::empty("file.cpp");
            return Ok(());
        }
        else if space_invalid_characters_copy {
            Error::spaces(&copy_file,"file");
            Error::invalid_characters(&copy_file,"file");
            println!("{BOLD}2 Errors detected{RESET}\n");
            return Ok(());
        }
        else if InvalidCharacter::invalid_characters(&copy_file,"file") {
            Error::invalid_characters(&copy_file,"file");
            return Ok(());
        }
        else if copy_file.contains(' ') {
            Error::spaces(&copy_file,"file");
            return Ok(());
        }
        else if is_cpp_copy && Checker::checker(&copy_file,"file") {
            println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_file}\"{RESET}");
            println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
            println!("{BOLD}Enter a name that does not exist{RESET}\n");
            return Ok(());
        }
        else if is_cpp_copy && !Checker::checker(&copy_file,"file") {
            std::fs::copy(&file_cp,&copy_file)?;
            return Ok(());
        }
        else if copy_file.contains('.') {
            Error::invalid_extension("copy",".cpp .cxx .c++ .cc .cp","C++");
            return Ok(());
        }
        else {
            println!(r#"

            Which of these extensions?

            [1] {copy_file}.cpp
            [2] {copy_file}.cxx
            [3] {copy_file}.c++
            [4] {copy_file}.cc
            [5] {copy_file}.cp 

            "#);
            let option = Input::input()?;
            let copy_cpp = format!("{copy_file}.cpp");
            let copy_cxx = format!("{copy_file}.cxx");
            let copy_cppp = format!("{copy_file}.c++");
            let copy_cc = format!("{copy_file}.cc");
            let copy_cp = format!("{copy_file}.cp");
            match option.trim() {
                "1" => {
                    if Checker::checker(&copy_cpp,"file") {
                        println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_file}\"{RESET}");
                        println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                        println!("{BOLD}Enter a name that does not exist{RESET}\n");
                        return Ok(());
                    }
                    else if !Checker::checker(&copy_cpp,"file") {
                        std::fs::copy(&file_cp,&copy_cpp)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                "2" => {
                    if Checker::checker(&copy_cxx,"file") {
                        println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_file}\"{RESET}");
                        println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                        println!("{BOLD}Enter a name that does not exist{RESET}\n");
                        return Ok(());
                    }
                    else if !Checker::checker(&copy_cxx,"file") {
                        std::fs::copy(&file_cp,&copy_cxx)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                "3" => {
                    if Checker::checker(&copy_cppp,"file") {
                        println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_file}\"{RESET}");
                        println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                        println!("{BOLD}Enter a name that does not exist{RESET}\n");
                        return Ok(());
                    }
                    else if !Checker::checker(&copy_cppp,"file") {
                        std::fs::copy(&file_cp,&copy_cppp)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                "4" => {
                    if Checker::checker(&copy_cc,"file") {
                        println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_file}\"{RESET}");
                        println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                        println!("{BOLD}Enter a name that does not exist{RESET}\n");
                        return Ok(());
                    }
                    else if !Checker::checker(&copy_cc,"file") {
                        std::fs::copy(&file_cp,&copy_cc)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                "5" => {
                    if Checker::checker(&copy_cp,"file") {
                        println!("{BOLD}{RED}error{RESET}{BOLD}: already exists a file with this name \"{copy_file}\"{RESET}");
                        println!("{BOLD}{BLUE}Help{RESET}{BOLD}:{RESET}");
                        println!("{BOLD}Enter a name that does not exist{RESET}\n");
                        return Ok(());
                    }
                    else if !Checker::checker(&copy_cp,"file") {
                        std::fs::copy(&file_cp,&copy_cp)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                _=> {
                    println!("{BOLD}{RED}error{RESET}{BOLD}: Not a valid option{RESET}\n");
                    return Ok(());
                }
            }
        }
    }
    else if !Checker::checker(&file_cp,"file") {
        Error::not_exist(&file_cp,"file");
        return Ok(());
    }
    return Ok(());
}
_=> {
    println!("{BOLD}{RED}error{RESET}{BOLD}: Not a valid option{RESET}\n");
    return Ok(());
}
}
}
    }
fn copy_file() -> std::io::Result<()> {
        let languages = ["[1] Rust (.rs)","[2] C++ (.cpp) (.cxx) (.c++) (.cc) (.cp)","[3] C (.c)","[4] JavaScript (.js)","[5] Python (.py)","[6] Lua (.lua)"];
        for language in languages {
            println!("{language}");
        }
        let option = Input::input()?;
        match option.trim() {
            "1" => {
                Copy::copy_languages("Rust",".rs")?;
                return Ok(());
            }
            "2" => {
                Copy::copy_cpp()?;
                return Ok(());
            }
            "3" => {
                Copy::copy_languages("C",".c")?;
                return Ok(());
            }
            "4" => {
                Copy::copy_languages("JavaScript",".js")?;
                return Ok(());
            }
            "5" => {
                Copy::copy_languages("Python",".py")?;
                return Ok(());
            }
            "6" => {
                Copy::copy_languages("Lua",".lua")?;
                return Ok(());
            }
            _=> {
                println!("{BOLD}{RED}error{RESET}{BOLD}: Not a valid option{RESET}\n");
                return Ok(());
            }
        }
    }
}
impl Renames for Rename {
    fn rename_languages(language: &str,extension: &str) -> std::io::Result<()> {
        println!("type the file name");
        let file = Input::input()?;
        let space_invalid_characters = InvalidCharacter::invalid_characters(&file,"file") && file.contains(' ');
        let file_type = format!("{file}{extension}");
        if file.is_empty() {
            Error::empty("file.rs");
            return Ok(());
        }
        else if space_invalid_characters {
            Error::spaces(&file,"file");
            Error::invalid_characters(&file,"file");
            println!(r#"{BOLD}2 Errors detected{RESET}"#);
            return Ok(());
        }
        else if InvalidCharacter::invalid_characters(&file,"file") {
            Error::invalid_characters(&file,"file");
            return Ok(());
        }
        else if file.contains(' ') {
            Error::spaces(&file,"file");
            return Ok(());
        }
        else if file.ends_with(extension) && Checker::checker(&file,"file") {
            println!("type another name to \"{file}\"");
            let new_name = Input::input()?;
            let space_invalid_characters_new_name = InvalidCharacter::invalid_characters(&new_name,"file") && new_name.contains(' ');
            let new_name_type = format!("{new_name}{extension}");
            if new_name.is_empty() {
                Error::empty("new_name_file.rs");
                return Ok(());
            }
            else if space_invalid_characters_new_name {
                Error::spaces(&new_name,"file");
                Error::invalid_characters(&new_name,"file");
                println!(r#" {BOLD}2 Errors detected{RESET} "#);
                return Ok(());
            }
            else if InvalidCharacter::invalid_characters(&new_name,"file") {
                Error::invalid_characters(&new_name,"file");
                return Ok(());
            }
            else if new_name.contains(' ') {
                Error::spaces(&new_name,"file");
                return Ok(());
            }
            else if new_name == file {
                println!("{BOLD}{RED}error{RESET}{BOLD}: The file name remains the same, type a different name.{RESET}");
                return Ok(());
            }
            else if new_name.ends_with(extension) && Checker::checker(&new_name,"file") {
                Error::new_name_exist(&new_name,"file");
                return Ok(());
            }
            else if new_name.ends_with(extension) && !Checker::checker(&new_name,"file") {
                std::fs::rename(&file,&new_name)?;
                if extension == ".rs" || extension == ".c" {
                    let executable_rs = new_name.replace(".rs","");
                    let executable_c = new_name.replace(".c","");
                    let actual_rs = file.replace(".rs","");
                    let actual_c = file.replace(".c","");
                    if file.ends_with(".c") && Checker::checker(&actual_c,"file") {
                        std::fs::rename(&actual_c,&executable_c)?;
                        return Ok(());
                    }
                    else if file.ends_with(".rs") && Checker::checker(&actual_rs,"file") {
                        std::fs::rename(&actual_rs,&executable_rs)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                return Ok(());
            }
            else if new_name.contains('.') {
                Error::invalid_extension("rename",extension,language);
                return Ok(());
            }
            else if Checker::checker(&new_name_type,"file") {
                Error::new_name_exist(&new_name,"file");
                return Ok(());
            }
            else if !Checker::checker(&new_name_type,"file") {
                std::fs::rename(&file,&new_name_type)?;
                if extension == ".rs" || extension == ".c" {
                    let actual_rs = file.replace(".rs","");
                    let actual_c = file.replace(".c","");
                    if file.ends_with(".rs") && Checker::checker(&actual_rs,"file") {
                        std::fs::rename(&actual_rs,&new_name)?;
                        return Ok(());
                    }
                    else if file.ends_with(".c") && Checker::checker(&actual_c,"file") {
                        std::fs::rename(&actual_c,&new_name)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                return Ok(());
            }
            return Ok(());
        }
        else if file.ends_with(extension) && !Checker::checker(&file,"file") {
            Error::not_exist(&file,"file");
            return Ok(());
        }
        else if file.contains('.') {
            Error::invalid_extension("rename",extension,language);
            return Ok(());
        }
        else if Checker::checker(&file_type,"file") {
            println!("type another name to \"{file_type}\"");
            let new_name = Input::input()?;
            let space_invalid_characters_new_name = InvalidCharacter::invalid_characters(&new_name,"file") && new_name.contains(' ');
            let new_name_type = format!("{new_name}{extension}");
            if new_name.is_empty() {
                Error::empty("new_name.rs");
                return Ok(());
            }
            else if space_invalid_characters_new_name {
                Error::spaces(&new_name,"file");
                Error::invalid_characters(&new_name,"file");
                println!(r#" {BOLD}2 Errors detected{RESET} "#);
                return Ok(());
            }
            else if InvalidCharacter::invalid_characters(&new_name,"file") {
                Error::invalid_characters(&new_name,"file");
                return Ok(());
            }
            else if new_name.contains(' ') {
                Error::spaces(&file,"file");
                return Ok(());
            }
            else if new_name == file {
                println!("{BOLD}{RED}error{RESET}{BOLD}: The file name remains the same, type a different name.{RESET}");
                return Ok(());
            }
            else if new_name.ends_with(extension) && Checker::checker(&new_name,"file") {
                Error::new_name_exist(&new_name,"file");
                return Ok(());
            }
            else if new_name.ends_with(extension) && !Checker::checker(&new_name,"file") {
                std::fs::rename(&file_type,&new_name)?;
                if extension == ".rs" || extension == ".c" {
                    let executable_rs = new_name.replace(".rs","");
                    let executable_c = new_name.replace(".c","");
                    if extension == ".rs" && Checker::checker(&file,"file") {
                        std::fs::rename(&file,&executable_rs)?;
                        return Ok(());
                    }
                    else if extension == ".c" && Checker::checker(&file,"file") {
                        std::fs::rename(&file,&executable_c)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                return Ok(());
            }
            else if Checker::checker(&new_name_type,"file") {
                Error::new_name_exist(&new_name_type,"file");
                return Ok(());
            }
            else if !Checker::checker(&new_name_type,"file") {
                std::fs::rename(&file_type,&new_name_type)?;
                if extension == ".rs" || extension == ".c" {
                    if Checker::checker(&file,"file") {
                        std::fs::rename(&file,&new_name)?;
                        return Ok(());
                    }
                    return Ok(());
                }
                return Ok(());
            }
            return Ok(());
        }
        else if !Checker::checker(&file_type,"file") {
            Error::not_exist(&file_type,"file");
            return Ok(());
        }
        return Ok(());
    }
}
impl Vexas for Vexa {
    fn vexa() -> std::io::Result<bool> {
        let cmd = Input::input()?;
        match cmd.trim() {
            "exit" | "quit" => {
                return Ok(false);
            }
            "ls" => {
                Ls::ls()?;
            }
            "checker rs file" => {
                Checker::check("file",".rs")?;
            }
            "checker c++ file" => {
                Checker::check_cpp()?;
            }
            "checker c file" => {
                Checker::check("file",".c")?;
            }
            "checker js file" => {
                Checker::check("file",".js")?;
            }
            "checker py file" => {
                Checker::check("file",".py")?;
            }
            "checker lua file" => {
                Checker::check("file",".lua")?;
            }
            "checker folder" => {
                Checker::check_dir()?;
            }
            "configuration --change editor" => {
                Configuration::configuration()?;
            }
            "checker program" => {
                Checker::checker_program()?;
            }
            "create" => {
                Create::create()?;
            }
            "create dir" | "mkdir" => {
                Create::create_dir()?;
            }
            "create file" => {
                Create::create_file()?;
            }
            "create rust file" | "crt rs file" => {
                Create::create_languages("Rust",".rs")?;
            }
            "create c++ file" | "crt c++ file" => {
                Create::create_cpp()?;
            }
            "create c file" | "crt c file" => {
                Create::create_languages("C",".c")?;
            }
            "create js file" | "crt js file" => {
                Create::create_languages("JavaScript",".js")?;
            }
            "create py file" | "crt py file" => {
                Create::create_languages("Python",".py")?;
            }
            "create lua file" | "crt lua file" => {
                Create::create_languages("Lua",".lua")?;
            }
            "mv" => {
                Move::moves()?;
            }
            "mv dir" => {
                Move::move_dir()?;
            }
            "mv file" => {
                Move::move_file()?;
            }
            "mv rs file" => {
                Move::move_languages("Rust",".rs")?;
            }
            "mv c++ file" => {
                Move::move_cpp()?;
            }
            "mv c file" => {
                Move::move_languages("C",".c")?;
            }
            "mv js file" => {
                Move::move_languages("JavaScript",".js")?;
            }
            "mv py file" => {
                Move::move_languages("Python",".py")?;
            }
            "mv lua file" => {
                Move::move_languages("Lua",".lua")?;
            }
            "rnm rs file" => {
                Rename::rename_languages("Rust",".rs")?;
            }
            "rnm c file" => {
                Rename::rename_languages("C",".c")?;
            }
            "rnm js file" => {
                Rename::rename_languages("JavaScript",".js")?;
            }
            "rnm py file" => {
                Rename::rename_languages("Python",".py")?;
            }
            "rnm lua file" => {
                Rename::rename_languages("Lua",".lua")?;
            }
            "rm" => {
                Remove::remove()?;
            }
            "rm -f dir" => {
                Remove::remove_dir()?;
            }
            "rm file" => {
                Remove::remove_file()?;
            }
            "rm rs file" => {
                Remove::remove_languages("Rust",".rs")?;
            }
            "rm c++ file" => {
                Remove::remove_cpp()?;
            }
            "rm c file" => {
                Remove::remove_languages("C",".c")?;
            }
            "rm js file" => {
                Remove::remove_languages("JavaScript",".js")?;
            }
            "rm py file" => {
                Remove::remove_languages("Python",".py")?;
            }
            "rm lua file" => {
                Remove::remove_languages("Lua",".lua")?;
            }
            "edt file" => {
                Edit::edit_file()?;
            }
            "edt rs file" => {
                Edit::edit_languages("Rust",".rs")?;
            }
            "edt c++ file" => {
                Edit::edit_cpp()?;
            }
            "edt c file" => {
                Edit::edit_languages("C",".c")?;
            }
            "edt js file" => {
                Edit::edit_languages("JavaScript",".js")?;
            }
            "edt py file" => {
                Edit::edit_languages("Python",".py")?;
            }
            "edt lua file" => {
                Edit::edit_languages("Lua",".lua")?;
            }
            "copy rs file" => {
                Copy::copy_languages("Rust",".rs")?;
            }
            "copy c++ file" => {
                Copy::copy_cpp()?;
            }
            "copy c file" => {
                Copy::copy_languages("C",".c")?;
            }
            "copy js file" => {
                Copy::copy_languages("JavaScript",".js")?;
            }
            "copy py file" => {
                Copy::copy_languages("Python",".py")?;
            }
            "copy lua file" => {
                Copy::copy_languages("Lua",".lua")?;
            }
            "copy file" => {
                Copy::copy_file()?;
            }
            "rustc" => {
                Compile::compile_alone("rustc",".rs","Rust")?;
            }
            "clang" => {
                Compile::compile_alone("clang",".c","C")?;
            }
            "nodejs" => {
                Compile::compile_alone("nodejs",".js","JavaScript")?;
            }
            "python" => {
                Compile::compile_alone("python",".py","Python")?;
            }
            "lua" => {
                Compile::compile_alone("lua",".lua","Lua")?;
            }
            "rustc --explain E0001" => {
                Compile::rustc_explain("E0001")?;
            }
            "rustc --explain E0002" => {
                Compile::rustc_explain("E0002")?;
            }
            "rustc --explain E0003" => {
                Compile::rustc_explain("E0003")?;
            }
            "rustc --explain E0004" => {
                Compile::rustc_explain("E0004")?;
            }
            "rustc --explain E0005" => {
                Compile::rustc_explain("E0005")?;
            }
            "rustc --explain E0006" => {
                Compile::rustc_explain("E0006")?;
            }
            "rustc --explain E0007" => {
                Compile::rustc_explain("E0007")?;
            }
            "rustc --explain E0008" => {
                Compile::rustc_explain("E0008")?;
            }
            "rustc --explain E0009" => {
                Compile::rustc_explain("E0009")?;
            }
            "rustc --explain E0010" => {
                Compile::rustc_explain("E0010")?;
            }
            "rustc --explain E0011" => {
                Compile::rustc_explain("E0011")?;
            }
            "rustc --explain E0012" => {
                Compile::rustc_explain("E0012")?;
            }
            "rustc --explain E0013" => {
                Compile::rustc_explain("E0013")?;
            }
            "rustc --explain E0014" => {
                Compile::rustc_explain("E0014")?;
            }
            "rustc --explain E0015" => {
                Compile::rustc_explain("E0015")?;
            }
            "rustc --explain E0023" => {
                Compile::rustc_explain("E0023")?;
            }
            "rustc --explain E0025" => {
                Compile::rustc_explain("E0025")?;
            }
            "rustc --explain E0026" => {
                Compile::rustc_explain("E0026")?;
            }
            "rustc --explain E0027" => {
                Compile::rustc_explain("E0027")?;
            }
            "rustc --explain E0029" => {
                Compile::rustc_explain("E0029")?;
            }
            "rustc --explain E0030" => {
                Compile::rustc_explain("E0030")?;
            }
            "rustc --explain E0033" => {
                Compile::rustc_explain("E0033")?;
            }
            "rustc --explain E0034" => {
                Compile::rustc_explain("E0034")?;
            }
            "rustc --explain E0040" => {
                Compile::rustc_explain("E0040")?;
            }
            "rustc --explain E0044" => {
                Compile::rustc_explain("E0044")?;
            }
            "rustc --explain E0045" => {
                Compile::rustc_explain("E0045")?;
            }
            "rustc --explain E0046" => {
                Compile::rustc_explain("E0046")?;
            }
            "rustc --explain E0049" => {
                Compile::rustc_explain("E0049")?;
            }
            "rustc --explain E0050" => {
                Compile::rustc_explain("E0050")?;
            }
            "rustc --explain E0053" => {
                Compile::rustc_explain("E0053")?;
            }
            "rustc --explain E0054" => {
                Compile::rustc_explain("E0054")?;
            }
            "rustc --explain E0055" => {
                Compile::rustc_explain("E0055")?;
            }
            "rustc --explain E0056" => {
                Compile::rustc_explain("E0056")?;
            }
            "rustc --explain E0057" => {
                Compile::rustc_explain("E0057")?;
            }
            _=> {
                let starts_c = ["create","create dir","create rust file","create c++ file","create c file","create js file","create py file","create lua file","copy rs file","copy c++ file","copy c file","copy js file","copy py file","copy lua file","copy file","configuration --change editor","checker dir","checker rs file","checker c++ file","checker c file","checker js fike","checker py file","checker lua file","clang","clang++"];
                let starts_e = ["exit","edt file","edt rs file","edt c++ file","edt c file","edt js file","edt py file","edt lua file"];
                let starts_r = ["rm","rm file","rm -f dir","rm rs file","rm c++ file","rm c file","rm js file","rm py file","rm lua file","rustc"];
                let starts_m = ["mv","mv dir","mv file","mv rs file","mv c++ file","mv c file","mv js file","mv py file","mv lua file"];
                if cmd.is_empty() {}
                else if cmd.starts_with("c") {
                    println!("Command \"{cmd}\" not found, do you mean:\n");
                    for start_c in starts_c {
                        println!("{start_c}");
                    }
                }
                else if cmd.starts_with("e") {
                    println!("Command \"{cmd}\" not found, do you mean:\n");
                    for start_e in starts_e {
                        println!("{start_e}");
                    }
                }
                else if cmd.starts_with("r") {
                    println!("Command \"{cmd}\" not found, do you mean:\n");
                    for start_r in starts_r {
                        println!("{start_r}");
                    }
                }
                else if cmd.starts_with("m") {
                    println!("Command \"{cmd}\" not found, do you mean:\n");
                    for start_m in starts_m {
                        println!("{start_m}");
                    }
                }
                else if cmd.starts_with("p") {
                    println!("Command \"{cmd}\" not found, do you mean:\n");
                    println!("python");
                }
                else if cmd.starts_with("l") {
                    println!("Command \"{cmd}\" not found, do you mean:\n");
                    println!("lua");
                    println!("ls");
                }
                else if cmd.starts_with("n") {
                    println!("Command \"{cmd}\" not found, do you mean:\n");
                    println!("nodejs");
                }
                else if cmd == "joao gabriel" || cmd == "william vext" {
                    println!("{BOLD}{BLUE}Hi creator!, welcome back!{RESET}");
                }
                else {
                    println!("{cmd}: command not found");
                }
            }
        }
        return Ok(true);
    }
}
