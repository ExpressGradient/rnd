use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};
use crate::pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};

// For each System, an implementation of SystemDesc trait must be provided
#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    // The data system on operates on
    type SystemData = (
        // Mutates Transform Components
        WriteStorage<'s, Transform>,
        // Reads Paddle Components
        ReadStorage<'s, Paddle>,
        // Accesses InputHandler Resource
        Read<'s, InputHandler<StringBindings>>
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle")
            };

            if let Some(move_amount) = movement {
                let scaled_amount = 1.2 * move_amount as f32;
                let paddle_y = transform.translation().y;
                transform.set_translation_y(
                    (paddle_y + scaled_amount)
                        .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
                        .max(PADDLE_HEIGHT * 0.5)
                );
            }
        }
    }
}