use log::debug;
use rand::prelude::*;
use std::any::Any;
use rust_pixel::event::{Event, KeyCode};
use rust_pixel::{
    context::Context,
    event::event_emit,
    game::Model,
    util::{Dir, Point},
};

pub const PACMANW: usize = 60;
pub const PACMANH: usize = 36;

enum PacmanState {
    Normal,
    OverSelf,
    OverBorder,
}

pub struct PacmanModel {
    pub grid: [[i16; PACMANW]; PACMANH],
    pub seed: Point,
    pub body: Vec<Point>,
    pub dir: Dir,
    pub timeout_auto: f32,
}

impl PacmanModel {
    pub fn new() -> Self {
        Self {
            grid: [[0i16; PACMANW]; PACMANH],
            seed: Point { x: 0, y: 0 },
            body: vec![],
            dir: Dir::Down,
            timeout_auto: 0.0,
        }
    }

    pub fn make_grid(&mut self) {
        for i in 0..PACMANH {
            for j in 0..PACMANW {
                self.grid[i][j] = 0i16;
            }
        }
        for i in 0..self.body.len() {
            self.grid[self.body[i].y as usize][self.body[i].x as usize] = (i + 1) as i16;
        }
        self.grid[self.seed.y as usize][self.seed.x as usize] = 10000i16;
    }

    pub fn act(&mut self, d: Dir, context: &mut Context) {
        let dx: i16;
        let dy: i16;
        let cx: i16;
        let cy: i16;
        match d {
            Dir::Up => {
                if self.dir == Dir::Down {
                    return;
                };
                dx = 0;
                dy = -1
            }
            Dir::Down => {
                if self.dir == Dir::Up {
                    return;
                };
                dx = 0;
                dy = 1
            }
            Dir::Left => {
                if self.dir == Dir::Right {
                    return;
                };
                dx = -1;
                dy = 0
            }
            Dir::Right => {
                if self.dir == Dir::Left {
                    return;
                };
                dx = 1;
                dy = 0
            }
            _ => {
                dx = 0;
                dy = 0
            }
        }
        cx = self.body[0].x as i16 + dx;
        cy = self.body[0].y as i16 + dy;
        if cx >= PACMANW as i16 || cy >= PACMANH as i16 || cx < 0 || cy < 0 {
            context.state = PacmanState::OverBorder as u8;
            event_emit("Pacman.RedrawGrid");
            return;
        }
        if self.grid[cy as usize][cx as usize] == 10000 {
            let mut rng = thread_rng();
            for i in 0..888 {
                let nx = rng.gen_range(0..PACMANW) as u16;
                let ny = rng.gen_range(0..PACMANH) as u16;
                let np = self.grid[ny as usize][nx as usize];
                //if np == 10000 || np == 0 {
                if np == 0 {
                    self.seed.x = nx;
                    self.seed.y = ny;
                    debug!("{:?} {:?} {:?} {:?}", i, nx, ny, np);
                    for j in 0..PACMANH {
                        debug!("{:?}", self.grid[j]);
                    }
                    break;
                }
            }
        } else {
            if self.grid[cy as usize][cx as usize] != 0 {
                context.state = PacmanState::OverSelf as u8;
                event_emit("Pacman.RedrawGrid");
                return;
            }
            self.body.pop();
        }
        self.body.splice(
            0..0,
            vec![Point {
                x: cx as u16,
                y: cy as u16,
            }],
        );
        self.dir = d;
        self.make_grid();
        event_emit("Pacman.RedrawGrid");
    }
}

impl Model for PacmanModel {
    fn init(&mut self, context: &mut Context) {
        self.body.clear();
        self.body.push(Point {
            x: PACMANW as u16 / 2,
            y: PACMANH as u16 / 2,
        });
        let mut rng = thread_rng();
        self.seed.x = rng.gen_range(0..PACMANW) as u16;
        self.seed.y = rng.gen_range(0..PACMANH) as u16;
        self.make_grid();
        self.dir = Dir::Down;
        context.input_events.clear();
        context.state = PacmanState::Normal as u8;
        event_emit("Pacman.RedrawGrid");
    }

    fn handle_input(&mut self, context: &mut Context, _dt: f32) {
        let es = context.input_events.clone();
        for e in &es {
            match e {
                Event::Key(key) => {
                    let mut d: Option<Dir> = None;
                    match key.code {
                        KeyCode::Char('w') => d = Some(Dir::Up),
                        KeyCode::Char('a') => d = Some(Dir::Left),
                        KeyCode::Char('s') => d = Some(Dir::Down),
                        KeyCode::Char('d') => d = Some(Dir::Right),
                        _ => {}
                    }
                    if d != None {
                        self.act(d.unwrap(), context);
                    }
                }
                _ => {}
            }
        }
        context.input_events.clear();
    }

    fn handle_auto(&mut self, context: &mut Context, dt: f32) {
        if self.timeout_auto > 0.4 {
            self.timeout_auto = 0.0;
            self.act(self.dir, context);
        } else {
            self.timeout_auto += dt;
        }
    }

    fn handle_event(&mut self, _context: &mut Context, _dt: f32) {}
    fn handle_timer(&mut self, _context: &mut Context, _dt: f32) {}

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
