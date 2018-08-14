#![feature(proc_macro)]
#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate serde_derive;

extern crate serde;

#[macro_use]
extern crate lazy_static;

use std::sync::Arc;
use std::sync::atomic::{ AtomicBool, Ordering };

use stdweb::js_export;
use stdweb::traits::*;
use stdweb::unstable::TryInto;

use stdweb::web::{
  window,
  document,
  HtmlElement,
  CanvasRenderingContext2d
};
use stdweb::web::html_element::CanvasElement;

use stdweb::web::event::{
  ClickEvent,
  MouseMoveEvent,
  MouseDownEvent,
  MouseUpEvent
};

lazy_static! {
  static ref flag: AtomicBool = AtomicBool::new(false);
  static ref canvas: CanvasElement = document().query_selector("#canvas")
    .unwrap().unwrap().try_into().unwrap();
  static ref app: HtmlElement = document().query_selector(".app")
    .unwrap().unwrap().try_into().unwrap();
  static ref pixel: CanvasRenderingContext2d = canvas.get_context().unwrap();
}

#[js_export]
fn Init(w: u32, h: u32) {
  let colors: HtmlElement = document().query_selector(".color__list")
    .unwrap().unwrap().try_into().unwrap();
  /* ========== Global settings ========== */
  canvas.set_height(h);
  canvas.set_width(w);
  canvas.set_attribute("style", "border: 3px solid black");
  
  pixel.set_fill_style_color("black");

  /* ========== Events ========== */
  colors.add_event_listener(move |_ev: ClickEvent| {
    let cl_value: String = js! {
      return @{_ev}.target.classList[1];
    }.try_into().unwrap();

    pixel.set_fill_style_color(cl_value.as_str());
  });

  canvas.add_event_listener(move |_ev: MouseDownEvent| {
    flag.store(true, Ordering::Relaxed);
  });

  canvas.add_event_listener(move |_ev: MouseMoveEvent| {
    if flag.load(Ordering::Relaxed) == true {
      #[derive(Deserialize, Debug)]
      struct Point { x: f64, y: f64 };
      js_deserializable!( Point );

      let val = js! {
        let e = @{_ev};
        let app = document.querySelector(".app");
        let rect = app.getBoundingClientRect();
        let dx = e.pageX - rect.left;
        let dy = e.pageY - rect.top;

        return {
          x: dx,
          y: dy
        };
      };

      let points: Point = val.try_into().unwrap();
      pixel.fill_rect(points.x, points.y, 10f64, 10f64);
    }
  });

  canvas.add_event_listener(move |_ev: MouseUpEvent| {
    flag.store(false, Ordering::Relaxed);
  });

  stdweb::event_loop();
  /* ========== Events end ========== */
}