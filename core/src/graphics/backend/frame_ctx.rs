use std::any::Any;

#[derive(Debug)]
pub enum FrameCtx {
    Skipped,
    Data(Box<dyn Any>),
}

impl FrameCtx {
    pub fn new<T: 'static>(v: T) -> Self {
        Self::Data(Box::new(v))
    }

    pub fn skip() -> Self {
        Self::Skipped
    }

    pub fn as_mut<T: 'static>(&mut self) -> Option<&mut T> {
        match self {
            FrameCtx::Data(d) => d.downcast_mut::<T>(),
            FrameCtx::Skipped => None,
        }
    }

    pub fn into_inner<T: 'static>(self) -> Result<T, Self> {
        match self {
            FrameCtx::Data(d) => match d.downcast::<T>() {
                Ok(boxed) => Ok(*boxed),
                Err(orig) => Err(FrameCtx::Data(orig)),
            },
            FrameCtx::Skipped => Err(FrameCtx::Skipped),
        }
    }
}
