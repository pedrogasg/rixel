use std::fs;

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::{
        default, AssetServer, BuildChildren, Button, ButtonBundle, Changed, Children, Color,
        Commands, Component, DespawnRecursiveExt, Entity, EventReader, NodeBundle, Plugin, Query,
        Res, ResMut, Resource, State, SystemSet, TextBundle, With,
    },
    text::TextStyle,
    ui::{
        AlignItems, AlignSelf, BackgroundColor, FlexDirection, Interaction, JustifyContent, Node,
        Overflow, Size, Style, UiRect, Val,
    },
};

use crate::AssetPath;
use crate::{AppState, MainLayout};

#[derive(Resource)]
struct MenuData {
    menu_entity: Entity,
}

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct LayoutsMenu;

impl Plugin for LayoutsMenu {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu))
            .add_system_set(
                SystemSet::on_update(AppState::Menu)
                    .with_system(menu)
                    .with_system(mouse_scroll),
            )
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(cleanup_menu));
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut menu = commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        ..default()
    });

    menu.with_children(|menu| {
        menu.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                ..default()
            },
            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .with_children(|parent| {
            // Title
            parent.spawn(
                TextBundle::from_section(
                    "Layouts list",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 25.,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    size: Size::new(Val::Undefined, Val::Px(25.)),
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        ..default()
                    },
                    ..default()
                }),
            );

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_self: AlignSelf::Center,
                        size: Size::new(Val::Percent(100.0), Val::Percent(75.0)),
                        overflow: Overflow::Hidden,
                        ..default()
                    },
                    background_color: Color::rgb(0.10, 0.10, 0.10).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Moving panel
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    flex_grow: 1.0,
                                    max_size: Size::UNDEFINED,
                                    ..default()
                                },
                                ..default()
                            },
                            ScrollingList::default(),
                        ))
                        .with_children(|parent| {
                            // List items
                            for file in fs::read_dir("./assets/layouts").unwrap() {
                                let file_name = file.unwrap().path().display().to_string();
                                parent
                                    .spawn(ButtonBundle {
                                        style: Style {
                                            flex_shrink: 0.,
                                            size: Size::new(Val::Percent(100.0), Val::Px(30.)),
                                            // center button
                                            margin: UiRect {
                                                left: Val::Auto,
                                                right: Val::Auto,
                                                ..default()
                                            },
                                            // horizontally center child text
                                            justify_content: JustifyContent::FlexStart,
                                            // vertically center child text
                                            align_items: AlignItems::FlexStart,
                                            flex_direction: FlexDirection::Column,
                                            ..default()
                                        },
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    })
                                    .insert(AssetPath {
                                        path: file_name.clone(),
                                    })
                                    .with_children(|parent| {
                                        parent.spawn(TextBundle::from_section(
                                            format!(" Layout {file_name}"),
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: 20.,
                                                color: Color::WHITE,
                                            },
                                        ));
                                    });
                            }
                        });
                });
        });
    });

    let menu_entity = menu.id();
    commands.insert_resource(MenuData {
        menu_entity: menu_entity,
    });
}

fn menu(
    mut state: ResMut<State<AppState>>,
    mut main_layout: ResMut<MainLayout>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &AssetPath),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, path) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                main_layout.path = path.path.clone();
                state.set(AppState::Loading).unwrap();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.menu_entity).despawn_recursive();
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Children, &Node)>,
    query_item: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, children, uinode) in &mut query_list {
            let items_height: f32 = children
                .iter()
                .map(|entity| query_item.get(*entity).unwrap().size().y)
                .sum();
            let panel_height = uinode.size().y;
            let max_scroll = (items_height - panel_height).max(0.);
            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}
