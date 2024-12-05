use std::ops::Range;

use miette::Diagnostic;
use miette::IntoDiagnostic;

use miette::Result;
use mlua::ExternalResult as _;
use mlua::FromLua;
use mlua::FromLuaMulti;
use mlua::IntoLua;
use mlua::IntoLuaMulti;
use mlua::Lua;
use mlua::MaybeSend;
use mlua::Value;
use regex::Regex;

use crate::yolk::EvalMode;

use super::stdlib;

pub const YOLK_TEXT_NAME: &str = "YOLK_TEXT";

#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("Error in lua code: {}", .message)]
pub struct LuaError {
    message: String,
    #[label()]
    span: Range<usize>,
    origin: mlua::Error,
}

impl LuaError {
    pub fn from_mlua(err: mlua::Error) -> Self {
        Self {
            message: err.to_string(),
            span: 0..0,
            origin: err,
        }
    }
    pub fn from_mlua_with_source(source_code: &str, err: mlua::Error) -> Self {
        let mut msg = err.to_string();
        let re = Regex::new(r"^.*: \[.*?\]:(\d+): (.*)$").unwrap();

        let mut span = 0..0;
        if let Some(caps) = re.captures(&msg) {
            let line_nr = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let err_msg = caps.get(2).unwrap().as_str();
            let offset_start = source_code
                .lines()
                .take(line_nr - 1)
                .map(|x| x.len())
                .sum::<usize>();
            let offset_end = offset_start
                + source_code
                    .lines()
                    .nth(line_nr - 1)
                    .map(|x| x.len())
                    .unwrap_or_default();
            span = offset_start..offset_end;
            msg = err_msg.to_string();
        }
        Self {
            message: msg,
            span,
            origin: err,
        }
    }
}

pub struct EvalCtx {
    lua: Lua,
}

impl Default for EvalCtx {
    fn default() -> Self {
        Self::new_empty()
    }
}

impl EvalCtx {
    pub fn new_empty() -> Self {
        Self { lua: Lua::new() }
    }

    pub fn new_in_mode(mode: EvalMode) -> Result<Self> {
        let ctx = Self::new_empty();
        stdlib::setup_tag_functions(&ctx)?;
        if mode == EvalMode::Local {
            stdlib::setup_impure_functions(&ctx)?;
        }
        Ok(ctx)
    }

    pub fn eval_lua<T: FromLuaMulti>(&self, name: &str, content: &str) -> Result<T> {
        self.lua()
            .load(content)
            .set_name(name)
            .eval()
            .map_err(|e| LuaError::from_mlua_with_source(content, e))
            .into_diagnostic()
            .map_err(|e| e.with_source_code(content.to_string()))
    }
    pub fn exec_lua(&self, name: &str, content: &str) -> Result<()> {
        self.lua()
            .load(content)
            .set_name(name)
            .exec()
            .map_err(|e| LuaError::from_mlua_with_source(content, e))
            .into_diagnostic()
            .map_err(|e| e.with_source_code(content.to_string()))
    }

    pub fn eval_text_transformation(&self, text: &str, expr: &str) -> Result<String> {
        let old_text = self.get_global::<Value>(YOLK_TEXT_NAME)?;
        self.set_global(YOLK_TEXT_NAME, text)?;
        let result = self.eval_lua("template tag", expr)?;
        self.set_global(YOLK_TEXT_NAME, old_text)?;
        Ok(result)
    }

    pub fn set_global<T: IntoLua>(&self, name: impl IntoLua, value: T) -> Result<()> {
        self.lua.globals().set(name, value).into_diagnostic()
    }
    pub fn get_global<T: FromLua>(&self, name: impl IntoLua) -> Result<T> {
        self.lua.globals().get::<T>(name).into_diagnostic()
    }

    pub fn register_fn<F, A, R>(&self, name: &str, func: F) -> Result<()>
    where
        F: Fn(&Lua, A) -> Result<R> + MaybeSend + 'static + Send + Sync,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        self.set_global(
            name,
            self.lua
                .create_function(move |lua, x| func(lua, x).into_lua_err())
                .into_diagnostic()?,
        )
    }

    pub fn lua(&self) -> &Lua {
        &self.lua
    }
    pub fn lua_mut(&mut self) -> &mut Lua {
        &mut self.lua
    }
}
