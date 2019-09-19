use image::RgbImage;

// G = (V, w, P)
//
// V = vocabulary       (0, 1, [, ])
// w = Starting string  (0)
// P = Production rules (1 -> 11
//                       0 -> 1[0]0 )
pub struct G {
    alphabet: String,
    axiom: String,
    rules: Vec<String>,
    gen: usize,
}

impl G {
    pub fn new(vocabulary: &str, axiom: &str, production_rules: Vec<&str>) -> Self {
        let mut g = Self {
            alphabet: String::new(),
            axiom: String::new(),
            rules: Vec::new(),
            gen: 0,
        };

        g.alphabet = String::from(vocabulary);
        g.axiom    = String::from(axiom);

        for &rule in production_rules.iter() {
            g.rules.push(String::from(rule));
        }

        g
    }

    pub fn next_generation(&mut self) {
        let mut to_replace: Vec<(usize, &str)> = Vec::new();

        for (i, character) in self.axiom.chars().enumerate() {
            for rules in self.rules.iter() {
                match rules.find("->") {
                    Some(_) => (),
                    None => continue,
                };

                let rules: Vec<&str> = rules.split("->").collect();

                // "F => FF"
                //
                // from = "F"
                // to   = "FF"
                let from = match rules.get(0) {
                    Some(from) => from.trim(), // Get the first part "F"
                    None => continue,
                };

                let from = match from.chars().nth(0) {
                    Some(from) => from, // Get the first character
                    None => continue,
                };

                let to = match rules.get(1) {
                    Some(to) => to.trim(),
                    None => continue,
                };

                if character == from {
                    to_replace.push((i, to));
                }
            }
        }

        for (i, to) in to_replace.iter().rev() {
            self.axiom.replace_range(i..&(i + 1), to);
        }

        self.gen += 1;
    }
    pub fn advance_by(&mut self, n: u8) -> &String {
        for _i in 0..n {
            self.next_generation();
        }

        self.current_generation()
    }
    pub fn current_generation(&self) -> &String {
        &self.axiom
    }

    pub fn draw(&self, width: u32, height: u32, filename: &str) {
        let mut img = RgbImage::new(width, height);

        // Fill the picture with white background
        for pixel in img.pixels_mut() {
            pixel.0 = [255, 255, 255];
        }

        // Start drawing in the middle of the image
        let mut position = ((img.width() - 1) / 2, (img.height() - 1) / 2);

        let size:      u32 = 5;
        let leaf_size: u32 = 3;
        let mut angle: i16 = 90; // Starting angle (facing up)
        let angle_interval = 45; // Rotate 45º at a time

        // Push and Pop the position + angle
        let mut stack: Vec<(u32, u32, i16)> = Vec::new();

        for character in self.axiom.chars() {
            match character {
                '0' => {
                    draw_line(&mut img, &mut position, leaf_size, angle);
                },
                '1' => {
                    draw_line(&mut img, &mut position, size, angle);
                },
                '[' => {
                    stack.push((position.0, position.1, angle));

                    angle += angle_interval;
                    angle = angle % 360;
                    if angle < 0 {
                        angle += 360;
                    }
                },
                ']' => {
                    let (x, y, ang) = stack.pop().unwrap();

                    position = (x, y);
                    angle    = ang;

                    angle -= angle_interval;
                    angle = angle % 360;
                    if angle < 0 {
                        angle += 360;
                    }
                },
                _ => (),
            }
        }

        match img.save(filename) {
            Ok(_)  => (),
            Err(e) => panic!("Could not print image!\n{}", e),
        }
    }
}

// Draws 45º intervaled line segments
fn draw_line(
    image: &mut RgbImage,
    position: &mut (u32, u32),
    size: u32,
    angle: i16)
{
    // Draw a black line
    let color = [0, 0, 0];
   

    // Check if the line doesn't exceed the image border
    if position.0 >= image.height() - 1 - size || position.0 <= size {
        return;
    }
    if position.1 >= image.width() - 1 - size  || position.1 <= size {
        return;
    }

    match angle {
          0..=44  => {
            for _i in 0..size {
                position.0 += 1;

                let pixel = image.get_pixel_mut(position.0, position.1);
                pixel.0   = color;
            }
          }
         45..=89  => {
            for _i in 0..size {
                position.0 += 1;
                position.1 -= 1;

                let pixel = image.get_pixel_mut(position.0, position.1);
                pixel.0 = color;
            }
         },
         90..=134 => {
            for _i in 0..size {
                position.1 -= 1;

                let pixel = image.get_pixel_mut(position.0, position.1);
                pixel.0 = color;
            }
         }
        135..=179 => {
            for _i in 0..size {
                position.0 -= 1;
                position.1 -= 1;

                let pixel = image.get_pixel_mut(position.0, position.1);
                pixel.0 = color;
            }
         },
        180..=224 => {
            for _i in 0..size {
                position.0 -= 1;

                let pixel = image.get_pixel_mut(position.0, position.1);
                pixel.0 = color;
            }
         },
        225..=269 => {
            for _i in 0..size {
                position.0 -= 1;
                position.1 += 1;

                let pixel = image.get_pixel_mut(position.0, position.1);
                pixel.0 = color;
            }
         },
        270..=314 => {
            for _i in 0..size {
                position.1 += 1;

                let pixel = image.get_pixel_mut(position.0, position.1);
                pixel.0 = color;
            }
         },
        315..=360 => {
            for _i in 0..size {
                position.0 += 1;
                position.1 += 1;

                let pixel = image.get_pixel_mut(position.0, position.1);
                pixel.0 = color;
            }
         },

        // Panic if the angle is not properly bounded
        std::i16::MIN..=0   => panic!("Invalid range!"),
        360..=std::i16::MAX => panic!("Invalid range!"),
    }
}

