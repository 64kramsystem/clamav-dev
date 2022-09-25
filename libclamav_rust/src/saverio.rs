#[export_name = "ohyeah"]
pub extern "C" fn ohyeah() {
    //
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct message {}

#[allow(non_camel_case_types)]
type mime_type = libc::c_uint;

#[allow(dead_code, non_snake_case)]
pub(crate) fn messageGetMimeType(_m: *const message) -> mime_type {
    // m.mimeType
    todo!()
}
