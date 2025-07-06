mod framebuffer;
mod line;
mod poligons;

use framebuffer::Framebuffer;
use poligons::*;
use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height, Color::BLACK);

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));
    framebuffer.clear();

    // Polígono 1
    let xs1 = [165, 185, 180, 207, 233, 230, 250, 220, 205, 193];
    let ys1 = [380, 360, 330, 345, 330, 360, 380, 385, 410, 383];
    fill_polygon_on_framebuffer(&mut framebuffer, &xs1, &ys1, Color::RED, "polygon1.bmp");

    // Polígono 2
    let xs2 = [321, 288, 339, 374];
    let ys2 = [335, 286, 251, 302];
    fill_polygon_on_framebuffer(&mut framebuffer, &xs2, &ys2, Color::GREEN, "polygon2.bmp");

    // Polígono 3
    let xs3 = [377, 411, 436];
    let ys3 = [249, 197, 249];
    fill_polygon_on_framebuffer(&mut framebuffer, &xs3, &ys3, Color::BLUE, "polygon3.bmp");

    // Polígono 4
    let xs4 = [
        413, 448, 502, 553, 535, 676, 660, 750, 761, 672, 659, 615, 632, 580, 597, 552, 517, 466,
    ];
    let ys4 = [
        177, 159, 88, 53, 36, 37, 52, 145, 179, 192, 214, 214, 230, 230, 215, 214, 144, 180,
    ];
    fill_polygon_on_framebuffer(&mut framebuffer, &xs4, &ys4, Color::YELLOW, "polygon4.bmp");

    // Polígono 5
    let xs5 = [682, 708, 735, 739];
    let ys5 = [175, 120, 148, 170];
    fill_polygon_on_framebuffer(&mut framebuffer, &xs5, &ys5, Color::PURPLE, "polygon5.bmp");
}
