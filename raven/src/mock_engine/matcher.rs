use std::collections::HashMap;

use raven_logger::MatchError;

use crate::{handler::match_wildcard, mock_engine::MockBlock, request::ResolvableRequest};

pub enum Resolution<'a> {
    Matched {
        mock: &'a MockBlock,
        diagnostics: Option<MatchError>,
    },
    Mismatch {
        diagnostics: MatchError,
    },
    NotFound,
}

pub trait Resolver {
    fn resolve<'a, R>(
        &self,
        request: &R,
        mocks: &'a HashMap<String, MockBlock>,
    ) -> anyhow::Result<Resolution<'a>>
    where
        R: ResolvableRequest;
}

pub struct HttpResolver;

impl Resolver for HttpResolver {
    fn resolve<'a, R>(
        &self,
        request: &R,
        mocks: &'a HashMap<String, MockBlock>,
    ) -> anyhow::Result<Resolution<'a>>
    where
        R: ResolvableRequest,
    {
        let route = normalize_route(request.route());

        if let Some(mock) = mocks.get(&route) {
            return match mock.r#match.matches(request) {
                Ok(()) => Ok(Resolution::Matched {
                    mock,
                    diagnostics: None,
                }),
                Err(e) => Ok(Resolution::Mismatch { diagnostics: e }),
            };
        }

        let mut candidate: Option<(&String, &MockBlock, HashMap<String, String>)> = None;

        for (key, mock) in mocks {
            if let Some(params) = match_wildcard(&route, key) {
                if let Some(c) = candidate {
                    return Err(anyhow::anyhow!(
                        "Multiple mocks match route '{}': '{}' and '{}'",
                        route,
                        key,
                        c.0
                    ));
                }
                candidate = Some((key, mock, params));
            }
        }

        if let Some((_key, mock, _params)) = candidate {
            return match mock.r#match.matches(request) {
                Ok(()) => Ok(Resolution::Matched {
                    mock,
                    diagnostics: None,
                }),

                Err(e) => Ok(Resolution::Mismatch { diagnostics: e }),
            };
        }

        Ok(Resolution::NotFound)
    }
}

/// Normalize a route into Raven's canonical form.
///
/// Rules:
/// - strip query string
/// - trim leading/trailing slashes
/// - split path segments
/// - join with `::`
///
/// Examples:
/// - "/users/42"       -> "users::42"
/// - "users/42/"       -> "users::42"
/// - "/users?x=1"      -> "users"
pub fn normalize_route(route: &str) -> String {
    let path = route.split('?').next().unwrap_or(route);

    let trimmed = path.trim_matches('/');

    trimmed
        .split('/')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("::")
}
