use bevy::prelude::*;

use crate::{
    animation::fade_mask::spawn_mask,
    core::{
        health::{PlayerDied, PlayerHealth, SewageDamageAccum},
        state::GameState,
    },
    entities::player::{Player, SpawnPoint},
    levels::day1::{
        scene1::{Day1Finished, Picked, Scene1DoorState},
        scene2::TrapdoorState,
        scene3::{Scene3ChestState, Scene3CoverState, Scene3DoorState},
    },
};

pub fn sewage_damage_system(
    time: Res<Time>,
    mut accum: ResMut<SewageDamageAccum>,
    mut health: ResMut<PlayerHealth>,
    players: Query<&Player>,
    mut died_writer: EventWriter<PlayerDied>,
) {
    if health.dead {
        return;
    }

    let in_water = players.iter().any(|p| p.in_water());
    if !in_water {
        accum.seconds = 0.0;
        return;
    }

    accum.seconds += time.delta_seconds();
    while accum.seconds >= 1.0 {
        accum.seconds -= 1.0;
        health.current = (health.current - 10).max(0);

        if health.current == 0 {
            health.dead = true;
            died_writer.send(PlayerDied);
            break;
        }
    }
}

pub fn on_player_died(
    mut events: EventReader<PlayerDied>,
    mut commands: Commands,
    mut picked: ResMut<Picked>,
    mut scene1_door_state: ResMut<Scene1DoorState>,
    mut day1_finished: ResMut<Day1Finished>,
    mut trapdoor_state: ResMut<TrapdoorState>,
    mut scene3_door_state: ResMut<Scene3DoorState>,
    mut scene3_chest_state: ResMut<Scene3ChestState>,
    mut scene3_cover_state: ResMut<Scene3CoverState>,
    mut spawn_point: ResMut<SpawnPoint>,
    mut accum: ResMut<SewageDamageAccum>,
) {
    if events.read().next().is_none() {
        return;
    }

    *picked = Picked::None;
    *scene1_door_state = Scene1DoorState::Closed;
    *day1_finished = Day1Finished::No;

    *trapdoor_state = TrapdoorState::Closed;

    *scene3_door_state = Scene3DoorState::Closed;
    *scene3_chest_state = Scene3ChestState::Packed;
    *scene3_cover_state = Scene3CoverState::Packed;

    spawn_point.0 = Transform::from_xyz(-100.0, -68.0, 0.0);
    accum.seconds = 0.0;

    spawn_mask(&mut commands, GameState::Day1Scene1);
}

pub fn reset_health_on_scene1_enter(
    mut health: ResMut<PlayerHealth>,
    mut accum: ResMut<SewageDamageAccum>,
) {
    health.current = health.max;
    health.dead = false;
    accum.seconds = 0.0;
}
