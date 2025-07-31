use eframe::egui::*;

pub trait InputHandler {
    fn is_clicked_by(&self, button: PointerButton) -> bool;
    fn is_dragged_by(&self, button: PointerButton) -> bool;
    fn is_hovered(&self) -> bool;
    fn is_mouse_down(&self, button: PointerButton) -> bool;
    fn is_mouse_up(&self, button: PointerButton) -> bool;

    fn get_mouse_pos(&self) -> Option<Pos2>;
    fn get_scroll_delta(&self) -> Vec2;
    fn get_drag_delta(&self) -> Vec2;

    fn update(&mut self, response: &Response, ctx: &Context);
}

pub struct UserInputHandler {
    response: Option<Response>,
    pointer: Option<PointerState>,
    pointer_pos: Option<Pos2>,
    scroll_delta: Option<Vec2>,
}

impl UserInputHandler {
    pub fn new() -> Self {
        UserInputHandler {
            response: None,
            pointer: None,
            pointer_pos: None,
            scroll_delta: None,
        }
    }
}

impl InputHandler for UserInputHandler {
    fn is_clicked_by(&self, button: PointerButton) -> bool {
        self.response
            .as_ref()
            .map(|r| r.clicked_by(button))
            .unwrap_or(false)
    }

    fn is_dragged_by(&self, button: PointerButton) -> bool {
        self.response
            .as_ref()
            .map(|r| r.dragged_by(button))
            .unwrap_or(false)
    }

    fn is_hovered(&self) -> bool {
        self.response.as_ref().map(|r| r.hovered()).unwrap_or(false)
    }

    fn is_mouse_down(&self, button: PointerButton) -> bool {
        self.pointer
            .as_ref()
            .map(|p| p.button_down(button))
            .unwrap_or(false)
    }

    fn is_mouse_up(&self, button: PointerButton) -> bool {
        self.pointer
            .as_ref()
            .map(|p| p.button_released(button))
            .unwrap_or(false)
    }

    fn get_mouse_pos(&self) -> Option<Pos2> {
        self.pointer_pos
    }

    fn get_scroll_delta(&self) -> Vec2 {
        self.scroll_delta.unwrap_or(Vec2::ZERO)
    }

    fn get_drag_delta(&self) -> Vec2 {
        self.response
            .as_ref()
            .map(|r| r.drag_delta())
            .unwrap_or(Vec2::ZERO)
    }

    fn update(&mut self, response: &Response, ctx: &Context) {
        self.response = Some(response.clone());
        self.pointer = Some(ctx.input(|i| i.pointer.clone()));
        self.pointer_pos = response.interact_pointer_pos();
        self.scroll_delta = Some(ctx.input(|i| i.raw_scroll_delta));
    }
}
