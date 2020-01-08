use druid::lens::{self, LensExt};
use druid::widget::{Flex, Label, List, Padding, Scroll, TextBox, WidgetExt};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WindowDesc};
use std::sync::{Arc, Mutex};
use std::{thread, time};
use druid::Size;
use druid::{BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, PaintCtx, UpdateCtx};

#[derive(Clone, Data, Default, Lens, Debug)]
struct AppData {
    searched: String,
    found: Arc<Vec<String>>,
}

pub struct Focus<T: Data> {
    inner: Box<dyn Widget<T>>,
    should_focus: bool,
}

impl<T: Data> Focus<T> {
    pub fn new(inner: impl Widget<T> + 'static) -> Self {
        Self {
            inner: Box::new(inner),
            should_focus: true,
        }
    }
}

impl<T: Data> Widget<T> for Focus<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        dbg!(&event);
        let edit: Option<Event> = match event {
            Event::LifeCycle(LifeCycle::WindowConnected) => {
                ctx.request_focus();
                None
            }
            _ => None,
        };
        self.inner.event(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: Option<&T>, data: &T, env: &Env) {
        self.inner.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        bc.debug_check("Focus");
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.inner.paint(paint_ctx, data, env);
    }
}


fn build_widget() -> impl Widget<AppData> {
    Flex::column()
        .with_child(
            Flex::row().with_child(
                Padding::new(
                    5.0,
                    Focus::new(
                        TextBox::new().lens(lens::Id.map(
                            |d: &AppData| d.searched.clone(),
                            |d: &mut AppData, new_value: String| {
                                d.searched = new_value;
                                let items = vec!["apple", "pineapple", "orange"];
                                d.found = Arc::new(
                                    items
                                        .iter()
                                        .filter(|i| {
                                            if d.searched.as_ref() as &str == "" {
                                                return true;
                                            }
                                            i.contains(&d.searched)
                                        })
                                        .map(|s| s.to_string())
                                        .collect::<Vec<String>>(),
                                );
                            },
                        )),
                    )
                ),
                1.0,
            ),
            0.0,
        )
        //.with_child(
        //    Scroll::new(List::new(|| {
        //        Label::new(|item: &String, _env: &_| format!("{}", item)).padding(5.0)
        //    }))
        //    .lens(AppData::found),
        //    1.0,
        //)
}

fn main() {
    dbg!("launch_gui");

    let window = WindowDesc::new(build_widget);
    let mut data = AppData::default();
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");

    dbg!("GUI killed");
}

