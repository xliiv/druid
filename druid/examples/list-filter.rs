// Copyright 2019 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use druid::widget::{Flex, Label, List, Scroll, TextBox, WidgetExt};
use druid::{AppLauncher, Data, Lens, Widget, WindowDesc};
use std::sync::Arc;

#[derive(Clone, Data, Lens)]
struct AppData {
    searched: String,
    listing: Arc<Vec<String>>,
}

fn main() {
    let window = WindowDesc::new(build_widget);
    let data = AppData {
        searched: "".to_string(),
        listing: Arc::new(
            "Apple Bananas Orange Pineapple"
                .split(' ')
                .map(|s| s.to_string())
                .collect(),
        ),
    };
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}

fn build_widget() -> impl Widget<AppData> {
    Flex::column()
        .with_child(TextBox::new()
            .lens(AppData::searched), 0.0)
        .with_child(
            Scroll::new(List::new(|| {
                Label::new(|item: &String, _env: &_| format!("{}", item))
            }))
            .lens(AppData::listing),
            0.0,
        )
}
