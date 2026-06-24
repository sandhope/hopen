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

    /// Unicode icon character for the sidebar nav item.
    /// Using Unicode symbols as placeholder — will be replaced with proper icon font later.
    pub fn icon(&self) -> &'static str {
        match self {
            Page::Dashboard => "\u{25A3}",  // ◻
            Page::Proxies => "\u{21C4}",    // ⇄
            Page::Profiles => "\u{2630}",   // ☰
            Page::Requests => "\u{21F5}",   // ⇵
            Page::Connections => "\u{26D3}", // ⛓
            Page::Resources => "\u{25A8}",  // ▨
            Page::Logs => "\u{2261}",       // ≡
            Page::Tools => "\u{2699}",      // ⚙
        }
    }

    /// Short description for the page.
    pub fn description(&self) -> &'static str {
        match self {
            Page::Dashboard => "Overview of proxy status, traffic, and quick controls",
            Page::Proxies => "Manage proxy groups and select active proxies",
            Page::Profiles => "Manage subscription profiles and configurations",
            Page::Requests => "Monitor real-time network requests",
            Page::Connections => "View active connections and manage them",
            Page::Resources => "Manage GeoIP, GeoSite, and other resource files",
            Page::Logs => "View proxy core logs and diagnostics",
            Page::Tools => "Application settings, theme, backup, and more",
        }
    }
}
