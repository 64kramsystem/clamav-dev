use crate::{
    message_h::{self, message, mime_type_C, NOMIME},
    message_r,
    sys::cli_errmsg,
};

#[no_mangle]
pub unsafe extern "C" fn messageGetMimeType(m: *const message_h::message_C) -> mime_type_C {
    if m.is_null() {
        cli_errmsg(
            b"Internal email parser error: message is pointer is NULL when trying to get MIME type\n\0"
                as *const u8 as *const libc::c_char,
        );
        return NOMIME;
    }
    message_r::messageGetMimeType(&*(m as *const message)) as mime_type_C
}
