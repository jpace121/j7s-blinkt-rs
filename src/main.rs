// Copyright 2021 James Pace
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
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
