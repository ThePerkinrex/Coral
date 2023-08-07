use std::sync::Arc;

use super::{Context, CoralError, Or};

#[derive(Default)]
pub struct PrintingContext {
    ctx_stack: Vec<Arc<str>>,
}

impl PrintingContext {
    fn print_stack(&self) {
        for s in &self.ctx_stack {
            eprint!("[{s}]")
        }
    }
}

impl Context for PrintingContext {
    type Error = ();
    type Or<'ctx> = PrintingOr<'ctx>;

    fn message<T: Into<CoralError>>(&mut self, msg: T) -> Self::Error {
        self.print_stack();
        eprintln!("[MSG] {}", msg.into())
    }

    fn report<T, E: Into<CoralError>>(&mut self, res: Result<T, E>) -> Result<T, Self::Error> {
        res.map_err(|msg| {
            self.print_stack();
            eprintln!("[REPORT] {}", msg.into());
        })
    }

    fn or(&mut self) -> Self::Or<'_> {
        PrintingOr(self)
    }

    fn enter_context<S: std::fmt::Display>(&mut self, name: S) {
        // self.print_stack();
        // eprintln!(" Entering ctx {name}");
        self.ctx_stack.push(Arc::from(name.to_string()))
    }

    fn exit_ctx(&mut self) {
        if let Some(_ctx) = self.ctx_stack.pop() {
            // self.print_stack();
            // eprintln!(" Exiting ctx {ctx}")
        }
    }
}

pub struct PrintingOr<'ctx>(&'ctx mut PrintingContext);

impl<'ctx> Or for PrintingOr<'ctx> {
    type OptionContext = OptionCtx<PrintingContext>;

    fn option(&self, name: &'static str) -> Self::OptionContext {
        OptionCtx {
            name,
            ctx: PrintingContext {
                ctx_stack: self.0.ctx_stack.clone(),
            },
        }
    }

    fn accept(self, _: Self::OptionContext) {}

    fn discard<const N: usize>(self, _: [Self::OptionContext; N]) {}
}

pub struct OptionCtx<Ctx> {
    name: &'static str,
    ctx: Ctx,
}

impl<C: Context> Context for OptionCtx<C> {
    type Error = C::Error;

    type Or<'t> = C::Or<'t> where C: 't;

    fn message<T: Into<CoralError>>(&mut self, msg: T) -> Self::Error {
        self.ctx.message(msg)
    }

    fn report<T, E: Into<CoralError>>(&mut self, res: Result<T, E>) -> Result<T, Self::Error> {
        self.ctx.report(res)
    }

    fn or(&mut self) -> Self::Or<'_> {
        self.ctx.or()
    }

    fn enter_context<S: std::fmt::Display>(&mut self, name: S) {
        self.ctx.enter_context(name)
    }

    fn exit_ctx(&mut self) {
        self.ctx.exit_ctx()
    }
}
