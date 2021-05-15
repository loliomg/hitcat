use bevy::core::FixedTimestep;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::DefaultPlugins;
use rand::Rng;
use wasm_bindgen::prelude::*;

use crate::materials::Materials;
use crate::position::Position;
use crate::size::Size;

pub mod materials;
pub mod position;
pub mod size;
pub mod systems;

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

#[wasm_bindgen]
pub fn wasm_run() {
    run()
}

pub fn run() {
    let mut app = App::build();
    app.insert_resource(WindowDescriptor {
        title: "屑猫猫".to_string(),
        scale_factor_override: Some(1f64),
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource(BadCat(0))
    .insert_resource(EscapeCat(0))
    .add_startup_system(setup.system())
    .add_startup_stage("game_setup", SystemStage::single(game_setup.system()))
    .add_system(mouse_motion.system())
    .add_system(click_mouse.system().after("hanle_bc_event"))
    .add_system(retract.system().after("hanle_bc_event"))
    .add_system(hanle_bc_event.system().label("hanle_bc_event"))
    .add_system(change_text.system().before("hanle_bc_event"))
    .add_system(update_fps_text.system())
    .add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new()
            .with_system(systems::position_system::position_translation.system())
            .with_system(systems::position_system::size_scaling.system()),
    )
    .add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(0.6))
            .with_system(spawn_hole.system()),
    )
    .add_plugins(DefaultPlugins)
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_event::<BedCatEvent>();
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        hammer: materials.add(asset_server.load("hammer.png").into()),
        hole: materials.add(asset_server.load("cat.png").into()),
    });
    commands
        .insert_resource(materials.add(ColorMaterial::color(Color::hex("03fc20").unwrap()).into()));

    let font = asset_server
        .load("fonts/jb-mono+ShangShouFengMangTi.ttf")
        .into();
    let text_style = TextStyle {
        font,
        font_size: 25.0,
        color: Color::GREEN,
    };
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "fps:".into(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: String::new(),
                        style: text_style.clone(),
                    },
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Top,
                    horizontal: HorizontalAlign::Right,
                },
            },
            transform: Transform::from_xyz(
                window.width() / 2f32 - 70f32,
                window.height() / 2f32 - 23f32,
                10f32,
            ),
            ..Default::default()
        })
        .insert(FpsText);
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: String::new(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "只猫猫惨遭杀害".into(),
                        style: text_style.clone(),
                    },
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Top,
                    horizontal: HorizontalAlign::Left,
                },
            },
            transform: Transform::from_xyz(
                150f32 - window.width() / 2f32,
                window.height() / 2f32 - 23f32,
                10f32,
            ),
            ..Default::default()
        })
        .insert(BadCatText);
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: String::new(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "只屑猫猫逃跑了".into(),
                        style: text_style.clone(),
                    },
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Top,
                    horizontal: HorizontalAlign::Left,
                },
            },
            transform: Transform::from_xyz(
                150f32 - window.width() / 2f32,
                window.height() / 2f32 - 50f32,
                10f32,
            ),
            ..Default::default()
        })
        .insert(EscapeCatText);
}

struct Hole;

struct Hammer;

struct FpsText;
struct BadCatText;
struct BadCat(usize);
struct EscapeCatText;
struct EscapeCat(usize);

struct BedCatEvent(Entity);

fn game_setup(mut commands: Commands, materials: Res<Materials>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(false);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            material: materials.hammer.clone(),
            ..Default::default()
        })
        .insert(Hammer)
        .insert(Position::new(
            (ARENA_WIDTH / 2) as f32,
            (ARENA_HEIGHT / 2) as f32,
            5f32,
        ))
        .insert(Size::square(0.7));
}

fn spawn_hole(mut commands: Commands, materials: Res<Materials>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            material: materials.hole.clone(),
            ..Default::default()
        })
        .insert(Hole)
        .insert(Position::new_2d(
            rand::thread_rng().gen_range(0.0..window.width() as f32),
            rand::thread_rng().gen_range(0.0..window.height() as f32),
        ))
        .insert(Size::square(0.9))
        .insert(Timer::from_seconds(1.0, false));
}

fn update_fps_text(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    query.iter_mut().for_each(|mut text| {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[1].value = format!("{:.2}", average);
            }
        }
    })
}

fn change_text(
    mut query: QuerySet<(
        Query<&mut Text, With<BadCatText>>,
        Query<&mut Text, With<EscapeCatText>>,
    )>,
    mut bc_event: EventReader<BedCatEvent>,
    bc: ResMut<BadCat>,
    ec: ResMut<EscapeCat>,
) {
    bc_event.iter().for_each(|_| {
        query
            .q0_mut()
            .iter_mut()
            .for_each(|mut text| text.sections[0].value = bc.0.to_string());
        query
            .q1_mut()
            .iter_mut()
            .for_each(|mut text| text.sections[0].value = ec.0.to_string())
    })
}

fn retract(
    time: Res<Time>,
    mut hole_timer: Query<(&mut Timer, Entity), With<Hole>>,
    mut bc_event: EventWriter<BedCatEvent>,
    mut ec: ResMut<EscapeCat>,
) {
    hole_timer.iter_mut().for_each(|(mut timer, entity)| {
        if timer.tick(time.delta()).finished() {
            ec.0 += 1;
            bc_event.send(BedCatEvent(entity))
        }
    })
}

fn hanle_bc_event(mut commands: Commands, mut bc_event: EventReader<BedCatEvent>) {
    bc_event.iter().for_each(|event| {
        commands.entity(event.0).despawn_recursive();
    })
}

fn click_mouse(
    windows: Res<Windows>,
    mouse_input: Res<Input<MouseButton>>,
    hammer_positions: Query<&Position, With<Hammer>>,
    hole_positions: Query<(Entity, &Sprite, &Position), With<Hole>>,
    mut bc_event: EventWriter<BedCatEvent>,
    mut bc: ResMut<BadCat>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        hammer_positions.iter().for_each(|hammer_positions| {
            let window = windows.get_primary().unwrap();

            let mut hammer_positions = hammer_positions.clone();
            hammer_positions.x = hammer_positions.x - window.width() / 2f32;
            hammer_positions.y = hammer_positions.y - window.height() / 2f32;

            hole_positions.for_each(|(entity, sprite, hole_positions): (Entity, &Sprite, _)| {
                let mut hole_positions = hole_positions.clone();
                hole_positions.x = hole_positions.x - sprite.size.x / 2f32 - window.width() / 2f32;
                hole_positions.y = hole_positions.y - sprite.size.y / 2f32 - window.height() / 2f32;
                let diagonal = hole_positions + Position::new_2d(sprite.size.x, sprite.size.y);
                if hammer_positions.x >= hole_positions.x
                    && hammer_positions.y >= hole_positions.y
                    && hammer_positions.x <= diagonal.x
                    && hammer_positions.y <= diagonal.y
                {
                    bc.0 += 1;
                    bc_event.send(BedCatEvent(entity))
                }
            })
        });
    }
}

fn mouse_motion(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut hammer_positions: Query<&mut Position, With<Hammer>>,
) {
    for event in cursor_moved_events.iter() {
        hammer_positions.iter_mut().for_each(|mut pos| {
            pos.x = event.position.x;
            pos.y = event.position.y;
            pos.absolute = true;
        });
    }
}
