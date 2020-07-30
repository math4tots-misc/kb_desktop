use crate::rterr;
use crate::Val;
use crate::Handler;
use crate::Request;
use crate::Response;
use crate::DefaultHandler;
use std::sync::mpsc;
use ggez::{
    Context,
    ContextBuilder,
    GameResult,
    event::{self, EventHandler},
    graphics,
};

pub struct DesktopHandler {
    qtx: mpsc::Sender<Request>,
    srx: mpsc::Receiver<Response>,
}

impl DesktopHandler {
    fn new(qtx: mpsc::Sender<Request>, srx: mpsc::Receiver<Response>) -> Self {
        Self {
            qtx,
            srx,
        }
    }
    fn req(&mut self, req: Request) -> Result<Response, Val> {
        match self.qtx.send(req) {
            Ok(()) => {}
            Err(error) => return Err(rterr(format!("{:?}", error))),
        }
        match self.srx.recv() {
            Ok(resp) => Ok(resp),
            Err(error) => return Err(rterr(format!("{:?}", error))),
        }
    }
}

impl Handler for DesktopHandler {
    fn run(source_roots: Vec<String>, module_name: String) {
        run(source_roots, module_name, false)
    }
    fn test(source_roots: Vec<String>, module_name: String) {
        run(source_roots, module_name, true)
    }
    fn send(&mut self, code: u32, args: Vec<Val>) -> Result<Val, Val> {
        match code {
            // init
            10 => {
                self.req(Request::Init)?;
                Ok(Val::Nil)
            }
            _ => Err(rterr(format!("Unrecognized code: {}", code))),
        }
    }
}

pub struct GgezHandler {
}

impl EventHandler for GgezHandler {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
}

fn run(source_roots: Vec<String>, module_name: String, test: bool) {
    let (qtx, qrx) = mpsc::channel::<Request>();
    let (stx, srx) = mpsc::channel::<Response>();
    let _handle = std::thread::Builder::new()
        .name("kb-main".to_owned())
        .spawn(move || {
            let handler = DesktopHandler::new(qtx, srx);
            let handler =
                DefaultHandler::run_with_handler(handler, source_roots, module_name, test)
                    .into_handler();
            handler.qtx.send(Request::Quit).unwrap();
        })
        .unwrap();

    match qrx.recv() {
        Ok(Request::Init) => {
            let (mut ctx, mut event_loop) =
                ggez::ContextBuilder::new("name", "author")
                    .build()
                    .unwrap();

            let mut event_handler = GgezHandler {};

            stx.send(Response::Ok).unwrap();

            match event::run(&mut ctx, &mut event_loop, &mut event_handler) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("ERROR: {:?}", e);
                    std::process::exit(1);
                }
            }
        }

        Ok(_) => stx
            .send(Response::Err(format!(
                "GUI request made before being initialized"
            )))
            .unwrap(),

        // GUI mode was never requested
        Err(mpsc::RecvError) => {}
    }

    _handle.join().unwrap();
}
