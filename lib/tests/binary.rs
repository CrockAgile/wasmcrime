use wasmcrime::binary;

pub fn encode_wat(wat: &str) -> Vec<u8> {
    let buf = wast::parser::ParseBuffer::new(wat).expect("WAT parse buffer created");
    let mut buf = wast::parser::parse::<wast::Wat>(&buf).expect("WAT is valid to parse");
    buf.encode().expect("WAT is valid to encode")
}

#[test]
fn decode_module_header() {
    let buf = encode_wat(
        r#" (module
        (memory $memory 16)
        (global $g0 (mut i32) (i32.const 1048576))
        (global $__data_end i32 (i32.const 1048576))
        (global $__heap_base i32 (i32.const 1048576))
        (export "memory" (memory 0))
        (export "__data_end" (global 1))
    ) "#,
    );

    let mut decoder = binary::Decoder::new();
    let module = decoder.read(&buf).unwrap();
}
