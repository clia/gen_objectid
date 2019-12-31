use pgxr::bindings::*;
use pgxr::*;
use bson::oid;
use log::{debug, error, info, warn};

PG_MODULE_MAGIC!();

PG_FUNCTION_INFO_V1!(pg_finfo_gen_object_id);

#[no_mangle]
pub extern "C" fn gen_object_id(_fcinfo: FunctionCallInfo) -> Datum {
    PG_RETURN_TEXT(gen_objectid())
}

pub fn gen_objectid() -> String {
    let mut s = "".to_owned();
    match oid::ObjectId::new() {
        Ok(objectid) => {
            s = objectid.to_hex();
        },
        Err(err) => {
            warn!("gen_objectid error: {}", err);
        },
    }
    s
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_gen_objectid() {
        println!("{}", gen_objectid())
    }
}
