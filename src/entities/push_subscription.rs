pub struct PushSubscription {
    pub id: String,
    pub endpoint: String,
    pub server_key: String,
    pub alerts: Alerts,
}

pub struct Alerts {
    pub follow: bool,
    pub favourite: bool,
    pub mention: bool,
    pub reblog: bool,
    pub poll: bool,
}
