use crate::users::UserInfo;

pub struct App {
    pub users: Vec<UserInfo>,
    pub selected_user_index: Option<usize>,
    pub should_quit: bool,
    pub focus_users: bool, // true = focus on user list, false = focus on stats
}

impl App {
    pub fn new(users: Vec<UserInfo>) -> Self {
        let selected = if !users.is_empty() { Some(0) } else { None };
        App {
            users,
            selected_user_index: selected,
            should_quit: false,
            focus_users: true,
        }
    }

    pub fn next(&mut self) {
        if !self.users.is_empty() {
            let i = match self.selected_user_index {
                Some(i) => {
                    if i >= self.users.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.selected_user_index = Some(i);
        }
    }

    pub fn previous(&mut self) {
        if !self.users.is_empty() {
            let i = match self.selected_user_index {
                Some(i) => {
                    if i == 0 {
                        self.users.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.selected_user_index = Some(i);
        }
    }

    pub fn toggle_focus(&mut self) {
        self.focus_users = !self.focus_users;
    }

    pub fn selected_user(&self) -> Option<&UserInfo> {
        self.selected_user_index.map(|i| &self.users[i])
    }

    pub fn on_tick(&mut self) {
        // Update any dynamic components if needed
    }
    
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
