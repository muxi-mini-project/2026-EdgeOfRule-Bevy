use bevy::prelude::*;

use crate::{
    animation::fade_mask::spawn_mask,
    animation::hurt_shake::HurtShake,
    control::ghost::Day2GhostManager,
    core::{health::PlayerDied, health::PlayerHealth, health::SewageDamageAccum, state::GameState},
    entities::player::{Player, SpawnPoint},
    levels::day1::{
        scene1::{Day1Finished, Picked, Scene1DoorState},
        scene2::TrapdoorState,
        scene3::{Scene3ChestState, Scene3CoverState, Scene3DoorState},
    },
};

const DAY2_SCENE1_RESPAWN: Vec3 = Vec3::new(-268.0, -68.0, 0.0);

pub fn sewage_damage_system(
    mut commands: Commands,
    time: Res<Time>,
    mut accum: ResMut<SewageDamageAccum>,
    mut health: ResMut<PlayerHealth>,
    players: Query<(Entity, &Player)>,
    mut died_writer: EventWriter<PlayerDied>,
) {
    if health.dead {
        return;
    }

    let in_water = players.iter().any(|(_, p)| p.in_water());
    if !in_water {
        accum.seconds = 0.0;
        return;
    }

    accum.seconds += time.delta_seconds();
    while accum.seconds >= 1.0 {
        accum.seconds -= 1.0;
        health.current = (health.current - 10).max(0);

        for (entity, player) in &players {
            if player.in_water() {
                commands.entity(entity).insert(HurtShake::on_damage_tick());
            }
        }

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
    state: Res<State<GameState>>,
    mut picked: ResMut<Picked>,
    mut scene1_door_state: ResMut<Scene1DoorState>,
    mut day1_finished: ResMut<Day1Finished>,
    mut trapdoor_state: ResMut<TrapdoorState>,
    mut scene3_door_state: ResMut<Scene3DoorState>,
    mut scene3_chest_state: ResMut<Scene3ChestState>,
    mut scene3_cover_state: ResMut<Scene3CoverState>,
    mut spawn_point: ResMut<SpawnPoint>,
    mut health: ResMut<PlayerHealth>,
    mut accum: ResMut<SewageDamageAccum>,
    ghost_manager: Option<ResMut<Day2GhostManager>>,
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

    let s = *state.get();
    if matches!(
        s,
        GameState::Day2Scene1
            | GameState::Day2Scene2
            | GameState::Day2Scene3
            | GameState::Day2Scene4
            | GameState::Day2Scene5
    ) {
        spawn_point.0 = Transform::from_translation(DAY2_SCENE1_RESPAWN);
        health.current = health.max;
        health.dead = false;
        if let Some(mut gm) = ghost_manager {
            *gm = Day2GhostManager::default();
        }
        spawn_mask(&mut commands, GameState::Day2Scene1);
    } else {
        spawn_mask(&mut commands, GameState::Day1Scene1);
    }
}

pub fn reset_health_on_scene1_enter(
    mut health: ResMut<PlayerHealth>,
    mut accum: ResMut<SewageDamageAccum>,
) {
    health.current = health.max;
    health.dead = false;
    accum.seconds = 0.0;
}
