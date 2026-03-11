#[cfg(feature = "redis-cache")]
pub fn resolve_redis_url() -> Option<String> {
    std::env::var("RUSTOK_REDIS_URL")
        .ok()
        .or_else(|| std::env::var("REDIS_URL").ok())
        .filter(|url| !url.trim().is_empty())
}

#[cfg(not(feature = "redis-cache"))]
pub fn resolve_redis_url() -> Option<String> {
    None
}

#[cfg(feature = "redis-cache")]
pub fn resolve_redis_client() -> Option<redis::Client> {
    resolve_redis_url().and_then(|url| redis::Client::open(url).ok())
}

#[cfg(not(feature = "redis-cache"))]
pub fn resolve_redis_client() -> Option<redis::Client> {
    None
}
