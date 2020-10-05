use crate::{api::prelude::*, proc_macros::*, Grid};

// --- KEYS --
static CONTENT_GRID: &str = "id_content_grid";
// --- KEYS --

/// Use this enum to trigger navigation actions on a master detail widget.
#[derive(Debug, Clone, PartialEq)]
pub enum MasterDetailAction {
    /// Shows the master. If the master is shown nothing will happen.
    ShowMaster,

    // Shows the detail. If the detail is shown nothing will happen
    ShowDetail,
}

// Internal actions.
enum InternalAction {
    Expand,
    Collapse,
}

/// Handles request and layout changed of the `MasterDetail` widget.
#[derive(Default, Clone, Debug, AsAny)]
pub struct MasterDetailState {
    content_grid: Entity,
    master: Option<Entity>,
    detail: Option<Entity>,
    expanded: bool,
    event_adapter: EventAdapter,
}

impl MasterDetailState {
    // sets the master and detail widget (entity)
    fn init_master_detail(&mut self, ctx: &mut Context) {
        if self.master.is_none() && self.detail.is_none() {
            return;
        }

        ctx.clear_children_of(self.content_grid);

        if let Some(master) = self.master {
            ctx.append_child_entity_to(master, self.content_grid);
            ctx.build_context()
                .register_property::<usize>("column", master, 0);
        }

        if let Some(detail) = self.detail {
            ctx.append_child_entity_to(detail, self.content_grid);
            ctx.build_context()
                .register_property::<usize>("column", detail, 0);
            ctx.get_widget(detail)
                .set("visibility", Visibility::Collapsed);
        }
    }

    // expands the widget (two column layout)
    fn expand(&mut self, ctx: &mut Context) {
        self.expanded = true;
        if let Some(master) = self.master {
            ctx.get_widget(master)
                .set("visibility", Visibility::Visible);
        }

        if let Some(detail) = self.detail {
            ctx.get_widget(detail)
                .set("visibility", Visibility::Visible);
            ctx.get_widget(detail).set::<usize>("column", 1);
        }

        let master_width = *MasterDetail::master_width_ref(&ctx.widget());

        Grid::columns_set(
            &mut ctx.get_widget(self.content_grid),
            Columns::create().push(master_width).push("*").build(),
        );
    }

    // collapse the widget (one column layout)
    fn collapse(&mut self, ctx: &mut Context) {
        self.expanded = false;

        if let Some(master) = self.master {
            ctx.get_widget(master)
                .set("visibility", Visibility::Visible);
        }

        if let Some(detail) = self.detail {
            ctx.get_widget(detail)
                .set("visibility", Visibility::Collapsed);
            ctx.get_widget(detail).set::<usize>("column", 0);
        }
        Grid::columns_set(
            &mut ctx.get_widget(self.content_grid),
            Columns::create().push("*").build(),
        );
    }

    fn int_show_master(&self, ctx: &mut Context) {
        if let Some(master) = self.master {
            ctx.get_widget(master)
                .set("visibility", Visibility::Visible);
        }

        if let Some(detail) = self.detail {
            ctx.get_widget(detail)
                .set("visibility", Visibility::Collapsed);
        }
    }

    fn int_show_detail(&self, ctx: &mut Context) {
        if let Some(master) = self.master {
            ctx.get_widget(master)
                .set("visibility", Visibility::Collapsed);
        }

        if let Some(detail) = self.detail {
            ctx.get_widget(detail)
                .set("visibility", Visibility::Visible);
        }
    }
}

impl State for MasterDetailState {
    fn init(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        self.content_grid = ctx.child(CONTENT_GRID).entity();
        self.event_adapter = ctx.event_adapter();
        self.init_master_detail(ctx)
    }

    fn messages(
        &mut self,
        mut messages: MessageReader,
        _registry: &mut Registry,
        ctx: &mut Context,
    ) {
        for action in messages.read::<MasterDetailAction>() {
            let responsive = *MasterDetail::responsive_ref(&ctx.widget());

            match action {
                MasterDetailAction::ShowMaster => {
                    if !self.expanded || !responsive {
                        self.int_show_master(ctx);
                    }
                }
                MasterDetailAction::ShowDetail => {
                    if !self.expanded || !responsive {
                        self.int_show_detail(ctx);
                    }
                }
            }
        }

        for action in messages.read::<InternalAction>() {
            match action {
                InternalAction::Expand => self.expand(ctx),
                InternalAction::Collapse => self.collapse(ctx),
            }
        }
    }

    fn update_post_layout(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        if !*MasterDetail::responsive_ref(&ctx.widget()) {
            return;
        }

        let width = ctx
            .get_widget(self.content_grid)
            .get::<Rectangle>("bounds")
            .width();
        let break_point: f64 = *MasterDetail::break_point_ref(&ctx.widget());

        if self.expanded && width <= break_point {
            ctx.send_message(ctx.entity(), InternalAction::Collapse);
            MasterDetail::navigation_visibility_set(&mut ctx.widget(), Visibility::Visible);
        } else if !self.expanded && width > break_point {
            ctx.send_message(ctx.entity(), InternalAction::Expand);
            MasterDetail::navigation_visibility_set(&mut ctx.widget(), Visibility::Hidden);
        }
    }
}

widget!(
    /// `MasterDetail` is a responsive navigation widget with a master child and a detail child.
    ///
    /// If `responsive` property is set to `true` if the width of the `MasterDetail` widget crosses the given `break_point` the widget switch between a one column
    /// and two column layout. On on column layout or if `responsive` is set to `false` navigation between master and details is possible.
    ///
    /// # Example
    ///
    /// ```rust
    /// MasterDetail::new()
    ///     .responsive(true)
    ///     .break_point(300)
    ///     .master_detail(TextBlock::new().text("Master").build(ctx), TextBlock::new().text("Detail").build(ctx))
    ///     .build(ctx);
    /// ```
    MasterDetail<MasterDetailState> {
        /// Describes if the change between a one and two column layout on the `break_point`.
        responsive: bool,

        /// Describes the switch point between the one and two column layout.
        break_point: f64,

        /// Describes the width of the master widget on `expanded` state.
        master_width: f64,

        /// Read the visibility of navigation. If `expanded` is `false` or `responsive` is false it's `Visible` otherwise `Hidden`.
        navigation_visibility: Visibility
    }
);

impl MasterDetail {
    /// Register a master and a detail widget (entity).
    pub fn master_detail(mut self, master: Entity, detail: Entity) -> Self {
        self.state_mut().master = Some(master);
        self.state_mut().detail = Some(detail);
        self
    }

    /// Shows the master widget. If the master widget is visible nothing will happen.
    pub fn show_master(ctx: &mut Context, entity: Entity) {
        MasterDetail::panics_on_wrong_type(&ctx.get_widget(entity));
        ctx.send_message(entity, MasterDetailAction::ShowMaster);
    }

    /// Shows the detail widget. If the detail widget is visible nothing will happen.
    pub fn show_detail(ctx: &mut Context, entity: Entity) {
        MasterDetail::panics_on_wrong_type(&ctx.get_widget(entity));
        ctx.send_message(entity, MasterDetailAction::ShowDetail);
    }
}

impl Template for MasterDetail {
    fn template(self, _: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MasterDetails")
            .master_width(374)
            .child(Grid::new().id(CONTENT_GRID).build(ctx))
    }
}
