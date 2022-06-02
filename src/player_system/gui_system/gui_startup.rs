use bevy::prelude::*;

use crate::{
    building_system::buildings::BuildingType,
    constants::{ShortToString, GUI_LOOKUP},
};

use super::gui::GuiOr;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

pub struct GuiSelectedBranch {
    pub id: String,
}

#[derive(Component)]
pub struct GuiButtons {
    pub content: GuiOr<String>,
    pub id: u32,
}

#[derive(Component)]
pub struct GuiButtonId {
    pub id: u32,
}

#[derive(Component)]
pub struct GuiTextId {
    pub id: u32,
}

#[derive(Component)]
pub struct GuiTextBox {
    pub id: u32,
}

pub struct PrevQPress {
    pub pressed: bool,
}

pub struct SelectedBuilding {
    pub id: Option<BuildingType>,
    pub changed: bool,
}

pub fn gui_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("gui start");

    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
    commands.insert_resource(GuiSelectedBranch { id: "base".ts() });
    commands.insert_resource(PrevQPress { pressed: false });
    commands.insert_resource(SelectedBuilding {
        id: None,
        changed: false,
    });
    let branch = GUI_LOOKUP.get(&"base".ts()).unwrap();
    let mut margin = Rect::default();
    margin.left = Val::Px(14.0);
    // base node bundle
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(153.0), Val::Px(656.0)),
                margin,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Baseline,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .with_children(|parent| {
            // spawn 4 buttons in the bundle as children
            for i in 0..=3 {
                let mut margin = Rect::default();
                let j = 3 - i;
                margin.bottom = Val::Px(5.0);
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(153.0), Val::Px(150.0)),
                            margin,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Baseline,
                            ..Default::default()
                        },
                        color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    })
                    .insert(GuiButtons {
                        content: branch[j].clone(),
                        id: j as u32,
                    })
                    .insert(GuiButtonId { id: 3 - j as u32 })
                    .with_children(|parent| {
                        let mut margin = Rect::default();
                        margin.bottom = Val::Px(114.0);
                        parent
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(147.0), Val::Px(33.0)),
                                    margin: margin,
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Baseline,
                                    ..Default::default()
                                },
                                color: Color::rgb(0.5, 0.5, 0.5).into(),
                                ..Default::default()
                            })
                            .remove::<Interaction>()
                            .insert(GuiTextBox { id: 3 - j as u32 })
                            .with_children(|parent| {
                                parent
                                    .spawn_bundle(TextBundle {
                                        text: Text::with_section(
                                            match &branch[j] {
                                                GuiOr::Id(e) => e.clone(),
                                                _ => panic!("you suck"),
                                            },
                                            TextStyle {
                                                font: asset_server.load("fonts/zekton-rg.ttf"),
                                                font_size: 26.0,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                            Default::default(),
                                        ),
                                        ..Default::default()
                                    })
                                    .insert(GuiTextId { id: j as u32 });
                            });
                    });
            }
        });
    info!("gui done");
}
