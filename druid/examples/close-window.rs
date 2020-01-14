use std::{thread, time};
use druid::widget::{Button, Flex};
use druid::{AppLauncher, Widget, WindowDesc};

// Clicking a button should close the window

//// This works like expected
//fn main () {
//    let window = WindowDesc::new(build_widget);
//    AppLauncher::with_window(window)
//        .use_simple_logger()
//        .launch(0_u32)
//        .expect("launch failed");
//
//    dbg!("GUI killed");
//}

// This not
fn main () {
    dbg!("launch_gui");

    let mut should_gui = true;
    loop {
        if should_gui == true {
            dbg!("INSIDE");
            should_gui = false;

            let window = WindowDesc::new(build_widget);
            AppLauncher::with_window(window)
                .use_simple_logger()
                .launch(0_u32)
                .expect("launch failed");

            dbg!("GUI killed");

        }
        let sleep_time = time::Duration::from_millis(100);
        thread::sleep(sleep_time);
    }

}

fn build_widget() -> impl Widget<u32> {
    Flex::column()
        .with_child(
            Button::sized(
                "Close",
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
