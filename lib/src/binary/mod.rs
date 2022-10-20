/// decoder of a wasm binary
#[derive(Debug, Default)]
pub struct Decoder {}

impl Decoder {
    /// New wasm decoder
    pub fn new() -> Self {
        Self::default()
    }
    /// TODO
    pub fn read(&mut self, buf: &[u8]) -> crate::Result<ModuleHeader<'_>> {
        todo!()
    }
}

/// the start of a wasm module binary
pub struct ModuleHeader<'buf> {
    buf: &'buf [u8],
}

impl<'buf> ModuleHeader<'buf> {}
