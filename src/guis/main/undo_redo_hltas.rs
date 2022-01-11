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
    EditLine {
        line: Line,
        index: usize,
    },
}

impl Action {
    // TODO is there a better way to implement this (the hltas and tab_menu_data)

    /// Takes action to the hltas file depending on what enum is selected
    ///
    /// * Returns a reverse of what action was taken in a new Action instance
    fn take_action(&self, hltas: &mut HLTAS, tab_menu_data: &mut HLTASMenuState) -> Self {
        match self {
            Action::DeleteLine { indexes_and_lines } => {
                for (i, line) in indexes_and_lines {
                    if hltas.lines.is_empty() {
                        tab_menu_data.push_hltas_line(line);
                        hltas.lines.push(line.to_owned());
                    } else {
                        tab_menu_data.insert_hltas_line(*i, line);
                        hltas.lines.insert(*i, line.to_owned());
                    }
                }

                Action::AddLine {
                    indexes: indexes_and_lines.iter().map(|(i, _)| *i).collect(),
                }
            }
            Action::AddLine { indexes } => {
                let indexes_and_lines = indexes
                    .iter()
                    .map(|i| (*i, hltas.lines[*i].to_owned()))
                    .collect();

                for i in indexes.iter().rev() {
                    tab_menu_data.remove_line_at_index(*i);
                    hltas.lines.remove(*i);
                }

                Action::DeleteLine { indexes_and_lines }
            }
            Action::EditLine { line, index } => {
                let line_before_edit = hltas.lines[*index].to_owned();

                hltas.lines[*index] = line.to_owned();

                Action::EditLine {
                    line: line_before_edit,
                    index: *index,
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct UndoRedoHandler {
    undo_stack: Vec<Action>,
    redo_stack: Vec<Action>,
}

impl UndoRedoHandler {
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

    // BUG undo on comment while having the cursor focused seems to break undo somehow
    pub fn edit_line(&mut self, prev_state: Line, index: usize) {
        self.redo_stack.clear();

        self.undo_stack.push(Action::EditLine {
            index,
            line: prev_state,
        });
    }
}
