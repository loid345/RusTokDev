# Security Audit Guide

Полное руководство по безопасности находится в [`docs/standards/security.md`](../standards/security.md).

## Краткое резюме

В текущем репозитории security audit покрывается двумя уровнями:

1. Платформенные runtime-механизмы безопасности в коде приложения (RBAC, JWT, rate limiting, secure headers, tenant isolation).
2. Отдельный модуль `rustok_core::security` и тесты `crates/rustok-core/tests/security_audit_test.rs`, которые проверяют OWASP-oriented controls: headers, rate limiting, input validation, SSRF protection и audit logging.

Актуальная карта защит по коду и тестам:

| # | Угроза | Механизм защиты |
|---|--------|----------------|
| A01 | Broken Access Control | RBAC enforcement и permission checks |
| A02 | Cryptographic Failures | HTTPS enforcement flag, secure headers, Argon2 password hashing |
| A03 | Injection | SeaORM parameterized queries, `InputValidator` tests против SQLi/XSS/path traversal |
| A04 | Insecure Design | Secure defaults и typed security config |
| A05 | Security Misconfiguration | `SecurityHeaders`, middleware security headers |
| A06 | Vulnerable Components | плановые `cargo deny` / audit проверки (не подтверждены в этой среде) |
| A07 | Auth Failures | rate limiting, JWT, session controls |
| A08 | Data Integrity | input validation и audit framework |
| A09 | Logging Failures | `AuditLogger` / `SecurityAudit` abstractions |
| A10 | SSRF | `SsrfProtection` и allowlist-oriented validation tests |

## Ключевые инварианты

- Каждый запрос к БД **обязан** содержать фильтр по `tenant_id`.
- Security audit tests в `crates/rustok-core/tests/security_audit_test.rs` покрывают: security headers, rate limiting, SQL injection, XSS, path traversal и SSRF-related validation.
- Текущий security guide должен опираться на реальные модули `rustok_core::security::{headers, rate_limit, validation, audit}`.

## Полная документация

→ [`docs/standards/security.md`](../standards/security.md)
