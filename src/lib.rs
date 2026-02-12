use std::error::Error;
use std::fmt;

#[derive(Debug, Default)]
pub struct Context {
    pub file: String,
    pub image: Option<String>,
    pub bytes: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flow {
    Continue,
    Stop,
}

#[derive(Debug)]
pub enum ChainError {
    MissingField(&'static str),
    InvalidInput(&'static str),
}

impl fmt::Display for ChainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChainError::MissingField(name) => write!(f, "missing field: {}", name),
            ChainError::InvalidInput(msg) => write!(f, "invalid input: {}", msg),
        }
    }
}

impl Error for ChainError {}

pub trait Handler {
    fn handle(&self, ctx: &mut Context) -> Result<Flow, ChainError>;
}

pub struct Chain {
    handlers: Vec<Box<dyn Handler>>,
}

impl Chain {
    pub fn new() -> Self {
        Self { handlers: Vec::new() }
    }

    pub fn add<H: Handler + 'static>(mut self, handler: H) -> Self {
        self.handlers.push(Box::new(handler));
        self
    }

    pub fn run(&self, ctx: &mut Context) -> Result<(), ChainError> {
        for handler in &self.handlers {
            match handler.handle(ctx)? {
                Flow::Continue => continue,
                Flow::Stop => break,
            }
        }
        Ok(())
    }
}

pub struct CheckFileType;
pub struct ReadImage;
pub struct DisplayImage;

impl Handler for CheckFileType {
    fn handle(&self, ctx: &mut Context) -> Result<Flow, ChainError> {
        if ctx.file.trim().is_empty() {
            return Err(ChainError::MissingField("file"));
        }

        ctx.image = Some("lena.png".to_string());
        Ok(Flow::Continue)
    }
}

impl Handler for ReadImage {
    fn handle(&self, ctx: &mut Context) -> Result<Flow, ChainError> {
        let image = ctx.image.as_ref().ok_or(ChainError::MissingField("image"))?;
        if image.trim().is_empty() {
            return Err(ChainError::InvalidInput("image"));
        }

        ctx.bytes = Some(b"fake-image-bytes".to_vec());
        Ok(Flow::Continue)
    }
}

impl Handler for DisplayImage {
    fn handle(&self, ctx: &mut Context) -> Result<Flow, ChainError> {
        let bytes = ctx
            .bytes
            .as_ref()
            .ok_or(ChainError::MissingField("bytes"))?;

        println!("{} bytes", bytes.len());
        Ok(Flow::Stop)
    }
}