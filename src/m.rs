use super::CLIHandler;
use super::GgezHandler;
use super::Handler;

pub fn main() {
    let mut module_name: Option<String> = None;
    let mut state = State::Normal;
    let mut test_flag = false;
    let mut source_roots = Vec::new();
    let mut handler_type = HandlerType::CLI;

    for argstring in std::env::args() {
        let arg: &str = &argstring;

        match &state {
            State::Normal => match arg {
                "-m" => state = State::Module,
                "-h" => state = State::Handler,
                "-t" => test_flag = true,
                _ => source_roots.push(argstring),
            },
            State::Module => {
                module_name = Some(arg.to_owned());
                state = State::Normal;
            }
            State::Handler => {
                handler_type = match arg {
                    "c" | "cli" => HandlerType::CLI,
                    "g" | "ggez" => HandlerType::Ggez,
                    _ => {
                        eprintln!("Unrecognized handler type: {:?}", arg);
                        std::process::exit(1);
                    }
                };
                state = State::Normal;
            }
        }
    }

    let module_name = if let Some(module_name) = module_name {
        module_name
    } else {
        eprintln!("Start module name not specified");
        std::process::exit(1);
    };

    if test_flag {
        let test = match handler_type {
            HandlerType::CLI => CLIHandler::test,
            HandlerType::Ggez => GgezHandler::test,
        };
        test(source_roots, module_name)
    } else {
        let run = match handler_type {
            HandlerType::CLI => CLIHandler::run,
            HandlerType::Ggez => GgezHandler::run,
        };
        run(source_roots, module_name);
    };
}

enum State {
    Normal,
    Module,
    Handler,
}

enum HandlerType {
    CLI,
    Ggez,
}
