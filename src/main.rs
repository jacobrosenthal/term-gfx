
extern crate stb_image;
extern crate nalgebra as na;

use stb_image::image;
use std::path;
use na::{Vector3};

static ANSI_256: [u32; 256] = [
    // 8x standard colors
    0x000000, 0x800000, 0x008000, 0x808000, 0x000080, 0x800080, 0x008080, 0xC0C0C0,
    // 8x high-intensity colors
    0x808080, 0xFF0000, 0x00FF00, 0xFFFF00, 0x0000FF, 0xFF00FF, 0x00FFFF, 0xFFFFFF,
    // 6x6x6 color cube
    0x000000, 0x00005F, 0x000087, 0x0000AF, 0x0000D7, 0x0000FF,
    0x005F00, 0x005F5F, 0x005F87, 0x005FAF, 0x005FD7, 0x005FFF,
    0x008700, 0x00875F, 0x008787, 0x0087AF, 0x0087D7, 0x0087FF,
    0x00AF00, 0x00AF5F, 0x00AF87, 0x00AFAF, 0x00AFD7, 0x00AFFF,
    0x00D700, 0x00D75F, 0x00D787, 0x00D7AF, 0x00D7D7, 0x00D7FF,
    0x00FF00, 0x00FF5F, 0x00FF87, 0x00FFAF, 0x00FFD7, 0x00FFFF,
    0x5F0000, 0x5F005F, 0x5F0087, 0x5F00AF, 0x5F00D7, 0x5F00FF,
    0x5F5F00, 0x5F5F5F, 0x5F5F87, 0x5F5FAF, 0x5F5FD7, 0x5F5FFF,
    0x5F8700, 0x5F875F, 0x5F8787, 0x5F87AF, 0x5F87D7, 0x5F87FF,
    0x5FAF00, 0x5FAF5F, 0x5FAF87, 0x5FAFAF, 0x5FAFD7, 0x5FAFFF,
    0x5FD700, 0x5FD75F, 0x5FD787, 0x5FD7AF, 0x5FD7D7, 0x5FD7FF,
    0x5FFF00, 0x5FFF5F, 0x5FFF87, 0x5FFFAF, 0x5FFFD7, 0x5FFFFF,
    0x870000, 0x87005F, 0x870087, 0x8700AF, 0x8700D7, 0x8700FF,
    0x875F00, 0x875F5F, 0x875F87, 0x875FAF, 0x875FD7, 0x875FFF,
    0x878700, 0x87875F, 0x878787, 0x8787AF, 0x8787D7, 0x8787FF,
    0x87AF00, 0x87AF5F, 0x87AF87, 0x87AFAF, 0x87AFD7, 0x87AFFF,
    0x87D700, 0x87D75F, 0x87D787, 0x87D7AF, 0x87D7D7, 0x87D7FF,
    0x87FF00, 0x87FF5F, 0x87FF87, 0x87FFAF, 0x87FFD7, 0x87FFFF,
    0xAF0000, 0xAF005F, 0xAF0087, 0xAF00AF, 0xAF00D7, 0xAF00FF,
    0xAF5F00, 0xAF5F5F, 0xAF5F87, 0xAF5FAF, 0xAF5FD7, 0xAF5FFF,
    0xAF8700, 0xAF875F, 0xAF8787, 0xAF87AF, 0xAF87D7, 0xAF87FF,
    0xAFAF00, 0xAFAF5F, 0xAFAF87, 0xAFAFAF, 0xAFAFD7, 0xAFAFFF,
    0xAFD700, 0xAFD75F, 0xAFD787, 0xAFD7AF, 0xAFD7D7, 0xAFD7FF,
    0xAFFF00, 0xAFFF5F, 0xAFFF87, 0xAFFFAF, 0xAFFFD7, 0xAFFFFF,
    0xD70000, 0xD7005F, 0xD70087, 0xD700AF, 0xD700D7, 0xD700FF,
    0xD75F00, 0xD75F5F, 0xD75F87, 0xD75FAF, 0xD75FD7, 0xD75FFF,
    0xD78700, 0xD7875F, 0xD78787, 0xD787AF, 0xD787D7, 0xD787FF,
    0xD7AF00, 0xD7AF5F, 0xD7AF87, 0xD7AFAF, 0xD7AFD7, 0xD7AFFF,
    0xD7D700, 0xD7D75F, 0xD7D787, 0xD7D7AF, 0xD7D7D7, 0xD7D7FF,
    0xD7FF00, 0xD7FF5F, 0xD7FF87, 0xD7FFAF, 0xD7FFD7, 0xD7FFFF,
    0xFF0000, 0xFF005F, 0xFF0087, 0xFF00AF, 0xFF00D7, 0xFF00FF,
    0xFF5F00, 0xFF5F5F, 0xFF5F87, 0xFF5FAF, 0xFF5FD7, 0xFF5FFF,
    0xFF8700, 0xFF875F, 0xFF8787, 0xFF87AF, 0xFF87D7, 0xFF87FF,
    0xFFAF00, 0xFFAF5F, 0xFFAF87, 0xFFAFAF, 0xFFAFD7, 0xFFAFFF,
    0xFFD700, 0xFFD75F, 0xFFD787, 0xFFD7AF, 0xFFD7D7, 0xFFD7FF,
    0xFFFF00, 0xFFFF5F, 0xFFFF87, 0xFFFFAF, 0xFFFFD7, 0xFFFFFF,
    // 24 color grayscale ramp
    0x080808, 0x121212, 0x1C1C1C, 0x262626, 0x303030, 0x3A3A3A,
    0x444444, 0x4E4E4E, 0x585858, 0x606060, 0x666666, 0x767676,
    0x808080, 0x8A8A8A, 0x949494, 0x9E9E9E, 0xA8A8A8, 0xB2B2B2,
    0xBCBCBC, 0xC6C6C6, 0xD0D0D0, 0xDADADA, 0xE4E4E4, 0xEEEEEE
];

// Unicode block glyphs for 2x2 super resolution trick
static GLYPHS: [(char, [bool; 4]); 15] = [
    ('█', [true , true , true , true ]),
    ('▀', [true , true , false, false]),
    ('▄', [false, false, true , true ]),
    ('▌', [true , false, true , false]),
    ('▐', [false, true , false, true ]),
    ('▘', [true , false, false, false]),
    ('▝', [false, true , false, false]),
    ('▖', [false, false, true , false]),
    ('▗', [false, false, false, true ]),
    ('▚', [true , false, false, true ]),
    ('▞', [false, true , true , false]),
    ('▙', [true , false, true , true ]),
    ('▛', [true , true , true , false]),
    ('▜', [true , true , false, true ]),
    ('▟', [false, true , true , true ])
];

type RGBF = Vector3<f32>;

fn rgb24_to_rgbf(col: u32) -> RGBF {
    RGBF::new( ((col & 0xFF0000) >> 16) as f32 / 255.0
             , ((col & 0x00FF00) >>  8) as f32 / 255.0
             , ((col & 0x0000FF) >>  0) as f32 / 255.0
             )
}

fn rgbf_to_ansi256(col: RGBF, ansi16: bool) -> (i32, RGBF) {
    let mut best_diff = 1000.0;
    let mut best_idx  = 0;
    let mut best_col  = RGBF::new(0.0, 0.0, 0.0);
    // TODO: Inefficient brute-force search to find the best match
    // TODO: We're just 'simulating' the 16 color mode, as we're still using the 256 color
    //       indices, not 8-color mode plus the high-intensity flag
    for i in 0..(if ansi16 { 16 } else { 256 }) {
        let col24    = ANSI_256[i];
        let col_ansi = rgb24_to_rgbf(col24);
        // TODO: Poor color distance function
        let diffc    = col - col_ansi;
        let diff     = na::dot(&diffc, &diffc);

        if diff < best_diff {
            best_diff = diff;
            best_idx  = i;
            best_col  = col_ansi;
        }
    }
    (best_idx as i32, best_col)
}

fn rgbf_to_intensity(col: RGBF) -> f32 {
    col.x * 0.2989 + col.y * 0.5870 + col.z * 0.1140
}

fn load_image(file_name: &str) -> (Vec<RGBF>, usize, usize) {
    let path = path::Path::new(file_name);
    if !path.exists() {
        panic!("File not found: {}", file_name);
    }
    let img = match image::load(path) {
        image::LoadResult::ImageF32(_)  => panic!("Not LDR: {}", file_name),
        image::LoadResult::ImageU8(img) => img,
        image::LoadResult::Error(err)   => panic!("{}: {}", err, file_name)
    };

    // Convert to float
    let npix = img.width * img.height;
    let mut imgf = Vec::new();
    imgf.resize(npix, RGBF::new(0.0, 0.0, 0.0));
    for i in 0..npix {
        imgf[i] = RGBF::new(
            img.data[i * 4 + 0] as f32 / 255.0,
            img.data[i * 4 + 1] as f32 / 255.0,
            img.data[i * 4 + 2] as f32 / 255.0
        );
    }

    (imgf, img.width, img.height)
}

fn main() {
    // TODO: Add image resizing code. We need to resize the input to fit our terminal, and
    //       we also need to compensate for the fact that our pixels / characters are roughly
    //       twice as high as they are wide. Also, the super resolution methods needs an image
    //       four times the size .For now we just load an already resized image with the right
    //       pixel aspect

    let (imgf, width, height) = load_image("lena_64x32.png");

    println!("\n> No Color <\n");

    // Display image with 1-bit colors
    for y in 0..height {
        for x in 0..width {
            let pixel     = imgf[x + y * width];
            let intensity = rgbf_to_intensity(pixel);
            let character =
                if      intensity < 1.0 * (1.0 / 5.0) { " " }
                else if intensity < 2.0 * (1.0 / 5.0) { "░" }
                else if intensity < 3.0 * (1.0 / 5.0) { "▒" }
                else if intensity < 4.0 * (1.0 / 5.0) { "▓" }
                else                                  { "█" };

            print!("{}", character);
        }
        println!("");
    }

    println!("");

    // Display image with 4/8/24-bit colors
    for mode in 0..3 {
        match mode {
            0 => println!("> 4-bit Color <\n"),
            1 => println!("> 8-bit Color <\n"),
            2 => println!("> 24-bit Color <\n"),
            _ => panic!("Invalid mode")
        }
        for y in 0..height {
            for x in 0..width {
                let pixel = imgf[x + y * width];
                if mode == 2 {
                    print!("\x1b[48;2;{};{};{}m ",
                        (pixel.x * 255.0) as i32,
                        (pixel.y * 255.0) as i32,
                        (pixel.z * 255.0) as i32);
                } else {
                    let (col_idx, _) = rgbf_to_ansi256(pixel, mode == 0);
                    print!("\x1b[48;5;{}m ", col_idx);
                }
            }
            println!("\x1b[0m");
        }
        println!("");
    }

    let (imgf, width, height) = load_image("lena_128x64.png");

    // Display image with 8/24-bit color and 2x2 super resolution trick
    for mode in 0..2 {
        if mode == 0 {
            println!("> 8-bit Color, 2x2 Super Resolution Trick <\n")
        } else {
            println!("\n> 24-bit Color, 2x2 Super Resolution Trick <\n")
        }

        for y in 0..height / 2 {
            for x in 0..width / 2 {
                // Indices of the four pixels in our 2x2 block
                let col_idx = [
                    x * 2 + 0 + (y * 2 + 0) * width,
                    x * 2 + 1 + (y * 2 + 0) * width,
                    x * 2 + 0 + (y * 2 + 1) * width,
                    x * 2 + 1 + (y * 2 + 1) * width
                ];

                // Glyph and colors with the lowest error
                let mut best_bg    = RGBF::new(0.0, 0.0, 0.0); 
                let mut best_fg    = RGBF::new(0.0, 0.0, 0.0);  
                let mut best_diff  = 10000000.0;
                let mut best_glyph = ' ';

                // Find the best glyph and its colors to represent the block
                // TODO: This is very inefficient
                for glyph in GLYPHS.iter() {
                    let mut fg        = RGBF::new(0.0, 0.0, 0.0); 
                    let mut bg        = RGBF::new(0.0, 0.0, 0.0); 
                    let mut fg_weight = 0.0;
                    let mut bg_weight = 0.0;

                    // Compute colors from coverage of current glyph
                    // TODO: We do averaging in non-linear space, de-gamma first
                    for pix_idx in 0..4 {
                        if glyph.1[pix_idx] {
                            // Foreground
                            fg         = fg + imgf[col_idx[pix_idx]];
                            fg_weight += 1.0;
                        } else {
                            // Background
                            bg         = bg + imgf[col_idx[pix_idx]];
                            bg_weight += 1.0;
                        }
                    }

                    // Compute colors, be sure to compute the error based on the actual
                    // ANSI approximations
                    // TODO: This is especially inefficient for 8-bit
                    fg = if mode == 0 {
                             rgbf_to_ansi256(fg / fg_weight, false).1
                         } else {
                             fg / fg_weight
                         };
                    bg = if mode == 0 {
                             rgbf_to_ansi256(bg / bg_weight, false).1
                         } else {
                             bg / bg_weight
                         };

                    // Error for all pixels
                    let mut diff = 0.0;
                    for pix_idx in 0..4 {
                        let col_pix   = imgf[col_idx[pix_idx]];
                        let col_glyph = if glyph.1[pix_idx] { fg } else { bg };
                        // TODO: Poor color distance function
                        let diffc     = col_pix - col_glyph;
                        diff += na::dot(&diffc, &diffc).sqrt();
                    }

                    // Best so far?
                    if diff < best_diff {
                        best_fg    = fg;
                        best_bg    = bg;
                        best_diff  = diff;
                        best_glyph = glyph.0;
                    }
                }

                // Set foreground, background and output glyph
                if mode == 0 {
                    print!("\x1b[38;5;{}m\x1b[48;5;{}m{}",
                        rgbf_to_ansi256(best_fg, false).0,
                        rgbf_to_ansi256(best_bg, false).0,
                        best_glyph);
                } else {
                    print!("\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m{}",
                        (best_fg.x * 255.0) as i32,
                        (best_fg.y * 255.0) as i32,
                        (best_fg.z * 255.0) as i32,
                        (best_bg.x * 255.0) as i32,
                        (best_bg.y * 255.0) as i32,
                        (best_bg.z * 255.0) as i32,
                        best_glyph);
                }

            }
            println!("\x1b[0m");
        }
    }

    println!("");
}

