//! This example showcases setting up a basic application and window, setting up some views to
//! work with autolayout, and some basic ways to handle colors.

use cacao::webview::{WebView, WebViewConfig, WebViewDelegate};

use cacao::input::{TextField, TextFieldDelegate};

use cacao::macos::{App, AppDelegate};
use cacao::macos::toolbar::{Toolbar, ToolbarItem, ItemIdentifier, ToolbarDelegate};
use cacao::macos::menu::{Menu, MenuItem};
use cacao::macos::window::{Window, WindowConfig, WindowDelegate};

struct BasicApp {
    window: Window<AppWindow>
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        App::set_menu(vec![
            Menu::new("", vec![
                MenuItem::Services,
                MenuItem::Separator,
                MenuItem::Hide,
                MenuItem::HideOthers,
                MenuItem::ShowAll,
                MenuItem::Separator,
                MenuItem::Quit
            ]),

            Menu::new("File", vec![
                MenuItem::CloseWindow
            ]),

            Menu::new("Edit", vec![
                MenuItem::Undo,
                MenuItem::Redo,
                MenuItem::Separator,
                MenuItem::Cut,
                MenuItem::Copy,
                MenuItem::Paste,
                MenuItem::Separator,
                MenuItem::SelectAll
            ]),

            Menu::new("View", vec![
                MenuItem::EnterFullScreen
            ]),

            Menu::new("Window", vec![
                MenuItem::Minimize,
                MenuItem::Zoom,
                MenuItem::Separator,
                MenuItem::new("Bring All to Front")
            ]),

            Menu::new("Help", vec![])
        ]);

        App::activate();
        self.window.show();
    }
}

pub struct URLBar;

impl TextFieldDelegate for URLBar {
    const NAME: &'static str = "URLBar";
}

struct BrowserToolbar {
    url_bar: TextField<URLBar>
}

impl BrowserToolbar {
    pub fn new() -> Self {
        BrowserToolbar {
            url_bar: TextField::with(URLBar)
        }
    }
}

impl ToolbarDelegate for BrowserToolbar {
    const NAME: &'static str = "BrowserToolbar";

    fn allowed_item_identifiers(&self) -> Vec<ItemIdentifier> { vec![] }
    fn default_item_identifiers(&self) -> Vec<ItemIdentifier> { vec![] }

    fn item_for(&self, _identifier: &str) -> &ToolbarItem { std::unreachable!(); }
}

#[derive(Default)]
pub struct WebViewInstance;

impl WebViewDelegate for WebViewInstance {}

struct AppWindow {
    toolbar: Toolbar<BrowserToolbar>,
    content: WebView<WebViewInstance>
}

impl AppWindow {
    pub fn new() -> Self {
        AppWindow {
            toolbar: Toolbar::new("com.example.BrowserToolbar", BrowserToolbar::new()),
            content: WebView::with(WebViewConfig::default(), WebViewInstance::default())
        }
    }
}

impl WindowDelegate for AppWindow {
    const NAME: &'static str = "WindowDelegate";

    fn did_load(&mut self, window: Window) {
        window.set_title("Browser Example");
        window.set_minimum_content_size(400., 400.);

        window.set_toolbar(&self.toolbar);
        window.set_content_view(&self.content);

        self.content.load_url("https://www.duckduckgo.com/");
    }
}

fn main() {
    App::new("com.test.window", BasicApp {
        window: Window::with(WindowConfig::default(), AppWindow::new())
    }).run();
}