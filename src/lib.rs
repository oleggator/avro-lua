use std::{io::Read};
use mlua::{LuaSerdeExt, prelude::*};
use avro_rs::Schema;
use std::convert::TryFrom;

struct Decoder {
    schema: Schema,
}

impl Decoder {
    pub fn new(schema: &str) -> Result<Self, avro_rs::Error> {
        Ok(Decoder {
            schema: Schema::parse_str(schema)?,
        })
    }

    pub fn decode<R: Read>(&self, reader: &mut R) -> Result<serde_json::Value, avro_rs::Error> {
        let result = avro_rs::from_avro_datum(&self.schema, reader, None).unwrap();
        let json_value = serde_json::Value::try_from(result).unwrap();

        Ok(json_value)
    }
}

impl mlua::UserData for Decoder {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("decode", |lua, this: &Decoder, blob: LuaString| {
            let json_value = this.decode(&mut blob.as_bytes()).unwrap();
            lua.to_value(&json_value)
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
            let avro = Decoder::new(schema.as_str()).unwrap();
            Ok(avro)
        })?,
    )?;

    Ok(exports)
}

