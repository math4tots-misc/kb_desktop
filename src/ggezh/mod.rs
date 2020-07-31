use crate::translate_files;
use crate::BasicError;
use crate::Code;
use crate::Handler;
use crate::Loader;
use crate::RcStr;
use crate::Val;
use crate::Vm;
use ggez::{
    event::{self, EventHandler},
    graphics::{self, Color, Text, TextFragment},
    Context, ContextBuilder, GameResult,
};
use std::rc::Rc;

mod conv;
mod send;

use conv::*;

pub struct GgezHandler {
    ctx: &'static mut Context,
}

impl Handler for GgezHandler {
    fn run(source_roots: Vec<String>, module_name: String) {
        match run(source_roots, module_name) {
            Ok(()) => {}
            Err(error) => {
                eprintln!("{}", error.format());
                std::process::exit(1);
            }
        }
    }
    fn send(&mut self, code: u32, args: Vec<Val>) -> Result<Val, Val> {
        self.send0(code, args)
    }
}

struct State {
    vm: Vm<GgezHandler>,
    update: Option<Rc<Code>>,
    draw: Option<Rc<Code>>,
    sleep_dur: std::time::Duration,
}

impl State {
    fn die_on_err<T>(&self, result: Result<T, Val>) -> T {
        match err_trace(&self.vm, result) {
            Ok(t) => t,
            Err(error) => {
                eprintln!("{}", error.format());
                std::process::exit(1);
            }
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if let Some(update) = &self.update {
            let result = self.vm.exec(update);
            self.die_on_err(result);
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if let Some(draw) = &self.draw {
            let result = self.vm.exec(draw);
            self.die_on_err(result);
            graphics::present(ctx)?;
        }
        std::thread::sleep(self.sleep_dur);
        Ok(())
    }
}

fn run(source_roots: Vec<String>, module_name: String) -> Result<(), BasicError> {
    let module_name: RcStr = module_name.into();
    let mut loader = Loader::new();
    for source_root in source_roots {
        loader.add_source_root(source_root);
    }
    let files = loader.load(&module_name)?;
    let code = translate_files(files)?;

    let (mut ctx, mut event_loop) = ContextBuilder::new("name", "author").build().unwrap();

    let mut vm = Vm::new(GgezHandler {
        // Kinda yucky needing unsafe here, but difficult to avoid given the requirements
        ctx: unsafe { std::mem::transmute(&mut ctx) },
    });
    if let Err(error) = vm.exec(&code) {
        err_trace(&mut vm, Err(error))?;
    }
    let update = get_opt_callback(&mut vm, &format!("{}#Update", module_name).into())?;
    let draw = get_opt_callback(&mut vm, &format!("{}#Draw", module_name).into())?;
    let mut state = State {
        vm,
        update,
        draw,
        sleep_dur: std::time::Duration::from_secs_f64(1.0 / 20.0),
    };

    match event::run(&mut ctx, &mut event_loop, &mut state) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("ERROR: {:?}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn err_trace<H: Handler, T>(vm: &Vm<H>, r: Result<T, Val>) -> Result<T, BasicError> {
    match r {
        Ok(t) => Ok(t),
        Err(error) => Err(BasicError {
            marks: vm.trace().clone(),
            message: format!("{}", error.as_err()),
            help: None,
        }),
    }
}

fn get_opt_callback<H: Handler>(
    vm: &mut Vm<H>,
    name: &RcStr,
) -> Result<Option<Rc<Code>>, BasicError> {
    match vm.scope().get_global_by_name(name).cloned() {
        Some(callback) => Ok(Some(err_trace(vm, callback.expect_func())?.clone())),
        None => Ok(None),
    }
}

fn checkargc(args: &Vec<Val>, argc: usize) -> Result<(), Val> {
    if args.len() != argc {
        Err(rterr!("Expected {} args but got {}", argc, args.len()))
    } else {
        Ok(())
    }
}

fn converr<T, E: std::error::Error>(r: Result<T, E>) -> Result<T, Val> {
    match r {
        Ok(t) => Ok(t),
        Err(error) => Err(rterr!("{:?}", error)),
    }
}
