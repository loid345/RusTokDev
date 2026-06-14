use std::time::Duration;

#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// Максимум операций на один запуск
    pub max_operations: u64,

    /// Таймаут выполнения
    pub timeout: Duration,

    /// Максимум глубины вызова функций
    pub max_call_depth: usize,

    /// Максимум размера строки (bytes)
    pub max_string_size: usize,

    /// Максимум размера массива
    pub max_array_size: usize,

    /// Максимум размера object map. Имя сохранено для совместимости с ранним contract.
    pub max_map_depth: usize,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            max_operations: 50_000,
            timeout: Duration::from_millis(100),
            max_call_depth: 16,
            max_string_size: 64 * 1024,
            max_array_size: 10_000,
            max_map_depth: 16,
        }
    }
}

impl EngineConfig {
    pub fn relaxed() -> Self {
        Self {
            max_operations: 500_000,
            timeout: Duration::from_secs(5),
            ..Default::default()
        }
    }

    pub fn strict() -> Self {
        Self {
            max_operations: 10_000,
            timeout: Duration::from_millis(50),
            max_call_depth: 8,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EngineLimits {
    pub max_operations: u64,
    pub timeout_ms: u64,
    pub max_call_depth: usize,
    pub max_string_size: usize,
    pub max_array_size: usize,
    pub max_map_size: usize,
}

impl EngineConfig {
    pub fn limits(&self) -> EngineLimits {
        EngineLimits {
            max_operations: self.max_operations,
            timeout_ms: self.timeout.as_millis().try_into().unwrap_or(u64::MAX),
            max_call_depth: self.max_call_depth,
            max_string_size: self.max_string_size,
            max_array_size: self.max_array_size,
            max_map_size: self.max_map_depth,
        }
    }
}
