use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::rc::Rc;

use dces::{Entity, EntityComponentManager, System};

use backend::Backend;
use render_object::RenderObject;
use structs::{Point, Rect};
use theme::Selector;
use tree::Tree;
use widget::{Offset, WidgetContainer};

pub struct RenderSystem {
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<RenderObject>>>>,
    pub backend: Rc<RefCell<Backend>>,
    pub update: Rc<Cell<bool>>,
    pub debug_flag: Rc<Cell<bool>>,
}

impl System<Tree> for RenderSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        if !self.update.get() {
            return;
        }

        let mut backend = self.backend.borrow_mut();
        let render_context = backend.render_context();

        let mut offsets = BTreeMap::new();
        offsets.insert(tree.root, (0, 0));

        // render window background
        render_context.renderer.render(&render_context.theme);

        for node in tree.into_iter() {
            let mut global_position = Point::default();

            if let Some(offset) = offsets.get(&tree.parent[&node]) {
                global_position = Point::new(offset.0, offset.1);
            }

            let offset = {
                // get offset from scrollable parent
                if let Ok(offset) = ecm.borrow_component::<Offset>(tree.parent[&node])
                {
                    Point::new(offset.0, offset.1)
                } else {
                    Point::default()
                }
            };

            // render debug border for each widget
            if self.debug_flag.get() {
                if let Ok(bounds) = ecm.borrow_component::<Rect>(node) {
                    if let Ok(parent_bounds) = ecm.borrow_component::<Rect>(tree.parent[&node]) {
                        render_context.renderer.render_rectangle(
                            &render_context.theme,
                            bounds,
                            parent_bounds,
                            &Selector::new(Some(String::from("debugborder"))),
                            &offset,
                            &global_position,
                        );
                    }
                }
            }

            if let Some(render_object) = self.render_objects.borrow().get(&node) {
                render_object.render(
                    render_context.renderer,
                    &WidgetContainer::new(node, ecm, tree),
                    &render_context.theme,
                    &offset,
                    &global_position,
                );
            }

            let mut global_pos = (0, 0);

            if let Ok(bounds) = ecm.borrow_component::<Rect>(node) {
                global_pos = (global_position.x + bounds.x, global_position.y + bounds.y);
                offsets.insert(node, global_pos);
            }

            if let Ok(g_pos) = ecm.borrow_mut_component::<Point>(node) {
                g_pos.x = global_pos.0;
                g_pos.y = global_pos.1;
            }
        }
    }
}
