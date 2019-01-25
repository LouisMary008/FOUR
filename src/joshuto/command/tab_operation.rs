use std;
use std::path;
use std::env;

use joshuto::context::JoshutoContext;
use joshuto::command::JoshutoCommand;
use joshuto::command::JoshutoRunnable;
use joshuto::command::TabSwitch;
use joshuto::context::JoshutoTab;

#[derive(Clone, Debug)]
pub struct NewTab;

impl NewTab {
    pub fn new() -> Self { NewTab }
    pub const fn command() -> &'static str { "new_tab" }

    pub fn new_tab(context: &mut JoshutoContext)
    {
        let curr_path: path::PathBuf = match env::current_dir() {
            Ok(path) => { path },
            Err(e) => {
                eprintln!("{}", e);
                return;
            },
        };

        let tab = JoshutoTab::new(curr_path, &context.config_t.sort_type);

        context.tabs.push(tab);
        context.curr_tab_index = context.tabs.len() - 1;

        TabSwitch::tab_switch(context.tabs.len() as i32 - 1, context);
    }
}

impl JoshutoCommand for NewTab {}

impl std::fmt::Display for NewTab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for NewTab {
    fn execute(&self, context: &mut JoshutoContext)
    {
        Self::new_tab(context);
    }
}

#[derive(Clone, Debug)]
pub struct CloseTab;

impl CloseTab {
    pub fn new() -> Self { CloseTab }
    pub const fn command() -> &'static str { "close_tab" }

    pub fn close_tab(context: &mut JoshutoContext)
    {
        if context.tabs.len() <= 1 {
            return;
        }

        context.tabs.remove(context.curr_tab_index);
        if context.curr_tab_index > 0 {
            context.curr_tab_index = context.curr_tab_index - 1;
        }
        TabSwitch::tab_switch(context.curr_tab_index as i32, context);
    }
}

impl JoshutoCommand for CloseTab {}

impl std::fmt::Display for CloseTab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for CloseTab {
    fn execute(&self, context: &mut JoshutoContext)
    {
        Self::close_tab(context);
    }
}
