use std::{fmt::Display, ops::Mul};

use crate::color::Color;

mod vec3;
mod color;

fn main(){
    const image_width: u32 = 4;
    const image_height: u32 = 4;

    //Render

    println!("P3");
    println!("{} {}",image_width,image_height);
    println!("{}",255);
    
    for j in 0..image_height{
        for i in 0..image_width{
            let r = i as f64 / (image_width -1) as f64;
            let g = j as f64  / (image_height - 1) as f64;
            let b = 0.0;

            let pixel_color = Color::new(r, g, b);
            color::write_color(&pixel_color);
        }
    }
}

