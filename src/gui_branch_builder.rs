use crate::player_system::gui_system::gui::GuiOr;

pub struct GuiBranchBuilder([GuiOr; 4]);

impl GuiBranchBuilder {
    pub const fn new() -> Self {
        GuiBranchBuilder([GuiOr::None, GuiOr::None, GuiOr::None, GuiOr::None])
    }

    pub const fn item(mut self, item: &'static str) -> Self {
        match self.0[0] {
            GuiOr::None => {
                self.0[0] = GuiOr::Item(item);
                return self;
            }
            _ => (),
        }

        match self.0[1] {
            GuiOr::None => {
                self.0[1] = GuiOr::Item(item);
                return self;
            }
            _ => (),
        }

        match self.0[2] {
            GuiOr::None => {
                self.0[2] = GuiOr::Item(item);
                return self;
            }
            _ => (),
        }

        match self.0[3] {
            GuiOr::None => {
                self.0[3] = GuiOr::Item(item);
                return self;
            }
            _ => (),
        }

        panic!("Too many items passed, builder already full");
    }

    pub const fn branch(mut self, branch: &'static str) -> Self {
        match self.0[0] {
            GuiOr::None => {
                self.0[0] = GuiOr::Id(branch);
                return self;
            }
            _ => (),
        }

        match self.0[1] {
            GuiOr::None => {
                self.0[1] = GuiOr::Id(branch);
                return self;
            }
            _ => (),
        }

        match self.0[2] {
            GuiOr::None => {
                self.0[2] = GuiOr::Id(branch);
                return self;
            }
            _ => (),
        }

        match self.0[3] {
            GuiOr::None => {
                self.0[3] = GuiOr::Id(branch);
                return self;
            }
            _ => (),
        }

        panic!("Too many items passed, builder already full");
    }

    pub const fn build(self) -> [GuiOr; 4] {
        self.0
    }
}
