

use iced::{Background, Border, Color, Element, Length, Padding, alignment::{Horizontal, Vertical}, border::Radius, widget::{self, button, container, space::horizontal}};
use roxmltree::Node;
use crate::{define_struct_with_names, parser::{ParseError, tags::{Tag, check_duplicate_attributes, extract_attribute, extract_children}, types::Pixel}, render::Message};







define_struct_with_names!{
    #[derive(Debug)]
    pub struct Bttn {
        pub child: Box<Tag>,
        pub on_click: String,

        // container
        pub width: Length, 
        pub height: Length,
        pub padding: Padding,
        pub bg_color: Color,
        pub border_color: Color,
        pub border_width: Pixel,
        pub border_radius: Radius,
    }
}


impl Bttn {
    pub fn from_node_ctx<'a, 'input>(node: Node<'a, 'input>, path: &mut Vec<String>) -> Result<Self, ParseError> {
        path.push(String::from("bttn"));
        
        // check_duplicate_attributes(node.attributes()).map_err(|e| ParseError::DuplicatedAttribute { attribute_name: e.to_string(), tag_path: path.clone(), range: node.range() })?;
        if let Some(invalid_attr) = node.attributes().find(|attr| !Self::ALL_FIELDS.contains(&attr.name())){
            return Err(ParseError::InvalidAttributeName { attribute_name: invalid_attr.name().to_string(), tag_path: path.clone(), range: node.range() });
        }
        
        Ok(Self{
            child: Box::new(extract_children(&node).into_iter().map(|node| Tag::from_node_ctx(node, path) ).collect::<Result<Vec<Tag>,ParseError>>()?.into_iter().next().unwrap_or_default()),
            on_click: node.attribute(Self::on_click).unwrap_or_default().to_string(),
            // container
            width: extract_attribute(node, Self::width, &path)?.unwrap_or(Length::Shrink),
            height: extract_attribute(node, Self::height, &path)?.unwrap_or(Length::Shrink),
            padding: extract_attribute(node, Self::padding, &path)?.unwrap_or(Padding::ZERO),
            bg_color: extract_attribute(node, Self::bg_color, &path)?.unwrap_or(Color::TRANSPARENT),
            border_color: extract_attribute(node, Self::border_color, &path)?.unwrap_or(Color::TRANSPARENT),
            border_width: extract_attribute(node, Self::border_width, &path)?.unwrap_or(0 as f32),
            border_radius: extract_attribute(node, Self::border_radius, &path)?.unwrap_or(Radius::default())
        })
    }

    pub fn render<'a>(&self) -> Element<'a, Message> {
        let style = button::Style {
            background: Some(Background::Color(self.bg_color)),
            border: Border {
                color: self.border_color,
                width: self.border_width,
                radius: self.border_radius
            },
            ..Default::default()
        };

        iced::widget::button( self.child.render())
            .on_press(Message::Callback("bttn".to_string(),self.on_click.clone()))
        .width(self.width)
        .height(self.height)
        .padding(self.padding)
        .style(move |_theme,_status| style)
        .into()
    }
}


