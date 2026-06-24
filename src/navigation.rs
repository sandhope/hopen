/// Navigation page definitions for the Hopen GPUI client.
///
/// Each variant represents a top-level page accessible from the sidebar.
/// This mirrors FlClash's `PageLabel` enum but redesigned for a desktop-first experience.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Page {
    #[default]
    Dashboard,
    Proxies,
    Profiles,
    Requests,
    Connections,
    Resources,
    Logs,
    Tools,
}

/// Sub-pages within the Settings (Tools) page.
/// Used for drill-down navigation with a back button.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolsSubPage {
    Language,
}

impl Page {
    /// All pages in display order.
    pub const ALL: &'static [Page] = &[
        Page::Dashboard,
        Page::Proxies,
        Page::Profiles,
        Page::Requests,
        Page::Connections,
        Page::Resources,
        Page::Logs,
        Page::Tools,
    ];

    /// Display title for the page header.
    pub fn title(&self) -> &'static str {
        match self {
            Page::Dashboard => "Dashboard",
            Page::Proxies => "Proxies",
            Page::Profiles => "Profiles",
            Page::Requests => "Requests",
            Page::Connections => "Connections",
            Page::Resources => "Resources",
            Page::Logs => "Logs",
            Page::Tools => "Settings",
        }
    }

    /// SVG asset path for the sidebar nav icon (loaded via `AssetSource`).
    pub fn icon_path(&self) -> &'static str {
        match self {
            Page::Dashboard => "svg/dashboard.svg",
            Page::Proxies => "svg/proxies.svg",
            Page::Profiles => "svg/profiles.svg",
            Page::Requests => "svg/requests.svg",
            Page::Connections => "svg/connections.svg",
            Page::Resources => "svg/resources.svg",
            Page::Logs => "svg/logs.svg",
            Page::Tools => "svg/tools.svg",
        }
    }


}
