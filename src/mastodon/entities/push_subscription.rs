pub struct PushSubscription {
    id: String,
    endpoint: String,
    server_key: String,
    alerts: Alerts,
}

pub struct Alerts {
    follow: bool,
    favourite: bool,
    mention: bool,
    reblog: bool,
    poll: bool,
}
