#![allow(warnings, unused)]

use specs::prelude::*;
use specs::{
    Component, VecStorage, World, WorldExt, Builder, Read,
    ReadStorage, WriteStorage, System, RunNow, DispatcherBuilder,
    Entities, LazyUpdate, NullStorage,
};
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    x: f32,
    y: f32,
}
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    x: f32,
    y: f32,
}
#[derive(SystemData)]
pub struct SystemDataRead<'a> {
    positions: ReadStorage<'a, Position>,
    velocities: ReadStorage<'a, Velocity>,
}
#[derive(SystemData)]
pub struct SystemDataWrite<'a> {
    positions: WriteStorage<'a, Position>,
    velocities: WriteStorage<'a, Velocity>,
}
struct GameSystem;
impl<'a> System<'a> for GameSystem {
    type SystemData = SystemDataRead<'a>;

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        for vel in data.velocities.join() {
            print!("\nJoining!\n");
            print!("\ndata? {}\n", vel.x);
        }
    }
}
struct GameSystemWrite;
impl<'a> System<'a> for GameSystemWrite {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>
    );
//    type SystemData = SystemDataWrite<'a>;
    fn run(&mut self, (pos, mut vel): Self::SystemData) {
        use specs::Join;
        for (pos, vel) in (&pos, &mut vel).join() {
            print!("\nChanging velocity!\n");
            vel.x += 1.0;
            vel.y += 1.0;
        }
    }
}
fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();

    let mut dispatcher = DispatcherBuilder::new()
        .with(GameSystem, "g_sys", &[g_sys_w])
        .with(GameSystemWrite, "g_sys_w", &[])
        .build();

    let ball = world.create_entity()
        .with(Position {x:4.0, y:7.0})
        .with(Velocity {x:99.0, y:99.0})
        .build();

    dispatcher.dispatch(&mut world);

    world.maintain();
}
