error_chain! {
    foreign_links {
        Fmt(::std::fmt::Error);
        Hyper(::hyper::Error);
        Io(::std::io::Error);
        TomlDe(::toml::de::Error);
        TomlSer(::toml::ser::Error);
    }

    errors {
        InvalidBarcode(barcode: String) {
            description("invalid barcode")
            display("invalid barcode: '{}'", barcode)
        }
        InvalidCertificates(file: String) {
            description("certificates did not parse")
            display("certificates did not parse from file '{}'", file)
        }
        InvalidPrivateKeys(file: String) {
            description("private keys did not parse")
            display("private keys did not parse from file '{}'", file)
        }
        InvalidPrivateKeyCount(count: usize) {
            description("invalid private key count")
            display("wanted 1 private key, found {}", count)
        }
        NoCertificatesFound {
            description("no certificates found")
            display("no certificates found")
        }
        UnknownBarcodeType(ty: String) {
            description("unknown barcode type")
            display("unknown barcode type: '{}'", ty)
        }
    }
}
