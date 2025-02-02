// RustPixel
// copyright zhouxin@tuyoogame.com 2022~2024

use crate::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind},
    render::{
        adapter::{Adapter, AdapterBase},
        buffer::Buffer,
        image::to_error,
        sprite::Sprites,
        style::{Color, Modifier, ModifierDiff},
    },
    util::Rand,
    LOGO_FRAME,
};
#[cfg(not(feature = "sdl"))]
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{DisableMouseCapture, EnableMouseCapture},
    event::{Event as CEvent, KeyCode as CKeyCode, MouseButton as CMouseButton},
    execute, queue,
    style::{
        Attribute as CAttribute, Color as CColor, Print, SetAttribute, SetBackgroundColor,
        SetForegroundColor,
    },
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use std::any::Any;
use std::io::{self, Write};
use std::time::Duration;

#[cfg(not(feature = "sdl"))]
pub struct CrosstermAdapter {
    pub writer: Box<dyn Write>,
    pub base: AdapterBase,
    pub rd: Rand,
}

#[cfg(not(feature = "sdl"))]
impl CrosstermAdapter {
    pub fn new(gn: &str) -> Self {
        let stdout = io::stdout();
        Self {
            writer: Box::new(stdout),
            base: AdapterBase::new(gn),
            rd: Rand::new(),
        }
    }
}

#[cfg(not(feature = "sdl"))]
impl Adapter for CrosstermAdapter {
    fn init(&mut self, w: u16, h: u16, _rx: f32, _ry: f32, _s: String) {
        self.set_size(w, h);
        // 检查终端尺寸，如果小于绘制需要的尺寸，则警告退出
        // check terminal size, warns and exits if the size is smaller than the required size
        let (width, height) = terminal::size().unwrap();
        if w > width || h > height {
            self.reset();
            panic!(
                "\n\nTerminal too small!\n\
                Render required size:(width: {}, height: {})\n\
                Terminal size:(width : {}, height: {}).\n\n",
                w, h, width, height
            );
        }
        enable_raw_mode().unwrap();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    }

    fn get_base(&mut self) -> &mut AdapterBase {
        &mut self.base
    }

    fn reset(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.writer, LeaveAlternateScreen, DisableMouseCapture).unwrap();
        self.show_cursor().unwrap();
    }

    fn cell_width(&self) -> f32 {
        0.0
    }

    fn cell_height(&self) -> f32 {
        0.0
    }

    fn hide_cursor(&mut self) -> Result<(), String> {
        to_error(execute!(self.writer, Hide))?;
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<(), String> {
        to_error(execute!(self.writer, Show))?;
        Ok(())
    }

    fn get_cursor(&mut self) -> Result<(u16, u16), String> {
        crossterm::cursor::position().map_err(|e| e.to_string())
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), String> {
        to_error(execute!(self.writer, MoveTo(x, y)))
    }

    fn poll_event(&mut self, timeout: Duration, es: &mut Vec<Event>) -> bool {
        if crossterm::event::poll(timeout).unwrap() {
            let e = crossterm::event::read().unwrap();
            if let Some(et) = input_events_from_cross(&e) {
                es.push(et);
            }
            match e {
                CEvent::Key(key) => match key.code {
                    CKeyCode::Char('q') => {
                        return true;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        false
    }

    fn render_buffer(
        &mut self,
        current_buffer: &Buffer,
        previous_buffer: &Buffer,
        _pix: &mut Sprites,
        stage: u32,
    ) -> Result<(), String> {
        if stage <= LOGO_FRAME {
            let w = current_buffer.area().width as u16;
            let x = w - self.rd.rand() as u16 % w;
            let y = current_buffer.area().height as u16 / 2;
            let cc = CColor::from((
                self.rd.rand() as u8,
                self.rd.rand() as u8,
                self.rd.rand() as u8,
            ));
            to_error(queue!(self.writer, MoveTo(0, y)))?;
            to_error(queue!(self.writer, Print("                                                                                                                     ")))?;
            to_error(queue!(self.writer, MoveTo(x, y)))?;
            to_error(queue!(self.writer, SetForegroundColor(cc)))?;
            to_error(queue!(self.writer, Print("...RustPixel...")))?;
            if stage == LOGO_FRAME {
                // clear screen
                to_error(queue!(self.writer, MoveTo(x, y)))?;
                to_error(queue!(
                    self.writer,
                    Print("                                 ")
                ))?;
                // reset pen color
                to_error(queue!(
                    self.writer,
                    SetForegroundColor(CColor::from((192, 192, 192)))
                ))?;
            }
            return Ok(());
        }
        let updates = previous_buffer.diff(current_buffer);

        let mut fg = Color::Reset;
        let mut bg = Color::Reset;
        let mut modifier = Modifier::empty();
        let mut last_pos: Option<(u16, u16)> = None;
        for (x, y, cell) in updates {
            // Move the cursor if the previous location was not (x - 1, y)
            if !matches!(last_pos, Some(p) if x == p.0 + 1 && y == p.1) {
                to_error(queue!(self.writer, MoveTo(x, y)))?;
            }
            last_pos = Some((x, y));
            if cell.modifier != modifier {
                let diff = ModifierDiff {
                    from: modifier,
                    to: cell.modifier,
                };
                to_error(diff.queue(&mut self.writer))?;
                modifier = cell.modifier;
            }
            if cell.fg != fg {
                let color = CColor::from(cell.fg);
                to_error(queue!(self.writer, SetForegroundColor(color)))?;
                fg = cell.fg;
            }
            if cell.bg != bg {
                let color = CColor::from(cell.bg);
                to_error(queue!(self.writer, SetBackgroundColor(color)))?;
                bg = cell.bg;
            }

            to_error(queue!(self.writer, Print(&cell.symbol)))?;
        }
        to_error(queue!(
            self.writer,
            SetForegroundColor(CColor::Reset),
            SetBackgroundColor(CColor::Reset),
            SetAttribute(CAttribute::Reset)
        ))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// 转换crossterm的输入事件为pixel输入事件，方便游戏采用统一接口处理
/// 键盘和鼠标事件的处理，请参考game/unblock/model.rs的handle_input方法
/// Convert crossterm I/O events to RustPixel event, for the sake of unified event processing
/// For keyboard and mouse event, please refer to the handle_input method in game/unblock/model.rs
#[cfg(not(feature = "sdl"))]
pub fn input_events_from_cross(e: &CEvent) -> Option<Event> {
    let mut mcte: Option<MouseEvent> = None;
    match e {
        CEvent::Key(key) => {
            let kc;
            match key.code {
                CKeyCode::Char(cc) => kc = cc,
                _ => {
                    return None;
                }
            }
            let cte = KeyEvent::new(KeyCode::Char(kc), KeyModifiers::NONE);
            return Some(Event::Key(cte));
        }
        CEvent::Mouse(mouse) => {
            let mk = match mouse.kind {
                crossterm::event::MouseEventKind::Down(b) => {
                    let eb = match b {
                        CMouseButton::Left => MouseButton::Left,
                        CMouseButton::Right => MouseButton::Right,
                        CMouseButton::Middle => MouseButton::Middle,
                    };
                    MouseEventKind::Down(eb)
                }
                crossterm::event::MouseEventKind::Up(b) => {
                    let eb = match b {
                        CMouseButton::Left => MouseButton::Left,
                        CMouseButton::Right => MouseButton::Right,
                        CMouseButton::Middle => MouseButton::Middle,
                    };
                    MouseEventKind::Up(eb)
                }
                crossterm::event::MouseEventKind::Drag(b) => {
                    let eb = match b {
                        CMouseButton::Left => MouseButton::Left,
                        CMouseButton::Right => MouseButton::Right,
                        CMouseButton::Middle => MouseButton::Middle,
                    };
                    MouseEventKind::Drag(eb)
                }
                crossterm::event::MouseEventKind::Moved => MouseEventKind::Moved,
                _ => MouseEventKind::Moved,
            };
            let cte = MouseEvent {
                kind: mk,
                column: mouse.column as u16,
                row: mouse.row as u16,
                modifiers: KeyModifiers::NONE,
            };
            mcte = Some(cte);
        }
        _ => {}
    }
    if let Some(mc) = mcte {
        return Some(Event::Mouse(mc));
    }
    None
}
