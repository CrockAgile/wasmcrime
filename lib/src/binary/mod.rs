use crate::{Error, Result};

/// decoder of a wasm binary
#[derive(Debug, Default)]
pub struct Decoder {}

impl Decoder {
    /// New wasm decoder
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// TODO
    /// # Errors
    pub fn read<'buf>(&mut self, buf: &'buf [u8]) -> Result<ModuleHeader<'buf>> {
        ModuleHeader::read(buf)
    }
}

/// the start of a wasm module binary
pub struct ModuleHeader<'buf> {
    buf: &'buf [u8],
}

impl<'buf> ModuleHeader<'buf> {
    const MAGIC: &[u8; 4] = b"\0asm";
    const VERSION: &[u8; 4] = &1u32.to_le_bytes();
    /// TODO
    /// # Errors
    pub fn read(buf: &'buf [u8]) -> Result<Self> {
        if buf.len() < Self::MAGIC.len() {
            return Err(Error::Incomplete("incomplete module magic"));
        }
        if !buf.starts_with(Self::MAGIC) {
            return Err(Error::Invalid("invalid module magic"));
        }
        let buf = &buf[Self::MAGIC.len()..];

        if buf.len() < Self::VERSION.len() {
            return Err(Error::Incomplete("incomplete module version"));
        }
        if !buf.starts_with(Self::VERSION) {
            return Err(Error::Invalid("unknown module version"));
        }
        let buf = &buf[Self::VERSION.len()..];

        Ok(Self { buf })
    }
}
