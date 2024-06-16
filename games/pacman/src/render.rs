use crate::model::{PacmanModel, PACMANH, PACMANW};
use log::info;
#[cfg(any(feature = "sdl", target_arch = "wasm32"))]
use rust_pixel::render::cell::cellsym;
use rust_pixel::{
    asset::AssetType,
    asset2sprite,
    context::Context,
    event::{event_check, event_register, timer_fire, timer_register},
    game::{Model, Render},
    render::panel::Panel,
    render::sprite::{Sprite, Sprites},
    render::style::{Color, Style},
};

const COLORS: [Color; 14] = [
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Cyan,
    Color::Gray,
    Color::DarkGray,
    Color::LightRed,
    Color::LightGreen,
    Color::LightBlue,
    Color::LightYellow,
    Color::LightMagenta,
    Color::LightCyan,
];

pub struct PacmanRender {
    pub panel: Panel,
    pub main_scene: Sprites,
}

impl PacmanRender {
    #[allow(unused_mut)]
    pub fn new() -> Self {
        let mut t = Panel::new();
        let mut s = Sprites::new("main");
        let mut l = Sprite::new(0, 0, (PACMANW + 2) as u16, (PACMANH + 2) as u16);

        // #[cfg(feature = "sdl")]
        // let mut ssf = SequenceFrames::new();
        //#[cfg(feature = "sdl")]
        // ssf.load_ssffile("./assets/sdq/2.ssf");

        #[cfg(any(feature = "sdl", target_arch = "wasm32"))]
        {
            let mut pl = Sprite::new(4, 6, 1, 1);
            pl.content.set_str(
                0,
                0,
                cellsym(20),
                Style::default()
                    .fg(Color::Indexed(222))
                    .bg(Color::Indexed(1)),
            );
            t.add_pixel_sprite(pl, "PL1");
        }
        #[cfg(not(any(feature = "sdl", target_arch = "wasm32")))]
        l.content.set_str(
            18,
            0,
            "PACMAN [RustPixel]",
            Style::default().fg(Color::Indexed(222)),
        );

        // 测试画线
        #[cfg(any(feature = "sdl", target_arch = "wasm32"))]
        {
            let pdata = [
                [40, 40, 80, 40],
                //[80, 60, 40, 40],
                [40, 40, 80, 60],
                [40, 40, 80, 80],
                //[80, 100, 40, 40],
                [30, 30, 40, 45],
                [40, 40, 40, 80],
                [40, 40, 20, 80],
                [40, 40, 0, 80],
                //[0, 60, 40, 40],
                [40, 40, 0, 60],
                [40, 40, 0, 40],
                [40, 40, 0, 20],
                [40, 40, 0, 0],
                [40, 40, 20, 0],
                [40, 40, 40, 0],
                //[60, 0, 40, 40],
                [40, 40, 60, 0],
                [40, 40, 80, 0],
                [40, 40, 80, 20],
                [64, 98, 68, 29],
            ];
            for pi in 3..4 {
                let dy = pdata[pi][3] as f32 - pdata[pi][1] as f32;
                let dx = pdata[pi][2] as f32 - pdata[pi][0] as f32;
                let mut angle = dy.atan2(dx);
                if angle < 0.0 {
                    angle = angle + 3.1415926 * 2.0;
                }
                info!("line angle...{:?}", angle / 3.1415926 * 180.0);
                angle = angle / 3.1415926;
                if (angle > 0.0 && angle < 0.5)
                    || (angle > 0.75 && angle < 1.0)
                    || (angle > 1.5 && angle < 1.75)
                {
                    l.draw_line(
                        pdata[pi][2],
                        pdata[pi][3],
                        pdata[pi][0],
                        pdata[pi][1],
                        None,
                        222,
                        1,
                    );
                } else {
                    l.draw_line(
                        pdata[pi][0],
                        pdata[pi][1],
                        pdata[pi][2],
                        pdata[pi][3],
                        None,
                        222,
                        1,
                    );
                }
            }
        }

        /*
        //Test serde
        let serialized = bincode::serialize(&l.content).unwrap();
        l.content = bincode::deserialize(&serialized[..]).unwrap();
        let mut outf = File::create("tmp/pacman.out").unwrap();
        outf.write_all(&serialized).unwrap();
        info!("{:?}-{:?}", serialized, serialized.len());
        */

        s.add_by_tag(l, "PACMAN-BORDER");
        s.add_by_tag(Sprite::new(1, 1, PACMANW as u16, PACMANH as u16), "PACMAN");
        s.add_by_tag(
            Sprite::new(0, (PACMANH + 3) as u16, PACMANW as u16, 1u16),
            "PACMAN-MSG",
        );

        event_register("Pacman.RedrawGrid", "draw_grid");
        timer_register("Pacman.TestTimer", 0.1, "test_timer");
        timer_fire("Pacman.TestTimer", 8u8);

        Self {
            panel: t,
            // #[cfg(feature = "sdl")]
            // ssf: ssf,
            main_scene: s,
        }
    }

    pub fn draw_grid<G: Model>(&mut self, context: &mut Context, model: &mut G) {
        let d = model.as_any().downcast_ref::<PacmanModel>().unwrap();
        let ml = self.main_scene.get_by_tag("PACMAN-MSG");
        ml.content.set_str(0, 0, "pacman", Style::default());
        let l = self.main_scene.get_by_tag("PACMAN");
        info!("draw_grid...");
        for i in 0..PACMANH {
            for j in 0..PACMANW {
                let gv = d.grid[i][j];
                match gv {
                    0 => {
                        #[cfg(not(any(feature = "sdl", target_arch = "wasm32")))]
                        l.content.set_str(
                            j as u16,
                            i as u16,
                            " ",
                            Style::default().fg(Color::Reset).bg(Color::Reset),
                        );
                        #[cfg(any(feature = "sdl", target_arch = "wasm32"))]
                        l.content.set_str(
                            j as u16,
                            i as u16,
                            " ",
                            Style::default().fg(Color::Reset).bg(Color::Reset),
                        );
                    }
                    1 => {
                        #[cfg(not(any(feature = "sdl", target_arch = "wasm32")))]
                        l.content
                            .set_str(j as u16, i as u16, "▇", Style::default().fg(Color::LightGreen));
                        #[cfg(any(feature = "sdl", target_arch = "wasm32"))]
                        l.content.set_str(
                            j as u16,
                            i as u16,
                            cellsym(0),
                            Style::default().fg(Color::LightGreen).bg(Color::Red),
                        );
                    }
                    10000 => {
                        let c = COLORS[(context.stage / 5) as usize % COLORS.len()];
                        #[cfg(not(any(feature = "sdl", target_arch = "wasm32")))]
                        l.content
                            .set_str(j as u16, i as u16, "∙", Style::default().fg(c));
                        #[cfg(any(feature = "sdl", target_arch = "wasm32"))]
                        l.content.set_str(
                            j as u16,
                            i as u16,
                            cellsym(83),
                            Style::default().fg(c).bg(Color::Red),
                        );
                    }
                    _ => {
                        let c = COLORS[gv as usize % COLORS.len()];
                        #[cfg(not(any(feature = "sdl", target_arch = "wasm32")))]
                        l.content
                            .set_str(j as u16, i as u16, "▒", Style::default().fg(c));
                        #[cfg(any(feature = "sdl", target_arch = "wasm32"))]
                        l.content.set_str(
                            j as u16,
                            i as u16,
                            cellsym(102),
                            Style::default().fg(c).bg(Color::Red),
                        );
                    }
                }
            }
        }
    }
}

impl Render for PacmanRender {
    fn init<G: Model>(&mut self, context: &mut Context, _data: &mut G) {
        context.adapter.init(
            PACMANW as u16 + 2,
            PACMANH as u16 + 4,
            1.0,
            1.0,
            "pacman".to_string(),
        );
        self.panel.init(context);
    }

    fn handle_event<G: Model>(&mut self, context: &mut Context, data: &mut G, _dt: f32) {
        if event_check("Pacman.RedrawGrid", "draw_grid") {
            self.draw_grid(context, data);
        }
    }

    fn handle_timer<G: Model>(&mut self, context: &mut Context, _model: &mut G, _dt: f32) {
        if event_check("Pacman.TestTimer", "test_timer") {
            let ml = self.main_scene.get_by_tag("PACMAN-MSG");
            ml.content.set_str(
                (context.stage / 6) as u16 % PACMANW as u16,
                0,
                "pacman",
                Style::default().fg(Color::Yellow),
            );
            timer_fire("Pacman.TestTimer", 8u8);
        }
    }

    #[allow(unused_variables)]
    fn draw<G: Model>(&mut self, context: &mut Context, _data: &mut G, _dt: f32) {
        let ss = &mut self.main_scene.get_by_tag("PACMAN-BORDER");
        asset2sprite!(
            ss,
            context,
            "sdq/dance.ssf",
            (context.stage / 3) as usize,
            1,
            1
        );
        #[cfg(any(feature = "sdl", target_arch = "wasm32"))]
        if context.stage % 8 == 0 {
            let pl = self.panel.get_pixel_sprite("PL1");
            pl.content.area.x += 2;
            pl.content.area.y += 2;
        }
        self.panel
            .draw(context, |a, f| {
                self.main_scene.render_all(a, f);
            })
            .unwrap();
    }
}
