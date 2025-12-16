use tao::event_loop::EventLoopBuilder;

fn main() {
  let ev = EventLoopBuilder::new()
    .build();

  ev.run(|| {

  });
}
