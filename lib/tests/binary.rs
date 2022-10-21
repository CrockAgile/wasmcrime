fn encode_wat(wat: &str) -> Vec<u8> {
    let buf = wast::parser::ParseBuffer::new(wat).expect("WAT parse buffer created");
    let mut buf = wast::parser::parse::<wast::Wat>(&buf).expect("WAT is valid to parse");
    buf.encode().expect("WAT is valid to encode")
}

mod module_header {
    use super::encode_wat;
    use wasmcrime::binary;

    #[test]
    fn decode_minimal_module() {
        let buf = encode_wat(
            r#"(module
                (memory $memory 16)
                (global $g0 (mut i32) (i32.const 1048576))
                (global $__data_end i32 (i32.const 1048576))
                (global $__heap_base i32 (i32.const 1048576))
                (export "memory" (memory 0))
                (export "__data_end" (global 1))
            ) "#,
        );

        let mut decoder = binary::Decoder::new();
        let _module = decoder.read(&buf).unwrap();
    }

    #[test]
    fn decode_empty_module() {
        let buf = encode_wat(r#"(module)"#);

        let mut decoder = binary::Decoder::new();
        let _module = decoder.read(&buf).unwrap();
    }

    #[test]
    fn fail_decode_invalid_magic() {
        let mut buf = encode_wat(r#"(module)"#);
        buf[2] = buf[2].wrapping_add(1);

        let mut decoder = binary::Decoder::new();
        let module = decoder.read(&buf);
        assert!(matches!(module, Err(wasmcrime::Error::Invalid(_))));
    }

    #[test]
    fn fail_decode_incomplete() {
        let buf = encode_wat(r#"(module)"#);

        let mut decoder = binary::Decoder::new();
        let module = decoder.read(&buf[..3]);
        assert!(matches!(module, Err(wasmcrime::Error::Incomplete(_))));
    }
}
