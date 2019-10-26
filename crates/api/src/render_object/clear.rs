use crate::{
    prelude::*,
    utils::{Brush, Point},
};

pub struct ClearRenderObject;

impl Into<Box<dyn RenderObject>> for ClearRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for ClearRenderObject {
    fn render_self(&self, context: &mut Context<'_>, _: &Point) {
        let background = context.widget().get::<Brush>("background").clone();

        context.render_context_2_d().clear(&background);
    }
}
