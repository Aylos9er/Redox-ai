use crate::{prelude::*, utils::*};

property!(
    /// `BorderBrush` describes the border brush.
    #[derive(Default)]
    BorderBrush(Brush) : &str,
    String
);
