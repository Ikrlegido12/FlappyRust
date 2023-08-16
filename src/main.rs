extern crate piston_window;
use piston_window::*;
use rand::Rng;
use std::time::{Instant, Duration};

struct GameState {
    y_position: f64,
    y_velocity: f64,
    objects: Vec<Object>,
    is_game_over: bool,
    last_object_spawn_time: Instant,
}

struct Object {
    x_position: f64,
    y_position: f64,
    width: f64,
    height: f64,
}

impl GameState {
    fn new() -> Self {
        GameState {
            y_position: 100.0,          //Inicio de juego
            y_velocity: 0.0,
            objects: vec![],
            is_game_over: false,
            last_object_spawn_time: Instant::now(),
        }
    }

    fn update(&mut self, dt: f64) {
        if self.is_game_over {
            return;
        }

        self.y_velocity += 9.8 * dt;
        self.y_position += self.y_velocity;

        if self.y_position > 550.0 {
            self.y_position = 550.0;
            self.y_velocity = 0.0;
        }


        if self.y_position < 0.0 {
            self.y_position = 0.0;
            self.y_velocity = 0.0;
        }


        for obj in &mut self.objects {
            obj.x_position -= 1.0; // Mueve los objetos hacia la izquierda

            // Detectar colisión con objetos
            if self.y_position + 50.0 > obj.y_position
                && self.y_position < obj.y_position + obj.height
                && 100.0 + 50.0 > obj.x_position
                && 100.0 < obj.x_position + obj.width
            {
                self.is_game_over = true;
            }
        }

        // Eliminar objetos que están fuera de la pantalla
        self.objects.retain(|obj| obj.x_position + obj.width > 0.0);

        // Spawn de objetos con intervalo aleatorio
        if self.last_object_spawn_time.elapsed() >= Duration::from_secs_f64(1.5) {
            self.spawn_random_object();
            self.last_object_spawn_time = Instant::now();
        }
    }

    fn render(&self, c: Context, g: &mut G2d) {
        clear([1.0; 4], g);

        rectangle(
            [1.0, 0.0, 0.0, 1.0],
            [100.0, self.y_position, 50.0, 50.0],
            c.transform,
            g,
        );

        for obj in &self.objects {
            rectangle(
                [0.0, 1.0, 0.0, 1.0],
                [obj.x_position, obj.y_position, obj.width, obj.height],
                c.transform,
                g,
            );
        }
    }

    fn jump(&mut self) {
        if !self.is_game_over {
            self.y_velocity = -5.0; // Ajusta la altura del salto a tu gusto
        }
    }

    fn restart(&mut self) {
        self.y_position = 100.0;
        self.y_velocity = 0.0;
        self.objects.clear();
        self.is_game_over = false;
        self.last_object_spawn_time = Instant::now();
    }

    fn add_object(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.objects.push(Object {
            x_position: x,
            y_position: y,
            width,
            height,
        });
    }

   fn spawn_random_object(&mut self) {
        let mut rng = rand::thread_rng();
        
        // Ajustar el rango de generación aleatoria para la posición Y
        let y_position = rng.gen_range(0.0..500.0);  // Incluye valores cercanos a cero
        
        let width = rng.gen_range(20.0..50.0);
        let height = rng.gen_range(20.0..50.0);
        self.add_object(800.0, y_position, width, height);
    }
}



fn main() {
    let mut window: PistonWindow = WindowSettings::new("FlappyRust", [1366, 768])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut state = GameState::new();
    let is_fullscreen = true; 

    while let Some(event) = window.next() {
        if let Some(args) = event.update_args() {
            state.update(args.dt);
        }

        if let Some(args) = event.render_args() {
            window.draw_2d(&event, |c, g, _| {
                state.render(c, g);
            });
        }

        if let Some(Button::Keyboard(Key::Space)) = event.press_args() {
            state.jump();
        }

        if state.is_game_over && event.press_args().is_some() {
            state.restart();
        }
    }
}