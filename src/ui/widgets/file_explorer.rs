use super::*;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FileExplorer {
    base: BaseWidget,
    current_path: PathBuf,
    selected_item: Option<PathBuf>,
    scroll_offset: f32,
    item_height: f32,
    on_select: Option<Box<dyn Fn(&Path) + Send + Sync>>,
    on_double_click: Option<Box<dyn Fn(&Path) + Send + Sync>>,
}

#[derive(Debug)]
struct FileItem {
    name: String,
    path: PathBuf,
    is_dir: bool,
    size: u64,
    modified: std::time::SystemTime,
}

impl FileExplorer {
    pub fn new<P: AsRef<Path>>(initial_path: P) -> Self {
        let mut base = BaseWidget::new(WidgetType::Container);

        // Set default style
        base.style = Style::new()
            .with_background_color(Vec4::new(0.15, 0.15, 0.15, 1.0))
            .with_border_color(Vec4::new(0.3, 0.3, 0.3, 1.0))
            .with_border_width(1.0)
            .with_corner_radius(4.0);

        Self {
            base,
            current_path: initial_path.as_ref().to_path_buf(),
            selected_item: None,
            scroll_offset: 0.0,
            item_height: 24.0,
            on_select: None,
            on_double_click: None,
        }
    }

    pub fn with_on_select<F>(mut self, callback: F) -> Self
    where
        F: Fn(&Path) + Send + Sync + 'static,
    {
        self.on_select = Some(Box::new(callback));
        self
    }

    pub fn with_on_double_click<F>(mut self, callback: F) -> Self
    where
        F: Fn(&Path) + Send + Sync + 'static,
    {
        self.on_double_click = Some(Box::new(callback));
        self
    }

    fn get_items(&self) -> Vec<FileItem> {
        let mut items = Vec::new();

        // Add parent directory entry if not at root
        if let Some(parent) = self.current_path.parent() {
            items.push(FileItem {
                name: "..".to_string(),
                path: parent.to_path_buf(),
                is_dir: true,
                size: 0,
                modified: std::time::SystemTime::now(),
            });
        }

        // Add current directory entries
        if let Ok(entries) = fs::read_dir(&self.current_path) {
            for entry in entries.filter_map(Result::ok) {
                if let Ok(metadata) = entry.metadata() {
                    items.push(FileItem {
                        name: entry.file_name().to_string_lossy().into_owned(),
                        path: entry.path(),
                        is_dir: metadata.is_dir(),
                        size: metadata.len(),
                        modified: metadata
                            .modified()
                            .unwrap_or_else(|_| std::time::SystemTime::now()),
                    });
                }
            }
        }

        // Sort items: directories first, then files, alphabetically
        items.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });

        items
    }

    fn handle_item_click(&mut self, item: &FileItem, double_click: bool) {
        if double_click {
            if item.is_dir {
                self.current_path = item.path.clone();
                self.selected_item = None;
            }
            if let Some(ref callback) = self.on_double_click {
                callback(&item.path);
            }
        } else {
            self.selected_item = Some(item.path.clone());
            if let Some(ref callback) = self.on_select {
                callback(&item.path);
            }
        }
    }
}

impl Widget for FileExplorer {
    // Implement standard Widget trait methods
    fn widget_type(&self) -> WidgetType {
        self.base.widget_type()
    }
    fn id(&self) -> u64 {
        self.base.id()
    }
    fn rect(&self) -> Rect {
        self.base.rect()
    }
    fn set_rect(&mut self, rect: Rect) {
        self.base.set_rect(rect);
    }

    fn update(&mut self, ctx: &mut UIContext) {
        let items = self.get_items();
        let rect = self.base.rect();
        let content_height = items.len() as f32 * self.item_height;

        // Handle scrolling
        if ctx.focused_widget == Some(self.id()) {
            self.scroll_offset = (self.scroll_offset - ctx.scroll_delta.y)
                .clamp(0.0, (content_height - rect.size.y).max(0.0));
        }

        // Handle clicks
        if ctx.pressed && rect.contains(ctx.mouse_position) {
            let local_y = ctx.mouse_position.y - rect.position.y + self.scroll_offset;
            let index = (local_y / self.item_height) as usize;

            if index < items.len() {
                let double_click = ctx.last_click_time.elapsed().as_millis() < 500;
                self.handle_item_click(&items[index], double_click);
            }
        }
    }

    fn draw(&self, renderer: &mut UIRenderer) {
        let items = self.get_items();
        let rect = self.base.rect();

        // Draw background
        renderer.draw_rect(rect, self.base.style.background_color.unwrap_or(Vec4::ZERO));

        // Calculate visible range
        let start_index = (self.scroll_offset / self.item_height) as usize;
        let end_index = ((self.scroll_offset + rect.size.y) / self.item_height).ceil() as usize;
        let visible_items = &items[start_index..end_index.min(items.len())];

        // Draw items
        for (i, item) in visible_items.iter().enumerate() {
            let item_y = rect.position.y + (i as f32 * self.item_height)
                - (self.scroll_offset % self.item_height);
            let item_rect = Rect::new(
                Vec2::new(rect.position.x, item_y),
                Vec2::new(rect.size.x, self.item_height),
            );

            // Draw selection highlight
            if Some(&item.path) == self.selected_item.as_ref() {
                renderer.draw_rect(item_rect, Vec4::new(0.2, 0.4, 0.8, 0.5));
            }

            // Draw item icon and name
            let icon = if item.is_dir { "📁 " } else { "📄 " };
            renderer.draw_text(
                &format!("{}{}", icon, item.name),
                item_rect.position + Vec2::new(8.0, 4.0),
                14.0,
                Vec4::ONE,
            );
        }

        // Draw scrollbar if needed
        let content_height = items.len() as f32 * self.item_height;
        if content_height > rect.size.y {
            let scroll_ratio = rect.size.y / content_height;
            let scroll_pos = (self.scroll_offset / content_height) * rect.size.y;
            let scroll_size = rect.size.y * scroll_ratio;

            renderer.draw_rect(
                Rect::new(
                    Vec2::new(
                        rect.position.x + rect.size.x - 8.0,
                        rect.position.y + scroll_pos,
                    ),
                    Vec2::new(8.0, scroll_size),
                ),
                Vec4::new(0.3, 0.3, 0.3, 1.0),
            );
        }
    }
}
