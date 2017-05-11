error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        InvalidBarcode(barcode: String) {
            description("invalid barcode")
            display("invalid barcode: '{}'", barcode)
        }
        UnknownBarcodeType(ty: String) {
            description("unknown barcode type")
            display("unknown barcode type: '{}'", ty)
        }
    }
}
