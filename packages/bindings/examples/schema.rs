use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use zodiatic_bindings::{LunarResponse, ZodiaticMsg, ZodiaticQuery};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(ZodiaticMsg), &out_dir);
    export_schema(&schema_for!(ZodiaticQuery), &out_dir);
    export_schema(&schema_for!(LunarResponse), &out_dir);
}
