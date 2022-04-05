use crate::constants::{GUI_LOOKUP, GUI_BACK_LOOKUP};

use super::gui_startup::*;
use bevy::{prelude::*, ecs::query::{QueryIter, WriteFetch, EntityFetch, ReadFetch}};

pub fn gui(
    commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &GuiButtonId),
        Changed<Interaction>
    >,
    mut button_query: QuerySet<(QueryState<(&mut GuiButtons, Entity, &Children)>, QueryState<&GuiButtons>)>,
    mut text_query: Query<(&mut Text, &GuiTextId, Entity)>,
    mut visibility_query: Query<&mut Visibility>,
    mut selected_branch: ResMut<GuiSelectedBranch>,

    mut prev_q: ResMut<PrevQPress>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut clicked = false;
    for (interaction, mut color, button_id) in interaction_query.iter_mut() {
        // info!("{:?}", interaction);
        match *interaction {
            Interaction::Clicked => {
                clicked = true;
                let clicked_button_content = &button_query.q1().iter().nth(button_id.id as usize).unwrap().content.clone();
                match clicked_button_content {
                    GuiOr::Id(e) => {
                        selected_branch.id = e.to_string();
                        let mut button_query_q0 = button_query.q0();
                        let mut button_iter = button_query_q0.iter_mut();
                        change_buttons(&e.to_string(), &mut button_iter, &mut text_query, &mut visibility_query);
                    }
                    GuiOr::Item(e) => {
                        info!("you selected {:?}!", e);
                    }
                    _ => (),
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }

    if !clicked {
        let mut pressed_id = -1;
        if keyboard_input.just_pressed(KeyCode::Z) {
            pressed_id = 3;
        } else if keyboard_input.just_pressed(KeyCode::X) {
            pressed_id = 2;
        } else if keyboard_input.just_pressed(KeyCode::C) {
            pressed_id = 1;
        } else if keyboard_input.just_pressed(KeyCode::V) {
            pressed_id = 0;
        }

        if pressed_id >= 0 {
            let clicked_button_content = &button_query.q1().iter().nth(pressed_id as usize).unwrap().content.clone();
            match clicked_button_content {
                GuiOr::Id(e) => {
                    selected_branch.id = e.to_string();
                    let mut button_query_q0 = button_query.q0();
                    let mut button_iter = button_query_q0.iter_mut();
                    change_buttons(&e.to_string(), &mut button_iter, &mut text_query, &mut visibility_query);
                }
                GuiOr::Item(e) => {
                    info!("you selected {:?}!", e);
                }
                _ => (),
            }
        }
    }

    if keyboard_input.pressed(KeyCode::Q) && !prev_q.pressed {
        let mut button_query_q0 = button_query.q0();
        let mut button_iter = button_query_q0.iter_mut();
        selected_branch.id = GUI_BACK_LOOKUP.get(&selected_branch.id).unwrap().to_string();
        change_buttons(&selected_branch.id, &mut button_iter, &mut text_query, &mut visibility_query);
    }
    prev_q.pressed = keyboard_input.pressed(KeyCode::Q);
}

#[derive(Clone, Debug)]
pub enum GuiOr<T> {
    Id(String),
    Item(T),
    None
}

fn change_buttons(
    branch_id: &String, 
    button_iter: &mut QueryIter<(&mut GuiButtons, Entity, &Children), (WriteFetch<GuiButtons>, EntityFetch, ReadFetch<Children>), ()>, 
    text_query: &mut Query<(&mut Text, &GuiTextId, Entity)>, 
    visibility_query: &mut Query<&mut Visibility>
) {
    let branch = GUI_LOOKUP.get(branch_id).unwrap();

    for (mut text, i, text_entity) in text_query.iter_mut() {
        let button_content = &branch[i.id  as usize];
        let (mut cur_button_content, cur_button_entity, cur_button_child) = button_iter.next().unwrap();

        visibility_query.get_mut(text_entity).unwrap().is_visible = true;
        visibility_query.get_mut(cur_button_entity).unwrap().is_visible = true;
        visibility_query.get_mut(cur_button_child[0]).unwrap().is_visible = true;
        match &button_content {
            // Set the text in the button                                  Ignore everything before underscores
            GuiOr::Id(e) => text.sections[0].value = e.to_string().rsplitn(2, "_").next().unwrap().to_string(),
            GuiOr::Item(e) => text.sections[0].value = e.to_string().rsplitn(2, "_").next().unwrap().to_string(),

            // Hide the button
            GuiOr::None => {
                visibility_query.get_mut(text_entity).unwrap().is_visible = false;
                visibility_query.get_mut(cur_button_entity).unwrap().is_visible = false;
                visibility_query.get_mut(cur_button_child[0]).unwrap().is_visible = false;
            },
        };
        cur_button_content.content = button_content.clone();
    }
}