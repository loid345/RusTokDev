# Phase 3: Admin Auth & User Security (Standard Flow, No SSO)

This document outlines the Phase 3 scope for **standard** multi-language admin authentication
and user security flows. It intentionally excludes SSO/OIDC/SAML to keep the first iteration
simple and production-ready.

## Goals

- Ship a production-grade login/register experience in the admin panel.
- Provide user profile management with password change and session management.
- Ensure full RU/EN localization coverage for UI and email templates.
- Keep flows consistent with multi-tenant access patterns.

## In Scope (MVP)

### 1) Authentication

- **Login page**
  - Tenant slug + email + password.
  - Clear error messages for invalid credentials and missing fields.
  - Remember language choice (persisted client-side).
- **Password reset**
  - Request reset email.
  - Reset link with token + new password.
  - Token expiration handling.
- **Email verification**
  - Verify email after registration (or optional soft-verify for internal users).
  - Resend verification email action.

### 2) Registration

- **Sign-up form**
  - Email + password + tenant slug.
  - Optional name field.
  - Password strength hints.
- **Invite-based onboarding**
  - Accept invitation links with role pre-selected.
  - Expired invitation handling.

### 3) User Profile & Security

- **Profile page**
  - Update name, avatar, timezone, preferred language.
- **Change password**
  - Requires current password.
  - Show password policy hints.
- **Active sessions**
  - List recent sessions (device, IP, last active).
  - “Sign out all” action.
- **Login history**
  - Successful/failed logins with timestamps and IPs.

## Localization (RU/EN)

- All auth/profile UI strings are localized.
- Email templates are localized: verify email, reset password, invite.
- Locale selection persists across sessions.

## Data & Audit

- Track audit events for:
  - Logins (success/failure).
  - Password changes.
  - Session invalidations.
  - Email verification changes.

## UX Notes

- Keep forms minimal and mobile-friendly.
- Use inline validation with precise messages.
- Use clear empty states for sessions/log history.

## Out of Scope (Phase 3)

- SSO (OIDC/SAML).
- Passwordless magic links.
- 2FA / TOTP (planned for future phase).
