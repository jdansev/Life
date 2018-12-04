extern crate piston_window;
extern crate image as im;

use piston_window::*;

const BLOCK_SIZE: f64 = 5.0;
const GAME_WIDTH: u32 = 120;
const GAME_HEIGHT: u32 = 120;
const WINDOW_WIDTH: u32 = GAME_WIDTH * BLOCK_SIZE as u32;
const WINDOW_HEIGHT: u32 = GAME_HEIGHT * BLOCK_SIZE as u32;

const GRAY: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const DARK: [f32; 4] = [0.1, 0.1, 0.1, 1.0];

mod universe;
mod patterns;
mod spaceships;
mod oscillators;
mod guns;
mod still_lifes;

use still_lifes::Block;
use still_lifes::Beehive;
use still_lifes::Loaf;
use still_lifes::Boat;
use still_lifes::Tub;

use oscillators::Blinker;
use oscillators::Beacon;
use oscillators::Pulsar;
use oscillators::Pentadecathlon;
use oscillators::KoksGalaxy;

use universe::Universe;
use patterns::Pattern;
use spaceships::Glider;
use spaceships::EdgeRepair1;
use oscillators::Toad;
use guns::GospersGliderGun;


fn main() {

    let mut universe = Universe::new(GAME_WIDTH, GAME_HEIGHT);

    let mut _edge_repair1: EdgeRepair1 = Pattern::new(60, 10);
    let mut _edge_repair2: EdgeRepair1 = Pattern::new(110, 50);
    let mut _gun1: GospersGliderGun = Pattern::new(5, 5);
    let mut _gun2: GospersGliderGun = Pattern::new(50, 45);
    let mut _gun3: GospersGliderGun = Pattern::new(30, 75);
    let mut _glider: Glider = Pattern::new(15, 22);
    let mut _toad1: Toad = Pattern::new(26, 20);
    let mut _toad2: Toad = Pattern::new(30, 49);
    let mut _toad2: Toad = Pattern::new(50, 20);
    
    universe.add_pattern(&mut _glider);
    universe.add_pattern(&mut _toad1);
    universe.add_pattern(&mut _toad2);
    universe.add_pattern(&mut _edge_repair1);
    universe.add_pattern(&mut _edge_repair2);
    universe.add_pattern(&mut _gun1);
    universe.add_pattern(&mut _gun2);
    universe.add_pattern(&mut _gun3);

    let opengl = OpenGL::V3_2;
    let (width, height) = (WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut window: PistonWindow =
        WindowSettings::new("Game of Life", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let canvas = im::ImageBuffer::new(width, height);
    let mut texture: G2dTexture = Texture::from_image(
        &mut window.factory,
        &canvas,
        &TextureSettings::new()
    ).unwrap();



    let mut _block: Block = Pattern::new(5, 5);
    // _block.print();
    
    let mut _beehive: Beehive = Pattern::new(5, 5);
    // _beehive.print();

    let mut _loaf: Loaf = Pattern::new(5, 5);
    // _loaf.print();

    let mut _boat: Boat = Pattern::new(5, 5);
    // _boat.print();

    let mut _tub: Tub = Pattern::new(5, 5);
    // _tub.print();

    let mut _blinker: Blinker = Pattern::new(5, 5);
    // _blinker.print();

    let mut _beacon: Beacon = Pattern::new(5, 5);
    // _beacon.print();

    let mut _pulsar: Pulsar = Pattern::new(5, 20);
    // _pulsar.print();

    let mut _pentadecathlon: Pentadecathlon = Pattern::new(5, 5);
    // _pentadecathlon.print();

    let mut _koks_galaxy: KoksGalaxy = Pattern::new(5, 40);
    _koks_galaxy.print();

    universe.add_pattern(&mut _pulsar);
    universe.add_pattern(&mut _pentadecathlon);
    universe.add_pattern(&mut _koks_galaxy);

    while let Some(e) = window.next() {

        if let Some(_) = e.render_args() {
            texture.update(&mut window.encoder, &canvas).unwrap();

            window.draw_2d(&e, |c, g| {

                clear(DARK, g);
                image(&texture, c.transform, g);

                for hi in 0..GAME_HEIGHT {
                    let hp = hi as f64 * BLOCK_SIZE;

                    for wi in 0..GAME_WIDTH {
                        let wp = wi as f64 * BLOCK_SIZE;
                        if universe.cells()[universe.get_index(hi as i32, wi as i32)] {
                            rectangle(
                                WHITE,
                                [wp as f64, hp as f64, BLOCK_SIZE, BLOCK_SIZE],
                                c.transform,
                                g
                            );
                        }
                    }
                }

                universe.next_generation();
            });
        }

    }
}
