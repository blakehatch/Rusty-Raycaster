use std::f64::consts::PI;
use error_iter::ErrorIter as _;
use log::{debug, error};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit_input_helper::WinitInputHelper;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use raw_window_handle::HasWindowHandle;



const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;

// This would typically be in a separate file or module
struct Player {
    x: f64,
    y: f64,
    angle: f64,
    fov: f64,
}

impl Player {
    fn new(x: f64, y: f64, angle: f64, fov: f64) -> Self {
        Player { x, y, angle, fov }
    }

    fn rotate(&mut self, angle: f64) {
        self.angle = (self.angle + angle) % (2.0 * PI);
    }

    fn move_forward(&mut self, distance: f64) {
        let dx = distance * self.angle.cos();
        let dy = distance * self.angle.sin();
        self.x += dx;
        self.y += dy;
    }

    fn move_backward(&mut self, distance: f64) {
        self.move_forward(-distance);
    }
}

// This would typically be in a separate file or module
struct Map {
    data: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(data: Vec<Vec<u8>>) -> Self {
        let height = data.len();
        let width = data[0].len();
        Map { data, width, height }
    }

    fn get_at(&self, x: usize, y: usize) -> u8 {
        if x >= self.width || y >= self.height {
            return 0;
        }
        self.data[y][x]
    }
}

// This function could be moved to a separate file or module
fn raycast(player: &Player, map: &Map, width: usize, height: usize) -> Vec<u8> {
    let mut buffer = vec![0; width * height];

    for x in 0..width {
        let ray_angle = player.angle - player.fov / 2.0 + player.fov * x as f64 / width as f64;

        let mut distance = 0.0;
        let mut hit = false;
        let mut map_x = player.x as isize;
        let mut map_y = player.y as isize;

        loop {
            distance += 1.0;

            let ray_x = player.x + distance * ray_angle.cos();
            let ray_y = player.y + distance * ray_angle.sin();

            map_x = ray_x as isize;
            map_y = ray_y as isize;

            if map.get_at(map_x as usize, map_y as usize) == 1 {
                hit = true;
                break;
            }
        }

        let ceiling = (height / 2) as usize;
        let wall_height = (height as f64 / distance) as usize;
        let start = (ceiling - wall_height / 2).max(0);
        let end = start + wall_height.min(height - start);

        for y in 0..height {
            let idx = y * width + x;
            if y >= start && y < end {
                buffer[idx] = 255;
            } else {
                buffer[idx] = 0;
            }
        }
    }

    buffer
}

fn main() {
    let map_data = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let map = Map::new(map_data);
    let mut player = Player::new(4.0, 4.0, PI / 4.0, PI / 2.0);
    let width = 640;
    let height = 480;

    env_logger::init();
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let mut input = WinitInputHelper::new();

    let window = Window::new(&event_loop).unwrap();
    let size = window.inner_size();

    let mut pixels = {
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
        
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();
    };


    // Rendering loop (you would typically use a graphics library or framework)
    event_loop.run(move |event, control_flow| {
        let buffer = raycast(&player, &map, width, height);

        // Display the buffer (e.g., using a graphics library or writing to a file)
        // ...

        // Handle user input (e.g., move the player)
        // ...

        // Example: Rotate the player
        player.rotate(0.1);
    }).unwrap();
}
