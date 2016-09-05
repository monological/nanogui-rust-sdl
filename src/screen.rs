extern crate sdl2;
extern crate sdl2_sys;
extern crate nanovg;

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use widget::{Widget, WidgetObj};
use theme::Theme;
use layout::Layout;
use window::Window;

pub struct Screen {
    widget: WidgetObj,
    nanovg_context: nanovg::Context,
    //focuspath?
    framebuffer_size: (u32, u32),
    pixel_ratio: f32,
    mouse_state: i32,
    modifiers: i32,
    mouse_pos: (u32, u32),
    drag_active: bool,
    drag_widget: Option<WidgetObj>,
    last_interaction: u32,
    process_events: bool,
    background: (f32, f32, f32),
    caption: String
}

impl Widget for Screen {
    fn parent(&self) -> Option<&Weak<RefCell<Widget>>> {
        self.widget.parent.as_ref()
    }

    unsafe fn set_parent(&mut self, parent: Option<Rc<RefCell<Widget>>>) {
        self.widget.parent = match parent {
            Some(val) => { Some(Rc::downgrade(&val)) },
            None => None
        }
    }

    fn children(&self) -> Vec<Rc<RefCell<Widget>>> {
        self.widget.children.clone()
    }

    unsafe fn children_mut(&mut self) -> &mut Vec<Rc<RefCell<Widget>>> {
        &mut self.widget.children
    }

    fn id(&self) -> String {
        self.widget.id.clone()
    }

    fn pos(&self) -> (u32, u32) {
        self.widget.pos
    }

    fn set_pos(&mut self, p: (u32, u32)) {
        self.widget.pos = p;
    }

    fn size(&self) -> (u32, u32) {
        self.widget.size
    }

    fn set_size(&mut self, s: (u32, u32)) {
        self.widget.size = s;
    }

    fn fixed_size(&self) -> (u32, u32) {
        self.widget.fixed_size
    }

    fn set_fixed_size(&mut self, s: (u32, u32)) {
        self.widget.fixed_size = s;
    }

    fn font_size(&self) -> u32 {
        self.widget.font_size()
    }

    fn set_font_size(&mut self, s: Option<u32>) {
        self.widget.font_size = s;
    }

    fn theme(&self) -> Option<&Rc<RefCell<Theme>>> {
        self.widget.theme.as_ref()
    }

    fn set_theme(&mut self, theme: Option<Rc<RefCell<Theme>>>) {
        self.widget.theme = theme;
    }

    fn enabled(&self) -> bool {
        self.widget.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.widget.enabled = enabled;
    }

    fn tooltip(&self) -> String {
        self.widget.tooltip.clone()
    }

    fn set_tooltip(&mut self, tooltip: String) {
        self.widget.tooltip = tooltip;
    }

    fn visible(&self) -> bool {
        self.widget.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.widget.visible = visible;
    }

    fn focused(&self) -> bool {
        self.widget.focused
    }

    fn set_focused(&mut self, focused: bool) {
        self.widget.focused = focused;
    }

    fn layout(&self) -> Option<&Box<Layout>> {
        self.widget.layout.as_ref()
    }

    fn set_layout(&mut self, layout: Option<Box<Layout>>) {
        self.widget.layout = layout;
    }

    fn preferred_size(&self, _: &nanovg::Context) -> (u32, u32) {
        (0, 0)
    }

    fn draw(&self, nanovg_context: &nanovg::Context) {
        for child in &self.widget.children {
            let p_mut = child.borrow_mut();
            p_mut.draw(nanovg_context);
        }
    }

    fn absolute_position(&self) -> (u32, u32) {
        if let Some(ref val) = self.widget.parent {
            if let Some(ref val_upgraded) = val.upgrade() {
                let (par_x, par_y) = val_upgraded.borrow().absolute_position();
                return (par_x + self.widget.pos.0, par_y + self.widget.pos.1)
            }
        }

        return self.widget.pos.clone();
    }

    fn visible_recursive(&self) -> bool {
        if !self.widget.visible {
            return false
        }

        if let Some(ref val) = self.widget.parent {
            if let Some(ref val_upgraded) = val.upgrade() {
                return val_upgraded.borrow().visible_recursive();
            }
        }

        return true;
    }

    fn contains(&self, p: (u32, u32)) -> bool {
        return p.0 >= self.widget.pos.0 && p.1 >= self.widget.pos.1 && p.0 < self.widget.pos.0 + self.widget.size.0 && p.1 < self.widget.pos.1 + self.widget.size.1;
    }

    fn as_window(&self) -> Option<&Window> {
        None
    }
}

impl Screen {
    pub fn new(id: String, caption: String, window: &mut sdl2::video::WindowRef) -> Screen {

        let winsize = window.size();
        window.set_title(&caption);

        unsafe {
            let mut screen: Screen = Screen {
                widget: WidgetObj::new(id),
                nanovg_context: nanovg::Context::create_gl3(nanovg::ANTIALIAS | nanovg::STENCIL_STROKES),
                caption: caption,
                framebuffer_size: winsize,
                mouse_pos: (0, 0),
                mouse_state: 0,
                modifiers: 0,
                drag_active: false,
                drag_widget: None,
                last_interaction: sdl2_sys::sdl::SDL_GetTicks(),
                process_events: true,
                background: (0.3, 0.3, 0.3),
                pixel_ratio: 0.0
            };

            screen.set_size(winsize);
            screen
        }
    }

    pub fn draw_widgets(&self) {
        if !self.widget.visible {
            return
        }

        self.nanovg_context.begin_frame(self.widget.size.0, self.widget.size.1, 1.0);

        self.draw(&self.nanovg_context);

        self.nanovg_context.end_frame();
    }

    pub fn set_background(&mut self, background_color: (f32, f32, f32)) {
        self.background = background_color;
    }

    pub fn nanovg_context(&self) -> &nanovg::Context {
        &self.nanovg_context
    }
}
