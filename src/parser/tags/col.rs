use iced::{Background, Border, Color, Element, Length, Padding, border::Radius, keyboard::key::Named::Select, widget::{container, iced}};
use roxmltree::Node;
use crate::{define_struct_with_names, parser::{ParseError, tags::{Tag, check_duplicate_attributes, extract_attribute, extract_children}, types::Pixel}, render::Message};







define_struct_with_names!{
    #[derive(Debug)]
    pub struct Col {
        pub children: Vec<Tag>,
        pub spacing: Pixel,

        // container
        pub width: Length,
        pub height: Length,
        pub padding: Padding,
        pub bg_color: Color,
        pub border_width: Pixel,
        pub border_color: Color,
        pub border_radius: Radius,
    }
}

impl Col {
    pub fn from_node_ctx<'a, 'input>(node: Node<'a, 'input>, path: &mut Vec<String>) -> Result<Self, ParseError> {
        path.push("col".to_string());
        
        // check_duplicate_attributes(node.attributes()).map_err(|e| ParseError::DuplicatedAttribute { attribute_name: e.to_string(), tag_path: path.clone(), range: node.range() })?;
        if let Some(invalid_attr) = node.attributes().find(|attr| !Self::ALL_FIELDS.contains(&attr.name())){
            return Err(ParseError::InvalidAttributeName { attribute_name: invalid_attr.name().to_string(), tag_path: path.clone(), range: node.range() });
        }
        
        Ok(Self{
            children: extract_children(&node).into_iter().map(|node| Tag::from_node_ctx(node, path) ).collect::<Result<Vec<Tag>,ParseError>>()?,
            spacing: extract_attribute(node, Self::spacing, &path)?.unwrap_or(0 as f32),
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

        let style = container::Style {
            background: Some(Background::Color(self.bg_color)),
            border: Border {
                color: self.border_color,
                width: self.border_width,
                radius: self.border_radius
            },
            ..Default::default()
        };
        
        container(
            iced::widget::column(self.children.iter().map(|tag| tag.render()))
            .spacing(self.spacing)
        )
        .width(self.width)
        .height(self.height)
        .padding(self.padding)
        .style(move |_theme| style)
        .into()
    }

}




