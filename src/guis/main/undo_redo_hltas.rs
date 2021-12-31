use hltas::{types::Line, HLTAS};

use super::tab::HLTASMenuState;

#[derive(Clone, Debug)]
enum Action {
    DeleteLine {
        indexes_and_lines: Vec<(usize, Line)>,
    },
    AddLine {
        indexes: Vec<usize>,
    },
}

impl Action {
    // TODO is there a better way to implement this (the hltas and tab_menu_data)

    /// Takes action to the hltas file depending on what enum is selected
    ///
    /// * Returns a reverse of what action was taken in a new Action instance
    fn take_action(&self, hltas: &mut HLTAS, tab_menu_data: &mut HLTASMenuState) -> Self {
        match self {
            Action::DeleteLine {
                indexes_and_lines: lines_and_indexes,
            } => {
                for (i, line) in lines_and_indexes {
                    if hltas.lines.is_empty() {
                        tab_menu_data.push_hltas_line(&line);
                        hltas.lines.push(line.to_owned());
                    } else {
                        tab_menu_data.insert_hltas_line(*i, &line);
                        hltas.lines.insert(*i, line.to_owned());
                    }
                }

                Action::AddLine {
                    indexes: lines_and_indexes.into_iter().map(|(i, _)| *i).collect(),
                }
            }
            Action::AddLine { indexes } => {
                let mut indexes_and_lines = Vec::new();

                for i in indexes.into_iter().rev() {
                    tab_menu_data.remove_line_at_index(*i);
                    indexes_and_lines.push((*i, hltas.lines[*i].to_owned()));
                    hltas.lines.remove(*i);
                }

                Action::DeleteLine { indexes_and_lines }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct UndoRedoHandler {
    undo_stack: Vec<Action>,
    redo_stack: Vec<Action>,
}

impl UndoRedoHandler {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn undo(&mut self, hltas: &mut HLTAS, tab_menu_data: &mut HLTASMenuState) {
        if let Some(undo_action) = self.undo_stack.pop() {
            self.redo_stack
                .push(undo_action.take_action(hltas, tab_menu_data));

            tab_menu_data.got_modified();
        }
    }

    pub fn redo(&mut self, hltas: &mut HLTAS, tab_menu_data: &mut HLTASMenuState) {
        if let Some(redo_action) = self.redo_stack.pop() {
            self.undo_stack
                .push(redo_action.take_action(hltas, tab_menu_data));

            tab_menu_data.got_modified();
        }
    }

    pub fn delete_lines(&mut self, deleted_lines: Vec<(usize, Line)>) {
        self.redo_stack.clear();

        self.undo_stack.push(Action::DeleteLine {
            indexes_and_lines: deleted_lines,
        });
    }

    pub fn add_lines(&mut self, indexes: Vec<usize>) {
        self.redo_stack.clear();

        self.undo_stack.push(Action::AddLine { indexes });
    }
}
