use super::*;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct TextEditor {
    text: String,
    cursor_position: usize,
    selection_start: Option<usize>,
    scroll_offset: Vec2,
    last_click_time: Instant,
    click_count: u8,
    preferred_x: f32,
    undo_stack: Vec<TextState>,
    redo_stack: Vec<TextState>,
}

#[derive(Debug, Clone)]
struct TextState {
    text: String,
    cursor_position: usize,
    selection_start: Option<usize>,
    timestamp: Instant,
}

impl TextEditor {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            cursor_position: 0,
            selection_start: None,
            scroll_offset: Vec2::ZERO,
            last_click_time: Instant::now(),
            click_count: 0,
            preferred_x: 0.0,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn handle_input(&mut self, input: &str) {
        self.begin_edit();

        // Delete selected text if any
        if let Some(selection) = self.get_selection() {
            self.delete_range(selection);
        }

        // Insert new text
        self.text.insert_str(self.cursor_position, input);
        self.cursor_position += input.chars().count();
        self.selection_start = None;

        self.end_edit();
    }

    pub fn handle_key(&mut self, key: KeyCode, modifiers: ModifierState) {
        match key {
            KeyCode::Left => {
                if modifiers.ctrl {
                    self.move_word_left(modifiers.shift);
                } else {
                    self.move_left(modifiers.shift);
                }
            }
            KeyCode::Right => {
                if modifiers.ctrl {
                    self.move_word_right(modifiers.shift);
                } else {
                    self.move_right(modifiers.shift);
                }
            }
            KeyCode::Backspace => {
                self.begin_edit();
                if let Some(selection) = self.get_selection() {
                    self.delete_range(selection);
                } else if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                    self.text.remove(self.cursor_position);
                }
                self.end_edit();
            }
            KeyCode::Delete => {
                self.begin_edit();
                if let Some(selection) = self.get_selection() {
                    self.delete_range(selection);
                } else if self.cursor_position < self.text.len() {
                    self.text.remove(self.cursor_position);
                }
                self.end_edit();
            }
            KeyCode::Home => {
                self.move_line_start(modifiers.shift);
            }
            KeyCode::End => {
                self.move_line_end(modifiers.shift);
            }
            KeyCode::Z if modifiers.ctrl => {
                if modifiers.shift {
                    self.redo();
                } else {
                    self.undo();
                }
            }
            _ => {}
        }
    }

    pub fn handle_click(&mut self, position: Vec2, font_atlas: &FontAtlas) {
        let now = Instant::now();
        if now.duration_since(self.last_click_time).as_millis() < 500 {
            self.click_count = (self.click_count + 1) % 3;
        } else {
            self.click_count = 0;
        }
        self.last_click_time = now;

        match self.click_count {
            0 => {
                // Single click: Move cursor
                self.cursor_position = self.get_character_index_at(position, font_atlas);
                self.selection_start = None;
            }
            1 => {
                // Double click: Select word
                let word_range = self.get_word_at(position, font_atlas);
                self.cursor_position = word_range.end;
                self.selection_start = Some(word_range.start);
            }
            2 => {
                // Triple click: Select line
                let line_range = self.get_line_at(position, font_atlas);
                self.cursor_position = line_range.end;
                self.selection_start = Some(line_range.start);
            }
            _ => unreachable!(),
        }
    }

    pub fn handle_drag(&mut self, position: Vec2, font_atlas: &FontAtlas) {
        if self.selection_start.is_none() {
            self.selection_start = Some(self.cursor_position);
        }
        self.cursor_position = self.get_character_index_at(position, font_atlas);
    }

    fn begin_edit(&mut self) {
        let state = TextState {
            text: self.text.clone(),
            cursor_position: self.cursor_position,
            selection_start: self.selection_start,
            timestamp: Instant::now(),
        };
        self.undo_stack.push(state);
        self.redo_stack.clear();
    }

    fn end_edit(&mut self) {
        // Merge edits that happen quickly in succession
        while self.undo_stack.len() > 1 {
            let last = self.undo_stack.last().unwrap();
            let second_last = &self.undo_stack[self.undo_stack.len() - 2];
            if last
                .timestamp
                .duration_since(second_last.timestamp)
                .as_millis()
                > 1000
            {
                break;
            }
            self.undo_stack.remove(self.undo_stack.len() - 2);
        }
    }

    fn undo(&mut self) {
        if let Some(state) = self.undo_stack.pop() {
            let current = TextState {
                text: self.text.clone(),
                cursor_position: self.cursor_position,
                selection_start: self.selection_start,
                timestamp: Instant::now(),
            };
            self.redo_stack.push(current);

            self.text = state.text;
            self.cursor_position = state.cursor_position;
            self.selection_start = state.selection_start;
        }
    }

    fn redo(&mut self) {
        if let Some(state) = self.redo_stack.pop() {
            let current = TextState {
                text: self.text.clone(),
                cursor_position: self.cursor_position,
                selection_start: self.selection_start,
                timestamp: Instant::now(),
            };
            self.undo_stack.push(current);

            self.text = state.text;
            self.cursor_position = state.cursor_position;
            self.selection_start = state.selection_start;
        }
    }

    // Helper methods for text navigation and selection
    fn get_selection(&self) -> Option<Range<usize>> {
        self.selection_start.map(|start| {
            let end = self.cursor_position;
            if start <= end {
                start..end
            } else {
                end..start
            }
        })
    }

    fn delete_range(&mut self, range: Range<usize>) {
        self.text.replace_range(range.clone(), "");
        self.cursor_position = range.start;
        self.selection_start = None;
    }

    // ... Additional helper methods for word/line navigation
}
