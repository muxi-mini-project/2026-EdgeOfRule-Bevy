use bevy::prelude::*;

#[derive(Component)]
pub struct EasyStyleIntroduct;

#[derive(Component)]
pub struct MediumStyleIntroduct;

#[derive(Component)]
pub struct HardStyleIntroduct;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {

    // Easy Style Texts
    let easy_Text = [
        "观察者",
        " ",
        "“让我看看剧本再走”",
        " ",
        "你相信知识就是力量。",
        "提前预知规则，规划最优路径，",
        "用智慧而非蛮力突破困境。",
        " ",
        "适合：策略家、解谜爱好者、完美主义者",
        " ",
        " ",
        "你菜不菜？",
    ];

    let medium_Text = [
        "冒险者",
        " ",
        "“规则是拿来打破的”",
        " ",
        "你追求肾上腺素与极速。",
        "将违反规则转化为动力，",
        "在刀尖上舞蹈，发现所有隐藏捷径。",
        " ",
        "适合：速通玩家、探险家、风险爱好者",
        " ",
        " ",
        "你水平怎么样？",
    ];

    // Hard Style Texts
    let hard_Text = [
        "反叛者",
        " ",
        "“这规则不好，我改一条”",
        " ",
        "你不满足于选择。",
        "你不满足于选择。",
        "将固定关卡变为你的个人游乐场。",
        " ",
        "适合：创意实验家、规则黑客、混沌追求者",
        " ",
        " ",
        "你很懂吗？",
    ];

    // Easy Style Introduction
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(20.0),
                height: Val::Percent(47.0),
                padding: UiRect::all(Val::Percent(2.0)),
                position_type: PositionType::Absolute,
                top: Val::Percent(20.0),
                left: Val::Percent(11.9),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            z_index: ZIndex::Global(1),
            background_color: BackgroundColor::from(Color::rgb(35.0 / 255.0, 35.0 / 255.0, 52.0 / 255.0)),
            ..Default::default()
        },
        EasyStyleIntroduct,
    ))
    .with_children(|parent| {
        for (i, line) in easy_Text.iter().enumerate() {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    *line,
                    TextStyle {
                        font: asset_server.load("font/font/aLiFont.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                style: Style {
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    });

    // Medium Style Introduction
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(20.0),
                height: Val::Percent(47.0),
                padding: UiRect::all(Val::Percent(2.0)),
                position_type: PositionType::Absolute,
                top: Val::Percent(20.0),
                left: Val::Percent(40.0),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            z_index: ZIndex::Global(1),
            background_color: BackgroundColor::from(Color::rgb(35.0 / 255.0, 35.0 / 255.0, 52.0 / 255.0)),
            ..Default::default()
        },
        MediumStyleIntroduct,
    ))
    .with_children(|parent| {
        for (i, line) in medium_Text.iter().enumerate() {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    *line,
                    TextStyle {
                        font: asset_server.load("font/font/aLiFont.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                style: Style {
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    });

    // Hard Style Introduction
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(20.0),
                height: Val::Percent(47.0),
                padding: UiRect::all(Val::Percent(2.0)),
                position_type: PositionType::Absolute,
                top: Val::Percent(20.0),
                right: Val::Percent(11.9),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            z_index: ZIndex::Global(1),
            background_color: BackgroundColor::from(Color::rgb(35.0 / 255.0, 35.0 / 255.0, 52.0 / 255.0)),
            ..Default::default()
        },
        HardStyleIntroduct,
    ))
    .with_children(|parent| {
        for (i, line) in hard_Text.iter().enumerate() {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    *line,
                    TextStyle {
                        font: asset_server.load("font/font/aLiFont.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                style: Style {
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    });
}


pub fn despawn(mut commands: Commands, introducts: Query<Entity, Or<(With<EasyStyleIntroduct>, With<MediumStyleIntroduct>, With<HardStyleIntroduct>)>>) {
    for introduct in &introducts {
        commands.entity(introduct).despawn_recursive();
    }
}