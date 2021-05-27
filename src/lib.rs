use std::{io::Read};
use mlua::{LuaSerdeExt, prelude::*};
use avro_rs::Schema;
use std::convert::TryFrom;
use serde_json::Value;
use std::sync::Arc;

struct Avro {
    schema: Schema,
}

impl Avro {
    pub fn new(schema: &str) -> Result<Self, avro_rs::Error> {
        Ok(Avro {
            schema: Schema::parse_str(schema)?,
        })
    }

    pub fn decode<R: Read>(&self, reader: &mut R) -> Result<Value, avro_rs::Error> {
        let avro_value = avro_rs::from_avro_datum(&self.schema, reader, None)?;
        Ok(Value::try_from(avro_value)?)
    }

    pub fn encode(&self, value: Value) -> Result<Vec<u8>, avro_rs::Error> {
        let avro_value = avro_rs::types::Value::from(value);
        avro_rs::to_avro_datum(&self.schema, avro_value)
    }
}

impl mlua::UserData for Avro {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("decode", |lua, this: &Avro, blob: LuaString| {
            let json_value = this.decode(&mut blob.as_bytes())
                .map_err(|err| mlua::Error::ExternalError(Arc::new(err)))?;
            lua.to_value_with(&json_value, LuaSerializeOptions::new()
                .serialize_none_to_null(false)
                .serialize_unit_to_null(false))
        });

        methods.add_method("encode", |lua, this: &Avro, table: LuaValue| {
            let json_value: Value = lua.from_value(table)?;
            let blob = this.encode(json_value)
                .map_err(|err| mlua::Error::ExternalError(Arc::new(err)))?;
            lua.create_string(&blob)
        });
    }
}

#[mlua::lua_module]
fn avro(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set(
        "new",
        lua.create_function_mut(|_, (schema, )| {
            let schema: String = schema;
            let avro = Avro::new(schema.as_str())
                .map_err(|err| mlua::Error::ExternalError(Arc::new(err)))?;
            Ok(avro)
        })?,
    )?;

    Ok(exports)
}

