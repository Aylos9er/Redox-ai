use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::rc::Rc;

use dces::{Entity, EntityComponentManager, System};

use backend::Backend;
use layout_object::LayoutObject;
use structs::{Constraint, Rect};
use theme::{Selector, Theme};
use tree::Tree;

pub enum LayoutResult {
    Size((u32, u32)),
    RequestChild(Entity, Constraint),
}

pub struct LayoutSystem {
    pub layout_objects: Rc<RefCell<BTreeMap<Entity, Box<LayoutObject>>>>,
    pub backend: Rc<RefCell<Backend>>,
    pub update: Rc<Cell<bool>>,
}

impl System<Tree> for LayoutSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        fn layout_rec(
            ecm: &mut EntityComponentManager,
            tree: &Tree,
            constraint: &Constraint,
            entity: Entity,
            theme: &Theme,
            layout_objects: &Rc<RefCell<BTreeMap<Entity, Box<LayoutObject>>>>,
        ) -> (u32, u32) {
            let mut size: Option<(u32, u32)> = None;

            let mut constraint = Constraint {
                min_width: constraint.min_width,
                min_height: constraint.min_height,
                max_width: constraint.max_width,
                max_height: constraint.max_height,
            };

            if let Ok(selector) = ecm.borrow_component::<Selector>(entity) {
                let min_width = theme.uint("min-width", selector);
                let max_width = theme.uint("min-width", selector);
                let min_height = theme.uint("min_height", selector);
                let max_height = theme.uint("max_height", selector);
                let width = theme.uint("width", selector);
                let height = theme.uint("height", selector);

                if min_width > 0 {
                    constraint.min_width = min_width;
                }

                if max_width > 0 {
                    constraint.max_width = max_width;
                }

                if min_height > 0 {
                    constraint.min_height = min_height;
                }

                if max_height > 0 {
                    constraint.max_height = max_height;
                }

                if width > 0 {
                    constraint.min_width = width;
                    constraint.max_width = width;
                }

                if height > 0 {
                    constraint.min_height = height;
                    constraint.max_height = height;
                }
            }

            loop {
                let layout_result = {
                    let mut result = LayoutResult::Size((32, 32));
                    if let Some(layout) = layout_objects.borrow().get(&entity) {
                        result = layout.layout(
                            entity,
                            ecm,
                            &constraint,
                            &tree.children[&entity],
                            size,
                            theme,
                        );
                    }

                    result
                };

                match layout_result {
                    LayoutResult::Size(size) => {
                        if let Ok(bounds) = ecm.borrow_mut_component::<Rect>(entity) {
                            bounds.width = size.0;
                            bounds.height = size.1;
                        }

                        return size;
                    }
                    LayoutResult::RequestChild(child, child_bc) => {
                        size = Some(layout_rec(
                            ecm,
                            tree,
                            &child_bc,
                            child,
                            theme,
                            layout_objects,
                        ));
                    }
                }
            }
        }

        if !self.update.get() {
            return;
        }

        let root = tree.root;

        let mut backend = self.backend.borrow_mut();
        let layout_context = backend.layout_context();

        layout_rec(
            ecm,
            &tree,
            &Constraint {
                min_width: 0,
                min_height: 0,
                max_width: layout_context.window_size.0,
                max_height: layout_context.window_size.1,
            },
            root,
            &layout_context.theme,
            &self.layout_objects,
        );
    }
}
