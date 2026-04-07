use bevy::{
    color::palettes::tailwind::SKY_600,
    prelude::*,
    ui_widgets::observe,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use crate::{utilities::system_sets::InputSystems, world::character::SetCharacterActive};

use super::{Player, inspector_mode::InspectorModeState};

pub struct EscapeMenuPlugin;

impl Plugin for EscapeMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(EscapeMenuState::Disabled)
            .add_systems(OnEnter(EscapeMenuState::Enabled), on_escape_menu_enabled)
            .add_systems(OnEnter(EscapeMenuState::Disabled), on_escape_menu_disabled)
            .add_systems(Startup, spawn_escape_menu)
            .add_systems(
                Update,
                toggle_escape_menu_on_keypress
                    .run_if(in_state(InspectorModeState::Disabled))
                    .in_set(InputSystems),
            );
    }
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum EscapeMenuState {
    #[default]
    Disabled,
    Enabled,
}

#[derive(Component)]
struct EscapeMenu;

fn spawn_escape_menu(mut commands: Commands) {
    commands.spawn((
        EscapeMenu,
        Visibility::Hidden,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        observe(
            |_: On<Pointer<Click>>, mut next_menu_state: ResMut<NextState<EscapeMenuState>>| {
                next_menu_state.set(EscapeMenuState::Disabled);
            },
        ),
        children![(
            Button,
            Node {
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::from(SKY_600)),
            observe(on_exit_button_click),
            children![Text::new("Exit to desktop")],
        )],
    ));
}

fn on_exit_button_click(_: On<Pointer<Click>>, mut app_exit_writer: MessageWriter<AppExit>) {
    app_exit_writer.write(AppExit::Success);
}

fn toggle_escape_menu_on_keypress(
    key_input: Res<ButtonInput<KeyCode>>,
    menu_state: Res<State<EscapeMenuState>>,
    mut next_menu_state: ResMut<NextState<EscapeMenuState>>,
) {
    if key_input.just_pressed(KeyCode::Escape) {
        let next_state = match menu_state.get() {
            EscapeMenuState::Disabled => EscapeMenuState::Enabled,
            EscapeMenuState::Enabled => EscapeMenuState::Disabled,
        };

        next_menu_state.set(next_state);
    }
}

fn on_escape_menu_enabled(
    mut visibility: Single<&mut Visibility, With<EscapeMenu>>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
    player_entity: Single<Entity, With<Player>>,
    mut commands: Commands,
) {
    **visibility = Visibility::Inherited;
    cursor_options.visible = true;
    cursor_options.grab_mode = CursorGrabMode::None;

    commands.trigger(SetCharacterActive {
        entity: *player_entity,
        set_active: false,
    });
}

fn on_escape_menu_disabled(
    mut visibility: Single<&mut Visibility, With<EscapeMenu>>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
    player_entity: Single<Entity, With<Player>>,
    mut commands: Commands,
) {
    **visibility = Visibility::Hidden;
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Locked;

    commands.trigger(SetCharacterActive {
        entity: *player_entity,
        set_active: true,
    });
}
