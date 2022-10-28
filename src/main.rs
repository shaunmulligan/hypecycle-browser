// Copyright 2020-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
use std::env;

fn main() -> wry::Result<()> {
  use wry::{
    application::{
      event::{Event, WindowEvent},
      event_loop::{ControlFlow, EventLoop},
      window::{Fullscreen, WindowBuilder},
    },
    webview::WebViewBuilder,
  };

  let args: Vec<String> = env::args().collect();
  let url = &args[1];
  println!("Rendering url: {}", url);
  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .with_title("3D Render Test")
    .with_fullscreen(Some(Fullscreen::Borderless(None)))
    .build(&event_loop)
    .unwrap();
  let _webview = WebViewBuilder::new(window)
    .unwrap()
    .with_url(url)?
    .build()?;

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,
      _ => {}
    }
  });
}
