#![feature(proc_macro)]
#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

use std::sync::Arc;
use std::sync::atomic::{ AtomicBool, Ordering };

use stdweb::js_export;
use stdweb::traits::*;
use stdweb::unstable::TryInto;

use stdweb::web::{
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

#[js_export]
fn Init(w: u32, h: u32) {
  let colors: HtmlElement = document().query_selector(".color__list")
    .unwrap().unwrap().try_into().unwrap();
  let canvas: CanvasElement = document().query_selector("#canvas")
      .unwrap().unwrap().try_into().unwrap();

  lazy_static! {
    static ref flag: AtomicBool = AtomicBool::new(false);
    static pixel: CanvasRenderingContext2d = canvas.get_context().unwrap();
  }

  canvas.set_height(h);
  canvas.set_width(w);
  canvas.set_attribute("style", "border: 3px solid black");

  // let pixel: CanvasRenderingContext2d = canvas.get_context().unwrap();
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
      pixel.fill_rect(
        f64::from(_ev.client_x() - 90),
        f64::from(_ev.client_y() - 100),
        10f64,
        10f64
      )
    }
  });

  canvas.add_event_listener(move |_ev: MouseUpEvent| {
    flag.store(false, Ordering::Relaxed);
  });
  /* ========== Events end ========== */
}