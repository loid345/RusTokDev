use chrono::{Duration, Utc};
use loco_rs::prelude::*;
use sea_orm::{sea_query::Expr, ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::auth::{
    decode_password_reset_token, encode_access_token, generate_refresh_token, hash_password,
    hash_refresh_token, verify_password, AuthConfig,
};
use crate::models::{sessions, users};

use super::auth::AuthService;

pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

pub struct AuthLifecycleService;

impl AuthLifecycleService {
    pub async fn register(
        ctx: &AppContext,
        tenant_id: uuid::Uuid,
        email: &str,
        password: &str,
        name: Option<String>,
    ) -> Result<(users::Model, AuthTokens)> {
        let config = AuthConfig::from_ctx(ctx)?;

        if users::Entity::find_by_email(&ctx.db, tenant_id, email)
            .await?
            .is_some()
        {
            return Err(Error::BadRequest("Email already exists".into()));
        }

        let password_hash = hash_password(password)?;
        let mut user = users::ActiveModel::new(tenant_id, email, &password_hash);
        user.name = Set(name);
        let user = user.insert(&ctx.db).await?;

        let user_role = user.role.clone();
        AuthService::assign_role_permissions(&ctx.db, &user.id, &tenant_id, user_role).await?;

        let tokens =
            Self::create_session_and_tokens(ctx, tenant_id, &user, None, None, &config).await?;

        Ok((user, tokens))
    }

    pub async fn login(
        ctx: &AppContext,
        tenant_id: uuid::Uuid,
        email: &str,
        password: &str,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<(users::Model, AuthTokens)> {
        let config = AuthConfig::from_ctx(ctx)?;

        let user = users::Entity::find_by_email(&ctx.db, tenant_id, email)
            .await?
            .ok_or_else(|| Error::Unauthorized("Invalid credentials".into()))?;

        if !verify_password(password, &user.password_hash)? {
            return Err(Error::Unauthorized("Invalid credentials".into()));
        }

        if !user.is_active() {
            return Err(Error::Unauthorized("User is inactive".into()));
        }

        let now = Utc::now();
        let mut user_active: users::ActiveModel = user.clone().into();
        user_active.last_login_at = Set(Some(now.into()));
        let user = user_active.update(&ctx.db).await?;

        let tokens =
            Self::create_session_and_tokens(ctx, tenant_id, &user, ip_address, user_agent, &config)
                .await?;

        Ok((user, tokens))
    }

    pub async fn refresh(
        ctx: &AppContext,
        tenant_id: uuid::Uuid,
        refresh_token: &str,
    ) -> Result<(users::Model, AuthTokens)> {
        let config = AuthConfig::from_ctx(ctx)?;
        let token_hash = hash_refresh_token(refresh_token);

        let session = sessions::Entity::find_by_token_hash(&ctx.db, tenant_id, &token_hash)
            .await?
            .ok_or_else(|| Error::Unauthorized("Invalid refresh token".into()))?;

        if !session.is_active() {
            return Err(Error::Unauthorized("Session expired".into()));
        }

        let user = users::Entity::find_by_id(session.user_id)
            .one(&ctx.db)
            .await?
            .ok_or_else(|| Error::Unauthorized("User not found".into()))?;

        if !user.is_active() {
            return Err(Error::Unauthorized("User is inactive".into()));
        }

        let now = Utc::now();
        let new_refresh_token = generate_refresh_token();
        let new_token_hash = hash_refresh_token(&new_refresh_token);
        let expires_at = now + Duration::seconds(config.refresh_expiration as i64);

        let session_id = session.id;
        let mut session_model: sessions::ActiveModel = session.into();
        session_model.token_hash = Set(new_token_hash);
        session_model.expires_at = Set(expires_at.into());
        session_model.last_used_at = Set(Some(now.into()));
        session_model.update(&ctx.db).await?;

        let access_token =
            encode_access_token(&config, user.id, tenant_id, user.role.clone(), session_id)?;

        Ok((
            user,
            AuthTokens {
                access_token,
                refresh_token: new_refresh_token,
                expires_in: config.access_expiration,
            },
        ))
    }

    pub async fn confirm_password_reset(
        ctx: &AppContext,
        tenant_id: uuid::Uuid,
        token: &str,
        password: &str,
    ) -> Result<()> {
        let config = AuthConfig::from_ctx(ctx)?;
        let claims = decode_password_reset_token(&config, token)?;

        if claims.tenant_id != tenant_id {
            return Err(Error::Unauthorized("Invalid reset token".into()));
        }

        let user = users::Entity::find_by_email(&ctx.db, tenant_id, &claims.sub)
            .await?
            .ok_or_else(|| Error::Unauthorized("Invalid reset token".into()))?;

        let user_id = user.id;
        let mut user_active: users::ActiveModel = user.into();
        user_active.password_hash = Set(hash_password(password)?);
        user_active.update(&ctx.db).await?;

        Self::revoke_user_sessions(ctx, tenant_id, user_id, None).await?;

        Ok(())
    }

    pub async fn change_password(
        ctx: &AppContext,
        tenant_id: uuid::Uuid,
        user_id: uuid::Uuid,
        current_session_id: uuid::Uuid,
        current_password: &str,
        new_password: &str,
    ) -> Result<()> {
        let user = users::Entity::find_by_id(user_id)
            .filter(users::Column::TenantId.eq(tenant_id))
            .one(&ctx.db)
            .await?
            .ok_or_else(|| Error::Unauthorized("Invalid credentials".into()))?;

        if !verify_password(current_password, &user.password_hash)? {
            return Err(Error::Unauthorized("Invalid credentials".into()));
        }

        let mut user_active: users::ActiveModel = user.into();
        user_active.password_hash = Set(hash_password(new_password)?);
        user_active.update(&ctx.db).await?;

        Self::revoke_user_sessions(ctx, tenant_id, user_id, Some(current_session_id)).await?;

        Ok(())
    }

    async fn create_session_and_tokens(
        ctx: &AppContext,
        tenant_id: uuid::Uuid,
        user: &users::Model,
        ip_address: Option<String>,
        user_agent: Option<String>,
        config: &AuthConfig,
    ) -> Result<AuthTokens> {
        let now = Utc::now();
        let refresh_token = generate_refresh_token();
        let token_hash = hash_refresh_token(&refresh_token);
        let expires_at = now + Duration::seconds(config.refresh_expiration as i64);

        let session = sessions::ActiveModel::new(
            tenant_id, user.id, token_hash, expires_at, ip_address, user_agent,
        )
        .insert(&ctx.db)
        .await?;

        let access_token =
            encode_access_token(config, user.id, tenant_id, user.role.clone(), session.id)?;

        Ok(AuthTokens {
            access_token,
            refresh_token,
            expires_in: config.access_expiration,
        })
    }

    async fn revoke_user_sessions(
        ctx: &AppContext,
        tenant_id: uuid::Uuid,
        user_id: uuid::Uuid,
        except_session_id: Option<uuid::Uuid>,
    ) -> Result<()> {
        let mut query = sessions::Entity::update_many()
            .col_expr(sessions::Column::RevokedAt, Expr::value(Utc::now()))
            .filter(sessions::Column::TenantId.eq(tenant_id))
            .filter(sessions::Column::UserId.eq(user_id))
            .filter(sessions::Column::RevokedAt.is_null());

        if let Some(session_id) = except_session_id {
            query = query.filter(sessions::Column::Id.ne(session_id));
        }

        query.exec(&ctx.db).await?;
        Ok(())
    }
}
