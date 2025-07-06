/// Rellena un polígono sobre un Framebuffer usando arrays de x, y y un color.
/// Los arrays deben tener la misma longitud y al menos 3 puntos.
pub fn fill_polygon_on_framebuffer(
    framebuffer: &mut Framebuffer,
    xs: &[i32],
    ys: &[i32],
    color: Color,
    output_file: &str,
) {
    if xs.len() != ys.len() || xs.len() < 3 {
        // No es un polígono válido
        return;
    }
    let n = xs.len();
    let min_y = *ys.iter().min().unwrap();
    let max_y = *ys.iter().max().unwrap();
    framebuffer.set_current_color(color);
    // Para cada línea horizontal (scanline)
    for y in min_y..=max_y {
        let mut intersecciones = Vec::new();
        for i in 0..n {
            let j = (i + 1) % n;
            let (y0, y1) = (ys[i], ys[j]);
            let (x0, x1) = (xs[i], xs[j]);
            // Solo considerar aristas que cruzan la scanline
            if (y0 <= y && y1 > y) || (y1 <= y && y0 > y) {
                // Calcular la intersección de la arista con la scanline
                let x = x0 + ((y - y0) as f32 / (y1 - y0) as f32 * (x1 - x0) as f32).round() as i32;
                intersecciones.push(x);
            }
        }
        // Ordenar las intersecciones
        intersecciones.sort_unstable();
        // Rellenar entre pares de intersecciones
        let mut k = 0;
        while k + 1 < intersecciones.len() {
            let x_start = intersecciones[k].min(intersecciones[k + 1]);
            let x_end = intersecciones[k].max(intersecciones[k + 1]);
            for x in x_start..=x_end {
                framebuffer.set_pixel(x as u32, y as u32);
            }
            k += 2;
        }
    }
    framebuffer.render_to_file(output_file);
}
use crate::framebuffer::Framebuffer;
use raylib::prelude::*;

use crate::line::line;

pub fn draw_polygon_on_framebuffer(
    framebuffer: &mut Framebuffer,
    xs: &[i32],
    ys: &[i32],
    color: Color,
    output_file: &str,
) {
    if xs.len() != ys.len() || xs.len() < 3 {
        // No es un polígono válido
        return;
    }
    framebuffer.set_current_color(color);
    for i in 0..xs.len() {
        let x0 = xs[i] as f32;
        let y0 = ys[i] as f32;
        let x1 = xs[(i + 1) % xs.len()] as f32;
        let y1 = ys[(i + 1) % ys.len()] as f32;
        line(framebuffer, Vector2::new(x0, y0), Vector2::new(x1, y1));
    }
    framebuffer.render_to_file(output_file);
}
