use super::{Context, CoralError, Or};

#[derive(Debug, Default)]
pub struct MockContext;

impl Context for MockContext {
    type Error = ();
	type Or<'ctx> = MockOr<'ctx>;

    fn message<T: Into<CoralError>>(&mut self, _: T) -> Self::Error {}

    fn report<T, E: Into<CoralError>>(&mut self, res: Result<T, E>) -> Result<T, Self::Error> {
        res.map_err(|_| ())
    }

    fn or(&mut self) -> Self::Or<'_> {
        MockOr(self)
    }
}

pub struct MockOr<'ctx>(&'ctx mut MockContext);

impl<'ctx> Or for MockOr<'ctx> {
    type OptionContext = OptionCtx<MockContext>;

    fn option(&self, name: &'static str) -> Self::OptionContext {
        OptionCtx {name, ctx: MockContext}
    }

    fn accept(self, _: Self::OptionContext) {
        
    }

    fn discard<const N: usize>(self, _: [Self::OptionContext; N]) {
        
    }
}

pub struct OptionCtx<Ctx> {
	name: &'static str,
	ctx: Ctx
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
}

