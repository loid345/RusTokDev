use std::collections::{BTreeSet, HashSet};

use rustok_api::TenantContext;
use rustok_seo_targets::{SeoTargetBulkListRequest, SeoTargetCapabilityKind};

use crate::{SeoError, SeoResult};

use super::SeoService;
use super::TargetState;

#[derive(Debug, Clone)]
struct CrossLinkCandidate {
    label: String,
    route: String,
    score: usize,
}

impl SeoService {
    pub(super) async fn enrich_target_state_with_cross_links(
        &self,
        tenant: &TenantContext,
        state: &mut TargetState,
    ) -> SeoResult<()> {
        let settings = self.load_settings(tenant.id).await?;
        if !settings.cross_linking_enabled {
            return Ok(());
        }

        let insertion_points = normalize_cross_link_insertion_points(
            settings.cross_link_insertion_points.as_slice(),
        );
        if insertion_points.is_empty() {
            return Ok(());
        }

        let links = self
            .collect_cross_link_candidates(
                tenant,
                state,
                settings.cross_link_target_limit.clamp(1, 12) as usize,
            )
            .await?;
        if links.is_empty() {
            return Ok(());
        }

        let markdown_bullets = links
            .iter()
            .map(|item| format!("- [{}]({})", item.label, item.route))
            .collect::<Vec<_>>()
            .join("\n");
        let inline_markdown = links
            .iter()
            .map(|item| format!("[{}]({})", item.label, item.route))
            .collect::<Vec<_>>()
            .join(" · ");
        let plain_text = links
            .iter()
            .map(|item| format!("{} ({})", item.label, item.route))
            .collect::<Vec<_>>()
            .join("; ");

        state
            .template_fields
            .insert("cross_link_count".to_string(), links.len().to_string());
        state.template_fields.insert(
            "cross_links_markdown".to_string(),
            markdown_bullets.clone(),
        );
        state
            .template_fields
            .insert("cross_links_inline".to_string(), inline_markdown);
        state
            .template_fields
            .insert("cross_links_text".to_string(), plain_text);

        for (index, link) in links.iter().enumerate() {
            let number = index + 1;
            state.template_fields.insert(
                format!("cross_link_{number}_label"),
                link.label.clone(),
            );
            state.template_fields.insert(
                format!("cross_link_{number}_route"),
                link.route.clone(),
            );
            state.template_fields.insert(
                format!("cross_link_{number}_markdown"),
                format!("[{}]({})", link.label, link.route),
            );
        }

        for insertion_point in insertion_points {
            state.template_fields.insert(
                format!("cross_links_{insertion_point}"),
                markdown_bullets.clone(),
            );
        }

        Ok(())
    }

    async fn collect_cross_link_candidates(
        &self,
        tenant: &TenantContext,
        state: &TargetState,
        max_links: usize,
    ) -> SeoResult<Vec<CrossLinkCandidate>> {
        let source_tokens = target_tokens(state);
        if source_tokens.is_empty() {
            return Ok(Vec::new());
        }

        let mut candidates = Vec::new();
        for provider in self
            .registry
            .providers_with_capability(SeoTargetCapabilityKind::Bulk)
        {
            let summaries = provider
                .list_bulk_summaries(
                    &self.target_runtime(),
                    SeoTargetBulkListRequest {
                        tenant_id: tenant.id,
                        default_locale: tenant.default_locale.as_str(),
                        locale: state.effective_locale.as_str(),
                    },
                )
                .await
                .map_err(|error| {
                    SeoError::validation(format!(
                        "SEO target provider `{}` failed to list cross-link candidates: {error}",
                        provider.slug().as_str()
                    ))
                })?;

            for summary in summaries {
                if summary.target_kind == state.target_kind && summary.target_id == state.target_id {
                    continue;
                }

                let route = summary.route.trim();
                let label = summary.label.trim();
                if route.is_empty() || label.is_empty() {
                    continue;
                }

                let score = similarity_score(
                    source_tokens.as_ref(),
                    label,
                    route,
                    summary.target_kind == state.target_kind,
                );
                if score == 0 {
                    continue;
                }

                candidates.push(CrossLinkCandidate {
                    label: label.to_string(),
                    route: route.to_string(),
                    score,
                });
            }
        }

        candidates.sort_by(|left, right| {
            right
                .score
                .cmp(&left.score)
                .then_with(|| left.label.cmp(&right.label))
                .then_with(|| left.route.cmp(&right.route))
        });

        let mut deduped = Vec::new();
        let mut seen_routes = BTreeSet::new();
        for candidate in candidates {
            if !seen_routes.insert(candidate.route.clone()) {
                continue;
            }
            deduped.push(candidate);
            if deduped.len() >= max_links {
                break;
            }
        }

        Ok(deduped)
    }
}

pub(super) fn normalize_cross_link_insertion_points(values: &[String]) -> Vec<String> {
    let mut unique = BTreeSet::new();
    for value in values {
        let normalized = value
            .trim()
            .to_ascii_lowercase()
            .chars()
            .filter_map(|ch| {
                if ch.is_ascii_alphanumeric() {
                    Some(ch)
                } else if ch == '_' || ch == '-' || ch.is_whitespace() {
                    Some('_')
                } else {
                    None
                }
            })
            .collect::<String>()
            .split('_')
            .filter(|segment| !segment.is_empty())
            .collect::<Vec<_>>()
            .join("_");

        if normalized.is_empty() {
            continue;
        }

        unique.insert(normalized);
    }

    if unique.is_empty() {
        vec!["description".to_string()]
    } else {
        unique.into_iter().collect()
    }
}

fn target_tokens(state: &TargetState) -> HashSet<String> {
    let mut tokens = tokenize(state.title.as_str());
    if let Some(description) = state.description.as_deref() {
        tokens.extend(tokenize(description));
    }
    tokens
}

fn similarity_score(
    source_tokens: &HashSet<String>,
    label: &str,
    route: &str,
    same_target_kind: bool,
) -> usize {
    let mut candidate_tokens = tokenize(label);
    candidate_tokens.extend(tokenize(route));
    if candidate_tokens.is_empty() {
        return 0;
    }

    let overlap = source_tokens
        .intersection(&candidate_tokens)
        .count();
    if overlap == 0 {
        return 0;
    }

    overlap + usize::from(same_target_kind)
}

fn tokenize(value: &str) -> HashSet<String> {
    value
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter_map(|token| {
            let token = token.trim().to_ascii_lowercase();
            if token.len() < 3 {
                return None;
            }
            if token.chars().all(|ch| ch.is_ascii_digit()) {
                return None;
            }
            Some(token)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{normalize_cross_link_insertion_points, similarity_score, tokenize};

    #[test]
    fn normalize_cross_link_insertion_points_sanitizes_and_deduplicates() {
        let points = normalize_cross_link_insertion_points(&[
            " Description ".to_string(),
            "description".to_string(),
            "Open Graph".to_string(),
            "!!!".to_string(),
        ]);

        assert_eq!(points, vec!["description", "open_graph"]);
    }

    #[test]
    fn tokenize_discards_short_or_numeric_segments() {
        let tokens = tokenize("A/B test: Product 101 and price");

        assert!(!tokens.contains("a"));
        assert!(!tokens.contains("101"));
        assert!(tokens.contains("test"));
        assert!(tokens.contains("product"));
        assert!(tokens.contains("and"));
        assert!(tokens.contains("price"));
    }

    #[test]
    fn similarity_score_boosts_same_target_kind() {
        let source = tokenize("Demo product shoes");

        let same_kind = similarity_score(&source, "Demo shoes", "/modules/product", true);
        let other_kind = similarity_score(&source, "Demo shoes", "/modules/blog", false);

        assert!(same_kind > other_kind);
    }
}
