#![windows_subsystem = "windows"]

use std::{mem::transmute, num::NonZeroU32, thread};

use resvg::{
  tiny_skia::{Color, Pixmap},
  usvg::{Options, Transform, Tree},
};
use softbuffer::{Context, Surface};
use winit::{
  application::ApplicationHandler,
  dpi::LogicalSize,
  event::WindowEvent,
  event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
  platform::windows::{
    CornerPreference, EventLoopBuilderExtWindows, IconExtWindows, WindowAttributesExtWindows,
  },
  window::{Icon, Window, WindowAttributes},
};

use crate::net::handle_installer_loop;

mod net;

pub static SVG: &'static str = include_str!("../ui/splash.svg");

pub static KHULA_LIGHT_FONT: &'static [u8] = include_bytes!("../fonts/Khula-Light.ttf");
pub static KHULA_BOLD_FONT: &'static [u8] = include_bytes!("../fonts/Khula-Bold.ttf");

#[derive(Debug, Default)]
pub struct DrawRequest {
  perc: u64,
  txt: &'static str,
}

#[cfg(target_arch = "x86_64")]
pub static NEXT_SETUP: &'static [u8] =
  include_bytes!("../../bin/Release/net10.0-windows/win-x64/publish/AHQStoreWin32Setup.exe");

#[cfg(target_arch = "aarch64")]
pub static NEXT_SETUP: &'static [u8] =
  include_bytes!("../../bin/Release/net10.0-windows/win-arm64/publish/AHQStoreWin32Setup.exe");

fn main() {
  {
    use windows::Win32::System::Console::*;
    let _ = unsafe { AttachConsole(ATTACH_PARENT_PROCESS) };
  }

  println!(
    "We're displaying a few software licenses for the fonts used by us.\n{}",
    include_str!("../fonts/OFL.txt")
  );

  let ev_loop: EventLoop<DrawRequest> = EventLoopBuilder::default()
    .with_dpi_aware(true)
    .build()
    .expect("Unable to create event loop");

  ev_loop.set_control_flow(ControlFlow::Poll);

  ev_loop.set_control_flow(ControlFlow::Wait);

  let sink = ev_loop.create_proxy();

  thread::spawn(move || {
    handle_installer_loop(sink);
  });

  let mut app = App::default();
  ev_loop.run_app(&mut app).expect("Unable to run app");
}

#[derive(Default)]
struct App {
  window: Option<Window>,
  ctx: Option<Context<&'static Window>>,
  surface: Option<Surface<&'static Window, &'static Window>>,

  // Window Content Rendered
  opt: Option<Options<'static>>,
  content: String,
}

impl ApplicationHandler<DrawRequest> for App {
  fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
    if self.window.is_none() {
      self.content = SVG
        .replace("{WIDTH}", "100%")
        .replace("{TEXT}", "Checking for dependencies...");

      self.opt = Some({
        let mut opt = Options::default();

        opt.fontdb_mut().load_font_data(KHULA_BOLD_FONT.to_vec());
        opt.fontdb_mut().load_font_data(KHULA_LIGHT_FONT.to_vec());

        opt
      });

      let window = event_loop
        .create_window(
          WindowAttributes::default()
            .with_title("Preparing AHQ Store")
            .with_active(true)
            .with_resizable(false)
            .with_decorations(false)
            .with_inner_size(LogicalSize::new(600, 250))
            .with_min_inner_size(LogicalSize::new(600, 250))
            .with_max_inner_size(LogicalSize::new(600, 250))
            .with_transparent(true)
            .with_corner_preference(CornerPreference::DoNotRound)
            .with_window_icon(Some(
              Icon::from_resource(1, None).expect("Impossible since resource is set by winres"),
            )),
        )
        .expect("unable to create window");

      unsafe {
        let ctx = transmute(Context::new(&window).expect("Context"));

        self.ctx = Some(ctx);

        let surface = transmute(
          Surface::new(self.ctx.as_ref().unwrap(), &window).expect("Could not generate surface"),
        );

        self.surface = Some(surface);
      }

      if let Some(monitor) = window.current_monitor() {
        let monitor_size = monitor.size();
        let scale = monitor.scale_factor();

        let window_width = (600.0 * scale) as i32;
        let window_height = (250.0 * scale) as i32;

        let x = (monitor_size.width as i32 - window_width) / 2;
        let y = (monitor_size.height as i32 - window_height) / 2;

        window.set_outer_position(winit::dpi::PhysicalPosition::new(x, y));
      }

      self.window = Some(window);
    }
  }

  fn user_event(&mut self, _: &winit::event_loop::ActiveEventLoop, event: DrawRequest) {
    self.content = SVG
      .replace("{WIDTH}", format!("{}%", event.perc).as_str())
      .replace("{TEXT}", event.txt);

    self.window.as_ref().unwrap().request_redraw();
  }

  fn window_event(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    _: winit::window::WindowId,
    event: winit::event::WindowEvent,
  ) {
    match event {
      WindowEvent::CloseRequested => {
        event_loop.exit();
      }
      WindowEvent::RedrawRequested => {
        // Redraw the application.
        //
        // It's preferable for applications that do not render continuously to render in
        // this event rather than in AboutToWait, since rendering in here allows
        // the program to gracefully handle redraws requested by the OS.

        // Draw.
        let (width, height): (u32, u32) = self.window.as_ref().unwrap().inner_size().into();

        let surface = self.surface.as_mut().unwrap();

        let rtree =
          Tree::from_str(&self.content, self.opt.as_ref().unwrap()).expect("Unwanted Trees");
        let mut pixmap = Pixmap::new(width, height).unwrap();

        pixmap.fill(Color::TRANSPARENT);

        let svg_size = rtree.size();
        let scale_x = width as f32 / svg_size.width();
        let scale_y = height as f32 / svg_size.height();
        // Preserve aspect ratio by taking the smaller scale
        let scale = scale_x.min(scale_y);

        // Create a transform to scale the SVG
        let transform = Transform::from_scale(scale, scale);

        // Render!
        resvg::render(&rtree, transform, &mut pixmap.as_mut());

        // Render into window
        surface
          .resize(
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
          )
          .expect("Could not resize");

        let mut buffer = surface.buffer_mut().unwrap();
        let src_data = pixmap.data();

        for (i, chunk) in src_data.chunks_exact(4).enumerate() {
          // Determine destination index (ensure we don't go out of bounds)
          if i < buffer.len() {
            let r = chunk[0] as u32;
            let g = chunk[1] as u32;
            let b = chunk[2] as u32;
            let a = chunk[3] as u32; // 1. Get the Alpha channel

            // Pack into u32: AAAAAAAA RRRRRRRR GGGGGGGG BBBBBBBB
            buffer[i] = (a << 24) | (r << 16) | (g << 8) | b;
          }
        }

        buffer.present().unwrap();

        let window = self.window.as_ref().unwrap();

        if let Some(monitor) = window.current_monitor() {
          let monitor_size = monitor.size();
          let scale = monitor.scale_factor();

          let window_width = (600.0 * scale) as i32;
          let window_height = (250.0 * scale) as i32;

          let x = (monitor_size.width as i32 - window_width) / 2;
          let y = (monitor_size.height as i32 - window_height) / 2;

          window.set_outer_position(winit::dpi::PhysicalPosition::new(x, y));
        }
      }
      _ => (),
    }
  }
}
