use hltas::{types::Line, HLTAS};

use super::tab::HLTASMenuState;

#[derive(Clone, Debug)]
enum Action {
    DeleteLine { index: usize, lines: Vec<Line> },
    AddLine { index: usize, count: usize },
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

    // TODO is there a better way to implement this (the hltas and tab_menu_data)
    pub fn undo(&mut self, hltas: &mut HLTAS, tab_menu_data: &mut HLTASMenuState) {
        if let Some(undo_action) = self.undo_stack.pop() {
            match undo_action {
                Action::DeleteLine { index, lines } => {
                    if hltas.lines.is_empty() {
                        for line in lines.into_iter() {
                            tab_menu_data.push_hltas_line(&line);
                            hltas.lines.push(line);
                            tab_menu_data.got_modified();
                        }
                    } else {
                        for (i, line) in lines.into_iter().enumerate() {
                            tab_menu_data.insert_hltas_line(index + i, &line);
                            hltas.lines.insert(index + i, line);
                            tab_menu_data.got_modified();
                        }
                    }
                }
                Action::AddLine { index, count } => {
                    for _ in 0..count {
                        tab_menu_data.remove_line_at_index(index);
                        hltas.lines.remove(index);
                        tab_menu_data.got_modified();
                    }
                }
            }
        }
    }

    pub fn delete_lines(&mut self, starting_index: usize, deleted_lines: Vec<Line>) {
        self.redo_stack.clear();

        self.undo_stack.push(Action::DeleteLine {
            index: starting_index,
            lines: deleted_lines,
        });
    }

    pub fn add_lines(&mut self, starting_index: usize, line_count: usize) {
        self.redo_stack.clear();

        self.undo_stack.push(Action::AddLine {
            index: starting_index,
            count: line_count,
        });
    }
}
