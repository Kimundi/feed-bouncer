use rocket::serde::DeserializeOwned;
use rocket_dyn_templates::{
    handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError},
    Engines,
};

use rocket_dyn_templates::handlebars::HelperDef;

use crate::common::ItemOwned;

fn param_des<T: DeserializeOwned>(h: &Helper<'_, '_>, idx: usize) -> Result<T, RenderError> {
    let v = h
        .param(idx)
        .ok_or_else(|| RenderError::new(format!("param {} not found", idx)))?;
    let v = v.value();
    let v: T = serde_json::from_value(v.clone())
        .map_err(|e| RenderError::new(format!("param not of right format: {}", e)))?;
    Ok(v)
}

struct FeedList;
impl HelperDef for FeedList {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _r: &'reg Handlebars<'reg>,
        _ctx: &'rc Context,
        _rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let _items: Vec<ItemOwned> = param_des(h, 0)?;
        let _include_feed: bool = param_des(h, 1)?;

        out.write("Test<br>blub<br>foo")?;

        Ok(())
    }
}

pub fn register(engines: &mut Engines) {
    let engine = &mut engines.handlebars;
    engine.register_helper("feed_list", Box::new(FeedList));
}
