use x11_highlevel::Display;

pub fn disable() {
    let mut display = Display::new()
        .expect("Cannot open display");
    let reader = display.devices()
        .filter(|dev| dev.product_id() == Some((1204, 48289)))
        .next()
        .expect("Couldn't find card reader");
    println!("Reader: {:?}", reader.name());
    unimplemented!()
}
