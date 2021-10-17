// The primary representation of a Pixel.
struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
    brightness: f64,
}

impl Pixel {
    pub fn new(red: u8, green: u8, blue: u8, brightness: f64) -> Pixel {
        Pixel {
            red: red,
            green: green,
            blue: blue,
            brightness: brightness,
        }
    }
    pub fn new_off() -> Pixel {
        Pixel {
            red: 0,
            green: 0,
            blue: 0,
            brightness: 0.0,
        }
    }
}

// The representation of a Pixel going over the bus.
#[derive(Debug)]
struct BusPixel {
    red: u8,
    green: u8,
    blue: u8,
    brightness: u8,
}

impl BusPixel {
    pub fn from_pixel(pixel: &Pixel) -> BusPixel {
        let brightness = 0b11100000 | (((pixel.brightness * 31.0) as u8) & 0x1F);
        BusPixel {
            red: pixel.red,
            green: pixel.green,
            blue: pixel.blue,
            brightness: brightness,
        }
    }
}

struct Blinkt {
    pixel_array: [Pixel; 8],
    _chip: gpio_cdev::Chip,
    clock_line_handle: gpio_cdev::LineHandle,
    data_line_handle: gpio_cdev::LineHandle,
}

impl Blinkt {
    fn new() -> Result<Blinkt, gpio_cdev::Error> {
        let pixel_array: [Pixel; 8] = [
            Pixel::new_off(),
            Pixel::new_off(),
            Pixel::new_off(),
            Pixel::new_off(),
            Pixel::new_off(),
            Pixel::new_off(),
            Pixel::new_off(),
            Pixel::new_off(),
        ];
        let mut chip = gpio_cdev::Chip::new("/dev/gpiochip0")?;

        const DATA_PIN_NUM: u32 = 23;
        const CLOCK_PIN_NUM: u32 = 24;

        let clock_line_handle = chip.get_line(CLOCK_PIN_NUM)?.request(
            gpio_cdev::LineRequestFlags::OUTPUT,
            0,
            "j7s-blinkt",
        )?;
        let data_line_handle = chip.get_line(DATA_PIN_NUM)?.request(
            gpio_cdev::LineRequestFlags::OUTPUT,
            0,
            "j7s-blinkt",
        )?;

        Ok(Blinkt {
            pixel_array: pixel_array,
            _chip: chip,
            clock_line_handle: clock_line_handle,
            data_line_handle: data_line_handle,
        })
    }

    pub fn number_of_pixels(&self) -> usize {
        self.pixel_array.len()
    }

    pub fn get_bus_pixels(&self) -> Vec<BusPixel> {
        let mut bus_pixels = Vec::new();

        for pixel in self.pixel_array.iter() {
            bus_pixels.push(BusPixel::from_pixel(&pixel));
        }

        bus_pixels
    }

    pub fn clear(&mut self) -> Result<(), gpio_cdev::Error> {
        for pixel in self.pixel_array.iter_mut() {
            *pixel = Pixel::new_off();
        }

        self.display()
    }

    pub fn set_pixel(&mut self, index: usize, pixel: Pixel) {
        if index < self.number_of_pixels() {
            self.pixel_array[index] = pixel;
        }
    }

    pub fn display(&self) -> Result<(), gpio_cdev::Error> {
        self.start_frame()?;
        for bus_pixel in self.get_bus_pixels().iter() {
            self.write_byte(bus_pixel.brightness)?;
            self.write_byte(bus_pixel.blue)?;
            self.write_byte(bus_pixel.green)?;
            self.write_byte(bus_pixel.red)?;
        }
        self.end_frame()?;

        Ok(())
    }

    fn write_byte(&self, byte: u8) -> Result<(), gpio_cdev::Error> {
        const SLEEP_DURATION: std::time::Duration = std::time::Duration::from_micros(0);

        let mut byte_copy = byte;

        for _ in 0..8 {
            self.data_line_handle.set_value(byte_copy & 0x80)?;
            self.clock_line_handle.set_value(1)?;
            std::thread::sleep(SLEEP_DURATION);
            byte_copy = byte_copy << 1;
            self.clock_line_handle.set_value(0)?;
            std::thread::sleep(SLEEP_DURATION);
        }

        Ok(())
    }

    fn start_frame(&self) -> Result<(), gpio_cdev::Error> {
        const SLEEP_DURATION: std::time::Duration = std::time::Duration::from_micros(0);

        self.data_line_handle.set_value(0)?;
        for _ in 0..32 {
            self.clock_line_handle.set_value(1)?;
            std::thread::sleep(SLEEP_DURATION);
            self.clock_line_handle.set_value(0)?;
            std::thread::sleep(SLEEP_DURATION);
        }

        Ok(())
    }

    fn end_frame(&self) -> Result<(), gpio_cdev::Error> {
        const SLEEP_DURATION: std::time::Duration = std::time::Duration::from_micros(0);

        self.data_line_handle.set_value(0)?;
        for _ in 0..36 {
            self.clock_line_handle.set_value(1)?;
            std::thread::sleep(SLEEP_DURATION);
            self.clock_line_handle.set_value(0)?;
            std::thread::sleep(SLEEP_DURATION);
        }

        Ok(())
    }
}

fn main() {
    let mut blinkt = Blinkt::new().unwrap();

    blinkt.clear().unwrap();
    blinkt.set_pixel(0, Pixel::new(255, 255, 255, 1.0));
    blinkt.set_pixel(1, Pixel::new(255, 255, 255, 0.8));
    blinkt.set_pixel(2, Pixel::new(255, 255, 255, 0.6));
    blinkt.set_pixel(3, Pixel::new(255, 255, 255, 0.4));
    blinkt.set_pixel(4, Pixel::new(255, 255, 255, 0.2));
    blinkt.set_pixel(5, Pixel::new(255, 255, 255, 0.0));
    blinkt.display().unwrap();
}
