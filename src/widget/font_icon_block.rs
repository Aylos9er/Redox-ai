use crate::{
    layout::FontIconSizeLayout,
    // render_object::FontIconRenderObject,
    theme::Selector,
    widget::{Template, Widget},
};

/// The `FontIconBlock` widget is used to draw an font icon. It is not interactive.
///
/// # Properties
///
/// * `Selector` - CSS selector with  element name `fonticon`, used to request the theme of the font icon block.
///
/// # Others
///
/// * `ParentType`- None.
/// * `FontIconSizeLayout` - Used to layout the widget.
/// * `FontIconRenderObject` - Used to draw the text of the widget.
pub struct FontIconBlock;

impl Widget for FontIconBlock {
    fn create() -> Template {
        Template::default()
            .with_property(Selector::from("fonticon"))
            .with_layout(FontIconSizeLayout)
            // .with_render_object(FontIconRenderObject)
            .with_debug_name("FontIconBlock")
    }
}
