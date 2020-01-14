use druid::widget::{Button, Flex};
use druid::{AppLauncher, Widget, WindowDesc};

fn main () {
    dbg!("launch_gui");

    let window = WindowDesc::new(build_widget);
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(0_u32)
        .expect("launch failed");

    dbg!("GUI killed");
}

fn build_widget() -> impl Widget<u32> {
    Flex::column()
        .with_child(
            Button::sized(
                "Copy",
                |evt_ctx , data: _, _env| {
                    dbg!(&data);
                    evt_ctx.window().close();
                },
                80.0,
                20.0,
            ),
            0.0,
        )
}
