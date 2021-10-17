mod blinkt_interface;

fn main() {
    let mut blinkt = blinkt_interface::Blinkt::new().unwrap();

    blinkt.clear().unwrap();
    blinkt.set_pixel(0, blinkt_interface::Pixel::new(255, 255, 255, 1.0));
    blinkt.set_pixel(1, blinkt_interface::Pixel::new(255, 255, 255, 0.8));
    blinkt.set_pixel(2, blinkt_interface::Pixel::new(255, 255, 255, 0.6));
    blinkt.set_pixel(3, blinkt_interface::Pixel::new(255, 255, 255, 0.4));
    blinkt.set_pixel(4, blinkt_interface::Pixel::new(255, 255, 255, 0.2));
    blinkt.set_pixel(5, blinkt_interface::Pixel::new(255, 255, 255, 0.0));
    blinkt.display().unwrap();
}
