use barcodes;

error_chain! {
    links {
        Barcode(barcodes::Error, barcodes::ErrorKind);
    }
}
