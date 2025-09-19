#[cfg(feature = "ssr")]
pub struct ServerAppState {
    config: Option<String>, // dummy type for now, just exploring the conditional compilation patterns atm
}
