use iced::Element;

use crate::render::Message;




#[derive(Debug)]
pub struct Void {}
impl Void {
    pub fn render<'a>(&self) -> Element<'a, Message> {
        iced::widget::text("").into()
    }
}

