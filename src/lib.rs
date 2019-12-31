use pgxr::bindings::*;
use pgxr::*;
use bson::oid;
use log::warn;

PG_MODULE_MAGIC!();

PG_FUNCTION_INFO_V1!(pg_finfo_gen_objectid);

#[no_mangle]
pub extern "C" fn gen_objectid(_fcinfo: FunctionCallInfo) -> Datum {
    PG_RETURN_TEXT(gen_objectid_fn())
}

pub fn gen_objectid_fn() -> String {
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
        println!("{}", crate::gen_objectid_fn())
    }
}
