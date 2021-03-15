use crate::{api::prelude::*, proc_macros::*, theme_default::prelude::*};

// --- KEYS --

pub static STYLE_TEXT_BLOCK: &str = "text_block";

// --- KEYS --

enum TextAction {
    Localize,
}

/// Handles the localization of the text.
#[derive(Debug, Clone, Default, AsAny)]
pub struct TextBlockState;

impl TextBlockState {
    fn localize(&self, ctx: &mut Context) {
        if !*TextBlock::localizable_ref(&ctx.widget()) {
            return;
        }

        let text = TextBlock::text_clone(&ctx.widget());
        let localized_text = ctx.localize_text(text);

        TextBlock::localized_text_set(&mut ctx.widget(), localized_text);
    }
}

impl State for TextBlockState {
    fn init(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        self.localize(ctx);
    }

    fn update(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        self.localize(ctx);
    }

    fn messages(
        &mut self,
        mut messages: MessageReader,
        _registry: &mut Registry,
        ctx: &mut Context,
    ) {
        for message in messages.read::<TextAction>() {
            match message {
                TextAction::Localize => self.localize(ctx),
            }
        }
    }
}

widget!(
    /// The `TextBlock` widget is used to draw text. It is not interactive.
    ///
    /// **style:** `text_block`
    TextBlock<TextBlockState> {
        /// Sets or shares the foreground property.
        foreground: Brush,

        /// Sets or shares the font size property.
        font_size: f64,

        /// Sets or shares the font property.
        font: String,

        /// Defines if the text is localizable. If set to `false` the text will not be localized.
        localizable: bool,

        /// Support line wrapping using Ctrl-Enter key.
        line_wrap: bool,

        /// If the `TextBlock` is localizable and the localized text
        /// is not empty, the localized_text will be drawn.
        localized_text: String,

        /// Defines an extra offset that can be used to the text on x axis.
        offset: f64,

        /// Sets or shares the text property.
        text: String,

        /// Sets or shares the water_mark text property.
        water_mark: String

    }
);

impl Template for TextBlock {
    fn template(self, id: Entity, _: &mut BuildContext) -> Self {
        self.name("TextBlock")
            .style(STYLE_TEXT_BLOCK)
            .foreground(colors::LINK_WATER_COLOR)
            .font_size(fonts::FONT_SIZE_12)
            .font("Roboto-Regular")
            .line_wrap(true)
            .localizable(true)
            .text("")
            .on_changed("text", move |ctx, _| {
                ctx.send_message(TextAction::Localize, id)
            })
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        TextRenderObject.into()
    }

    fn layout(&self) -> Box<dyn Layout> {
        FixedSizeLayout::new().into()
    }
}
