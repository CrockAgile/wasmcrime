use crate::{Error, Result};

fn split_at<'a>(buf: &'a [u8], len: usize, context: &'static str) -> Result<(&'a [u8], &'a [u8])> {
    if buf.len() < len {
        return Err(Error::Incomplete(context));
    }
    Ok(buf.split_at(len))
}

fn split_at_const<'a, const N: usize>(
    buf: &'a [u8],
    context: &'static str,
) -> Result<(&'a [u8; N], &'a [u8])> {
    if buf.len() < N {
        return Err(Error::Incomplete(context));
    }
    let (split, rest) = buf.split_at(N);
    let split = unsafe { &*(split.as_ptr() as *const [u8; N]) };
    Ok((split, rest))
}

/// decoder of a wasm binary
#[derive(Debug, Default)]
pub struct Decoder {}

impl Decoder {
    /// New wasm decoder
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    pub fn read_module_header<'buf>(&self, buf: &'buf [u8]) -> Result<(ModuleHeader, &'buf [u8])> {
        ModuleHeader::read(buf)
    }
}

/// the start of a wasm module binary
pub struct ModuleHeader {}

impl ModuleHeader {
    pub fn read(buf: &[u8]) -> Result<(Self, &[u8])> {
        const MAGIC: &[u8; 4] = b"\0asm";
        const VERSION: &[u8; 4] = &1u32.to_le_bytes();
        if buf.len() < MAGIC.len() {
            return Err(Error::Incomplete("incomplete module magic"));
        }
        if !buf.starts_with(MAGIC) {
            return Err(Error::Invalid("invalid module magic"));
        }
        let buf = &buf[MAGIC.len()..];

        if buf.len() < VERSION.len() {
            return Err(Error::Incomplete("incomplete module version"));
        }
        if !buf.starts_with(VERSION) {
            return Err(Error::Invalid("unknown module version"));
        }
        let buf = &buf[VERSION.len()..];

        Ok((Self {}, buf))
    }

    pub fn read_section_header<'buf>(
        &self,
        buf: &'buf [u8],
    ) -> Result<(SectionHeader, &'buf [u8])> {
        SectionHeader::read(buf)
    }
}

pub struct SectionHeader {
    id: u8,
    size: usize,
}

impl SectionHeader {
    pub fn read(mut buf: &[u8]) -> Result<(Self, &[u8])> {
        const SECTION_ID_LENGTH: usize = core::mem::size_of::<u8>();
        const SECTION_SIZE_LENGTH: usize = core::mem::size_of::<u32>();

        if buf.len() < SECTION_ID_LENGTH {
            return Err(Error::Incomplete("incomplete section id"));
        }

        let id = buf[0];
        buf = &buf[SECTION_ID_LENGTH..];

        let (size, buf) = split_at_const::<SECTION_SIZE_LENGTH>(buf, "incomplete section length")?;
        let size = u32::from_le_bytes(*size) as usize;

        let section_header = Self { id, size };
        Ok((section_header, buf))
    }
    pub fn read_section(&self, buf: &[u8]) -> Result<(Section, &[u8])> {
        match self.id {
            Section::CUSTOM_ID => todo!(),
            Section::TYPE_ID => todo!(),
            Section::IMPORT_ID => todo!(),
            Section::FUNCTION_ID => todo!(),
            Section::TABLE_ID => todo!(),
            Section::MEMORY_ID => todo!(),
            Section::GLOBAL_ID => todo!(),
            Section::EXPORT_ID => todo!(),
            Section::START_ID => todo!(),
            Section::ELEMENT_ID => todo!(),
            Section::CODE_ID => todo!(),
            Section::DATA_ID => todo!(),
            Section::DATA_COUNT_ID => todo!(),
            _ => Err(Error::Invalid("unknown section id")),
        }
    }
}

/// Module sections
pub enum Section {
    Custom(CustomSection),
    Type,
    Import,
    Function,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Element,
    DataCount,
    Code,
    Data,
}

impl Section {
    const CUSTOM_ID: u8 = 0;
    const TYPE_ID: u8 = 1;
    const IMPORT_ID: u8 = 2;
    const FUNCTION_ID: u8 = 3;
    const TABLE_ID: u8 = 4;
    const MEMORY_ID: u8 = 5;
    const GLOBAL_ID: u8 = 6;
    const EXPORT_ID: u8 = 7;
    const START_ID: u8 = 8;
    const ELEMENT_ID: u8 = 9;
    const CODE_ID: u8 = 10;
    const DATA_ID: u8 = 11;
    const DATA_COUNT_ID: u8 = 12;
}

pub struct CustomSection {}

impl CustomSection {
    pub fn read(buf: &[u8]) -> Result<(Self, &[u8])> {
        todo!()
    }
}
