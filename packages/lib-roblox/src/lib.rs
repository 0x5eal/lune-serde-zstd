use mlua::prelude::*;

pub mod datatypes;
pub mod document;
pub mod instance;

use datatypes::types::*;
use datatypes::DatatypeTable;

fn make_dt<F>(lua: &Lua, f: F) -> LuaResult<LuaTable>
where
    F: Fn(&Lua, &LuaTable) -> LuaResult<()>,
{
    let tab = lua.create_table()?;
    f(lua, &tab)?;
    tab.set_readonly(true);
    Ok(tab)
}

pub fn module(lua: &Lua) -> LuaResult<LuaTable> {
    let datatypes = vec![("Vector3", make_dt(lua, Vector3::make_dt_table)?)];
    let exports = lua.create_table()?;
    for (name, tab) in datatypes {
        exports.set(name, tab)?;
    }
    Ok(exports)
}