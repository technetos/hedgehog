use tide::{Context, EndpointResult};
use tide::http::StatusCode;

pub trait ContextExt<'ctx, T> {
    fn hedgehog(&'ctx self) -> HedgeHog<'ctx, T>;
}

impl<'ctx, T> ContextExt<'ctx, T> for Context<T> {
    fn hedgehog(&'ctx self) -> HedgeHog<'ctx, T> {
        HedgeHog {
            cx: self
        }
    }
}

pub struct HedgeHog<'ctx, T> {
    cx: &'ctx Context<T>,
}

impl<'ctx, T> HedgeHog<'ctx, T> {
    pub fn protect(&'ctx self) -> EndpointResult {
        Err(StatusCode::UNAUTHORIZED.into())
    }
}
