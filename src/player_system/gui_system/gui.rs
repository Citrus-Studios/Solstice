use crate::constants::GUI_LOOKUP;

use super::gui_startup::*;
use bevy::prelude::*;

pub fn gui(
    commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children, &GuiButtonId),
        Changed<Interaction>
    >,
    mut button_query: QuerySet<(QueryState<&mut GuiButtons>, QueryState<&GuiButtons>)>,
    mut text_query: Query<(&mut Text, &GuiTextId, &mut Visibility)>,
) {
    let mut i = 0u32;
    for (interaction, mut color, children, button_id) in interaction_query.iter_mut() {
        // info!("{:?}", interaction);
        match *interaction {
            Interaction::Clicked => {
                let clicked_button_content = &button_query.q1().iter().nth(button_id.id as usize).unwrap().content;
                match clicked_button_content {
                    GuiOr::Id(e) => {
                        let branch = GUI_LOOKUP.get(e).unwrap();
                        let mut button_query_q0 = button_query.q0();
                        let mut button_iter = button_query_q0.iter_mut();
                        for (mut text, i, mut visibility) in text_query.iter_mut() {
                            let button_content = &branch[i.id  as usize];
                            match &button_content {
                                // Set the text in the button                                  Ignore everything before underscores
                                GuiOr::Id(e) => text.sections[0].value = e.to_string().rsplitn(2, "_").next().unwrap().to_string(),
                                GuiOr::Item(e) => text.sections[0].value = e.to_string().rsplitn(2, "_").next().unwrap().to_string(),

                                // Hide the button
                                GuiOr::None => {
                                    visibility.is_visible = false
                                },
                            };
                            let mut current_button = button_iter.next().unwrap();
                            current_button.content = button_content.clone();
                        }
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
        i += 1;
    }
}

#[derive(Clone, Debug)]
pub enum GuiOr<T> {
    Id(String),
    Item(T),
    None
}

