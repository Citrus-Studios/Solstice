use crate::constants::GUI_LOOKUP;

use super::gui_startup::*;
use bevy::prelude::*;

pub fn gui(
    commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children, &mut GuiButtons),
        (Changed<Interaction>, With<GuiButtons>),
    >,
    mut text_query: Query<(&mut Text, &GuiTextId)>,
) {
    let mut i = 0u32;
    for (interaction, mut color, children, mut button) in interaction_query.iter_mut() {
        // info!("{:?}", interaction);
        match *interaction {
            Interaction::Clicked => {
                match &button.content {
                    GuiOr::Id(e) => {
                        info!("ummmm hewo?");
                        let branch = GUI_LOOKUP.get(e).unwrap();
                        for (mut text, i) in text_query.iter_mut() {
                            let button_content = &branch[(3 - i.id)  as usize];
                            text.sections[0].value = match &button_content {
                                GuiOr::Id(e) => e.to_string().rsplitn(2, "_").next().unwrap().to_string(),
                                GuiOr::Item(e) => e.to_string().rsplitn(2, "_").next().unwrap().to_string(),
                                GuiOr::None => todo!(),
                            };
                            button.content = button_content.clone();
                        }
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

