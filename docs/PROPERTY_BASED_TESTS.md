# Property-Based Tests Implementation

**Task:** Sprint 4, Task 4.2 - Property-Based Tests  
**Status:** ✅ Complete  
**Date:** 2026-02-16

---

## Overview

This document describes the property-based tests (PBT) implementation for the RusToK platform. Property-based testing is a testing approach where properties (invariants) about the system are verified across hundreds or thousands of randomly generated test cases, helping catch edge cases that example-based tests might miss.

## What Was Implemented

### 1. Core Validator Property Tests

**File:** `crates/rustok-core/src/validation_proptest.rs`

Implemented comprehensive property-based tests for three main categories:

#### A. Tenant Validation Tests

Tests verify the following properties:

- **Slug Pattern Validity**: Valid slugs match pattern `[a-z0-9][a-z0-9-]{0,62}`
- **Case Normalization**: All valid slugs are normalized to lowercase
- **Length Boundaries**: Maximum 64 characters enforced
- **Hyphen Rules**: Cannot start or end with hyphen
- **Reserved Words**: System keywords are rejected
- **UUID Validation**: Valid UUIDs accepted and normalized
- **Hostname Validation**: Valid hostnames accepted, invalid patterns rejected
- **Auto-Detection**: Correctly identifies slug/UUID/hostname formats

Security properties tested:
- SQL injection patterns rejected
- XSS patterns rejected
- Path traversal patterns rejected
- Whitespace trimming works correctly

#### B. Event Validation Tests

Tests verify the following properties:

- **String Fields**: Empty fields rejected, whitespace-only fields rejected
- **Max Length**: Fields exceeding maximum length rejected
- **UUID Fields**: Nil UUIDs rejected, valid UUIDs accepted
- **Optional UUIDs**: None values accepted, nil values rejected when present
- **Range Validation**: Values within bounds accepted, out of bounds rejected
- **Boundary Cases**: Exact boundary values accepted, values just outside rejected

Event-specific tests:
- Event type length validation (max 64 characters)
- Empty event types rejected
- Node ID nil rejection

#### C. Event Serialization Tests

Tests verify the following properties:

- **Roundtrip Preservation**: Events serialize to JSON and deserialize back identically
- **Valid JSON Output**: All events produce valid JSON
- **Structure Consistency**: JSON structure has required fields (type, data, etc.)
- **Multiple Event Types**: Different event types serialize correctly
- **UUID Format Preservation**: UUIDs serialized as strings

### 2. Test Coverage

| Component | Tests Added | Lines of Code |
|-----------|-------------|---------------|
| Tenant Validation | 25+ properties | ~400 lines |
| Event Validation | 20+ properties | ~300 lines |
| Event Serialization | 10+ properties | ~200 lines |
| **Total** | **55+ properties** | **~900 lines** |

### 3. Dependencies Added

**File:** `crates/rustok-core/Cargo.toml`

Added `proptest = "1.5"` to dev-dependencies to support property-based testing.

## Property-Based Testing Benefits

### Why Property-Based Tests?

1. **Edge Case Discovery**: Randomly generated inputs find edge cases developers might miss
2. **Confidence**: Testing thousands of inputs provides higher confidence than manual test cases
3. **Specification as Tests**: Properties serve as executable specifications
4. **Refactoring Safety**: Properties catch regressions when code changes
5. **Documentation**: Properties document the intended behavior of the system

### Comparison: Example-Based vs Property-Based

| Aspect | Example-Based Tests | Property-Based Tests |
|--------|---------------------|---------------------|
| Test Cases | Hand-written specific examples | Randomly generated (100s-1000s) |
| Coverage | Limited to what you think of | Explores input space systematically |
| Edge Cases | Easy to miss | Automatically found |
| Maintenance | Add new cases for new bugs | Often catches bugs before they occur |
| Readability | Shows concrete examples | Shows invariants/rules |

## Running the Tests

### Run All Property-Based Tests

```bash
cargo test -p rustok-core -- proptest
```

### Run Specific Test Module

```bash
# Tenant validation tests
cargo test -p rustok-core validation_proptest::tenant_validation_tests

# Event validation tests
cargo test -p rustok-core validation_proptest::event_validation_tests

# Event serialization tests
cargo test -p rustok-core validation_proptest::event_serialization_tests
```

### Run with More Test Cases

By default, proptest runs 256 test cases per property. You can increase this:

```bash
PROPTEST_CASES=1000 cargo test -p rustok-core -- proptest
```

### Run Specific Property

```bash
cargo test -p rustok-core valid_slug_pattern
```

## Test Properties Explained

### Tenant Validation Properties

#### Valid Slug Pattern

```rust
proptest! {
    #[test]
    fn valid_slug_pattern(s in "[a-z0-9]([a-z0-9-]{0,62})?") {
        let result = TenantIdentifierValidator::validate_slug(&s);
        if !RESERVED_SLUGS.contains(&s.as_str()) {
            prop_assert!(result.is_ok());
        }
    }
}
```

**Property:** Any string matching the regex pattern `[a-z0-9][a-z0-9-]{0,62}` (except reserved words) should validate successfully.

#### SQL Injection Rejection

```rust
proptest! {
    #[test]
    fn sql_injection_patterns_rejected(s in "('[^']*'|;[^;]*|--[^\\n]*|\\/\\*.*\\*\\/)[a-z0-9-]*") {
        let result = TenantIdentifierValidator::validate_slug(&s);
        prop_assert!(result.is_err());
    }
}
```

**Property:** Strings containing SQL injection patterns (quotes, semicolons, comments) should be rejected.

### Event Validation Properties

#### Not Empty Validation

```rust
proptest! {
    #[test]
    fn validate_not_empty_accepts_non_empty(s in "[a-zA-Z0-9]{1,100}") {
        let result = validators::validate_not_empty("field", &s);
        prop_assert!(result.is_ok());
    }
}
```

**Property:** Any non-empty string should pass the "not empty" validation.

#### Max Length Validation

```rust
proptest! {
    #[test]
    fn validate_max_length_boundary_case(max in 10usize..100usize) {
        let exact = "a".repeat(max);
        let result = validators::validate_max_length("field", &exact, max);
        prop_assert!(result.is_ok());

        let over = "a".repeat(max + 1);
        let result_over = validators::validate_max_length("field", &over, max);
        prop_assert!(result_over.is_err());
    }
}
```

**Property:** A string of exact max length should pass, but a string one character longer should fail.

### Event Serialization Properties

#### Roundtrip Preservation

```rust
proptest! {
    #[test]
    fn event_serialization_roundtrip(kind in "[a-z]{1,50}") {
        let original = DomainEvent::NodeCreated {
            node_id: Uuid::new_v4(),
            kind: kind.clone(),
            author_id: Some(Uuid::new_v4()),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: DomainEvent = serde_json::from_str(&json).unwrap();

        prop_assert_eq!(original, deserialized);
    }
}
```

**Property:** Serializing an event to JSON and deserializing it back should produce an identical event.

## Integration with Existing Tests

### Existing Property-Based Tests

Property-based tests were already implemented for:
- `rustok-commerce` state machine (`crates/rustok-commerce/src/state_machine_proptest.rs`)
- `rustok-content` state machine (`crates/rustok-content/src/state_machine_proptest.rs`)

These tests verify:
- ID preservation across state transitions
- Tenant/customer isolation
- Monetary value preservation
- State transition validity

### New Tests

The newly added tests complement the existing state machine tests by focusing on:
- Input validation (tenant identifiers, event fields)
- Security properties (injection attacks)
- Serialization correctness

## Code Quality Impact

### Test Coverage Improvement

Before this implementation:
- State machines: Property-based tests ✅
- Validators: Only example-based tests ❌
- Event serialization: No property tests ❌

After this implementation:
- State machines: Property-based tests ✅
- Validators: Property-based tests ✅ (NEW)
- Event serialization: Property-based tests ✅ (NEW)

### Reliability Improvements

1. **Validator Robustness**: Tenant and event validators are now tested against thousands of inputs
2. **Security Confidence**: Injection attack patterns systematically tested
3. **Serialization Safety**: Event roundtrip verified across random inputs
4. **Edge Case Coverage**: Boundary conditions thoroughly tested

## Future Enhancements

### Potential Additions

1. **More Event Types**: Add property tests for all domain event variants
2. **Cross-Module Tests**: Test event flow between modules
3. **Performance Properties**: Add property tests for performance invariants
4. **Concurrency Properties**: Add tests using `proptest` with async operations

### Suggested Properties

```rust
// Example: Event ordering
proptest! {
    #[test]
    fn events_preserve_causality_order(events in vec(event_strategy(), 1..100)) {
        // Events should maintain causality order
    }
}

// Example: Cache consistency
proptest! {
    #[test]
    fn cache_get_returns_what_was_set(key in key_strategy(), value in value_strategy()) {
        // Cache get should return the same value that was set
    }
}
```

## Best Practices for Adding New Property Tests

1. **Start Simple**: Begin with basic properties, add complexity gradually
2. **Use Appropriate Strategies**: Choose or create strategies that match your data
3. **Test Invariants Not Implementation**: Focus on what should be true, not how it's done
4. **Provide Seed on Failure**: Proptest gives you a seed on failure - use it to debug
5. **Keep Tests Fast**: Avoid expensive operations in property tests
6. **Document Properties**: Add comments explaining what each property tests

## Troubleshooting

### Common Issues

**Issue: Test fails inconsistently**

```bash
# Re-run with the failing seed
cargo test -- --exact test_name --nocapture --test-threads=1
```

**Issue: Test is too slow**

```rust
// Reduce test cases
proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]
    
    #[test]
    fn my_test(s in some_strategy()) {
        // ...
    }
}
```

**Issue: Strategy generates invalid inputs**

```rust
// Use prop_assume to filter
proptest! {
    #[test]
    fn my_test(s in any::<String>()) {
        prop_assume!(!s.is_empty()); // Skip empty strings
        prop_assume!(s.len() <= 100); // Skip too long strings
        // Test logic here
    }
}
```

## References

- [proptest crate documentation](https://docs.rs/proptest/)
- [Property-Based Testing in Rust](https://blog.yossarian.net/2019/07/17/Property-based-testing-in-Rust)
- [Hypothesis (Python PBT library) - useful concepts](https://hypothesis.readthedocs.io/)

## Summary

✅ **Successfully implemented** property-based tests for:
- Tenant validation (25+ properties)
- Event validation (20+ properties)
- Event serialization (10+ properties)

**Key Achievements:**
- 55+ property tests covering critical validation logic
- Security properties tested (SQL injection, XSS, path traversal)
- Serialization correctness verified
- Edge cases systematically tested

**Impact:**
- Increased confidence in validation logic
- Better protection against injection attacks
- Improved test coverage for critical components
- Foundation for future property-based testing

**Status:** ✅ **Ready for Production Use**
