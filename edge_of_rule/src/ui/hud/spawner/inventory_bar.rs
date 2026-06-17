use bevy::prelude::*;

use crate::assets::hud::HudImageAssets;
use crate::core::inventory::{Inventory, InventoryItem};
use crate::core::state::GameState;

#[derive(Component)]
pub struct InventoryBar;

#[derive(Component)]
pub struct InventorySlot(pub usize);

#[derive(Component)]
pub struct InventorySlotSprite;

pub fn manage_inventory_bar(
    mut commands: Commands,
    state: Res<State<GameState>>,
    mut bar_entity: Local<Option<Entity>>,
    asset_server: Res<AssetServer>,
) {
    use GameState::*;
    let in_gameplay = matches!(
        state.get(),
        Day1Scene1
            | Day1Scene2
            | Day1Scene3
            | Day1Scene4
            | Day2Scene1
            | Day2Scene2
            | Day2Scene3
            | Day2Scene4
            | Day2Scene5
    );

    if in_gameplay && bar_entity.is_none() {
        let entity = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(74.0),
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(10.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(8.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor::from(Color::rgba(0.0, 0.0, 0.0, 0.35)),
                    z_index: ZIndex::Global(500),
                    ..Default::default()
                },
                InventoryBar,
            ))
            .with_children(|parent| {
                for i in 0..8 {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|wrapper| {
                            wrapper
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Px(56.0),
                                            height: Val::Px(56.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..Default::default()
                                        },
                                        background_color: BackgroundColor::from(Color::NONE),
                                        ..Default::default()
                                    },
                                    InventorySlot(i),
                                ))
                                .with_children(|slot_parent| {
                                    slot_parent.spawn((
                                        ImageBundle {
                                            image: UiImage::default(),
                                            style: Style {
                                                width: Val::Auto,
                                                height: Val::Auto,
                                                max_width: Val::Px(48.0),
                                                max_height: Val::Px(48.0),
                                                ..Default::default()
                                            },
                                            background_color: BackgroundColor::from(Color::WHITE),
                                            transform: Transform::from_scale(Vec3::splat(4.0)),
                                            ..Default::default()
                                        },
                                        InventorySlotSprite,
                                    ));
                                });
                            wrapper.spawn(TextBundle::from_section(
                                format!("{}", i + 1),
                                TextStyle {
                                    font: asset_server.load("font/font/aLiFont.ttf"),
                                    font_size: 12.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                }
            })
            .id();
        *bar_entity = Some(entity);
    } else if !in_gameplay && bar_entity.is_none() == false {
        commands.entity(bar_entity.unwrap()).despawn_recursive();
        *bar_entity = None;
    }
}

pub fn update_inventory_display(
    inventory: Res<Inventory>,
    hud_assets: Res<HudImageAssets>,
    mut slots: Query<(&InventorySlot, &mut BackgroundColor, &Children)>,
    mut images: Query<(&mut UiImage, &mut Visibility), With<InventorySlotSprite>>,
) {
    for (slot, mut bg, children) in &mut slots {
        if slot.0 == inventory.selected {
            *bg = BackgroundColor::from(Color::WHITE);
        } else {
            *bg = BackgroundColor::from(Color::NONE);
        }

        let item = inventory.slots[slot.0];
        for &child in children.iter() {
            if let Ok((mut image, mut visibility)) = images.get_mut(child) {
                match item {
                    Some(InventoryItem::Key) => {
                        image.texture = hud_assets.key_icon.clone();
                        *visibility = Visibility::Visible;
                    }
                    Some(InventoryItem::Screw) => {
                        image.texture = hud_assets.screw_icon.clone();
                        *visibility = Visibility::Visible;
                    }
                    Some(InventoryItem::Controller) => {
                        image.texture = hud_assets.controller_icon.clone();
                        *visibility = Visibility::Visible;
                    }
                    Some(InventoryItem::CoverPicked) => {
                        image.texture = hud_assets.cover_icon.clone();
                        *visibility = Visibility::Visible;
                    }
                    None => {
                        *visibility = Visibility::Hidden;
                    }
                };
            }
        }
    }
}
