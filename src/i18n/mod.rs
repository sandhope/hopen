//! Localised UI strings and runtime language selection.
//!
//! This module owns language packs, system-locale matching, and the global
//! manager used by components throughout the app. Visual styling remains in `theme`.
//!
//! Design follows velotype's i18n architecture:
//! - `I18nStrings` is a flat struct of named fields — no dynamic key-value maps.
//! - `I18nManager` is a GPUI global, accessed via `cx.global::<I18nManager>()`.
//! - Built-in languages: `en-US` (fallback), `zh-CN`, `ja-JP`, `ko-KR`, `de-DE`, `fr-FR`, `es-ES`, `pt-BR`.
//! - System locale is auto-detected on startup via `sys-locale`.

use std::sync::Arc;

use gpui::{App, Global};

// ─── The translatable string set ─────────────────────────────────

/// All localisable UI strings for Hopen.
///
/// Some fields are not yet read directly but are defined for future use
/// (theme label display).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct I18nStrings {
    // Dashboard cards
    pub dashboard_network_speed: &'static str,
    pub dashboard_proxy_control: &'static str,
    pub dashboard_lan_ip: &'static str,
    pub dashboard_traffic_usage: &'static str,
    pub dashboard_network_detection: &'static str,

    // Dashboard controls
    pub dashboard_upload: &'static str,
    pub dashboard_download: &'static str,
    pub dashboard_system_proxy: &'static str,
    pub dashboard_tun_mode: &'static str,
    pub dashboard_outbound_mode: &'static str,
    pub dashboard_status_on: &'static str,
    pub dashboard_status_off: &'static str,

    // Core button
    pub core_start: &'static str,
    pub core_stop: &'static str,

    // Outbound modes
    pub outbound_global: &'static str,
    pub outbound_rule: &'static str,
    pub outbound_direct: &'static str,

    // Settings page
    pub settings_language: &'static str,
    pub settings_language_subtitle: &'static str,
    pub settings_theme: &'static str,
    pub settings_theme_subtitle: &'static str,
    pub settings_basic_config: &'static str,
    pub settings_basic_config_subtitle: &'static str,
    pub settings_advanced_config: &'static str,
    pub settings_advanced_config_subtitle: &'static str,
    pub settings_hotkeys: &'static str,
    pub settings_hotkeys_subtitle: &'static str,
    pub settings_backup_restore: &'static str,
    pub settings_backup_restore_subtitle: &'static str,
    pub settings_about: &'static str,
    pub settings_about_subtitle: &'static str,

    // Placeholder pages
    pub placeholder_proxies_title: &'static str,
    pub placeholder_proxies_desc: &'static str,
    pub placeholder_profiles_title: &'static str,
    pub placeholder_profiles_desc: &'static str,
    pub placeholder_requests_title: &'static str,
    pub placeholder_requests_desc: &'static str,
    pub placeholder_connections_title: &'static str,
    pub placeholder_connections_desc: &'static str,
    pub placeholder_resources_title: &'static str,
    pub placeholder_resources_desc: &'static str,
    pub placeholder_logs_title: &'static str,
    pub placeholder_logs_desc: &'static str,

    // Network
    pub network_detecting: &'static str,
    pub network_na: &'static str,
    pub network_unknown: &'static str,
    pub network_ip_label: &'static str,
    pub network_isp_label: &'static str,

    // Page titles (header)
    pub page_title_dashboard: &'static str,
    pub page_title_proxies: &'static str,
    pub page_title_profiles: &'static str,
    pub page_title_requests: &'static str,
    pub page_title_connections: &'static str,
    pub page_title_resources: &'static str,
    pub page_title_logs: &'static str,
    pub page_title_tools: &'static str,

    // App
    pub app_name: &'static str,

    // Theme
    pub theme_dark: &'static str,
    pub theme_light: &'static str,
    pub theme_system: &'static str,
    pub theme_mode_label: &'static str,

    // Sub-page navigation
    pub nav_back: &'static str,
    pub page_title_language: &'static str,
    pub page_title_theme: &'static str,
    pub accent_color_label: &'static str,
}

impl I18nStrings {
    /// English (United States) — fallback strings.
    pub fn en_us() -> Self {
        Self {
            dashboard_network_speed: "Network Speed",
            dashboard_proxy_control: "Proxy Control",
            dashboard_lan_ip: "LAN IP",
            dashboard_traffic_usage: "Traffic Usage",
            dashboard_network_detection: "Network Detection",

            dashboard_upload: "Upload",
            dashboard_download: "Download",
            dashboard_system_proxy: "System Proxy",
            dashboard_tun_mode: "TUN Mode",
            dashboard_outbound_mode: "Outbound Mode",
            dashboard_status_on: "ON",
            dashboard_status_off: "OFF",

            core_start: "Start Core",
            core_stop: "Stop Core",

            outbound_global: "Global",
            outbound_rule: "Rule",
            outbound_direct: "Direct",

            settings_language: "Language",
            settings_language_subtitle: "Switch display language",
            settings_theme: "Theme",
            settings_theme_subtitle: "Dark / Light — tap to switch appearance",
            settings_basic_config: "Basic Config",
            settings_basic_config_subtitle: "Port, log level, mode",
            settings_advanced_config: "Advanced Config",
            settings_advanced_config_subtitle: "DNS, TUN, rules",
            settings_hotkeys: "Hotkeys",
            settings_hotkeys_subtitle: "Keyboard shortcuts",
            settings_backup_restore: "Backup & Restore",
            settings_backup_restore_subtitle: "WebDAV sync",
            settings_about: "About",
            settings_about_subtitle: "Version and license info",

            placeholder_proxies_title: "Proxy Groups",
            placeholder_proxies_desc: "Proxy groups will appear here when the core is connected.",
            placeholder_profiles_title: "Profiles",
            placeholder_profiles_desc: "Import or add subscription profiles to get started.",
            placeholder_requests_title: "Request Timeline",
            placeholder_requests_desc: "Real-time request tracking will appear here.",
            placeholder_connections_title: "Active Connections",
            placeholder_connections_desc: "Active connections will be listed here.",
            placeholder_resources_title: "Resources",
            placeholder_resources_desc: "GeoIP, GeoSite, and other resource files will be managed here.",
            placeholder_logs_title: "Core Logs",
            placeholder_logs_desc: "Logs from the proxy core will stream here.",

            network_detecting: "Detecting...",
            network_na: "N/A",
            network_unknown: "Unknown",
            network_ip_label: "IP",
            network_isp_label: "ISP",

            page_title_dashboard: "Dashboard",
            page_title_proxies: "Proxies",
            page_title_profiles: "Profiles",
            page_title_requests: "Requests",
            page_title_connections: "Connections",
            page_title_resources: "Resources",
            page_title_logs: "Logs",
            page_title_tools: "Settings",

            app_name: "Hopen",

            theme_dark: "Dark",
            theme_light: "Light",
            theme_system: "System",
            theme_mode_label: "Theme Mode",

            nav_back: "Back",
            page_title_language: "Language",
            page_title_theme: "Theme",
            accent_color_label: "Accent Color",
        }
    }

    /// 简体中文 (Simplified Chinese).
    pub fn zh_cn() -> Self {
        Self {
            dashboard_network_speed: "网络速度",
            dashboard_proxy_control: "代理控制",
            dashboard_lan_ip: "局域网 IP",
            dashboard_traffic_usage: "流量统计",
            dashboard_network_detection: "网络检测",

            dashboard_upload: "上传",
            dashboard_download: "下载",
            dashboard_system_proxy: "系统代理",
            dashboard_tun_mode: "TUN 模式",
            dashboard_outbound_mode: "出站模式",
            dashboard_status_on: "开",
            dashboard_status_off: "关",

            core_start: "启动核心",
            core_stop: "停止核心",

            outbound_global: "全局",
            outbound_rule: "规则",
            outbound_direct: "直连",

            settings_language: "语言",
            settings_language_subtitle: "切换显示语言",
            settings_theme: "主题",
            settings_theme_subtitle: "深色 / 浅色 — 点击切换外观",
            settings_basic_config: "基础配置",
            settings_basic_config_subtitle: "端口、日志级别、模式",
            settings_advanced_config: "高级配置",
            settings_advanced_config_subtitle: "DNS、TUN、规则",
            settings_hotkeys: "快捷键",
            settings_hotkeys_subtitle: "键盘快捷键设置",
            settings_backup_restore: "备份与恢复",
            settings_backup_restore_subtitle: "WebDAV 同步",
            settings_about: "关于",
            settings_about_subtitle: "版本与许可证信息",

            placeholder_proxies_title: "代理组",
            placeholder_proxies_desc: "核心连接后，代理组将显示在此处。",
            placeholder_profiles_title: "订阅配置",
            placeholder_profiles_desc: "导入或添加订阅配置以开始使用。",
            placeholder_requests_title: "请求时间线",
            placeholder_requests_desc: "实时请求追踪将在此处显示。",
            placeholder_connections_title: "活动连接",
            placeholder_connections_desc: "活动连接将在此处列出。",
            placeholder_resources_title: "资源管理",
            placeholder_resources_desc: "GeoIP、GeoSite 等资源文件将在此处管理。",
            placeholder_logs_title: "核心日志",
            placeholder_logs_desc: "代理核心的日志将在此处流式显示。",

            network_detecting: "检测中...",
            network_na: "无",
            network_unknown: "未知",
            network_ip_label: "IP",
            network_isp_label: "运营商",

            page_title_dashboard: "仪表盘",
            page_title_proxies: "代理",
            page_title_profiles: "订阅",
            page_title_requests: "请求",
            page_title_connections: "连接",
            page_title_resources: "资源",
            page_title_logs: "日志",
            page_title_tools: "设置",

            app_name: "Hopen",

            theme_dark: "深色",
            theme_light: "浅色",
            theme_system: "跟随系统",
            theme_mode_label: "主题模式",

            nav_back: "返回",
            page_title_language: "语言",
            page_title_theme: "主题",
            accent_color_label: "主题色彩",
        }
    }

    /// 日本語 (Japanese).
    pub fn ja_jp() -> Self {
        Self {
            dashboard_network_speed: "ネットワーク速度",
            dashboard_proxy_control: "プロキシ制御",
            dashboard_lan_ip: "LAN IP",
            dashboard_traffic_usage: "通信量統計",
            dashboard_network_detection: "ネットワーク検出",

            dashboard_upload: "アップロード",
            dashboard_download: "ダウンロード",
            dashboard_system_proxy: "システムプロキシ",
            dashboard_tun_mode: "TUN モード",
            dashboard_outbound_mode: "アウトバウンドモード",
            dashboard_status_on: "オン",
            dashboard_status_off: "オフ",

            core_start: "コア起動",
            core_stop: "コア停止",

            outbound_global: "グローバル",
            outbound_rule: "ルール",
            outbound_direct: "ダイレクト",

            settings_language: "言語",
            settings_language_subtitle: "表示言語を切り替え",
            settings_theme: "テーマ",
            settings_theme_subtitle: "ダーク / ライト — タップで切替",
            settings_basic_config: "基本設定",
            settings_basic_config_subtitle: "ポート、ログレベル、モード",
            settings_advanced_config: "詳細設定",
            settings_advanced_config_subtitle: "DNS、TUN、ルール",
            settings_hotkeys: "ホットキー",
            settings_hotkeys_subtitle: "キーボードショートカット",
            settings_backup_restore: "バックアップと復元",
            settings_backup_restore_subtitle: "WebDAV 同期",
            settings_about: "について",
            settings_about_subtitle: "バージョンとライセンス情報",

            placeholder_proxies_title: "プロキシグループ",
            placeholder_proxies_desc: "コア接続後、プロキシグループがここに表示されます。",
            placeholder_profiles_title: "プロファイル",
            placeholder_profiles_desc: "サブスクリプションをインポートまたは追加してください。",
            placeholder_requests_title: "リクエストタイムライン",
            placeholder_requests_desc: "リアルタイムのリクエスト追跡がここに表示されます。",
            placeholder_connections_title: "アクティブ接続",
            placeholder_connections_desc: "アクティブな接続がここに一覧表示されます。",
            placeholder_resources_title: "リソース管理",
            placeholder_resources_desc: "GeoIP、GeoSite などのリソースをここで管理します。",
            placeholder_logs_title: "コアログ",
            placeholder_logs_desc: "プロキシコアのログがここにストリーミング表示されます。",

            network_detecting: "検出中...",
            network_na: "N/A",
            network_unknown: "不明",
            network_ip_label: "IP",
            network_isp_label: "ISP",

            page_title_dashboard: "ダッシュボード",
            page_title_proxies: "プロキシ",
            page_title_profiles: "プロファイル",
            page_title_requests: "リクエスト",
            page_title_connections: "接続",
            page_title_resources: "リソース",
            page_title_logs: "ログ",
            page_title_tools: "設定",

            app_name: "Hopen",

            theme_dark: "ダーク",
            theme_light: "ライト",
            theme_system: "システムに合わせる",
            theme_mode_label: "テーマモード",

            nav_back: "戻る",
            page_title_language: "言語",
            page_title_theme: "テーマ",
            accent_color_label: "アクセントカラー",
        }
    }

    /// 한국어 (Korean).
    pub fn ko_kr() -> Self {
        Self {
            dashboard_network_speed: "네트워크 속도",
            dashboard_proxy_control: "프록시 제어",
            dashboard_lan_ip: "LAN IP",
            dashboard_traffic_usage: "트래픽 통계",
            dashboard_network_detection: "네트워크 감지",

            dashboard_upload: "업로드",
            dashboard_download: "다운로드",
            dashboard_system_proxy: "시스템 프록시",
            dashboard_tun_mode: "TUN 모드",
            dashboard_outbound_mode: "아웃바운드 모드",
            dashboard_status_on: "켜짐",
            dashboard_status_off: "꺼짐",

            core_start: "코어 시작",
            core_stop: "코어 중지",

            outbound_global: "글로벌",
            outbound_rule: "규칙",
            outbound_direct: "다이렉트",

            settings_language: "언어",
            settings_language_subtitle: "표시 언어 전환",
            settings_theme: "테마",
            settings_theme_subtitle: "다크 / 라이트 — 탭하여 전환",
            settings_basic_config: "기본 설정",
            settings_basic_config_subtitle: "포트, 로그 레벨, 모드",
            settings_advanced_config: "고급 설정",
            settings_advanced_config_subtitle: "DNS, TUN, 규칙",
            settings_hotkeys: "단축키",
            settings_hotkeys_subtitle: "키보드 단축키",
            settings_backup_restore: "백업 및 복원",
            settings_backup_restore_subtitle: "WebDAV 동기화",
            settings_about: "정보",
            settings_about_subtitle: "버전 및 라이선스 정보",

            placeholder_proxies_title: "프록시 그룹",
            placeholder_proxies_desc: "코어 연결 후 프록시 그룹이 여기에 표시됩니다.",
            placeholder_profiles_title: "프로필",
            placeholder_profiles_desc: "구독 프로필을 가져오거나 추가하여 시작하세요.",
            placeholder_requests_title: "요청 타임라인",
            placeholder_requests_desc: "실시간 요청 추적이 여기에 표시됩니다.",
            placeholder_connections_title: "활성 연결",
            placeholder_connections_desc: "활성 연결이 여기에 나열됩니다.",
            placeholder_resources_title: "리소스 관리",
            placeholder_resources_desc: "GeoIP, GeoSite 등 리소스 파일이 여기에서 관리됩니다.",
            placeholder_logs_title: "코어 로그",
            placeholder_logs_desc: "프록시 코어의 로그가 여기에 스트리밍됩니다.",

            network_detecting: "감지 중...",
            network_na: "N/A",
            network_unknown: "알 수 없음",
            network_ip_label: "IP",
            network_isp_label: "ISP",

            page_title_dashboard: "대시보드",
            page_title_proxies: "프록시",
            page_title_profiles: "프로필",
            page_title_requests: "요청",
            page_title_connections: "연결",
            page_title_resources: "리소스",
            page_title_logs: "로그",
            page_title_tools: "설정",

            app_name: "Hopen",

            theme_dark: "다크",
            theme_light: "라이트",
            theme_system: "시스템 설정",
            theme_mode_label: "테마 모드",

            nav_back: "뒤로",
            page_title_language: "언어",
            page_title_theme: "테마",
            accent_color_label: "강조 색상",
        }
    }

    /// Deutsch (German).
    pub fn de_de() -> Self {
        Self {
            dashboard_network_speed: "Netzwerkgeschwindigkeit",
            dashboard_proxy_control: "Proxy-Steuerung",
            dashboard_lan_ip: "LAN-IP",
            dashboard_traffic_usage: "Datenverbrauch",
            dashboard_network_detection: "Netzwerkerkennung",

            dashboard_upload: "Upload",
            dashboard_download: "Download",
            dashboard_system_proxy: "System-Proxy",
            dashboard_tun_mode: "TUN-Modus",
            dashboard_outbound_mode: "Ausgehender Modus",
            dashboard_status_on: "AN",
            dashboard_status_off: "AUS",

            core_start: "Core starten",
            core_stop: "Core stoppen",

            outbound_global: "Global",
            outbound_rule: "Regel",
            outbound_direct: "Direkt",

            settings_language: "Sprache",
            settings_language_subtitle: "Anzeigesprache wechseln",
            settings_theme: "Design",
            settings_theme_subtitle: "Dunkel / Hell — zum Umschalten tippen",
            settings_basic_config: "Grundeinstellungen",
            settings_basic_config_subtitle: "Port, Protokollstufe, Modus",
            settings_advanced_config: "Erweiterte Einstellungen",
            settings_advanced_config_subtitle: "DNS, TUN, Regeln",
            settings_hotkeys: "Tastenkürzel",
            settings_hotkeys_subtitle: "Tastaturkürzel",
            settings_backup_restore: "Sicherung & Wiederherstellung",
            settings_backup_restore_subtitle: "WebDAV-Sync",
            settings_about: "Über",
            settings_about_subtitle: "Version und Lizenzinformationen",

            placeholder_proxies_title: "Proxy-Gruppen",
            placeholder_proxies_desc: "Proxy-Gruppen werden hier angezeigt, sobald der Kern verbunden ist.",
            placeholder_profiles_title: "Profile",
            placeholder_profiles_desc: "Importieren oder fügen Sie Abonnementprofile hinzu.",
            placeholder_requests_title: "Anfrageverlauf",
            placeholder_requests_desc: "Echtzeit-Anfrageverfolgung wird hier angezeigt.",
            placeholder_connections_title: "Aktive Verbindungen",
            placeholder_connections_desc: "Aktive Verbindungen werden hier aufgelistet.",
            placeholder_resources_title: "Ressourcen",
            placeholder_resources_desc: "GeoIP-, GeoSite- und andere Ressourcendateien werden hier verwaltet.",
            placeholder_logs_title: "Core-Protokolle",
            placeholder_logs_desc: "Protokolle des Proxy-Kerns werden hier gestreamt.",

            network_detecting: "Erkennung...",
            network_na: "N/V",
            network_unknown: "Unbekannt",
            network_ip_label: "IP",
            network_isp_label: "ISP",

            page_title_dashboard: "Dashboard",
            page_title_proxies: "Proxys",
            page_title_profiles: "Profile",
            page_title_requests: "Anfragen",
            page_title_connections: "Verbindungen",
            page_title_resources: "Ressourcen",
            page_title_logs: "Protokolle",
            page_title_tools: "Einstellungen",

            app_name: "Hopen",

            theme_dark: "Dunkel",
            theme_light: "Hell",
            theme_system: "System",
            theme_mode_label: "Designmodus",

            nav_back: "Zurück",
            page_title_language: "Sprache",
            page_title_theme: "Design",
            accent_color_label: "Akzentfarbe",
        }
    }

    /// Français (French).
    pub fn fr_fr() -> Self {
        Self {
            dashboard_network_speed: "Vitesse réseau",
            dashboard_proxy_control: "Contrôle proxy",
            dashboard_lan_ip: "IP LAN",
            dashboard_traffic_usage: "Utilisation trafic",
            dashboard_network_detection: "Détection réseau",

            dashboard_upload: "Téléversement",
            dashboard_download: "Téléchargement",
            dashboard_system_proxy: "Proxy système",
            dashboard_tun_mode: "Mode TUN",
            dashboard_outbound_mode: "Mode sortant",
            dashboard_status_on: "ACTIF",
            dashboard_status_off: "INACTIF",

            core_start: "Démarrer le cœur",
            core_stop: "Arrêter le cœur",

            outbound_global: "Global",
            outbound_rule: "Règle",
            outbound_direct: "Direct",

            settings_language: "Langue",
            settings_language_subtitle: "Changer la langue d'affichage",
            settings_theme: "Thème",
            settings_theme_subtitle: "Sombre / Clair — appuyez pour changer",
            settings_basic_config: "Config de base",
            settings_basic_config_subtitle: "Port, niveau de journal, mode",
            settings_advanced_config: "Config avancée",
            settings_advanced_config_subtitle: "DNS, TUN, règles",
            settings_hotkeys: "Raccourcis",
            settings_hotkeys_subtitle: "Raccourcis clavier",
            settings_backup_restore: "Sauvegarde & Restauration",
            settings_backup_restore_subtitle: "Sync WebDAV",
            settings_about: "À propos",
            settings_about_subtitle: "Version et informations de licence",

            placeholder_proxies_title: "Groupes de proxy",
            placeholder_proxies_desc: "Les groupes de proxy apparaîtront une fois le cœur connecté.",
            placeholder_profiles_title: "Profils",
            placeholder_profiles_desc: "Importez ou ajoutez des profils d'abonnement pour commencer.",
            placeholder_requests_title: "Chronologie des requêtes",
            placeholder_requests_desc: "Le suivi des requêtes en temps réel apparaîtra ici.",
            placeholder_connections_title: "Connexions actives",
            placeholder_connections_desc: "Les connexions actives seront listées ici.",
            placeholder_resources_title: "Ressources",
            placeholder_resources_desc: "Les fichiers GeoIP, GeoSite et autres ressources seront gérés ici.",
            placeholder_logs_title: "Journaux du cœur",
            placeholder_logs_desc: "Les journaux du cœur proxy seront diffusés ici.",

            network_detecting: "Détection...",
            network_na: "N/D",
            network_unknown: "Inconnu",
            network_ip_label: "IP",
            network_isp_label: "FAI",

            page_title_dashboard: "Tableau de bord",
            page_title_proxies: "Proxys",
            page_title_profiles: "Profils",
            page_title_requests: "Requêtes",
            page_title_connections: "Connexions",
            page_title_resources: "Ressources",
            page_title_logs: "Journaux",
            page_title_tools: "Paramètres",

            app_name: "Hopen",

            theme_dark: "Sombre",
            theme_light: "Clair",
            theme_system: "Système",
            theme_mode_label: "Mode de thème",

            nav_back: "Retour",
            page_title_language: "Langue",
            page_title_theme: "Thème",
            accent_color_label: "Couleur d'accent",
        }
    }

    /// Español (Spanish).
    pub fn es_es() -> Self {
        Self {
            dashboard_network_speed: "Velocidad de red",
            dashboard_proxy_control: "Control de proxy",
            dashboard_lan_ip: "IP LAN",
            dashboard_traffic_usage: "Uso de tráfico",
            dashboard_network_detection: "Detección de red",

            dashboard_upload: "Subida",
            dashboard_download: "Descarga",
            dashboard_system_proxy: "Proxy del sistema",
            dashboard_tun_mode: "Modo TUN",
            dashboard_outbound_mode: "Modo de salida",
            dashboard_status_on: "ACTIVADO",
            dashboard_status_off: "DESACTIVADO",

            core_start: "Iniciar núcleo",
            core_stop: "Detener núcleo",

            outbound_global: "Global",
            outbound_rule: "Regla",
            outbound_direct: "Directo",

            settings_language: "Idioma",
            settings_language_subtitle: "Cambiar idioma de visualización",
            settings_theme: "Tema",
            settings_theme_subtitle: "Oscuro / Claro — toque para cambiar",
            settings_basic_config: "Config básica",
            settings_basic_config_subtitle: "Puerto, nivel de registro, modo",
            settings_advanced_config: "Config avanzada",
            settings_advanced_config_subtitle: "DNS, TUN, reglas",
            settings_hotkeys: "Atajos",
            settings_hotkeys_subtitle: "Atajos de teclado",
            settings_backup_restore: "Copia y restauración",
            settings_backup_restore_subtitle: "Sincronización WebDAV",
            settings_about: "Acerca de",
            settings_about_subtitle: "Versión e información de licencia",

            placeholder_proxies_title: "Grupos de proxy",
            placeholder_proxies_desc: "Los grupos de proxy aparecerán cuando el núcleo esté conectado.",
            placeholder_profiles_title: "Perfiles",
            placeholder_profiles_desc: "Importe o agregue perfiles de suscripción para comenzar.",
            placeholder_requests_title: "Línea de solicitudes",
            placeholder_requests_desc: "El seguimiento de solicitudes en tiempo real aparecerá aquí.",
            placeholder_connections_title: "Conexiones activas",
            placeholder_connections_desc: "Las conexiones activas se listarán aquí.",
            placeholder_resources_title: "Recursos",
            placeholder_resources_desc: "Archivos GeoIP, GeoSite y otros recursos se gestionarán aquí.",
            placeholder_logs_title: "Registros del núcleo",
            placeholder_logs_desc: "Los registros del núcleo proxy se transmitirán aquí.",

            network_detecting: "Detectando...",
            network_na: "N/D",
            network_unknown: "Desconocido",
            network_ip_label: "IP",
            network_isp_label: "ISP",

            page_title_dashboard: "Inicio",
            page_title_proxies: "Proxys",
            page_title_profiles: "Perfiles",
            page_title_requests: "Solicitudes",
            page_title_connections: "Conexiones",
            page_title_resources: "Recursos",
            page_title_logs: "Registros",
            page_title_tools: "Ajustes",

            app_name: "Hopen",

            theme_dark: "Oscuro",
            theme_light: "Claro",
            theme_system: "Sistema",
            theme_mode_label: "Modo de tema",

            nav_back: "Volver",
            page_title_language: "Idioma",
            page_title_theme: "Tema",
            accent_color_label: "Color de acento",
        }
    }

    /// Português (Portuguese — Brazil).
    pub fn pt_br() -> Self {
        Self {
            dashboard_network_speed: "Velocidade da rede",
            dashboard_proxy_control: "Controle de proxy",
            dashboard_lan_ip: "IP LAN",
            dashboard_traffic_usage: "Uso de tráfego",
            dashboard_network_detection: "Detecção de rede",

            dashboard_upload: "Upload",
            dashboard_download: "Download",
            dashboard_system_proxy: "Proxy do sistema",
            dashboard_tun_mode: "Modo TUN",
            dashboard_outbound_mode: "Modo de saída",
            dashboard_status_on: "LIGADO",
            dashboard_status_off: "DESLIGADO",

            core_start: "Iniciar núcleo",
            core_stop: "Parar núcleo",

            outbound_global: "Global",
            outbound_rule: "Regra",
            outbound_direct: "Direto",

            settings_language: "Idioma",
            settings_language_subtitle: "Alterar idioma de exibição",
            settings_theme: "Tema",
            settings_theme_subtitle: "Escuro / Claro — toque para alternar",
            settings_basic_config: "Config básica",
            settings_basic_config_subtitle: "Porta, nível de log, modo",
            settings_advanced_config: "Config avançada",
            settings_advanced_config_subtitle: "DNS, TUN, regras",
            settings_hotkeys: "Atalhos",
            settings_hotkeys_subtitle: "Atalhos de teclado",
            settings_backup_restore: "Backup e restauração",
            settings_backup_restore_subtitle: "Sincronização WebDAV",
            settings_about: "Sobre",
            settings_about_subtitle: "Versão e informações de licença",

            placeholder_proxies_title: "Grupos de proxy",
            placeholder_proxies_desc: "Os grupos de proxy aparecerão quando o núcleo estiver conectado.",
            placeholder_profiles_title: "Perfis",
            placeholder_profiles_desc: "Importe ou adicione perfis de assinatura para começar.",
            placeholder_requests_title: "Linha do tempo de requisições",
            placeholder_requests_desc: "O rastreamento de requisições em tempo real aparecerá aqui.",
            placeholder_connections_title: "Conexões ativas",
            placeholder_connections_desc: "As conexões ativas serão listadas aqui.",
            placeholder_resources_title: "Recursos",
            placeholder_resources_desc: "Arquivos GeoIP, GeoSite e outros recursos serão gerenciados aqui.",
            placeholder_logs_title: "Logs do núcleo",
            placeholder_logs_desc: "Os logs do núcleo proxy serão transmitidos aqui.",

            network_detecting: "Detectando...",
            network_na: "N/D",
            network_unknown: "Desconhecido",
            network_ip_label: "IP",
            network_isp_label: "ISP",

            page_title_dashboard: "Painel",
            page_title_proxies: "Proxies",
            page_title_profiles: "Perfis",
            page_title_requests: "Requisições",
            page_title_connections: "Conexões",
            page_title_resources: "Recursos",
            page_title_logs: "Logs",
            page_title_tools: "Configurações",

            app_name: "Hopen",

            theme_dark: "Escuro",
            theme_light: "Claro",
            theme_system: "Sistema",
            theme_mode_label: "Modo de tema",

            nav_back: "Voltar",
            page_title_language: "Idioma",
            page_title_theme: "Tema",
            accent_color_label: "Cor de destaque",
        }
    }

    /// Resolve the title for a given page.
    pub fn page_title(&self, page: crate::navigation::Page) -> &'static str {
        use crate::navigation::Page;
        match page {
            Page::Dashboard => self.page_title_dashboard,
            Page::Proxies => self.page_title_proxies,
            Page::Profiles => self.page_title_profiles,
            Page::Requests => self.page_title_requests,
            Page::Connections => self.page_title_connections,
            Page::Resources => self.page_title_resources,
            Page::Logs => self.page_title_logs,
            Page::Tools => self.page_title_tools,
        }
    }

    /// Resolve the label for an outbound mode.
    pub fn outbound_mode_label(&self, mode: crate::OutboundMode) -> &'static str {
        use crate::OutboundMode;
        match mode {
            OutboundMode::Global => self.outbound_global,
            OutboundMode::Rule => self.outbound_rule,
            OutboundMode::Direct => self.outbound_direct,
        }
    }
}

// ─── I18n Manager (GPUI Global) ──────────────────────────────────

/// Runtime language manager, stored as a GPUI global.
///
/// `current_language_id` will be used by runtime language switching.
pub struct I18nManager {
    /// Currently active language id, e.g. "en-US" or "zh-CN".
    pub current_language_id: String,
    /// All translated strings, wrapped in Arc for cheap sharing.
    strings: Arc<I18nStrings>,
}

impl Global for I18nManager {}

impl I18nManager {
    /// Initialize the i18n system with the given BCP 47 language id.
    ///
    /// Falls back to `en-US` if the id is not recognized.
    pub fn init_with_language_id(cx: &mut App, language_id: &str) {
        // Gracefully remove any trailing newline / whitespace from the user-config file.
        let id = language_id.trim();
        let strings = I18nStrings::from_language_id(id);
        cx.set_global(Self {
            current_language_id: id.to_owned(),
            strings: Arc::new(strings),
        });
    }

    /// Read current translated strings by reference.
    #[allow(dead_code)]
    pub fn strings(&self) -> &I18nStrings {
        &self.strings
    }

    /// Clone the `Arc` of translated strings — one atomic ref-count bump.
    ///
    /// Prefer this over `.strings().clone()` in hot rendering paths.
    pub fn strings_arc(&self) -> Arc<I18nStrings> {
        Arc::clone(&self.strings)
    }
}

// ─── Helpers ─────────────────────────────────────────────────────

impl I18nStrings {
    /// Build the string set for a known language id; falls back to `en-US`.
    fn from_language_id(id: &str) -> Self {
        match id {
            "zh-CN" | "zh-Hans" | "zh" => Self::zh_cn(),
            "ja-JP" | "ja" => Self::ja_jp(),
            "ko-KR" | "ko" => Self::ko_kr(),
            "de-DE" | "de" => Self::de_de(),
            "fr-FR" | "fr" => Self::fr_fr(),
            "es-ES" | "es" => Self::es_es(),
            "pt-BR" | "pt-PT" | "pt" => Self::pt_br(),
            _ => Self::en_us(),
        }
    }
}

/// Try to pick a BCP 47 language id from the system locale.
///
/// Uses `sys-locale` to query the OS preference, then maps the locale
/// to one of the supported language ids.
pub fn detect_system_language_id() -> String {
    let locale = sys_locale::get_locale().unwrap_or_else(|| String::from("en-US"));
    system_locale_to_language_id(&locale)
}

/// Map an OS locale string (e.g. "zh-CN", "zh-Hans-CN", "en-US") to a
/// supported language id. Unrecognised locales fall back to `en-US`.
fn system_locale_to_language_id(locale: &str) -> String {
    let normalized = locale.to_lowercase();

    // Simplified Chinese
    if normalized.starts_with("zh-cn")
        || normalized.starts_with("zh-hans")
        || normalized.starts_with("zh-sg")
        || normalized.starts_with("zh-hans-cn")
    {
        return String::from("zh-CN");
    }

    // Generic Chinese (zh, zh-TW -> fall back to zh-CN for now)
    if normalized.starts_with("zh") {
        return String::from("zh-CN");
    }

    // Japanese
    if normalized.starts_with("ja") {
        return String::from("ja-JP");
    }

    // Korean
    if normalized.starts_with("ko") {
        return String::from("ko-KR");
    }

    // German
    if normalized.starts_with("de") {
        return String::from("de-DE");
    }

    // French
    if normalized.starts_with("fr") {
        return String::from("fr-FR");
    }

    // Spanish
    if normalized.starts_with("es") {
        return String::from("es-ES");
    }

    // Portuguese
    if normalized.starts_with("pt") {
        return String::from("pt-BR");
    }

    // All others → English.
    String::from("en-US")
}

// ─── Language Display Name ──────────────────────────────────────

/// Returns the display name of a language in its own script.
/// e.g. "zh-CN" → "简体中文", "en-US" → "English".
pub fn language_display_name(language_id: &str) -> &'static str {
    match language_id {
        "zh-CN" | "zh-Hans" | "zh" => "简体中文",
        "ja-JP" | "ja" => "日本語",
        "ko-KR" | "ko" => "한국어",
        "de-DE" | "de" => "Deutsch",
        "fr-FR" | "fr" => "Français",
        "es-ES" | "es" => "Español",
        "pt-BR" | "pt-PT" | "pt" => "Português",
        _ => "English",
    }
}

// ─── Convenience accessor ────────────────────────────────────────

/// Convenience: retrieve the current `I18nStrings` from GPUI global state.
///
/// Usable from any `Context<T>` or `&App`.
#[allow(dead_code)]
pub fn strings(cx: &App) -> &I18nStrings {
    cx.global::<I18nManager>().strings()
}
