use super::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Shortcut {
    pub key: Key,
    pub modifiers: Modifiers,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Modifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool, // Command on macOS, Windows key on Windows
}

#[derive(Debug)]
pub struct ShortcutManager {
    shortcuts: HashMap<Shortcut, Arc<dyn Fn() + Send + Sync>>,
    active_modifiers: Modifiers,
}

impl Shortcut {
    pub fn new(key: Key, modifiers: Modifiers) -> Self {
        Self { key, modifiers }
    }

    pub fn ctrl(key: Key) -> Self {
        Self::new(
            key,
            Modifiers {
                ctrl: true,
                shift: false,
                alt: false,
                meta: false,
            },
        )
    }

    pub fn cmd(key: Key) -> Self {
        Self::new(
            key,
            Modifiers {
                ctrl: false,
                shift: false,
                alt: false,
                meta: true,
            },
        )
    }

    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();

        #[cfg(target_os = "macos")]
        {
            if self.modifiers.meta {
                parts.push("⌘");
            }
            if self.modifiers.shift {
                parts.push("⇧");
            }
            if self.modifiers.alt {
                parts.push("⌥");
            }
            if self.modifiers.ctrl {
                parts.push("⌃");
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            if self.modifiers.ctrl {
                parts.push("Ctrl");
            }
            if self.modifiers.shift {
                parts.push("Shift");
            }
            if self.modifiers.alt {
                parts.push("Alt");
            }
            if self.modifiers.meta {
                parts.push("Win");
            }
        }

        parts.push(&self.key.to_string());
        parts.join("+")
    }
}

impl ShortcutManager {
    pub fn new() -> Self {
        Self {
            shortcuts: HashMap::new(),
            active_modifiers: Modifiers {
                ctrl: false,
                shift: false,
                alt: false,
                meta: false,
            },
        }
    }

    pub fn register<F>(&mut self, shortcut: Shortcut, action: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.shortcuts.insert(shortcut, Arc::new(action));
    }

    pub fn update(&mut self, ctx: &UIContext) {
        // Update modifier state
        self.active_modifiers = Modifiers {
            ctrl: ctx.is_key_pressed(Key::Control),
            shift: ctx.is_key_pressed(Key::Shift),
            alt: ctx.is_key_pressed(Key::Alt),
            meta: ctx.is_key_pressed(Key::Meta),
        };

        // Check for shortcuts
        if let Some(key) = ctx.key_pressed {
            let shortcut = Shortcut {
                key,
                modifiers: self.active_modifiers,
            };

            if let Some(action) = self.shortcuts.get(&shortcut) {
                action();
            }
        }
    }
}

// Extension for MenuItem to support shortcuts
impl MenuItem {
    pub fn with_shortcut_key(mut self, shortcut: Shortcut) -> Self {
        self.shortcut = Some(shortcut.to_string());
        self
    }
}

// Example usage in application:
pub fn register_menu_shortcuts(app: &mut Application) {
    app.shortcuts.register(Shortcut::ctrl(Key::S), || {
        // Save action
    });

    app.shortcuts.register(Shortcut::ctrl(Key::O), || {
        // Open action
    });

    app.shortcuts.register(
        Shortcut::new(
            Key::S,
            Modifiers {
                ctrl: true,
                shift: true,
                alt: false,
                meta: false,
            },
        ),
        || {
            // Save As action
        },
    );
}

// Example menu creation with shortcuts:
pub fn create_file_menu() -> Vec<MenuItem> {
    vec![
        MenuItem::new("New")
            .with_icon("📄")
            .with_shortcut_key(Shortcut::ctrl(Key::N)),
        MenuItem::new("Open...")
            .with_icon("📂")
            .with_shortcut_key(Shortcut::ctrl(Key::O)),
        MenuItem::separator(),
        MenuItem::new("Save")
            .with_icon("💾")
            .with_shortcut_key(Shortcut::ctrl(Key::S)),
        MenuItem::new("Save As...").with_shortcut_key(Shortcut::new(
            Key::S,
            Modifiers {
                ctrl: true,
                shift: true,
                alt: false,
                meta: false,
            },
        )),
    ]
}
