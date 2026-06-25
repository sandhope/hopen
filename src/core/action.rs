/// Action protocol types — mirrors FlClash `lib/models/core.dart` and `lib/enum/enum.dart`.
///
/// All communication with the Go core uses JSON-encoded Action/ActionResult messages
/// carried over the binary frame transport.
use serde::{Deserialize, Serialize};

// ── ActionMethod ────────────────────────────────────────────────

/// Every RPC method supported by the Go clash.meta core.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ActionMethod {
    Message,
    InitClash,
    GetIsInit,
    ForceGc,
    Shutdown,
    ValidateConfig,
    UpdateConfig,
    GetConfig,
    GetProxies,
    ChangeProxy,
    GetTraffic,
    GetTotalTraffic,
    ResetTraffic,
    AsyncTestDelay,
    GetConnections,
    CloseConnections,
    ResetConnections,
    CloseConnection,
    GetExternalProviders,
    GetExternalProvider,
    UpdateGeoData,
    UpdateExternalProvider,
    SideLoadExternalProvider,
    StartLog,
    StopLog,
    StartListener,
    StopListener,
    GetCountryCode,
    GetMemory,
    Crash,
    SetupConfig,
    DeleteFile,
    /// Android-only
    SetState,
    StartTun,
    StopTun,
    GetRunTime,
    UpdateDns,
    GetAndroidVpnOptions,
    GetCurrentProfileName,
}

impl std::fmt::Display for ActionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ActionMethod::Message => "message",
            ActionMethod::InitClash => "initClash",
            ActionMethod::GetIsInit => "getIsInit",
            ActionMethod::ForceGc => "forceGc",
            ActionMethod::Shutdown => "shutdown",
            ActionMethod::ValidateConfig => "validateConfig",
            ActionMethod::UpdateConfig => "updateConfig",
            ActionMethod::GetConfig => "getConfig",
            ActionMethod::GetProxies => "getProxies",
            ActionMethod::ChangeProxy => "changeProxy",
            ActionMethod::GetTraffic => "getTraffic",
            ActionMethod::GetTotalTraffic => "getTotalTraffic",
            ActionMethod::ResetTraffic => "resetTraffic",
            ActionMethod::AsyncTestDelay => "asyncTestDelay",
            ActionMethod::GetConnections => "getConnections",
            ActionMethod::CloseConnections => "closeConnections",
            ActionMethod::ResetConnections => "resetConnections",
            ActionMethod::CloseConnection => "closeConnection",
            ActionMethod::GetExternalProviders => "getExternalProviders",
            ActionMethod::GetExternalProvider => "getExternalProvider",
            ActionMethod::UpdateGeoData => "updateGeoData",
            ActionMethod::UpdateExternalProvider => "updateExternalProvider",
            ActionMethod::SideLoadExternalProvider => "sideLoadExternalProvider",
            ActionMethod::StartLog => "startLog",
            ActionMethod::StopLog => "stopLog",
            ActionMethod::StartListener => "startListener",
            ActionMethod::StopListener => "stopListener",
            ActionMethod::GetCountryCode => "getCountryCode",
            ActionMethod::GetMemory => "getMemory",
            ActionMethod::Crash => "crash",
            ActionMethod::SetupConfig => "setupConfig",
            ActionMethod::DeleteFile => "deleteFile",
            ActionMethod::SetState => "setState",
            ActionMethod::StartTun => "startTun",
            ActionMethod::StopTun => "stopTun",
            ActionMethod::GetRunTime => "getRunTime",
            ActionMethod::UpdateDns => "updateDns",
            ActionMethod::GetAndroidVpnOptions => "getAndroidVpnOptions",
            ActionMethod::GetCurrentProfileName => "getCurrentProfileName",
        };
        write!(f, "{s}")
    }
}

// ── Action (request) ─────────────────────────────────────────────

/// An RPC request sent to the Go core.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    /// Unique request id — used to match the response.
    pub id: String,
    /// Which RPC method to call.
    pub method: ActionMethod,
    /// Method-specific payload (JSON value).
    pub data: serde_json::Value,
}

impl Action {
    pub fn new(method: ActionMethod, data: serde_json::Value) -> Self {
        let id = format!("{}-{}", method, uuid::Uuid::new_v4());
        Self { id, method, data }
    }
}

// ── ActionResult (response) ──────────────────────────────────────

/// An RPC response returned by the Go core.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    /// The method that produced this result.
    pub method: ActionMethod,
    /// Method-specific payload.
    pub data: serde_json::Value,
    /// Matches the request `id`, or is empty for unsolicited events.
    #[serde(default)]
    pub id: String,
    /// Success (0) or error (-1).
    #[serde(default = "default_code")]
    pub code: i32,
}

fn default_code() -> i32 {
    0
}

impl ActionResult {
    /// True when the core signalled success.
    pub fn is_ok(&self) -> bool {
        self.code == 0
    }

    /// True when this is an unsolicited event (no request id).
    pub fn is_event(&self) -> bool {
        self.id.is_empty()
    }
}

// ── Core Event Types ─────────────────────────────────────────────

/// High-level event kind emitted by the Go core.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreEventType {
    Log,
    Delay,
    Request,
    Loaded,
    Crash,
}

/// The raw core event carried inside an `ActionResult` when `id` is empty.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreEvent {
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(default)]
    pub data: serde_json::Value,
}

// ── Init Params ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitParams {
    #[serde(rename = "home-dir")]
    pub home_dir: String,
    pub version: i32,
}

// ── Change Proxy Params ──────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeProxyParams {
    #[serde(rename = "group-name")]
    pub group_name: String,
    #[serde(rename = "proxy-name")]
    pub proxy_name: String,
}

// ── Helper: JSON-extract common payload shapes ───────────────────

/// Extracts `ActionResult.data` as the requested type.
pub fn parse_result_data<T: serde::de::DeserializeOwned>(r: &ActionResult) -> Option<T> {
    serde_json::from_value(r.data.clone()).ok()
}
