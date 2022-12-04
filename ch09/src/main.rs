use std::{io, num::ParseFloatError};

/* 
 * Though it wasn't prompted in the challenge, I included the option to choose
 * between metric and Imperial units.
 */

fn user_input_is_metric() -> bool {
    loop {
        println!("Choose input units: (f)eet or (m)etres:");

        let mut unit = String::new();
        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line.");
        
        let unit_trimmed = unit.trim()
                                    .to_lowercase()
                                    .chars()
                                    .nth(0)
                                    .unwrap();
        match unit_trimmed {
            'f' => break false,
            'm' => break true,
            _ => println!("Option must be either \'f\' or \'m\'.")
        }
    }
}

fn user_input_get_length(unit_name: &str) -> f64 {
    loop {
        println!("What is the length of the room in {}?", &unit_name);

        let mut length = String::new();
        io::stdin()
            .read_line(&mut length)
            .expect("Failed to read line.");

        let length_trimmed = length.trim().parse::<f64>();
        if test(&length_trimmed) { break length_trimmed.unwrap(); }
    }
}

fn user_input_get_width(unit_name: &str) -> f64 {
    loop {
        println!("What is the width of the room in {}?", &unit_name);

        let mut width = String::new();
        io::stdin()
            .read_line(&mut width)
            .expect("Failed to read line.");
        
        let width_trimmed = width.trim().parse::<f64>();
        if test(&width_trimmed) { break width_trimmed.unwrap(); }
    }
}

fn test(input: &Result<f64,ParseFloatError>) -> bool {
    match input {
        Ok(ok) => if ok > &0.0 {
                return true
            } else {
                println!("Number must be non-zero and positive.");
                return false;
            },
        Err(..) => {
            println!("Input is not a number.");
            return false;
        }
    }
}

fn unit_name(is_metric: &bool) -> &str {
    if *is_metric {
        return "metres";
    } else {
        return "feet";
    }
}

fn area_name(is_metric: &bool) -> &str {
    if *is_metric {
        return "square metres"
    } else {
        return "square feet"
    }
}

fn volume_name(is_metric: &bool)-> &str {
    if *is_metric {
        return "litres"
    } else {
        return "gallons"
    }
}

// A lot of these functions would be better off as methods

fn calc_area(length: f64, width: f64) -> f64 {
    length * width
}

fn convert_area(area: f64, is_metric: bool) -> f64 {
    const SQFT_TO_SQM: f64 = 0.09290304;

    if is_metric {
        area * (1.0/SQFT_TO_SQM)
    } else {
        area
    }
}

fn convert_volume(vol: f64) -> f64 {
    const GAL_TO_L: f64 = 4.54609;
    vol * GAL_TO_L
}

// Oops! I had a bug where it would round up from exact integers
fn round_up(vol: f64) -> f64 {
    if vol % 1.0 < 0.5 && vol % 1.0 > 0.0 {
        vol.round() + 1.0
    } else {
        vol.round()
    }
}

fn calc_paint(area: f64, is_metric: bool) -> f64 {
    const SQFT_TO_GAL: f64 = 1.0/350.0;

    if is_metric {
        round_up(convert_volume(convert_area(area, true) * SQFT_TO_GAL))
    } else {
        round_up(area * SQFT_TO_GAL)
    }
}

// Converting functions to methods
struct Area {
    area: f64
}

impl Area {
    fn new(length: f64, width: f64) -> Area {
        Area { area: length * width }
    }

    fn noun(&self, is_metric: bool) -> &str {
        if is_metric {
            if self.area == 1.0 {
                return "square metre"
            } else {
                return "square metres"
            }
        } else {
            if self.area == 1.0 {
                return "square foot"
            } else {
                return "square feet"
            }
        }
    }
}

struct Volume {
    vol: f64
}

impl Volume {
    fn new(area: f64, is_metric: bool) -> Volume {
        const SQFT_TO_GAL: f64 = 1.0/350.0;
        const SQM_TO_L: f64 = SQFT_TO_GAL * 10.76391041671 * 4.54609;

        if is_metric {
            Volume { vol: area * SQM_TO_L}
        } else {
            Volume { vol: area * SQFT_TO_GAL}
        }
    }

    fn noun(&self, is_metric: bool) -> &str {
        if is_metric {
            if self.vol == 1.0 {
                return "litre"
            } else {
                return "litres"
            }
        } else {
            if self.vol == 1.0 {
                return "gallon"
            } else {
                return "gallons"
            }
        }
    }

    // Not working correctly
    /*fn round_up(&mut self) -> &mut Self {
        if self.vol % 1.0 < 0.5 && self.vol % 1.0 > 0.0 {
            self.vol.round() + 1.0;
            self
        } else {
            self.vol.round();
            self
        }
    }*/

    // Okay, this seems to work correctly, but it wasn't what I had in mind.
    fn round_up(&mut self) -> &mut Self {
        if self.vol % 1.0 > 0.0 {
            self.vol = self.vol - (self.vol % 1.0) + 1.0;
            self
        } else {
            self
        }
    }

    // This is essentially a repeat of above without chaining. For some reason
    // this works fine like this, but when chained it stops rounding the value
    // when displaying i.e. vol.round_up().show(), where the alternate version
    // of this method is simply returns self.vol as f64, however noun() works
    // correctly only when show() doesn't work correctly, and vice versa. At
    // this point I just give up.
    fn show(&self) -> f64 {
        if self.vol % 1.0 < 0.5 && self.vol % 1.0 > 0.0 {
            self.vol.round() + 1.0
        } else {
            self.vol.round()
        }
    }

    fn alt_show(&self) -> f64 {
        self.vol
    }
}

fn main() {
    let is_metric = user_input_is_metric();
    let unit_name = unit_name(&is_metric);
    let area_name = area_name(&is_metric);
    let volume_name = volume_name(&is_metric);
    let length = user_input_get_length(&unit_name);
    let width = user_input_get_width(&unit_name);

    println!("You entered dimensions of {} {} by {} {}.", length, unit_name, width, unit_name);

    let area_basic = calc_area(length, width);
    println!("Method 1: You will need to purchase {} {} of paint to cover {} {}.", calc_paint(area_basic, is_metric), volume_name, area_basic, area_name);

    let area = Area::new(length, width);
    let mut vol = Volume::new(area.area, is_metric);
    println!("Method 2: You will need to purchase {} {} of paint to cover {} {}.", vol.round_up().alt_show(), vol.round_up().noun(is_metric), area.area, area.noun(is_metric));
}
