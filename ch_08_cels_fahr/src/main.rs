#![allow(unused)]

#[derive(PartialEq)]
enum Scale {
    Celsius,
    Fahrenheit,
}

struct Temperature {
    degrees: f32,
    scale: Scale,
}

impl Temperature {
    fn new(degrees: f32) -> Self {
        Temperature {
            degrees,
            scale: Scale::Celsius,
        }
    }

    fn to_celsius(&self) -> f32 {
        match self.scale {
            Scale::Celsius => self.degrees,
            Scale::Fahrenheit => (self.degrees - 32f32) * 5f32 / 9f32,
        }
    }

    fn to_fahrenheit(&self) -> f32 {
        match self.scale {
            Scale::Celsius => self.degrees * 9f32 / 5f32 + 32f32,
            Scale::Fahrenheit => self.degrees,
        }
    }
}

fn main() {
    let temp = Temperature::new(20.0);

    println!("fun fact: 20°C is an integer in celsius and fahrenheit");
    println!(
        "          {:.1}°C = {:.1}°F",
        temp.to_celsius(),
        temp.to_fahrenheit()
    );
}

#[test]
fn one_degree() {
    let cold = Temperature::new(1.0);
    assert!((cold.to_fahrenheit() - 33.8) < 0.01);
    assert!((cold.to_fahrenheit() - 33.8) >= 0.0);
}

#[test]
fn boiling() {
    let hot = Temperature::new(100.0);
    assert!((hot.to_fahrenheit() - 212.0) < 0.01);
    assert!((hot.to_fahrenheit() - 212.0) >= 0.0);
}

#[test]
fn freezing() {
    let freezing = Temperature {
        degrees: Temperature::new(0.0).to_fahrenheit(),
        scale: Scale::Fahrenheit,
    };

    assert!(freezing.to_celsius() < 0.001);
    assert!(freezing.to_celsius() > -0.01);
}
