use std::ffi::CStr;
use std::os::raw::c_char;

macro_rules! to_doc {
    ($handle:expr) => {{
        let handle = $handle.as_mut();
        match handle {
            Some(b) => b,
            None => return AMresult::err("Invalid AMdoc pointer").into(),
        }
    }};
}

pub(crate) use to_doc;

macro_rules! to_obj_id {
    ($handle:expr) => {{
        match $handle.as_ref() {
            Some(obj_id) => obj_id,
            None => &automerge::ROOT,
        }
    }};
}

pub(crate) use to_obj_id;

pub(crate) unsafe fn to_str(c: *const c_char) -> String {
    CStr::from_ptr(c).to_string_lossy().to_string()
}
