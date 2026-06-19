pub mod btn;
pub mod row;
pub mod t;
pub mod col;
pub mod window;
// pub mod _none;

#[derive(Clone)]
pub enum Tag {
    T(t::T),
    Btn(btn::Btn),
    Col(col::Col),
    Row(row::Row),
    // None(_none::None)
}

impl Tag {
    fn render(&self) -> iced::Element<crate::renderer::Message> {
        match self {
            Tag::T(text) => text.render(),
            Tag::Btn(button) => button.render(),
            Tag::Col(column) => column.render(),
            Tag::Row(row) => row.render(),
            // Tag::None(none) => none.render(),
        }
    }
}

macro_rules! impl_from_for_tag {
    ($from_type:ty, $variant:path) => {
        impl From<$from_type> for Tag {
            fn from(value: $from_type) -> Self {
                $variant(value)
            }
        }
    };
}

impl_from_for_tag!(btn::Btn, Tag::Btn);
impl_from_for_tag!(t::T, Tag::T);
impl_from_for_tag!(col::Col, Tag::Col);
impl_from_for_tag!(row::Row, Tag::Row);
