use crate::DefaultHandler;
use crate::Handler;

pub struct CLIHandler;

impl Handler for CLIHandler {
    fn run(source_roots: Vec<String>, module_name: String) {
        DefaultHandler::run_with_handler(Self, source_roots, module_name, false);
    }
    fn test(source_roots: Vec<String>, module_name: String) {
        DefaultHandler::run_with_handler(Self, source_roots, module_name, true);
    }
}
