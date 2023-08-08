use super::{Context, ContextName};

#[derive(Debug)]
pub struct PrintingContext<CName> {
    stack: Vec<CName>,
}

impl<CName> Default for PrintingContext<CName> {
    fn default() -> Self {
        Self {
            stack: Default::default(),
        }
    }
}

struct PCStackPrinter<'a, CName>(&'a [CName]);

impl<'a, CName: std::fmt::Display> std::fmt::Display for PCStackPrinter<'a, CName> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for name in self.0 {
            write!(f, "[{name}]")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct PCError {
    _p: (),
}

impl<CName: std::fmt::Display + ContextName> Context<CName> for PrintingContext<CName> {
    type Error = PCError;

    fn enter_context(&mut self, name: CName) {
        self.stack.push(name);
    }

    fn exit_ctx(&mut self) {
        self.stack.pop();
    }

    fn message<T: Into<super::CoralError>>(&mut self, msg: T) -> Self::Error {
        eprintln!("{}|[MSG]| {:?}", PCStackPrinter(&self.stack), msg.into());
        PCError { _p: () }
    }

    fn report<T, E: Into<super::CoralError>>(
        &mut self,
        res: Result<T, E>,
    ) -> Result<T, Self::Error> {
        res.map_err(|err| {
            eprintln!("{}|[REP]| {:?}", PCStackPrinter(&self.stack), err.into());
            PCError { _p: () }
        })
    }
}
