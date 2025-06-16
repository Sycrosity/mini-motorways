use std::{collections::HashMap, sync::Arc};

use bevy::{prelude::*, render::texture};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, prelude::*, quick::WorldInspectorPlugin};
use leafwing_input_manager::{input_map, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, (setup, spawn_player).chain())
        .add_systems(Update, handle_movement.after(spawn_player))
        .run();
}

#[derive(Component)]
struct Player;

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum PlayerAction {
    #[actionlike(DualAxis)]
    Move,
}

impl PlayerAction {
    /// Define the default bindings to the input
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert_dual_axis(Self::Move, GamepadStick::LEFT);
        input_map.insert_dual_axis(Self::Move, VirtualDPad::wasd());

        input_map
    }
}

fn spawn_player(
    mut commands: Commands,
    sprite_sheet: Res<SpriteSheet>,
    // images: Res<Assets<Image>>,
    // texture_atlas_layouts: Res<Assets<TextureAtlasLayout>>,
) {
    let texture = sprite_sheet.0.clone();
    let texture_atlas_layout = sprite_sheet.1.clone();

    commands.spawn((
        PlayerAction::default_input_map(),
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
        Transform::from_scale(Vec3::splat(6.0)),
        Player,
    ));
}

fn handle_movement(
    mut query: Query<(&ActionState<PlayerAction>, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    let (action_state, mut transform) = query.single_mut().expect("Player actions not found");

    if action_state.axis_pair(&PlayerAction::Move) != Vec2::ZERO {
        let movement = action_state.clamped_axis_pair(&PlayerAction::Move);
        println!("{}",time.delta_secs());
        transform.translation += Vec3::new(movement.x, movement.y, 0f32) * 1000. * time.delta_secs()
    }
}

#[derive(Debug, Resource)]
pub struct SpriteSheet(Handle<Image>, Handle<TextureAtlasLayout>);

// #[derive(Debug, Resource)]
// pub struct SpriteHash(HashMap<&'static str, ((u32, u32), (u32, u32))>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // let texture = asset_server.load("textures/sprite_sheet.png");
    // let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 10, 10, None, None);
    // let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let guard = Arc::<()>::new(());

    // let ron = asset_server.load("textures/sprite_sheet.ron");

    // let columns = 10;

    // let assets: HashMap<&str, (u32, (u32, u32))> = HashMap::from([
    //     ("green car", (0 + 0 * 10, (1, 1))),
    //     ("red car", (0 + 1 * 10, (1, 1))),
    //     ("green house", (1 + 0 * 10, (1, 1))),
    //     ("red house", (1 + 1 * 10, (1, 1))),
    //     ("orange car", (0 + 2 * 10, (1, 1))),
    //     ("blue car", (0 + 3 * 10, (1, 1))),
    //     ("blue house", (1 + 3 * 10, (1, 1))),
    //     ("orange house", (1 + 2 * 10, (1, 1))),
    //     ("green supermarket horizontal", (2 + 0 * 10, (2, 2))),
    //     ("orange supermarket horizontal", (2 + 2 * 10, (2, 2))),
    //     ("blue supermarket horizontal", (4 + 2 * 10, (2, 2))),
    //     ("red supermarket horizontal", (4 + 0 * 10, (2, 2))),
    //     ("green supermarket vertical", (0 + 4 * 10, (2, 2))),
    //     ("red supermarket vertical", (2 + 4 * 10, (2, 2))),
    //     ("orange supermarket vertical", (4 + 4 * 10, (2, 2))),
    //     ("blue supermarket vertical", (6 + 4 * 10, (2, 2))),
    //     ("supermarket lot horizontal", (5 + 7 * 10, (3, 2))),
    //     ("supermarket lot vertical", (8 + 6 * 10, (2, 3))),
    // ]);

    commands.insert_resource(SpriteHash::new(assets));

    // serde_json::de::from_str(&serde_json::to_string(&guard).unwrap())
    //     .expect("Failed to deserialize guard");

    commands.insert_resource(SpriteSheet(
        asset_server.load_acquire("textures/sprite_sheet.png", guard.clone()),
        texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::splat(32),
            10,
            10,
            None,
            None,
        )),
    ));

    commands.spawn(Camera2d);
}
