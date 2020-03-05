#![allow(warnings, unused)]

use specs::{
    Component, VecStorage, World, WorldExt, Builder,
    ReadStorage, WriteStorage, System, RunNow, DispatcherBuilder,
};
struct HelloWorld;
struct UpdatePos;

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}
#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
}
impl Component for Position { type Storage = VecStorage<Self>; }
impl Component for Velocity{ type Storage = VecStorage<Self>; }
impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;
    fn run (&mut self, data: Self::SystemData) {
        use specs::Join;
        for pos in data.join() {
            print!("\nHello, {:?}\n", &pos);
        }
    }
}
impl<'a> System<'a> for UpdatePos {
    type SystemData = (ReadStorage<'a, Velocity>,
                       WriteStorage<'a, Position>);
    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        use specs::Join;
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * 0.05;
            pos.y += vel.y * 0.05;
        }
    }
}

fn main() {
    let mut world = World::new();
//    let mut hello_world = HelloWorld;
    world.register::<Position>();
    world.register::<Velocity>();

    world.create_entity()
        .with(Position {x: 4.0, y: 7.0})
        .build();
    world.create_entity()
        .with(Position {x: 2.0, y: 5.0})
        .with(Velocity {x: 0.1, y: 0.2})
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(HelloWorld, "hello_world", &[])
        .with(UpdatePos, "update_pos", &["hello_world"])
        .with(HelloWorld, "hello_updated", &["update_pos"])
        .build();
    dispatcher.dispatch(&mut world);

//    hello_world.run_now(&world);
    world.maintain();
}
