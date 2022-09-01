mod msg;
mod querier;
mod query;
mod types;

pub use msg::ZodiaticMsg;
pub use querier::ZodiaticQuerier;
pub use query::{LunarResponse, ZodiaticQuery};
pub use types::Lunar;

// This is a signal, such that any contract that imports these helpers will only run on the
// zodiatic blockchain
#[no_mangle]
extern "C" fn requires_zodiatic() {}
