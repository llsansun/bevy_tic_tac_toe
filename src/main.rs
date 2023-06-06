use bevy::prelude*;

[derive(Copy, Clone, Eq, PartialEq)
enum Player {
    Cross,
    Circle,
}

[derive(Copy, Clone, Eq, PartialEq)]
enum State {
    Empty,
    Occupied(Player),
}

struct GameBoard([[State; 3]; 3]);

impl Default for GameBoard {
    fn default() -> Self {
        Self([[State::Empty; 3]; 3])
    }
}

fun main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .run();
}
