use super::*;

pub struct FileExplorer {
    files: Vec<String>,
    selected_file: Option<String>,
}

impl FileExplorer {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            selected_file: None,
        }
    }
}

// Moved implementation outside the struct
impl DragSource for FileExplorer {
    fn can_drag(&self) -> bool {
        self.selected_file.is_some()
    }

    fn begin_drag(&self) -> Option<DragData> {
        self.selected_file.as_ref().map(|file| DragData {
            data_type: "file".to_string(),
            payload: Arc::new(file.clone()),
            preview: Some("📄".to_string()),
            allowed_drops: vec!["folder".to_string()],
        })
    }

    fn end_drag(&mut self, completed: bool) {
        if completed {
            self.selected_file = None;
        }
    }
}

pub struct TreeNode {
    text: String,
    children: Vec<TreeNode>,
    is_folder: bool,
}

impl TreeNode {
    pub fn new(text: &str, is_folder: bool) -> Self {
        Self {
            text: text.to_string(),
            children: Vec::new(),
            is_folder,
        }
    }
}

// Moved implementation outside the struct
impl DropTarget for TreeNode {
    fn can_drop(&self, data: &DragData) -> bool {
        self.is_folder && data.data_type == "file"
    }

    fn drop(&mut self, data: DragData) {
        if let Ok(file_name) = Arc::try_unwrap(data.payload) {
            self.children.push(TreeNode::new(&file_name, false));
        }
    }
}
