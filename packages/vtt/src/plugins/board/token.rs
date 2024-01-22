use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Token;

#[derive(Bundle, Default)]
pub struct TokenBundle {
    token: Token,
    transform: Transform,
}
