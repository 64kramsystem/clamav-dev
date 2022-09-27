use ::c2rust_bitfields;
use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type _yara_global;
    pub type cli_bcengine;
    pub type cli_bc_dbgnode;
    pub type cli_bc_type;
    pub type cli_bc_func;
    pub type MP;
    pub type CACHE;
    pub type pcre2_real_match_context_8;
    pub type pcre2_real_code_8;
    pub type filter;
    pub type re_guts;
    pub type regex_matcher;
    pub type json_object;
    pub type cli_events;
    pub type _xmlDict;
    pub type _xmlTextReader;
    pub type msxml_ictx;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE) -> *mut libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> *mut libc::c_char;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn rewinddir(__dirp: *mut DIR);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __fxstat(__ver: libc::c_int, __fildes: libc::c_int, __stat_buf: *mut stat) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn cl_engine_get_str(
        engine: *const cl_engine,
        field: cl_engine_field,
        err: *mut libc::c_int,
    ) -> *const libc::c_char;
    fn cl_hash_data(
        alg: *const libc::c_char,
        buf: *const libc::c_void,
        len: size_t,
        obuf: *mut libc::c_uchar,
        olen: *mut libc::c_uint,
    ) -> *mut libc::c_uchar;
    fn cli_compare_ftm_file(
        buf: *const libc::c_uchar,
        buflen: size_t,
        engine: *const cl_engine,
    ) -> cli_file_t;
    fn json_object_object_get_ex(
        obj: *const json_object,
        key: *const libc::c_char,
        value: *mut *mut json_object,
    ) -> json_bool;
    fn json_object_array_length(obj: *const json_object) -> size_t;
    fn json_object_array_get_idx(obj: *const json_object, idx: size_t) -> *mut json_object;
    fn json_object_get_string(obj: *mut json_object) -> *const libc::c_char;
    fn cli_errmsg(str: *const libc::c_char, _: ...);
    fn __cli_strcasestr(
        haystack: *const libc::c_char,
        needle: *const libc::c_char,
    ) -> *const libc::c_char;
    fn cli_chomp(string: *mut libc::c_char) -> libc::c_int;
    fn cli_str2hex(string: *const libc::c_char, len: libc::c_uint) -> *mut libc::c_char;
    fn cli_strtokbuf(
        input: *const libc::c_char,
        fieldno: libc::c_int,
        delim: *const libc::c_char,
        output: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn cli_strlcat(dst: *mut libc::c_char, src: *const libc::c_char, sz: size_t) -> size_t;
    fn cli_append_virus(ctx: *mut cli_ctx, virname: *const libc::c_char) -> cl_error_t;
    fn cli_dbgmsg(str: *const libc::c_char, _: ...);
    fn cli_malloc(nmemb: size_t) -> *mut libc::c_void;
    fn cli_calloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn cli_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn cli_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn cli_unlink(pathname: *const libc::c_char) -> libc::c_int;
    fn cli_gettmpdir() -> *const libc::c_char;
    fn cli_append_virus_if_heur_exceedsmax(_: *mut cli_ctx, _: *mut libc::c_char);
    fn cli_strerror(
        errnum: libc::c_int,
        buf: *mut libc::c_char,
        len: size_t,
    ) -> *const libc::c_char;
    fn cli_warnmsg(str: *const libc::c_char, _: ...);
    fn blobDestroy(b: *mut blob);
    fn blobGetData(b: *const blob) -> *mut libc::c_uchar;
    fn blobGetDataSize(b: *const blob) -> size_t;
    fn fileblobCreate() -> *mut fileblob;
    fn fileblobScanAndDestroy(fb: *mut fileblob) -> libc::c_int;
    fn fileblobDestroy(fb: *mut fileblob);
    fn fileblobSetFilename(
        fb: *mut fileblob,
        dir: *const libc::c_char,
        filename: *const libc::c_char,
    );
    fn fileblobGetFilename(fb: *const fileblob) -> *const libc::c_char;
    fn fileblobSetCTX(fb: *mut fileblob, ctx: *mut cli_ctx);
    fn fileblobAddData(fb: *mut fileblob, data: *const libc::c_uchar, len: size_t) -> libc::c_int;
    fn fileblobInfected(fb: *const fileblob) -> libc::c_int;
    fn cli_json_addowner(
        owner: *mut json_object,
        child: *mut json_object,
        key: *const libc::c_char,
        idx: libc::c_int,
    ) -> cl_error_t;
    fn messageCreate() -> *mut message;
    fn messageDestroy(m: *mut message);
    fn messageReset(m: *mut message);
    fn messageSetMimeType(m: *mut message, type_0: *const libc::c_char) -> libc::c_int;
    fn messageGetMimeType(m: *const message) -> mime_type;
    fn messageSetMimeSubtype(m: *mut message, subtype_0: *const libc::c_char);
    fn messageGetMimeSubtype(m: *const message) -> *const libc::c_char;
    fn messageSetDispositionType(m: *mut message, disptype: *const libc::c_char);
    fn messageGetDispositionType(m: *const message) -> *const libc::c_char;
    fn messageAddArgument(m: *mut message, arg: *const libc::c_char);
    fn messageAddArguments(m: *mut message, arg: *const libc::c_char);
    fn messageSetEncoding(m: *mut message, enctype: *const libc::c_char);
    fn messageGetEncoding(m: *const message) -> encoding_type;
    fn messageAddLine(m: *mut message, line: *mut line_t) -> libc::c_int;
    fn messageAddStr(m: *mut message, data: *const libc::c_char) -> libc::c_int;
    fn messageMoveText(m: *mut message, t: *mut text, old_message: *mut message) -> libc::c_int;
    fn messageGetBody(m: *mut message) -> *mut text;
    fn messageToFileblob(
        m: *mut message,
        dir: *const libc::c_char,
        destroy: libc::c_int,
    ) -> *mut fileblob;
    fn messageToBlob(m: *mut message, destroy: libc::c_int) -> *mut blob;
    fn binhexBegin(m: *mut message) -> *mut text;
    fn bounceBegin(m: *mut message) -> *mut text;
    fn encodingLine(m: *mut message) -> *mut text;
    fn isuuencodebegin(line: *const libc::c_char) -> libc::c_int;
    fn messageSetCTX(m: *mut message, ctx: *mut cli_ctx);
    fn messageContainsVirus(m: *const message) -> libc::c_int;
    fn messageSavePartial(
        m: *mut message,
        dir: *const libc::c_char,
        id: *const libc::c_char,
        part: libc::c_uint,
    ) -> libc::c_int;
    fn messageGetJObj(m: *mut message) -> *mut json_object;
    fn textDestroy(t_head: *mut text);
    fn textAddMessage(aText: *mut text, aMessage: *mut message) -> *mut text;
    fn textToBlob(t: *mut text, b: *mut blob, destroy: libc::c_int) -> *mut blob;
    fn textToFileblob(t: *mut text, fb: *mut fileblob, destroy: libc::c_int) -> *mut fileblob;
    fn uudecodeFile(
        m: *mut message,
        firstline: *const libc::c_char,
        dir: *const libc::c_char,
        map: *mut fmap_t,
        at: *mut size_t,
    ) -> libc::c_int;
    fn cli_jsonobj(obj: *mut json_object, key: *const libc::c_char) -> *mut json_object;
    fn tableCreate() -> *mut table;
    fn tableDestroy(table: *mut table_t);
    fn tableInsert(
        table: *mut table_t,
        key: *const libc::c_char,
        value: libc::c_int,
    ) -> libc::c_int;
    fn cli_jsonarray(obj: *mut json_object, key: *const libc::c_char) -> *mut json_object;
    fn tableFind(table: *const table_t, key: *const libc::c_char) -> libc::c_int;
    fn cli_jsonint(obj: *mut json_object, key: *const libc::c_char, i: int32_t) -> cl_error_t;
    fn messageGetFilename(m: *const message) -> *mut libc::c_char;
    fn cli_jsonstr(
        obj: *mut json_object,
        key: *const libc::c_char,
        s: *const libc::c_char,
    ) -> cl_error_t;
    fn cli_json_parse_error(root: *mut json_object, errstr: *const libc::c_char) -> cl_error_t;
    fn lineGetData(line: *const line_t) -> *const libc::c_char;
    fn lineUnlink(line: *mut line_t) -> *mut line_t;
    fn messageHasFilename(m: *const message) -> libc::c_int;
    fn sanitiseName(name: *mut libc::c_char);
    fn messageFindArgument(m: *const message, variable: *const libc::c_char) -> *mut libc::c_char;
    fn xmlFreeDoc(cur: xmlDocPtr);
    fn xmlFreeTextReader(reader: xmlTextReaderPtr);
    fn xmlTextReaderClose(reader: xmlTextReaderPtr) -> libc::c_int;
    fn cli_msxml_parse_document(
        ctx: *mut cli_ctx,
        reader: xmlTextReaderPtr,
        keys: *const key_entry,
        num_keys: size_t,
        flags: uint32_t,
        mxctx: *mut msxml_ctx,
    ) -> libc::c_int;
    fn xmlReaderForMemory(
        buffer: *const libc::c_char,
        size: libc::c_int,
        URL: *const libc::c_char,
        encoding: *const libc::c_char,
        options: libc::c_int,
    ) -> xmlTextReaderPtr;
    fn xmlReaderWalker(doc: xmlDocPtr) -> xmlTextReaderPtr;
    fn xmlGetDocCompressMode(doc: *const xmlDoc) -> libc::c_int;
    fn htmlReadMemory(
        buffer: *const libc::c_char,
        size: libc::c_int,
        URL: *const libc::c_char,
        encoding: *const libc::c_char,
        options: libc::c_int,
    ) -> htmlDocPtr;
    fn htmlGetMetaEncoding(doc: htmlDocPtr) -> *const xmlChar;
    fn html_normalise_mem(
        in_buff: *mut libc::c_uchar,
        in_size: off_t,
        dirname: *const libc::c_char,
        hrefs: *mut tag_arguments_t,
        dconf: *const cli_dconf,
    ) -> libc::c_int;
    fn html_tag_arg_free(tags: *mut tag_arguments_t);
    fn html_tag_arg_add(
        tags: *mut tag_arguments_t,
        tag: *const libc::c_char,
        value: *mut libc::c_char,
    );
    fn phishingScan(ctx: *mut cli_ctx, hrefs: *mut tag_arguments_t) -> cl_error_t;
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_0 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_0 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_0 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_0 = 0;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type cl_error_t = libc::c_uint;
pub const CL_ELAST_ERROR: cl_error_t = 35;
pub const CL_ERROR: cl_error_t = 34;
pub const CL_VERIFIED: cl_error_t = 33;
pub const CL_ESTATE: cl_error_t = 32;
pub const CL_EBUSY: cl_error_t = 31;
pub const CL_ELOCK: cl_error_t = 30;
pub const CL_EBYTECODE_TESTFAIL: cl_error_t = 29;
pub const CL_EBYTECODE: cl_error_t = 28;
pub const CL_EPARSE: cl_error_t = 27;
pub const CL_EFORMAT: cl_error_t = 26;
pub const CL_EMAXFILES: cl_error_t = 25;
pub const CL_EMAXSIZE: cl_error_t = 24;
pub const CL_EMAXREC: cl_error_t = 23;
pub const CL_BREAK: cl_error_t = 22;
pub const CL_ETIMEOUT: cl_error_t = 21;
pub const CL_EMEM: cl_error_t = 20;
pub const CL_EMAP: cl_error_t = 19;
pub const CL_ETMPDIR: cl_error_t = 18;
pub const CL_ETMPFILE: cl_error_t = 17;
pub const CL_EACCES: cl_error_t = 16;
pub const CL_EDUP: cl_error_t = 15;
pub const CL_EWRITE: cl_error_t = 14;
pub const CL_ESEEK: cl_error_t = 13;
pub const CL_EREAD: cl_error_t = 12;
pub const CL_ESTAT: cl_error_t = 11;
pub const CL_EUNLINK: cl_error_t = 10;
pub const CL_ECREAT: cl_error_t = 9;
pub const CL_EOPEN: cl_error_t = 8;
pub const CL_EUNPACK: cl_error_t = 7;
pub const CL_EVERIFY: cl_error_t = 6;
pub const CL_ECVD: cl_error_t = 5;
pub const CL_EMALFDB: cl_error_t = 4;
pub const CL_EARG: cl_error_t = 3;
pub const CL_ENULLARG: cl_error_t = 2;
pub const CL_VIRUS: cl_error_t = 1;
pub const CL_SUCCESS: cl_error_t = 0;
pub const CL_CLEAN: cl_error_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cl_scan_options {
    pub general: uint32_t,
    pub parse: uint32_t,
    pub heuristic: uint32_t,
    pub mail: uint32_t,
    pub dev: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cl_engine {
    pub refcount: uint32_t,
    pub sdb: uint32_t,
    pub dboptions: uint32_t,
    pub dbversion: [uint32_t; 2],
    pub ac_only: uint32_t,
    pub ac_mindepth: uint32_t,
    pub ac_maxdepth: uint32_t,
    pub tmpdir: *mut libc::c_char,
    pub keeptmp: uint32_t,
    pub engine_options: uint64_t,
    pub maxscantime: uint32_t,
    pub maxscansize: uint64_t,
    pub maxfilesize: uint64_t,
    pub max_recursion_level: uint32_t,
    pub maxfiles: uint32_t,
    pub min_cc_count: uint32_t,
    pub min_ssn_count: uint32_t,
    pub root: *mut *mut cli_matcher,
    pub hm_hdb: *mut cli_matcher,
    pub hm_mdb: *mut cli_matcher,
    pub hm_imp: *mut cli_matcher,
    pub hm_fp: *mut cli_matcher,
    pub cdb: *mut cli_cdb,
    pub allow_list_matcher: *mut regex_matcher,
    pub domain_list_matcher: *mut regex_matcher,
    pub phishcheck: *mut phishcheck,
    pub dconf: *mut cli_dconf,
    pub ftypes: *mut cli_ftype,
    pub ptypes: *mut cli_ftype,
    pub pwdbs: *mut *mut cli_pwdb,
    pub test_root: *mut cli_matcher,
    pub ignored: *mut cli_matcher,
    pub pua_cats: *mut libc::c_char,
    pub iconcheck: *mut icon_matcher,
    pub cache: *mut CACHE,
    pub dbinfo: *mut cli_dbinfo,
    pub num_total_signatures: size_t,
    pub mempool: *mut mpool_t,
    pub cmgr: crtmgr,
    pub cb_pre_cache: clcb_pre_cache,
    pub cb_pre_scan: clcb_pre_scan,
    pub cb_post_scan: clcb_post_scan,
    pub cb_virus_found: clcb_virus_found,
    pub cb_sigload: clcb_sigload,
    pub cb_sigload_ctx: *mut libc::c_void,
    pub cb_hash: clcb_hash,
    pub cb_meta: clcb_meta,
    pub cb_file_props: clcb_file_props,
    pub cb_sigload_progress: clcb_progress,
    pub cb_sigload_progress_ctx: *mut libc::c_void,
    pub cb_engine_compile_progress: clcb_progress,
    pub cb_engine_compile_progress_ctx: *mut libc::c_void,
    pub cb_engine_free_progress: clcb_progress,
    pub cb_engine_free_progress_ctx: *mut libc::c_void,
    pub bcs: cli_all_bc,
    pub hooks: [*mut libc::c_uint; 7],
    pub hooks_cnt: [libc::c_uint; 7],
    pub hook_lsig_ids: libc::c_uint,
    pub bytecode_security: bytecode_security,
    pub bytecode_timeout: uint32_t,
    pub bytecode_mode: bytecode_mode,
    pub maxembeddedpe: uint64_t,
    pub maxhtmlnormalize: uint64_t,
    pub maxhtmlnotags: uint64_t,
    pub maxscriptnormalize: uint64_t,
    pub maxziptypercg: uint64_t,
    pub stats_data: *mut libc::c_void,
    pub cb_stats_add_sample: clcb_stats_add_sample,
    pub cb_stats_remove_sample: clcb_stats_remove_sample,
    pub cb_stats_decrement_count: clcb_stats_decrement_count,
    pub cb_stats_submit: clcb_stats_submit,
    pub cb_stats_flush: clcb_stats_flush,
    pub cb_stats_get_num: clcb_stats_get_num,
    pub cb_stats_get_size: clcb_stats_get_size,
    pub cb_stats_get_hostid: clcb_stats_get_hostid,
    pub maxpartitions: uint32_t,
    pub maxiconspe: uint32_t,
    pub maxrechwp3: uint32_t,
    pub pcre_match_limit: uint64_t,
    pub pcre_recmatch_limit: uint64_t,
    pub pcre_max_filesize: uint64_t,
    pub yara_global: *mut _yara_global,
}
pub type clcb_stats_get_hostid =
    Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_char>;
pub type clcb_stats_get_size = Option<unsafe extern "C" fn(*mut libc::c_void) -> size_t>;
pub type clcb_stats_get_num = Option<unsafe extern "C" fn(*mut libc::c_void) -> size_t>;
pub type clcb_stats_flush = Option<unsafe extern "C" fn(*mut cl_engine, *mut libc::c_void) -> ()>;
pub type clcb_stats_submit = Option<unsafe extern "C" fn(*mut cl_engine, *mut libc::c_void) -> ()>;
pub type clcb_stats_decrement_count = Option<
    unsafe extern "C" fn(
        *const libc::c_char,
        *const libc::c_uchar,
        size_t,
        *mut libc::c_void,
    ) -> (),
>;
pub type clcb_stats_remove_sample = Option<
    unsafe extern "C" fn(
        *const libc::c_char,
        *const libc::c_uchar,
        size_t,
        *mut libc::c_void,
    ) -> (),
>;
pub type clcb_stats_add_sample = Option<
    unsafe extern "C" fn(
        *const libc::c_char,
        *const libc::c_uchar,
        size_t,
        *mut stats_section_t,
        *mut libc::c_void,
    ) -> (),
>;
pub type stats_section_t = cli_stats_sections;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_stats_sections {
    pub nsections: size_t,
    pub sections: *mut cli_section_hash,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_section_hash {
    pub md5: [libc::c_uchar; 16],
    pub len: size_t,
}
pub type bytecode_mode = libc::c_uint;
pub const CL_BYTECODE_MODE_OFF: bytecode_mode = 4;
pub const CL_BYTECODE_MODE_TEST: bytecode_mode = 3;
pub const CL_BYTECODE_MODE_INTERPRETER: bytecode_mode = 2;
pub const CL_BYTECODE_MODE_JIT: bytecode_mode = 1;
pub const CL_BYTECODE_MODE_AUTO: bytecode_mode = 0;
pub type bytecode_security = libc::c_uint;
pub const CL_BYTECODE_TRUST_NOTHING: bytecode_security = 2;
pub const CL_BYTECODE_TRUST_SIGNED: bytecode_security = 1;
pub const CL_BYTECODE_TRUST_ALL: bytecode_security = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_all_bc {
    pub all_bcs: *mut cli_bc,
    pub count: libc::c_uint,
    pub engine: *mut cli_bcengine,
    pub env: cli_environment,
    pub inited: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_environment {
    pub platform_id_a: uint32_t,
    pub platform_id_b: uint32_t,
    pub platform_id_c: uint32_t,
    pub c_version: uint32_t,
    pub cpp_version: uint32_t,
    pub functionality_level: uint32_t,
    pub dconf_level: uint32_t,
    pub engine_version: [int8_t; 65],
    pub triple: [int8_t; 65],
    pub cpu: [int8_t; 65],
    pub sysname: [int8_t; 65],
    pub release: [int8_t; 65],
    pub version: [int8_t; 65],
    pub machine: [int8_t; 65],
    pub big_endian: uint8_t,
    pub sizeof_ptr: uint8_t,
    pub arch: uint8_t,
    pub os_category: uint8_t,
    pub os: uint8_t,
    pub compiler: uint8_t,
    pub has_jit_compiled: uint8_t,
    pub os_features: uint8_t,
    pub reserved0: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_bc {
    pub metadata: bytecode_metadata,
    pub id: libc::c_uint,
    pub kind: libc::c_uint,
    pub num_types: libc::c_uint,
    pub num_func: libc::c_uint,
    pub funcs: *mut cli_bc_func,
    pub types: *mut cli_bc_type,
    pub globals: *mut *mut uint64_t,
    pub globaltys: *mut uint16_t,
    pub num_globals: size_t,
    pub state: bc_state,
    pub uses_apis: *mut bitset_tag,
    pub lsig: *mut libc::c_char,
    pub vnameprefix: *mut libc::c_char,
    pub vnames: *mut *mut libc::c_char,
    pub vnames_cnt: libc::c_uint,
    pub start_tid: uint16_t,
    pub dbgnodes: *mut cli_bc_dbgnode,
    pub dbgnode_cnt: libc::c_uint,
    pub hook_lsig_id: libc::c_uint,
    pub trusted: libc::c_uint,
    pub numGlobalBytes: uint32_t,
    pub globalBytes: *mut uint8_t,
    pub sigtime_id: uint32_t,
    pub sigmatch_id: uint32_t,
    pub hook_name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bitset_tag {
    pub bitset: *mut libc::c_uchar,
    pub length: libc::c_ulong,
}
pub type bc_state = libc::c_uint;
pub const bc_disabled: bc_state = 4;
pub const bc_interp: bc_state = 3;
pub const bc_jit: bc_state = 2;
pub const bc_loaded: bc_state = 1;
pub const bc_skip: bc_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bytecode_metadata {
    pub compiler: *mut libc::c_char,
    pub sigmaker: *mut libc::c_char,
    pub timestamp: uint64_t,
    pub formatlevel: libc::c_uint,
    pub minfunc: libc::c_uint,
    pub maxfunc: libc::c_uint,
    pub maxresource: libc::c_uint,
    pub targetExclude: libc::c_uint,
}
pub type clcb_progress =
    Option<unsafe extern "C" fn(size_t, size_t, *mut libc::c_void) -> cl_error_t>;
pub type clcb_file_props = Option<
    unsafe extern "C" fn(*const libc::c_char, libc::c_int, *mut libc::c_void) -> libc::c_int,
>;
pub type clcb_meta = Option<
    unsafe extern "C" fn(
        *const libc::c_char,
        libc::c_ulong,
        *const libc::c_char,
        libc::c_ulong,
        libc::c_int,
        libc::c_uint,
        *mut libc::c_void,
    ) -> cl_error_t,
>;
pub type clcb_hash = Option<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_ulonglong,
        *const libc::c_uchar,
        *const libc::c_char,
        *mut libc::c_void,
    ) -> (),
>;
pub type clcb_sigload = Option<
    unsafe extern "C" fn(
        *const libc::c_char,
        *const libc::c_char,
        libc::c_uint,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type clcb_virus_found =
    Option<unsafe extern "C" fn(libc::c_int, *const libc::c_char, *mut libc::c_void) -> ()>;
pub type clcb_post_scan = Option<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        *const libc::c_char,
        *mut libc::c_void,
    ) -> cl_error_t,
>;
pub type clcb_pre_scan =
    Option<unsafe extern "C" fn(libc::c_int, *const libc::c_char, *mut libc::c_void) -> cl_error_t>;
pub type clcb_pre_cache =
    Option<unsafe extern "C" fn(libc::c_int, *const libc::c_char, *mut libc::c_void) -> cl_error_t>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crtmgr {
    pub crts: *mut cli_crt,
    pub items: libc::c_uint,
}
pub type cli_crt = cli_crt_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_crt_t {
    pub name: *mut libc::c_char,
    pub raw_subject: [uint8_t; 64],
    pub raw_issuer: [uint8_t; 64],
    pub raw_serial: [uint8_t; 64],
    pub subject: [uint8_t; 20],
    pub issuer: [uint8_t; 20],
    pub serial: [uint8_t; 20],
    pub ignore_serial: libc::c_int,
    pub tbshash: [uint8_t; 64],
    pub n: fp_int,
    pub e: fp_int,
    pub sig: fp_int,
    pub not_before: time_t,
    pub not_after: time_t,
    pub hashtype: cli_crt_hashtype,
    pub certSign: libc::c_int,
    pub codeSign: libc::c_int,
    pub timeSign: libc::c_int,
    pub isBlocked: libc::c_int,
    pub prev: *mut cli_crt_t,
    pub next: *mut cli_crt_t,
}
pub type cli_crt_hashtype = libc::c_uint;
pub const CLI_SHA512RSA: cli_crt_hashtype = 7;
pub const CLI_SHA384RSA: cli_crt_hashtype = 6;
pub const CLI_SHA256RSA: cli_crt_hashtype = 5;
pub const CLI_RSA: cli_crt_hashtype = 4;
pub const CLI_MD2RSA: cli_crt_hashtype = 3;
pub const CLI_MD5RSA: cli_crt_hashtype = 2;
pub const CLI_SHA1RSA: cli_crt_hashtype = 1;
pub const CLI_HASHTYPE_ANY: cli_crt_hashtype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fp_int {
    pub dp: [fp_digit; 72],
    pub used: libc::c_int,
    pub sign: libc::c_int,
}
pub type fp_digit = ulong64;
pub type ulong64 = libc::c_ulonglong;
pub type mpool_t = MP;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_dbinfo {
    pub name: *mut libc::c_char,
    pub hash: *mut libc::c_char,
    pub size: size_t,
    pub cvd: *mut cl_cvd,
    pub next: *mut cli_dbinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cl_cvd {
    pub time: *mut libc::c_char,
    pub version: libc::c_uint,
    pub sigs: libc::c_uint,
    pub fl: libc::c_uint,
    pub md5: *mut libc::c_char,
    pub dsig: *mut libc::c_char,
    pub builder: *mut libc::c_char,
    pub stime: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icon_matcher {
    pub group_names: [*mut *mut libc::c_char; 2],
    pub group_counts: [libc::c_uint; 2],
    pub icons: [*mut icomtr; 3],
    pub icon_counts: [libc::c_uint; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icomtr {
    pub group: [libc::c_uint; 2],
    pub color_avg: [libc::c_uint; 3],
    pub color_x: [libc::c_uint; 3],
    pub color_y: [libc::c_uint; 3],
    pub gray_avg: [libc::c_uint; 3],
    pub gray_x: [libc::c_uint; 3],
    pub gray_y: [libc::c_uint; 3],
    pub bright_avg: [libc::c_uint; 3],
    pub bright_x: [libc::c_uint; 3],
    pub bright_y: [libc::c_uint; 3],
    pub dark_avg: [libc::c_uint; 3],
    pub dark_x: [libc::c_uint; 3],
    pub dark_y: [libc::c_uint; 3],
    pub edge_avg: [libc::c_uint; 3],
    pub edge_x: [libc::c_uint; 3],
    pub edge_y: [libc::c_uint; 3],
    pub noedge_avg: [libc::c_uint; 3],
    pub noedge_x: [libc::c_uint; 3],
    pub noedge_y: [libc::c_uint; 3],
    pub rsum: libc::c_uint,
    pub gsum: libc::c_uint,
    pub bsum: libc::c_uint,
    pub ccount: libc::c_uint,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_matcher {
    pub type_0: libc::c_uint,
    pub bm_shift: *mut uint8_t,
    pub bm_suffix: *mut *mut cli_bm_patt,
    pub bm_pattab: *mut *mut cli_bm_patt,
    pub soff: *mut uint32_t,
    pub soff_len: uint32_t,
    pub bm_offmode: uint32_t,
    pub bm_patterns: uint32_t,
    pub bm_reloff_num: uint32_t,
    pub bm_absoff_num: uint32_t,
    pub hm: cli_hash_patt,
    pub hwild: cli_hash_wild,
    pub ac_partsigs: uint32_t,
    pub ac_nodes: uint32_t,
    pub ac_lists: uint32_t,
    pub ac_patterns: uint32_t,
    pub ac_lsigs: uint32_t,
    pub ac_lsigtable: *mut *mut cli_ac_lsig,
    pub ac_root: *mut cli_ac_node,
    pub ac_nodetable: *mut *mut cli_ac_node,
    pub ac_listtable: *mut *mut cli_ac_list,
    pub ac_pattable: *mut *mut cli_ac_patt,
    pub ac_reloff: *mut *mut cli_ac_patt,
    pub ac_reloff_num: uint32_t,
    pub ac_absoff_num: uint32_t,
    pub ac_mindepth: uint8_t,
    pub ac_maxdepth: uint8_t,
    pub filter: *mut filter,
    pub maxpatlen: uint16_t,
    pub ac_only: uint8_t,
    pub pcre_metas: uint32_t,
    pub pcre_metatable: *mut *mut cli_pcre_meta,
    pub pcre_reloff_num: uint32_t,
    pub pcre_absoff_num: uint32_t,
    pub bcomp_metas: uint32_t,
    pub bcomp_metatable: *mut *mut cli_bcomp_meta,
    pub fuzzy_hashmap: fuzzyhashmap_t,
    pub linked_bcs: uint32_t,
    pub mempool: *mut mpool_t,
}
pub type fuzzyhashmap_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_bcomp_meta {
    pub ref_subsigid: uint16_t,
    pub lsigid: [uint32_t; 3],
    pub offset: ssize_t,
    pub options: uint16_t,
    pub byte_len: size_t,
    pub comps: *mut *mut cli_bcomp_comp,
    pub comp_count: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_bcomp_comp {
    pub comp_symbol: libc::c_char,
    pub comp_value: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_pcre_meta {
    pub trigger: *mut libc::c_char,
    pub lsigid: [uint32_t; 3],
    pub pdata: cli_pcre_data,
    pub offdata: [uint32_t; 4],
    pub offset_min: uint32_t,
    pub offset_max: uint32_t,
    pub flags: uint32_t,
    pub statname: *mut libc::c_char,
    pub sigtime_id: uint32_t,
    pub sigmatch_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_pcre_data {
    pub re: *mut pcre2_code_8,
    pub mctx: *mut pcre2_match_context_8,
    pub options: libc::c_int,
    pub expression: *mut libc::c_char,
    pub search_offset: uint32_t,
}
pub type pcre2_match_context_8 = pcre2_real_match_context_8;
pub type pcre2_code_8 = pcre2_real_code_8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_ac_patt {
    pub pattern: *mut uint16_t,
    pub prefix: *mut uint16_t,
    pub length: [uint16_t; 3],
    pub prefix_length: [uint16_t; 3],
    pub mindist: uint32_t,
    pub maxdist: uint32_t,
    pub sigid: uint32_t,
    pub lsigid: [uint32_t; 3],
    pub ch: [uint16_t; 2],
    pub virname: *mut libc::c_char,
    pub customdata: *mut libc::c_void,
    pub ch_mindist: [uint16_t; 2],
    pub ch_maxdist: [uint16_t; 2],
    pub parts: uint16_t,
    pub partno: uint16_t,
    pub special: uint16_t,
    pub special_pattern: uint16_t,
    pub special_table: *mut *mut cli_ac_special,
    pub rtype: uint16_t,
    pub type_0: uint16_t,
    pub offdata: [uint32_t; 4],
    pub offset_min: uint32_t,
    pub offset_max: uint32_t,
    pub boundary: uint32_t,
    pub depth: uint8_t,
    pub sigopts: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_ac_special {
    pub alt: C2RustUnnamed_1,
    pub len: [uint16_t; 2],
    pub num: uint16_t,
    pub type_0: uint16_t,
    pub negative: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub byte: *mut libc::c_uchar,
    pub f_str: *mut *mut libc::c_uchar,
    pub v_str: *mut cli_alt_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_alt_node {
    pub str_0: *mut uint16_t,
    pub len: uint16_t,
    pub unique: uint8_t,
    pub next: *mut cli_alt_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_ac_list {
    pub me: *mut cli_ac_patt,
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub next_same: *mut cli_ac_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub node: *mut cli_ac_node,
    pub next: *mut cli_ac_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_ac_node {
    pub list: *mut cli_ac_list,
    pub trans: *mut *mut cli_ac_node,
    pub fail: *mut cli_ac_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_ac_lsig {
    pub id: uint32_t,
    pub bc_idx: libc::c_uint,
    pub type_0: lsig_type_t,
    pub flag: uint8_t,
    pub u: C2RustUnnamed_3,
    pub virname: *mut libc::c_char,
    pub tdb: cli_lsig_tdb,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_lsig_tdb {
    pub val: *mut uint32_t,
    pub range: *mut uint32_t,
    pub str_0: *mut libc::c_char,
    pub cnt: [tdb_type_t; 3],
    pub subsigs: uint32_t,
    pub target: *const uint32_t,
    pub engine: *const uint32_t,
    pub nos: *const uint32_t,
    pub ep: *const uint32_t,
    pub filesize: *const uint32_t,
    pub container: *const uint32_t,
    pub handlertype: *const uint32_t,
    pub intermediates: *const uint32_t,
    pub icongrp1: *const libc::c_char,
    pub icongrp2: *const libc::c_char,
    pub macro_ptids: *mut uint32_t,
    pub mempool: *mut mpool_t,
}
pub type tdb_type_t = tdb_type;
pub type tdb_type = libc::c_uint;
pub const CLI_TDB_FTYPE_EXPR: tdb_type = 5;
pub const CLI_TDB_FTYPE: tdb_type = 4;
pub const CLI_TDB_RANGE2: tdb_type = 3;
pub const CLI_TDB_STR: tdb_type = 2;
pub const CLI_TDB_RANGE: tdb_type = 1;
pub const CLI_TDB_UINT: tdb_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub logic: *mut libc::c_char,
    pub code_start: *mut uint8_t,
}
pub type lsig_type_t = lsig_type;
pub type lsig_type = libc::c_uint;
pub const CLI_YARA_OFFSET: lsig_type = 2;
pub const CLI_YARA_NORMAL: lsig_type = 1;
pub const CLI_LSIG_NORMAL: lsig_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_hash_wild {
    pub hashes: [cli_sz_hash; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_sz_hash {
    pub hash_array: *mut uint8_t,
    pub virusnames: *mut *const libc::c_char,
    pub items: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_hash_patt {
    pub sizehashes: [cli_htu32; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_htu32 {
    pub htable: *mut cli_htu32_element,
    pub capacity: size_t,
    pub used: size_t,
    pub maxfill: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_htu32_element {
    pub key: uint32_t,
    pub data: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub as_size_t: size_t,
    pub as_ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_bm_patt {
    pub pattern: *mut libc::c_uchar,
    pub prefix: *mut libc::c_uchar,
    pub virname: *mut libc::c_char,
    pub offdata: [uint32_t; 4],
    pub offset_min: uint32_t,
    pub offset_max: uint32_t,
    pub next: *mut cli_bm_patt,
    pub length: uint16_t,
    pub prefix_length: uint16_t,
    pub cnt: uint16_t,
    pub pattern0: libc::c_uchar,
    pub boundary: uint32_t,
    pub filesize: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_pwdb {
    pub name: *mut libc::c_char,
    pub passwd: *mut libc::c_char,
    pub length: uint16_t,
    pub next: *mut cli_pwdb,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_ftype {
    pub type_0: cli_file_t,
    pub offset: uint32_t,
    pub magic: *mut libc::c_uchar,
    pub tname: *mut libc::c_char,
    pub next: *mut cli_ftype,
    pub length: uint16_t,
}
pub type cli_file_t = cli_file;
pub type cli_file = libc::c_uint;
pub const CL_TYPE_IGNORED: cli_file = 581;
pub const CL_TYPE_OTHER: cli_file = 580;
pub const CL_TYPE_LNK: cli_file = 579;
pub const CL_TYPE_MHTML: cli_file = 578;
pub const CL_TYPE_HWPOLE2: cli_file = 577;
pub const CL_TYPE_XML_HWP: cli_file = 576;
pub const CL_TYPE_XML_XL: cli_file = 575;
pub const CL_TYPE_XML_WORD: cli_file = 574;
pub const CL_TYPE_XDP: cli_file = 573;
pub const CL_TYPE_APM: cli_file = 572;
pub const CL_TYPE_GPT: cli_file = 571;
pub const CL_TYPE_DMG: cli_file = 570;
pub const CL_TYPE_ISO9660: cli_file = 569;
pub const CL_TYPE_ISHIELD_MSI: cli_file = 568;
pub const CL_TYPE_AUTOIT: cli_file = 567;
pub const CL_TYPE_NULSFT: cli_file = 566;
pub const CL_TYPE_EGGSFX: cli_file = 565;
pub const CL_TYPE_ARJSFX: cli_file = 564;
pub const CL_TYPE_CABSFX: cli_file = 563;
pub const CL_TYPE_7ZSFX: cli_file = 562;
pub const CL_TYPE_RARSFX: cli_file = 561;
pub const CL_TYPE_ZIPSFX: cli_file = 560;
pub const CL_TYPE_SFX: cli_file = 559;
pub const CL_TYPE_MAIL: cli_file = 558;
pub const CL_TYPE_HTML: cli_file = 557;
pub const CL_TYPE_MBR: cli_file = 556;
pub const CL_TYPE_PART_HFSPLUS: cli_file = 555;
pub const CL_TYPE_PART_ANY: cli_file = 554;
pub const CL_TYPE_EGG: cli_file = 553;
pub const CL_TYPE_PS: cli_file = 552;
pub const CL_TYPE_OOXML_HWP: cli_file = 551;
pub const CL_TYPE_HWP3: cli_file = 550;
pub const CL_TYPE_INTERNAL: cli_file = 549;
pub const CL_TYPE_OOXML_XL: cli_file = 548;
pub const CL_TYPE_OOXML_PPT: cli_file = 547;
pub const CL_TYPE_OOXML_WORD: cli_file = 546;
pub const CL_TYPE_XZ: cli_file = 545;
pub const CL_TYPE_XAR: cli_file = 544;
pub const CL_TYPE_JAVA: cli_file = 543;
pub const CL_TYPE_SWF: cli_file = 542;
pub const CL_TYPE_7Z: cli_file = 541;
pub const CL_TYPE_RTF: cli_file = 540;
pub const CL_TYPE_HTML_UTF16: cli_file = 539;
pub const CL_TYPE_SCRIPT: cli_file = 538;
pub const CL_TYPE_UUENCODED: cli_file = 537;
pub const CL_TYPE_PDF: cli_file = 536;
pub const CL_TYPE_CRYPTFF: cli_file = 535;
pub const CL_TYPE_TNEF: cli_file = 534;
pub const CL_TYPE_BINHEX: cli_file = 533;
pub const CL_TYPE_RIFF: cli_file = 532;
pub const CL_TYPE_TIFF: cli_file = 531;
pub const CL_TYPE_JPEG: cli_file = 530;
pub const CL_TYPE_PNG: cli_file = 529;
pub const CL_TYPE_GIF: cli_file = 528;
pub const CL_TYPE_GRAPHICS: cli_file = 527;
pub const CL_TYPE_SCRENC: cli_file = 526;
pub const CL_TYPE_SIS: cli_file = 525;
pub const CL_TYPE_MSCHM: cli_file = 524;
pub const CL_TYPE_MSCAB: cli_file = 523;
pub const CL_TYPE_MSOLE2: cli_file = 522;
pub const CL_TYPE_MSSZDD: cli_file = 521;
pub const CL_TYPE_ARJ: cli_file = 520;
pub const CL_TYPE_RAR: cli_file = 519;
pub const CL_TYPE_BZ: cli_file = 518;
pub const CL_TYPE_ZIP: cli_file = 517;
pub const CL_TYPE_GZ: cli_file = 516;
pub const CL_TYPE_CPIO_CRC: cli_file = 515;
pub const CL_TYPE_CPIO_NEWC: cli_file = 514;
pub const CL_TYPE_CPIO_ODC: cli_file = 513;
pub const CL_TYPE_CPIO_OLD: cli_file = 512;
pub const CL_TYPE_OLD_TAR: cli_file = 511;
pub const CL_TYPE_POSIX_TAR: cli_file = 510;
pub const CL_TYPE_MACHO_UNIBIN: cli_file = 509;
pub const CL_TYPE_MACHO: cli_file = 508;
pub const CL_TYPE_ELF: cli_file = 507;
pub const CL_TYPE_MSEXE: cli_file = 506;
pub const CL_TYPE_ERROR: cli_file = 505;
pub const CL_TYPE_BINARY_DATA: cli_file = 504;
pub const CL_TYPE_TEXT_UTF16BE: cli_file = 503;
pub const CL_TYPE_TEXT_UTF16LE: cli_file = 502;
pub const CL_TYPE_TEXT_UTF8: cli_file = 501;
pub const CL_TYPE_TEXT_ASCII: cli_file = 500;
pub const CL_TYPE_ANY: cli_file = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_dconf {
    pub pe: uint32_t,
    pub elf: uint32_t,
    pub macho: uint32_t,
    pub archive: uint32_t,
    pub doc: uint32_t,
    pub mail: uint32_t,
    pub other: uint32_t,
    pub phishing: uint32_t,
    pub bytecode: uint32_t,
    pub stats: uint32_t,
    pub pcre: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct phishcheck {
    pub preg_numeric: regex_t,
    pub is_disabled: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regex_t {
    pub re_magic: libc::c_int,
    pub re_nsub: size_t,
    pub re_endp: *const libc::c_char,
    pub re_g: *mut re_guts,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_cdb {
    pub virname: *mut libc::c_char,
    pub ctype: cli_file_t,
    pub name: regex_t,
    pub csize: [size_t; 2],
    pub fsizec: [size_t; 2],
    pub fsizer: [size_t; 2],
    pub encrypted: libc::c_int,
    pub filepos: [libc::c_uint; 2],
    pub res1: libc::c_int,
    pub res2: *mut libc::c_void,
    pub next: *mut cli_cdb,
}
pub type cl_engine_field = libc::c_uint;
pub const CL_ENGINE_PE_DUMPCERTS: cl_engine_field = 35;
pub const CL_ENGINE_DISABLE_PE_CERTS: cl_engine_field = 34;
pub const CL_ENGINE_PCRE_MAX_FILESIZE: cl_engine_field = 33;
pub const CL_ENGINE_PCRE_RECMATCH_LIMIT: cl_engine_field = 32;
pub const CL_ENGINE_PCRE_MATCH_LIMIT: cl_engine_field = 31;
pub const CL_ENGINE_MAX_SCANTIME: cl_engine_field = 30;
pub const CL_ENGINE_MAX_RECHWP3: cl_engine_field = 29;
pub const CL_ENGINE_MAX_ICONSPE: cl_engine_field = 28;
pub const CL_ENGINE_MAX_PARTITIONS: cl_engine_field = 27;
pub const CL_ENGINE_STATS_TIMEOUT: cl_engine_field = 26;
pub const CL_ENGINE_DISABLE_PE_STATS: cl_engine_field = 25;
pub const CL_ENGINE_DISABLE_CACHE: cl_engine_field = 24;
pub const CL_ENGINE_FORCETODISK: cl_engine_field = 23;
pub const CL_ENGINE_MAX_ZIPTYPERCG: cl_engine_field = 22;
pub const CL_ENGINE_MAX_SCRIPTNORMALIZE: cl_engine_field = 21;
pub const CL_ENGINE_MAX_HTMLNOTAGS: cl_engine_field = 20;
pub const CL_ENGINE_MAX_HTMLNORMALIZE: cl_engine_field = 19;
pub const CL_ENGINE_MAX_EMBEDDEDPE: cl_engine_field = 18;
pub const CL_ENGINE_BYTECODE_MODE: cl_engine_field = 17;
pub const CL_ENGINE_BYTECODE_TIMEOUT: cl_engine_field = 16;
pub const CL_ENGINE_BYTECODE_SECURITY: cl_engine_field = 15;
pub const CL_ENGINE_KEEPTMP: cl_engine_field = 14;
pub const CL_ENGINE_TMPDIR: cl_engine_field = 13;
pub const CL_ENGINE_AC_MAXDEPTH: cl_engine_field = 12;
pub const CL_ENGINE_AC_MINDEPTH: cl_engine_field = 11;
pub const CL_ENGINE_AC_ONLY: cl_engine_field = 10;
pub const CL_ENGINE_DB_TIME: cl_engine_field = 9;
pub const CL_ENGINE_DB_VERSION: cl_engine_field = 8;
pub const CL_ENGINE_DB_OPTIONS: cl_engine_field = 7;
pub const CL_ENGINE_PUA_CATEGORIES: cl_engine_field = 6;
pub const CL_ENGINE_MIN_SSN_COUNT: cl_engine_field = 5;
pub const CL_ENGINE_MIN_CC_COUNT: cl_engine_field = 4;
pub const CL_ENGINE_MAX_FILES: cl_engine_field = 3;
pub const CL_ENGINE_MAX_RECURSION: cl_engine_field = 2;
pub const CL_ENGINE_MAX_FILESIZE: cl_engine_field = 1;
pub const CL_ENGINE_MAX_SCANSIZE: cl_engine_field = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cl_fmap {
    pub handle: *mut libc::c_void,
    pub pread_cb: clcb_pread,
    pub data: *const libc::c_void,
    pub mtime: time_t,
    pub pages: uint64_t,
    pub pgsz: uint64_t,
    pub paged: uint64_t,
    pub aging: uint16_t,
    pub dont_cache_flag: uint16_t,
    pub handle_is_fd: uint16_t,
    pub offset: size_t,
    pub nested_offset: size_t,
    pub real_len: size_t,
    pub len: size_t,
    pub unmap: Option<unsafe extern "C" fn(*mut fmap_t) -> ()>,
    pub need: Option<
        unsafe extern "C" fn(*mut fmap_t, size_t, size_t, libc::c_int) -> *const libc::c_void,
    >,
    pub need_offstr:
        Option<unsafe extern "C" fn(*mut fmap_t, size_t, size_t) -> *const libc::c_void>,
    pub gets: Option<
        unsafe extern "C" fn(
            *mut fmap_t,
            *mut libc::c_char,
            *mut size_t,
            size_t,
        ) -> *const libc::c_void,
    >,
    pub unneed_off: Option<unsafe extern "C" fn(*mut fmap_t, size_t, size_t) -> ()>,
    pub have_maphash: bool,
    pub maphash: [libc::c_uchar; 16],
    pub bitmap: *mut uint64_t,
    pub name: *mut libc::c_char,
}
pub type fmap_t = cl_fmap_t;
pub type cl_fmap_t = cl_fmap;
pub type clcb_pread =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, size_t, off_t) -> off_t>;
pub type json_bool = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_ctx_tag {
    pub target_filepath: *mut libc::c_char,
    pub sub_filepath: *const libc::c_char,
    pub sub_tmpdir: *mut libc::c_char,
    pub virname: *mut *const libc::c_char,
    pub num_viruses: libc::c_uint,
    pub scanned: *mut libc::c_ulong,
    pub root: *const cli_matcher,
    pub engine: *const cl_engine,
    pub scansize: uint64_t,
    pub options: *mut cl_scan_options,
    pub scannedfiles: libc::c_uint,
    pub found_possibly_unwanted: libc::c_uint,
    pub corrupted_input: libc::c_uint,
    pub recursion_stack: *mut recursion_level_t,
    pub recursion_stack_size: uint32_t,
    pub recursion_level: uint32_t,
    pub fmap: *mut fmap_t,
    pub next_layer_is_normalized: bool,
    pub handlertype_hash: [libc::c_uchar; 16],
    pub dconf: *mut cli_dconf,
    pub hook_lsig_matches: *mut bitset_t,
    pub cb_ctx: *mut libc::c_void,
    pub perf: *mut cli_events_t,
    pub properties: *mut json_object,
    pub wrkproperty: *mut json_object,
    pub time_limit: timeval,
    pub limit_exceeded: bool,
    pub abort_scan: bool,
}
pub type cli_events_t = cli_events;
pub type bitset_t = bitset_tag;
pub type recursion_level_t = recursion_level_tag;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recursion_level_tag {
    pub type_0: cli_file_t,
    pub size: size_t,
    pub fmap: *mut cl_fmap_t,
    pub recursion_level_buffer: uint32_t,
    pub recursion_level_buffer_fmap: uint32_t,
    pub is_normalized_layer: bool,
    pub image_fuzzy_hash: image_fuzzy_hash_t,
    pub calculated_image_fuzzy_hash: bool,
}
pub type image_fuzzy_hash_t = image_fuzzy_hash;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct image_fuzzy_hash {
    pub hash: [uint8_t; 8],
}
pub type cli_ctx = cli_ctx_tag;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_32 {
    pub una_u32: uint32_t,
    pub una_s32: int32_t,
}
pub type mime_type = libc::c_uint;
pub const MEXTENSION: mime_type = 8;
pub const VIDEO: mime_type = 7;
pub const TEXT: mime_type = 6;
pub const MULTIPART: mime_type = 5;
pub const MESSAGE: mime_type = 4;
pub const IMAGE: mime_type = 3;
pub const AUDIO: mime_type = 2;
pub const APPLICATION: mime_type = 1;
pub const NOMIME: mime_type = 0;
pub type encoding_type = libc::c_uint;
pub const BINHEX: encoding_type = 8;
pub const EEXTENSION: encoding_type = 7;
pub const YENCODE: encoding_type = 6;
pub const UUENCODE: encoding_type = 5;
pub const BINARY: encoding_type = 4;
pub const EIGHTBIT: encoding_type = 3;
pub const BASE64: encoding_type = 2;
pub const QUOTEDPRINTABLE: encoding_type = 1;
pub const NOENCODING: encoding_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tableEntry {
    pub key: *mut libc::c_char,
    pub next: *mut tableEntry,
    pub value: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table {
    pub tableHead: *mut tableEntry,
    pub tableLast: *mut tableEntry,
    pub flags: libc::c_uint,
}
pub type table_t = table;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blob {
    pub name: *mut libc::c_char,
    pub data: *mut libc::c_uchar,
    pub len: off_t,
    pub size: off_t,
    pub isClosed: libc::c_int,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct fileblob {
    pub fp: *mut FILE,
    pub fd: libc::c_int,
    pub b: blob,
    pub fullname: *mut libc::c_char,
    pub ctx: *mut cli_ctx,
    pub bytes_scanned: libc::c_ulong,
    #[bitfield(name = "isNotEmpty", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "isInfected", ty = "libc::c_uint", bits = "1..=1")]
    pub isNotEmpty_isInfected: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type line_t = libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct text {
    pub t_line: *mut line_t,
    pub t_next: *mut text,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct message {
    pub encodingTypes: *mut encoding_type,
    pub mimeType: mime_type,
    pub numberOfEncTypes: libc::c_int,
    pub mimeSubtype: *mut libc::c_char,
    pub mimeArguments: *mut *mut libc::c_char,
    pub mimeDispositionType: *mut libc::c_char,
    pub body_first: *mut text,
    pub body_last: *mut text,
    pub ctx: *mut cli_ctx,
    pub numberOfArguments: size_t,
    pub base64chars: libc::c_int,
    pub bounce: *mut text,
    pub binhex: *mut text,
    pub yenc: *mut text,
    pub encoding: *mut text,
    pub dedupedThisFar: *const text,
    pub base64_1: libc::c_char,
    pub base64_2: libc::c_char,
    pub base64_3: libc::c_char,
    #[bitfield(name = "isInfected", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "isTruncated", ty = "libc::c_uint", bits = "1..=1")]
    pub isInfected_isTruncated: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub jobj: *mut json_object,
}
pub const VIRUS: mbox_status = 3;
pub const MAXFILES: mbox_status = 5;
pub const MAXREC: mbox_status = 4;
pub const FAIL: mbox_status = 0;
pub const OK_ATTACHMENTS_NOT_SAVED: mbox_status = 2;
pub const OK: mbox_status = 1;
pub type mbox_status = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbox_ctx {
    pub dir: *const libc::c_char,
    pub rfc821Table: *const table_t,
    pub subtypeTable: *const table_t,
    pub ctx: *mut cli_ctx,
    pub files: libc::c_uint,
    pub wrkobj: *mut json_object,
}
pub type tag_arguments_t = tag_arguments_tag;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tag_arguments_tag {
    pub count: libc::c_int,
    pub scanContents: libc::c_int,
    pub tag: *mut *mut libc::c_uchar,
    pub value: *mut *mut libc::c_uchar,
    pub contents: *mut *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tableinit {
    pub key: *const libc::c_char,
    pub value: libc::c_int,
}
pub type htmlDocPtr = xmlDocPtr;
pub type xmlDocPtr = *mut xmlDoc;
pub type xmlDoc = _xmlDoc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDoc {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *mut libc::c_char,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub compression: libc::c_int,
    pub standalone: libc::c_int,
    pub intSubset: *mut _xmlDtd,
    pub extSubset: *mut _xmlDtd,
    pub oldNs: *mut _xmlNs,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub ids: *mut libc::c_void,
    pub refs: *mut libc::c_void,
    pub URL: *const xmlChar,
    pub charset: libc::c_int,
    pub dict: *mut _xmlDict,
    pub psvi: *mut libc::c_void,
    pub parseFlags: libc::c_int,
    pub properties: libc::c_int,
}
pub type xmlChar = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNs {
    pub next: *mut _xmlNs,
    pub type_0: xmlNsType,
    pub href: *const xmlChar,
    pub prefix: *const xmlChar,
    pub _private: *mut libc::c_void,
    pub context: *mut _xmlDoc,
}
pub type xmlElementType = xmlNsType;
pub type xmlNsType = libc::c_uint;
pub const XML_DOCB_DOCUMENT_NODE: xmlNsType = 21;
pub const XML_XINCLUDE_END: xmlNsType = 20;
pub const XML_XINCLUDE_START: xmlNsType = 19;
pub const XML_NAMESPACE_DECL: xmlNsType = 18;
pub const XML_ENTITY_DECL: xmlNsType = 17;
pub const XML_ATTRIBUTE_DECL: xmlNsType = 16;
pub const XML_ELEMENT_DECL: xmlNsType = 15;
pub const XML_DTD_NODE: xmlNsType = 14;
pub const XML_HTML_DOCUMENT_NODE: xmlNsType = 13;
pub const XML_NOTATION_NODE: xmlNsType = 12;
pub const XML_DOCUMENT_FRAG_NODE: xmlNsType = 11;
pub const XML_DOCUMENT_TYPE_NODE: xmlNsType = 10;
pub const XML_DOCUMENT_NODE: xmlNsType = 9;
pub const XML_COMMENT_NODE: xmlNsType = 8;
pub const XML_PI_NODE: xmlNsType = 7;
pub const XML_ENTITY_NODE: xmlNsType = 6;
pub const XML_ENTITY_REF_NODE: xmlNsType = 5;
pub const XML_CDATA_SECTION_NODE: xmlNsType = 4;
pub const XML_TEXT_NODE: xmlNsType = 3;
pub const XML_ATTRIBUTE_NODE: xmlNsType = 2;
pub const XML_ELEMENT_NODE: xmlNsType = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDtd {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDoc,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub notations: *mut libc::c_void,
    pub elements: *mut libc::c_void,
    pub attributes: *mut libc::c_void,
    pub entities: *mut libc::c_void,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub pentities: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNode {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub content: *mut xmlChar,
    pub properties: *mut _xmlAttr,
    pub nsDef: *mut xmlNs,
    pub psvi: *mut libc::c_void,
    pub line: libc::c_ushort,
    pub extra: libc::c_ushort,
}
pub type xmlNs = _xmlNs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlAttr {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlAttr,
    pub prev: *mut _xmlAttr,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub atype: xmlAttributeType,
    pub psvi: *mut libc::c_void,
}
pub type xmlAttributeType = libc::c_uint;
pub const XML_ATTRIBUTE_NOTATION: xmlAttributeType = 10;
pub const XML_ATTRIBUTE_ENUMERATION: xmlAttributeType = 9;
pub const XML_ATTRIBUTE_NMTOKENS: xmlAttributeType = 8;
pub const XML_ATTRIBUTE_NMTOKEN: xmlAttributeType = 7;
pub const XML_ATTRIBUTE_ENTITIES: xmlAttributeType = 6;
pub const XML_ATTRIBUTE_ENTITY: xmlAttributeType = 5;
pub const XML_ATTRIBUTE_IDREFS: xmlAttributeType = 4;
pub const XML_ATTRIBUTE_IDREF: xmlAttributeType = 3;
pub const XML_ATTRIBUTE_ID: xmlAttributeType = 2;
pub const XML_ATTRIBUTE_CDATA: xmlAttributeType = 1;
pub type xmlTextReaderPtr = *mut xmlTextReader;
pub type xmlTextReader = _xmlTextReader;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msxml_ctx {
    pub scan_cb: msxml_scan_cb,
    pub scan_data: *mut libc::c_void,
    pub comment_cb: msxml_comment_cb,
    pub comment_data: *mut libc::c_void,
    pub ictx: *mut msxml_ictx,
}
pub type msxml_comment_cb = Option<
    unsafe extern "C" fn(
        *const libc::c_char,
        *mut cli_ctx,
        *mut libc::c_void,
        *mut libc::c_void,
    ) -> cl_error_t,
>;
pub type msxml_scan_cb = Option<
    unsafe extern "C" fn(
        libc::c_int,
        *const libc::c_char,
        *mut cli_ctx,
        libc::c_int,
        *mut attrib_entry,
        *mut libc::c_void,
    ) -> cl_error_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct attrib_entry {
    pub key: *const libc::c_char,
    pub value: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_entry {
    pub key: *const libc::c_char,
    pub name: *const libc::c_char,
    pub type_0: uint32_t,
}
pub const XML_PARSE_NONET: C2RustUnnamed_5 = 2048;
pub const XML_PARSE_NOERROR: C2RustUnnamed_5 = 32;
pub const HTML_PARSE_NOWARNING: C2RustUnnamed_6 = 64;
pub type ReadStruct = _ReadStruct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ReadStruct {
    pub buffer: [libc::c_char; 1025],
    pub bufferLen: size_t,
    pub next: *mut _ReadStruct,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const XML_PARSE_BIG_LINES: C2RustUnnamed_5 = 4194304;
pub const XML_PARSE_IGNORE_ENC: C2RustUnnamed_5 = 2097152;
pub const XML_PARSE_OLDSAX: C2RustUnnamed_5 = 1048576;
pub const XML_PARSE_HUGE: C2RustUnnamed_5 = 524288;
pub const XML_PARSE_NOBASEFIX: C2RustUnnamed_5 = 262144;
pub const XML_PARSE_OLD10: C2RustUnnamed_5 = 131072;
pub const XML_PARSE_COMPACT: C2RustUnnamed_5 = 65536;
pub const XML_PARSE_NOXINCNODE: C2RustUnnamed_5 = 32768;
pub const XML_PARSE_NOCDATA: C2RustUnnamed_5 = 16384;
pub const XML_PARSE_NSCLEAN: C2RustUnnamed_5 = 8192;
pub const XML_PARSE_NODICT: C2RustUnnamed_5 = 4096;
pub const XML_PARSE_XINCLUDE: C2RustUnnamed_5 = 1024;
pub const XML_PARSE_SAX1: C2RustUnnamed_5 = 512;
pub const XML_PARSE_NOBLANKS: C2RustUnnamed_5 = 256;
pub const XML_PARSE_PEDANTIC: C2RustUnnamed_5 = 128;
pub const XML_PARSE_NOWARNING: C2RustUnnamed_5 = 64;
pub const XML_PARSE_DTDVALID: C2RustUnnamed_5 = 16;
pub const XML_PARSE_DTDATTR: C2RustUnnamed_5 = 8;
pub const XML_PARSE_DTDLOAD: C2RustUnnamed_5 = 4;
pub const XML_PARSE_NOENT: C2RustUnnamed_5 = 2;
pub const XML_PARSE_RECOVER: C2RustUnnamed_5 = 1;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const HTML_PARSE_IGNORE_ENC: C2RustUnnamed_6 = 2097152;
pub const HTML_PARSE_COMPACT: C2RustUnnamed_6 = 65536;
pub const HTML_PARSE_NOIMPLIED: C2RustUnnamed_6 = 8192;
pub const HTML_PARSE_NONET: C2RustUnnamed_6 = 2048;
pub const HTML_PARSE_NOBLANKS: C2RustUnnamed_6 = 256;
pub const HTML_PARSE_PEDANTIC: C2RustUnnamed_6 = 128;
pub const HTML_PARSE_NOERROR: C2RustUnnamed_6 = 32;
pub const HTML_PARSE_NODEFDTD: C2RustUnnamed_6 = 4;
pub const HTML_PARSE_RECOVER: C2RustUnnamed_6 = 1;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn fstat(mut __fd: libc::c_int, mut __statbuf: *mut stat) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn fmap_need_off_once(
    m: *mut fmap_t,
    at: size_t,
    len: size_t,
) -> *const libc::c_void {
    return ((*m).need).expect("non-null function pointer")(m, at, len, 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn fmap_gets(
    m: *mut fmap_t,
    dst: *mut libc::c_char,
    at: *mut size_t,
    max_len: size_t,
) -> *const libc::c_void {
    return ((*m).gets).expect("non-null function pointer")(m, dst, at, max_len);
}
static mut rfc821headers: [tableinit; 4] = [
    {
        let init = tableinit {
            key: b"Content-Type\0" as *const u8 as *const libc::c_char,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"Content-Transfer-Encoding\0" as *const u8 as *const libc::c_char,
            value: 2 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"Content-Disposition\0" as *const u8 as *const libc::c_char,
            value: 3 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: 0 as *const libc::c_char,
            value: 0 as libc::c_int,
        };
        init
    },
];
static mut mimeSubtypes: [tableinit; 20] = [
    {
        let init = tableinit {
            key: b"plain\0" as *const u8 as *const libc::c_char,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"enriched\0" as *const u8 as *const libc::c_char,
            value: 2 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"html\0" as *const u8 as *const libc::c_char,
            value: 3 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"richtext\0" as *const u8 as *const libc::c_char,
            value: 4 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"mixed\0" as *const u8 as *const libc::c_char,
            value: 5 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"alternative\0" as *const u8 as *const libc::c_char,
            value: 6 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"digest\0" as *const u8 as *const libc::c_char,
            value: 7 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"signed\0" as *const u8 as *const libc::c_char,
            value: 8 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"parallel\0" as *const u8 as *const libc::c_char,
            value: 9 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"related\0" as *const u8 as *const libc::c_char,
            value: 10 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"report\0" as *const u8 as *const libc::c_char,
            value: 11 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"appledouble\0" as *const u8 as *const libc::c_char,
            value: 12 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"fax-message\0" as *const u8 as *const libc::c_char,
            value: 5 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"encrypted\0" as *const u8 as *const libc::c_char,
            value: 13 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"x-bfile\0" as *const u8 as *const libc::c_char,
            value: 10 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"knowbot\0" as *const u8 as *const libc::c_char,
            value: 14 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"knowbot-metadata\0" as *const u8 as *const libc::c_char,
            value: 14 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"knowbot-code\0" as *const u8 as *const libc::c_char,
            value: 14 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"knowbot-state\0" as *const u8 as *const libc::c_char,
            value: 14 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: 0 as *const libc::c_char,
            value: 0 as libc::c_int,
        };
        init
    },
];
static mut mimeTypeStr: [tableinit; 10] = [
    {
        let init = tableinit {
            key: b"NOMIME\0" as *const u8 as *const libc::c_char,
            value: NOMIME as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"APPLICATION\0" as *const u8 as *const libc::c_char,
            value: APPLICATION as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"AUDIO\0" as *const u8 as *const libc::c_char,
            value: AUDIO as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"IMAGE\0" as *const u8 as *const libc::c_char,
            value: IMAGE as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"MESSAGE\0" as *const u8 as *const libc::c_char,
            value: MESSAGE as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"MULTIPART\0" as *const u8 as *const libc::c_char,
            value: MULTIPART as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"TEXT\0" as *const u8 as *const libc::c_char,
            value: TEXT as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"VIDEO\0" as *const u8 as *const libc::c_char,
            value: VIDEO as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"MEXTENSION\0" as *const u8 as *const libc::c_char,
            value: MEXTENSION as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: 0 as *const libc::c_char,
            value: 0 as libc::c_int,
        };
        init
    },
];
static mut encTypeStr: [tableinit; 10] = [
    {
        let init = tableinit {
            key: b"NOENCODING\0" as *const u8 as *const libc::c_char,
            value: NOENCODING as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"QUOTEDPRINTABLE\0" as *const u8 as *const libc::c_char,
            value: QUOTEDPRINTABLE as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"BASE64\0" as *const u8 as *const libc::c_char,
            value: BASE64 as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"EIGHTBIT\0" as *const u8 as *const libc::c_char,
            value: EIGHTBIT as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"BINARY\0" as *const u8 as *const libc::c_char,
            value: BINARY as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"UUENCODE\0" as *const u8 as *const libc::c_char,
            value: UUENCODE as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"YENCODE\0" as *const u8 as *const libc::c_char,
            value: YENCODE as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"EEXTENSION\0" as *const u8 as *const libc::c_char,
            value: EEXTENSION as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: b"BINHEX\0" as *const u8 as *const libc::c_char,
            value: BINHEX as libc::c_int,
        };
        init
    },
    {
        let init = tableinit {
            key: 0 as *const libc::c_char,
            value: 0 as libc::c_int,
        };
        init
    },
];
static mut tables_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut rfc821: *mut table_t = 0 as *const table_t as *mut table_t;
static mut subtype: *mut table_t = 0 as *const table_t as *mut table_t;
#[no_mangle]
pub unsafe extern "C" fn cli_mbox(dir: *const libc::c_char, ctx: *mut cli_ctx) -> libc::c_int {
    if dir.is_null() {
        cli_dbgmsg(b"cli_mbox called with NULL dir\n\0" as *const u8 as *const libc::c_char);
        return CL_ENULLARG as libc::c_int;
    }
    return cli_parse_mbox(dir, ctx);
}
unsafe extern "C" fn cli_parse_mbox(
    dir: *const libc::c_char,
    mut ctx: *mut cli_ctx,
) -> libc::c_int {
    let mut retcode: libc::c_int = 0;
    let mut body: *mut message = 0 as *mut message;
    let mut buffer: [libc::c_char; 1001] = [0; 1001];
    let mut mctx: mbox_ctx = mbox_ctx {
        dir: 0 as *const libc::c_char,
        rfc821Table: 0 as *const table_t,
        subtypeTable: 0 as *const table_t,
        ctx: 0 as *mut cli_ctx,
        files: 0,
        wrkobj: 0 as *mut json_object,
    };
    let mut at: size_t = 0 as libc::c_int as size_t;
    let map: *mut fmap_t = (*ctx).fmap;
    cli_dbgmsg(b"in mbox()\n\0" as *const u8 as *const libc::c_char);
    if (fmap_gets(
        map,
        buffer.as_mut_ptr(),
        &mut at,
        (::std::mem::size_of::<[libc::c_char; 1001]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ))
    .is_null()
    {
        return CL_CLEAN as libc::c_int;
    }
    pthread_mutex_lock(&mut tables_mutex);
    if initialiseTables(&mut rfc821, &mut subtype) < 0 as libc::c_int {
        pthread_mutex_unlock(&mut tables_mutex);
        return CL_EMEM as libc::c_int;
    }
    pthread_mutex_unlock(&mut tables_mutex);
    retcode = CL_SUCCESS as libc::c_int;
    body = 0 as *mut message;
    mctx.dir = dir;
    mctx.rfc821Table = rfc821;
    mctx.subtypeTable = subtype;
    mctx.ctx = ctx;
    mctx.files = 0 as libc::c_int as libc::c_uint;
    mctx.wrkobj = (*ctx).wrkproperty;
    if strncmp(
        buffer.as_mut_ptr(),
        b"From \0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        let mut lastLineWasEmpty: bool = false;
        let mut messagenumber: libc::c_int = 0;
        let mut m: *mut message = messageCreate();
        if m.is_null() {
            return CL_EMEM as libc::c_int;
        }
        lastLineWasEmpty = 0 as libc::c_int != 0;
        messagenumber = 1 as libc::c_int;
        messageSetCTX(m, ctx);
        let mut current_block_43: u64;
        loop {
            cli_chomp(buffer.as_mut_ptr());
            if lastLineWasEmpty as libc::c_int != 0
                && strncmp(
                    buffer.as_mut_ptr(),
                    b"From \0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                let fresh0 = messagenumber;
                messagenumber = messagenumber + 1;
                cli_dbgmsg(
                    b"Deal with message number %d\n\0" as *const u8 as *const libc::c_char,
                    fresh0,
                );
                let mut heuristicFound: bool = 0 as libc::c_int != 0;
                body = parseEmailHeaders(m, rfc821, &mut heuristicFound);
                if body.is_null() {
                    messageReset(m);
                    messageSetCTX(m, ctx);
                    if heuristicFound {
                        retcode = CL_VIRUS as libc::c_int;
                        break;
                    } else {
                        current_block_43 = 6669252993407410313;
                    }
                } else {
                    messageSetCTX(body, ctx);
                    messageDestroy(m);
                    if !(messageGetBody(body)).is_null() {
                        let rc: mbox_status = parseEmailBody(
                            body,
                            0 as *mut text,
                            &mut mctx,
                            0 as libc::c_int as libc::c_uint,
                        );
                        if rc as libc::c_uint == FAIL as libc::c_int as libc::c_uint {
                            m = body;
                            messageReset(m);
                            messageSetCTX(m, ctx);
                            current_block_43 = 6669252993407410313;
                        } else if rc as libc::c_uint == VIRUS as libc::c_int as libc::c_uint {
                            cli_dbgmsg(
                                b"Message number %d is infected\n\0" as *const u8
                                    as *const libc::c_char,
                                messagenumber - 1 as libc::c_int,
                            );
                            retcode = CL_VIRUS as libc::c_int;
                            m = 0 as *mut message;
                            break;
                        } else {
                            current_block_43 = 1924505913685386279;
                        }
                    } else {
                        current_block_43 = 1924505913685386279;
                    }
                    match current_block_43 {
                        6669252993407410313 => {}
                        _ => {
                            m = body;
                            messageReset(m);
                            messageSetCTX(m, ctx);
                            cli_dbgmsg(
                                b"Finished processing message\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            current_block_43 = 7018308795614528254;
                        }
                    }
                }
            } else {
                lastLineWasEmpty = buffer[0 as libc::c_int as usize] as libc::c_int == '\0' as i32;
                current_block_43 = 7018308795614528254;
            }
            match current_block_43 {
                7018308795614528254 => {
                    if isuuencodebegin(buffer.as_mut_ptr()) != 0 {
                        if uudecodeFile(m, buffer.as_mut_ptr(), dir, map, &mut at)
                            < 0 as libc::c_int
                        {
                            if messageAddStr(m, buffer.as_mut_ptr()) < 0 as libc::c_int {
                                break;
                            }
                        }
                    } else if messageAddStr(m, buffer.as_mut_ptr()) < 0 as libc::c_int {
                        break;
                    }
                }
                _ => {}
            }
            if (fmap_gets(
                map,
                buffer.as_mut_ptr(),
                &mut at,
                (::std::mem::size_of::<[libc::c_char; 1001]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ))
            .is_null()
            {
                break;
            }
        }
        if retcode == CL_SUCCESS as libc::c_int {
            cli_dbgmsg(
                b"Extract attachments from email %d\n\0" as *const u8 as *const libc::c_char,
                messagenumber,
            );
            let mut heuristicFound_0: bool = 0 as libc::c_int != 0;
            body = parseEmailHeaders(m, rfc821, &mut heuristicFound_0);
            if heuristicFound_0 {
                retcode = CL_VIRUS as libc::c_int;
            }
        }
        if !m.is_null() {
            messageDestroy(m);
        }
    } else {
        if strncmp(
            buffer.as_mut_ptr(),
            b"P I \0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            while !(fmap_gets(
                map,
                buffer.as_mut_ptr(),
                &mut at,
                (::std::mem::size_of::<[libc::c_char; 1001]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ))
            .is_null()
                && (strchr(
                    b"\r\n\0" as *const u8 as *const libc::c_char,
                    buffer[0 as libc::c_int as usize] as libc::c_int,
                ))
                .is_null()
            {}
        }
        while !(strchr(
            b"\r\n\0" as *const u8 as *const libc::c_char,
            buffer[0 as libc::c_int as usize] as libc::c_int,
        ))
        .is_null()
            && !(getline_from_mbox(
                buffer.as_mut_ptr(),
                (::std::mem::size_of::<[libc::c_char; 1001]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                map,
                &mut at,
            ))
            .is_null()
        {}
        buffer[(::std::mem::size_of::<[libc::c_char; 1001]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize] =
            '\0' as i32 as libc::c_char;
        let mut heuristicFound_1: bool = 0 as libc::c_int != 0;
        body = parseEmailFile(
            map,
            &mut at,
            rfc821,
            buffer.as_mut_ptr(),
            dir,
            ctx,
            &mut heuristicFound_1,
        );
        if heuristicFound_1 {
            retcode = CL_VIRUS as libc::c_int;
        }
    }
    if !body.is_null() {
        if retcode == CL_SUCCESS as libc::c_int && !(messageGetBody(body)).is_null() {
            messageSetCTX(body, ctx);
            match parseEmailBody(
                body,
                0 as *mut text,
                &mut mctx,
                0 as libc::c_int as libc::c_uint,
            ) as libc::c_uint
            {
                0 => {
                    retcode = CL_EFORMAT as libc::c_int;
                }
                4 => {
                    retcode = CL_EMAXREC as libc::c_int;
                    cli_append_virus_if_heur_exceedsmax(
                        ctx,
                        b"Heuristics.Limits.Exceeded.MaxRecursion\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                5 => {
                    retcode = CL_EMAXFILES as libc::c_int;
                    cli_append_virus_if_heur_exceedsmax(
                        ctx,
                        b"Heuristics.Limits.Exceeded.MaxFiles\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
                3 => {
                    retcode = CL_VIRUS as libc::c_int;
                }
                1 | 2 | _ => {}
            }
        }
        if (*body).isTruncated() as libc::c_int != 0 && retcode == CL_SUCCESS as libc::c_int {
            retcode = CL_EMEM as libc::c_int;
        }
        messageDestroy(body);
    }
    if retcode == CL_CLEAN as libc::c_int
        && (*ctx).found_possibly_unwanted != 0
        && ((*(*ctx).virname).is_null()
            || (*(*ctx).options).general & 0x1 as libc::c_int as libc::c_uint != 0)
    {
        retcode = cli_append_virus(
            ctx,
            b"Heuristics.Phishing.Email\0" as *const u8 as *const libc::c_char,
        ) as libc::c_int;
        (*ctx).found_possibly_unwanted = 0 as libc::c_int as libc::c_uint;
    }
    cli_dbgmsg(
        b"cli_mbox returning %d\n\0" as *const u8 as *const libc::c_char,
        retcode,
    );
    return retcode;
}
unsafe extern "C" fn appendReadStruct(
    mut rs: *mut ReadStruct,
    buffer: *const libc::c_char,
) -> *mut ReadStruct {
    let mut spaceLeft: size_t = 0;
    if rs.is_null() {
        cli_dbgmsg(b"appendReadStruct: Invalid argument\n\0" as *const u8 as *const libc::c_char);
    } else {
        spaceLeft = (1024 as libc::c_int as libc::c_ulong).wrapping_sub((*rs).bufferLen);
        if strlen(buffer) > spaceLeft {
            let mut next: *mut ReadStruct = 0 as *mut ReadStruct;
            let part: libc::c_int = spaceLeft as libc::c_int;
            strncpy(
                &mut *((*rs).buffer).as_mut_ptr().offset((*rs).bufferLen as isize),
                buffer,
                part as libc::c_ulong,
            );
            let ref mut fresh1 = (*rs).bufferLen;
            *fresh1 =
                (*fresh1 as libc::c_ulong).wrapping_add(part as libc::c_ulong) as size_t as size_t;
            next = cli_calloc(
                1 as libc::c_int as size_t,
                ::std::mem::size_of::<ReadStruct>() as libc::c_ulong,
            ) as *mut ReadStruct;
            if !next.is_null() {
                let ref mut fresh2 = (*rs).next;
                *fresh2 = next;
                strcpy(
                    ((*next).buffer).as_mut_ptr(),
                    &*buffer.offset(part as isize),
                );
                (*next).bufferLen = strlen(&*buffer.offset(part as isize));
                rs = next;
            }
        } else {
            strcpy(
                &mut *((*rs).buffer).as_mut_ptr().offset((*rs).bufferLen as isize),
                buffer,
            );
            let ref mut fresh3 = (*rs).bufferLen;
            *fresh3 = (*fresh3 as libc::c_ulong).wrapping_add(strlen(buffer)) as size_t as size_t;
        }
    }
    return rs;
}
unsafe extern "C" fn getMallocedBufferFromList(head: *const ReadStruct) -> *mut libc::c_char {
    let mut rs: *const ReadStruct = head;
    let mut bufferLen: libc::c_int = 1 as libc::c_int;
    let mut working: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    while !rs.is_null() {
        bufferLen = (bufferLen as libc::c_ulong).wrapping_add((*rs).bufferLen) as libc::c_int
            as libc::c_int;
        rs = (*rs).next;
    }
    working = malloc(bufferLen as libc::c_ulong) as *mut libc::c_char;
    if !working.is_null() {
        rs = head;
        bufferLen = 0 as libc::c_int;
        while !rs.is_null() {
            memcpy(
                &mut *working.offset(bufferLen as isize) as *mut libc::c_char as *mut libc::c_void,
                ((*rs).buffer).as_ptr() as *const libc::c_void,
                (*rs).bufferLen,
            );
            bufferLen = (bufferLen as libc::c_ulong).wrapping_add((*rs).bufferLen) as libc::c_int
                as libc::c_int;
            *working.offset(bufferLen as isize) = 0 as libc::c_int as libc::c_char;
            rs = (*rs).next;
        }
        ret = working;
    }
    if ret.is_null() {
        if !working.is_null() {
            free(working as *mut libc::c_void);
            working = 0 as *mut libc::c_char;
        }
    }
    return ret;
}
unsafe extern "C" fn freeList(mut head: *mut ReadStruct) {
    while !head.is_null() {
        let rs: *mut ReadStruct = (*head).next;
        if !head.is_null() {
            free(head as *mut libc::c_void);
            head = 0 as *mut ReadStruct;
        }
        head = rs;
    }
}
unsafe extern "C" fn doContinueMultipleEmptyOptions(
    line: *const libc::c_char,
    lastWasOnlySemi: *mut bool,
) -> bool {
    if !line.is_null() {
        let mut i: size_t = 0 as libc::c_int as size_t;
        let mut doCont: libc::c_int = 1 as libc::c_int;
        while i < strlen(line) {
            if !(*(*__ctype_b_loc()).offset(*line.offset(i as isize) as libc::c_int as isize)
                as libc::c_int
                & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
                != 0)
            {
                if !(';' as i32 == *line.offset(i as isize) as libc::c_int) {
                    doCont = 0 as libc::c_int;
                    break;
                }
            }
            i = i.wrapping_add(1);
        }
        if 1 as libc::c_int == doCont {
            if *lastWasOnlySemi {
                return 1 as libc::c_int != 0;
            }
            *lastWasOnlySemi = 1 as libc::c_int != 0;
        } else {
            *lastWasOnlySemi = 0 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn hitLineFoldCnt(
    line: *const libc::c_char,
    lineFoldCnt: *mut size_t,
    ctx: *mut cli_ctx,
    heuristicFound: *mut bool,
) -> bool {
    if !line.is_null() {
        if *(*__ctype_b_loc())
            .offset(*line.offset(0 as libc::c_int as isize) as libc::c_int as isize)
            as libc::c_int
            & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            *lineFoldCnt = (*lineFoldCnt).wrapping_add(1);
        } else {
            *lineFoldCnt = 0 as libc::c_int as size_t;
        }
        if *lineFoldCnt >= (256 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong {
            if (*(*ctx).options).heuristic & 0x4 as libc::c_int as libc::c_uint != 0 {
                cli_append_virus(
                    ctx,
                    b"Heuristics.Limits.Exceeded.EmailLineFoldCnt\0" as *const u8
                        as *const libc::c_char,
                );
                *heuristicFound = 1 as libc::c_int != 0;
            }
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn haveTooManyHeaderBytes(
    totalLen: size_t,
    ctx: *mut cli_ctx,
    heuristicFound: *mut bool,
) -> bool {
    if totalLen > (1024 as libc::c_int * 256 as libc::c_int) as libc::c_ulong {
        if (*(*ctx).options).heuristic & 0x4 as libc::c_int as libc::c_uint != 0 {
            cli_append_virus(
                ctx,
                b"Heuristics.Limits.Exceeded.EmailHeaderBytes\0" as *const u8
                    as *const libc::c_char,
            );
            *heuristicFound = 1 as libc::c_int != 0;
        }
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn haveTooManyEmailHeaders(
    totalHeaderCnt: size_t,
    ctx: *mut cli_ctx,
    heuristicFound: *mut bool,
) -> bool {
    if totalHeaderCnt > 1024 as libc::c_int as libc::c_ulong {
        if (*(*ctx).options).heuristic & 0x4 as libc::c_int as libc::c_uint != 0 {
            cli_append_virus(
                ctx,
                b"Heuristics.Limits.Exceeded.EmailHeaders\0" as *const u8 as *const libc::c_char,
            );
            *heuristicFound = 1 as libc::c_int != 0;
        }
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn haveTooManyMIMEPartsPerMessage(
    mimePartCnt: size_t,
    ctx: *mut cli_ctx,
    rc: *mut mbox_status,
) -> bool {
    if mimePartCnt >= 1024 as libc::c_int as libc::c_ulong {
        if (*(*ctx).options).heuristic & 0x4 as libc::c_int as libc::c_uint != 0 {
            cli_append_virus(
                ctx,
                b"Heuristics.Limits.Exceeded.EmailMIMEPartsPerMessage\0" as *const u8
                    as *const libc::c_char,
            );
            *rc = VIRUS;
        }
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn haveTooManyMIMEArguments(
    argCnt: size_t,
    ctx: *mut cli_ctx,
    heuristicFound: *mut bool,
) -> bool {
    if argCnt >= 256 as libc::c_int as libc::c_ulong {
        if (*(*ctx).options).heuristic & 0x4 as libc::c_int as libc::c_uint != 0 {
            cli_append_virus(
                ctx,
                b"Heuristics.Limits.Exceeded.EmailMIMEArguments\0" as *const u8
                    as *const libc::c_char,
            );
            *heuristicFound = 1 as libc::c_int != 0;
        }
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn parseEmailFile(
    map: *mut fmap_t,
    at: *mut size_t,
    rfc821_0: *const table_t,
    firstLine: *const libc::c_char,
    dir: *const libc::c_char,
    ctx: *mut cli_ctx,
    heuristicFound: *mut bool,
) -> *mut message {
    let mut current_block: u64;
    let mut inHeader: bool = 1 as libc::c_int != 0;
    let mut bodyIsEmpty: bool = 1 as libc::c_int != 0;
    let mut lastWasBlank: bool = 0 as libc::c_int != 0;
    let mut lastBodyLineWasBlank: bool = 0 as libc::c_int != 0;
    let mut ret: *mut message = 0 as *mut message;
    let mut anyHeadersFound: bool = 0 as libc::c_int != 0;
    let mut commandNumber: libc::c_int = -(1 as libc::c_int);
    let mut boundary: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buffer: [libc::c_char; 1001] = [0; 1001];
    let mut lastWasOnlySemi: bool = 0 as libc::c_int != 0;
    let mut err: libc::c_int = 1 as libc::c_int;
    let mut totalHeaderBytes: size_t = 0 as libc::c_int as size_t;
    let mut totalHeaderCnt: size_t = 0 as libc::c_int as size_t;
    let mut lineFoldCnt: size_t = 0 as libc::c_int as size_t;
    *heuristicFound = 0 as libc::c_int != 0;
    let mut head: *mut ReadStruct = 0 as *mut ReadStruct;
    let mut curr: *mut ReadStruct = 0 as *mut ReadStruct;
    cli_dbgmsg(b"parseEmailFile\n\0" as *const u8 as *const libc::c_char);
    ret = messageCreate();
    if ret.is_null() {
        return 0 as *mut message;
    }
    head = cli_calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<ReadStruct>() as libc::c_ulong,
    ) as *mut ReadStruct;
    if !head.is_null() {
        curr = head;
        strncpy(
            buffer.as_mut_ptr(),
            firstLine,
            (::std::mem::size_of::<[libc::c_char; 1001]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        loop {
            let mut line: *const libc::c_char = 0 as *const libc::c_char;
            cli_chomp(buffer.as_mut_ptr());
            if buffer[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
                line = 0 as *const libc::c_char;
            } else {
                line = buffer.as_mut_ptr();
            }
            if !doContinueMultipleEmptyOptions(line, &mut lastWasOnlySemi) {
                if hitLineFoldCnt(line, &mut lineFoldCnt, ctx, heuristicFound) {
                    current_block = 14557636130817708122;
                    break;
                }
                if lastWasBlank {
                    lastWasBlank = 0 as libc::c_int != 0;
                    if boundaryStart(buffer.as_mut_ptr(), boundary) != 0 {
                        cli_dbgmsg(
                            b"Found a header line with space that should be blank\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        inHeader = 0 as libc::c_int != 0;
                    }
                }
                if inHeader {
                    cli_dbgmsg(
                        b"parseEmailFile: check '%s'\n\0" as *const u8 as *const libc::c_char,
                        buffer.as_mut_ptr(),
                    );
                    if !line.is_null()
                        && *(*__ctype_b_loc()).offset(
                            (*line.offset(0 as libc::c_int as isize) as libc::c_int
                                & 0xff as libc::c_int) as isize,
                        ) as libc::c_int
                            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                    {
                        let mut copy: [libc::c_char; 1001] = [0; 1001];
                        strcpy(copy.as_mut_ptr(), buffer.as_mut_ptr());
                        strstrip(copy.as_mut_ptr());
                        if copy[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
                            if (*head).bufferLen != 0 {
                                let mut header: *mut libc::c_char = getMallocedBufferFromList(head);
                                let mut needContinue: libc::c_int = 0 as libc::c_int;
                                if header.is_null() {
                                    current_block = 16850746923508545940;
                                    break;
                                }
                                totalHeaderCnt = totalHeaderCnt.wrapping_add(1);
                                if haveTooManyEmailHeaders(totalHeaderCnt, ctx, heuristicFound) {
                                    if !header.is_null() {
                                        free(header as *mut libc::c_void);
                                        header = 0 as *mut libc::c_char;
                                    }
                                    current_block = 14557636130817708122;
                                    break;
                                } else {
                                    needContinue = (parseEmailHeader(
                                        ret,
                                        header,
                                        rfc821_0,
                                        ctx,
                                        heuristicFound,
                                    ) < 0 as libc::c_int)
                                        as libc::c_int;
                                    if *heuristicFound {
                                        if !header.is_null() {
                                            free(header as *mut libc::c_void);
                                            header = 0 as *mut libc::c_char;
                                        }
                                        current_block = 14557636130817708122;
                                        break;
                                    } else {
                                        if !header.is_null() {
                                            free(header as *mut libc::c_void);
                                            header = 0 as *mut libc::c_char;
                                        }
                                        if curr != head {
                                            freeList((*head).next);
                                        }
                                        (*head).bufferLen = 0 as libc::c_int as size_t;
                                        let ref mut fresh4 = (*head).next;
                                        *fresh4 = 0 as *mut _ReadStruct;
                                        curr = head;
                                        if needContinue != 0 {
                                            current_block = 18317007320854588510;
                                        } else {
                                            current_block = 18325745679564279244;
                                        }
                                    }
                                }
                            } else {
                                current_block = 18325745679564279244;
                            }
                            match current_block {
                                18317007320854588510 => {}
                                _ => {
                                    if !boundary.is_null() || {
                                        boundary = messageFindArgument(
                                            ret,
                                            b"boundary\0" as *const u8 as *const libc::c_char,
                                        );
                                        !boundary.is_null()
                                    } {
                                        lastWasBlank = 1 as libc::c_int != 0;
                                        current_block = 18317007320854588510;
                                    } else {
                                        current_block = 8102658916883067714;
                                    }
                                }
                            }
                        } else {
                            current_block = 8102658916883067714;
                        }
                    } else {
                        current_block = 8102658916883067714;
                    }
                    match current_block {
                        18317007320854588510 => {}
                        _ => {
                            if line.is_null()
                                && 0 as libc::c_int as libc::c_ulong == (*head).bufferLen
                            {
                                if anyHeadersFound {
                                    cli_dbgmsg(
                                        b"End of header information\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    inHeader = 0 as libc::c_int != 0;
                                    bodyIsEmpty = 1 as libc::c_int != 0;
                                }
                            } else {
                                let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut lookahead: *const libc::c_char = 0 as *const libc::c_char;
                                let mut lineAdded: bool = 1 as libc::c_int != 0;
                                if 0 as libc::c_int as libc::c_ulong == (*head).bufferLen {
                                    let mut cmd: [libc::c_char; 1001] = [0; 1001];
                                    let mut out: [libc::c_char; 1001] = [0; 1001];
                                    if *(*__ctype_b_loc())
                                        .offset(*line.offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            as isize)
                                        as libc::c_int
                                        & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
                                        != 0
                                    {
                                        current_block = 18317007320854588510;
                                    } else if (strchr(line, ':' as i32)).is_null()
                                        || (cli_strtokbuf(
                                            line,
                                            0 as libc::c_int,
                                            b":\0" as *const u8 as *const libc::c_char,
                                            cmd.as_mut_ptr(),
                                        ))
                                        .is_null()
                                    {
                                        if strncmp(
                                            line,
                                            b"From \0" as *const u8 as *const libc::c_char,
                                            5 as libc::c_int as libc::c_ulong,
                                        ) == 0 as libc::c_int
                                        {
                                            anyHeadersFound = 1 as libc::c_int != 0;
                                        }
                                        current_block = 18317007320854588510;
                                    } else {
                                        ptr = rfc822comments(cmd.as_mut_ptr(), out.as_mut_ptr());
                                        commandNumber = tableFind(
                                            rfc821_0,
                                            if !ptr.is_null() {
                                                ptr
                                            } else {
                                                cmd.as_mut_ptr()
                                            },
                                        );
                                        match commandNumber {
                                            2 | 3 | 1 => {
                                                anyHeadersFound = 1 as libc::c_int != 0;
                                                curr = appendReadStruct(curr, line);
                                                if curr.is_null() {
                                                    if !ret.is_null() {
                                                        (*ret).set_isTruncated(
                                                            1 as libc::c_int as libc::c_uint,
                                                        );
                                                    }
                                                    current_block = 14557636130817708122;
                                                    break;
                                                } else {
                                                    current_block = 14652688882591975137;
                                                }
                                            }
                                            _ => {
                                                if !anyHeadersFound {
                                                    anyHeadersFound = usefulHeader(
                                                        commandNumber,
                                                        cmd.as_mut_ptr(),
                                                    );
                                                }
                                                current_block = 18317007320854588510;
                                            }
                                        }
                                    }
                                } else {
                                    if !line.is_null() {
                                        curr = appendReadStruct(curr, line);
                                    } else {
                                        lineAdded = 0 as libc::c_int != 0;
                                    }
                                    current_block = 14652688882591975137;
                                }
                                match current_block {
                                    18317007320854588510 => {}
                                    _ => {
                                        if lineAdded {
                                            totalHeaderBytes = (totalHeaderBytes as libc::c_ulong)
                                                .wrapping_add(strlen(line))
                                                as size_t
                                                as size_t;
                                            if haveTooManyHeaderBytes(
                                                totalHeaderBytes,
                                                ctx,
                                                heuristicFound,
                                            ) {
                                                current_block = 14557636130817708122;
                                                break;
                                            }
                                        }
                                        lookahead = fmap_need_off_once(
                                            map,
                                            *at,
                                            1 as libc::c_int as size_t,
                                        )
                                            as *const libc::c_char;
                                        if !lookahead.is_null() {
                                            if *(*__ctype_b_loc())
                                                .offset(*lookahead as libc::c_int as isize)
                                                as libc::c_int
                                                & _ISblank as libc::c_int as libc::c_ushort
                                                    as libc::c_int
                                                != 0
                                            {
                                                current_block = 18317007320854588510;
                                            } else {
                                                current_block = 6712573431332408062;
                                            }
                                        } else {
                                            current_block = 6712573431332408062;
                                        }
                                        match current_block {
                                            18317007320854588510 => {}
                                            _ => {
                                                let mut header_0: *mut libc::c_char =
                                                    getMallocedBufferFromList(head);
                                                let mut needContinue_0: libc::c_int =
                                                    0 as libc::c_int;
                                                if header_0.is_null() {
                                                    current_block = 16850746923508545940;
                                                    break;
                                                }
                                                needContinue_0 = (*header_0.offset(
                                                    (strlen(header_0)).wrapping_sub(
                                                        1 as libc::c_int as libc::c_ulong,
                                                    )
                                                        as isize,
                                                )
                                                    as libc::c_int
                                                    == ';' as i32)
                                                    as libc::c_int;
                                                if 0 as libc::c_int == needContinue_0 {
                                                    needContinue_0 = (!line.is_null()
                                                        && count_quotes(header_0)
                                                            & 1 as libc::c_int
                                                            != 0)
                                                        as libc::c_int;
                                                }
                                                if 0 as libc::c_int == needContinue_0 {
                                                    totalHeaderCnt = totalHeaderCnt.wrapping_add(1);
                                                    if haveTooManyEmailHeaders(
                                                        totalHeaderCnt,
                                                        ctx,
                                                        heuristicFound,
                                                    ) {
                                                        if !header_0.is_null() {
                                                            free(header_0 as *mut libc::c_void);
                                                            header_0 = 0 as *mut libc::c_char;
                                                        }
                                                        current_block = 14557636130817708122;
                                                        break;
                                                    } else {
                                                        needContinue_0 = (parseEmailHeader(
                                                            ret,
                                                            header_0,
                                                            rfc821_0,
                                                            ctx,
                                                            heuristicFound,
                                                        ) < 0 as libc::c_int)
                                                            as libc::c_int;
                                                        if *heuristicFound {
                                                            if !header_0.is_null() {
                                                                free(header_0 as *mut libc::c_void);
                                                                header_0 = 0 as *mut libc::c_char;
                                                            }
                                                            current_block = 14557636130817708122;
                                                            break;
                                                        }
                                                    }
                                                }
                                                if !header_0.is_null() {
                                                    free(header_0 as *mut libc::c_void);
                                                    header_0 = 0 as *mut libc::c_char;
                                                }
                                                if !(needContinue_0 != 0) {
                                                    if curr != head {
                                                        freeList((*head).next);
                                                    }
                                                    (*head).bufferLen = 0 as libc::c_int as size_t;
                                                    let ref mut fresh5 = (*head).next;
                                                    *fresh5 = 0 as *mut _ReadStruct;
                                                    curr = head;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if !line.is_null() && isuuencodebegin(line) != 0 {
                    bodyIsEmpty = 0 as libc::c_int != 0;
                    if uudecodeFile(ret, line, dir, map, at) < 0 as libc::c_int {
                        if messageAddStr(ret, line) < 0 as libc::c_int {
                            current_block = 14557636130817708122;
                            break;
                        }
                    }
                } else {
                    if line.is_null() {
                        if lastBodyLineWasBlank as libc::c_int != 0
                            && messageGetMimeType(ret) as libc::c_uint
                                != TEXT as libc::c_int as libc::c_uint
                        {
                            cli_dbgmsg(
                                b"Ignoring consecutive blank lines in the body\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            current_block = 18317007320854588510;
                        } else {
                            lastBodyLineWasBlank = 1 as libc::c_int != 0;
                            current_block = 13262925809421758307;
                        }
                    } else {
                        if bodyIsEmpty {
                            if newline_in_header(line) {
                                current_block = 18317007320854588510;
                            } else {
                                bodyIsEmpty = 0 as libc::c_int != 0;
                                current_block = 12387625063048049585;
                            }
                        } else {
                            current_block = 12387625063048049585;
                        }
                        match current_block {
                            18317007320854588510 => {}
                            _ => {
                                lastBodyLineWasBlank = 0 as libc::c_int != 0;
                                current_block = 13262925809421758307;
                            }
                        }
                    }
                    match current_block {
                        18317007320854588510 => {}
                        _ => {
                            if messageAddStr(ret, line) < 0 as libc::c_int {
                                current_block = 14557636130817708122;
                                break;
                            }
                        }
                    }
                }
            }
            if (getline_from_mbox(
                buffer.as_mut_ptr(),
                (::std::mem::size_of::<[libc::c_char; 1001]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                map,
                at,
            ))
            .is_null()
            {
                current_block = 14557636130817708122;
                break;
            }
        }
        match current_block {
            16850746923508545940 => {}
            _ => {
                err = 0 as libc::c_int;
            }
        }
    }
    if err != 0 {
        cli_errmsg(b"parseEmailFile: ERROR parsing file\n\0" as *const u8 as *const libc::c_char);
        (*ret).set_isTruncated(1 as libc::c_int as libc::c_uint);
    }
    if !boundary.is_null() {
        free(boundary as *mut libc::c_void);
        boundary = 0 as *mut libc::c_char;
    }
    freeList(head);
    if !anyHeadersFound {
        messageDestroy(ret);
        cli_dbgmsg(
            b"parseEmailFile: no headers found, assuming it isn't an email\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut message;
    }
    if *heuristicFound {
        messageDestroy(ret);
        cli_dbgmsg(b"parseEmailFile: found heuristic\n\0" as *const u8 as *const libc::c_char);
        return 0 as *mut message;
    }
    cli_dbgmsg(b"parseEmailFile: return\n\0" as *const u8 as *const libc::c_char);
    return ret;
}
unsafe extern "C" fn parseEmailHeaders(
    m: *mut message,
    rfc821_0: *const table_t,
    heuristicFound: *mut bool,
) -> *mut message {
    let mut inHeader: bool = 1 as libc::c_int != 0;
    let mut bodyIsEmpty: bool = 1 as libc::c_int != 0;
    let mut t: *mut text = 0 as *mut text;
    let mut ret: *mut message = 0 as *mut message;
    let mut anyHeadersFound: bool = 0 as libc::c_int != 0;
    let mut commandNumber: libc::c_int = -(1 as libc::c_int);
    let mut fullline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fulllinelength: size_t = 0 as libc::c_int as size_t;
    let mut lastWasOnlySemi: bool = 0 as libc::c_int != 0;
    let mut lineFoldCnt: size_t = 0 as libc::c_int as size_t;
    let mut totalHeaderCnt: size_t = 0 as libc::c_int as size_t;
    cli_dbgmsg(b"parseEmailHeaders\n\0" as *const u8 as *const libc::c_char);
    *heuristicFound = 0 as libc::c_int != 0;
    if m.is_null() {
        return 0 as *mut message;
    }
    ret = messageCreate();
    let mut current_block_43: u64;
    t = messageGetBody(m);
    while !t.is_null() {
        let mut line: *const libc::c_char = 0 as *const libc::c_char;
        if !((*t).t_line).is_null() {
            line = lineGetData((*t).t_line);
        } else {
            line = 0 as *const libc::c_char;
        }
        if !doContinueMultipleEmptyOptions(line, &mut lastWasOnlySemi) {
            if hitLineFoldCnt(line, &mut lineFoldCnt, (*m).ctx, heuristicFound) {
                break;
            }
            if inHeader {
                cli_dbgmsg(
                    b"parseEmailHeaders: check '%s'\n\0" as *const u8 as *const libc::c_char,
                    if !line.is_null() {
                        line
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                );
                if line.is_null() {
                    cli_dbgmsg(
                        b"End of header information\n\0" as *const u8 as *const libc::c_char,
                    );
                    if !anyHeadersFound {
                        cli_dbgmsg(
                            b"Nothing interesting in the header\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        break;
                    } else {
                        inHeader = 0 as libc::c_int != 0;
                        bodyIsEmpty = 1 as libc::c_int != 0;
                    }
                } else {
                    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut lineAdded: bool = 1 as libc::c_int != 0;
                    if fullline.is_null() {
                        let mut cmd: [libc::c_char; 1001] = [0; 1001];
                        if *(*__ctype_b_loc())
                            .offset(*line.offset(0 as libc::c_int as isize) as libc::c_int as isize)
                            as libc::c_int
                            & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                        {
                            current_block_43 = 7746791466490516765;
                        } else if (strchr(line, ':' as i32)).is_null()
                            || (cli_strtokbuf(
                                line,
                                0 as libc::c_int,
                                b":\0" as *const u8 as *const libc::c_char,
                                cmd.as_mut_ptr(),
                            ))
                            .is_null()
                        {
                            if strncmp(
                                line,
                                b"From \0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int as libc::c_ulong,
                            ) == 0 as libc::c_int
                            {
                                anyHeadersFound = 1 as libc::c_int != 0;
                            }
                            current_block_43 = 7746791466490516765;
                        } else {
                            ptr = rfc822comments(cmd.as_mut_ptr(), 0 as *mut libc::c_char);
                            commandNumber = tableFind(
                                rfc821_0,
                                if !ptr.is_null() {
                                    ptr
                                } else {
                                    cmd.as_mut_ptr()
                                },
                            );
                            if !ptr.is_null() {
                                free(ptr as *mut libc::c_void);
                            }
                            match commandNumber {
                                2 | 3 | 1 => {
                                    anyHeadersFound = 1 as libc::c_int != 0;
                                    fullline = cli_strdup(line);
                                    fulllinelength = (strlen(line))
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong);
                                    current_block_43 = 9353995356876505083;
                                }
                                _ => {
                                    if !anyHeadersFound {
                                        anyHeadersFound =
                                            usefulHeader(commandNumber, cmd.as_mut_ptr());
                                    }
                                    current_block_43 = 7746791466490516765;
                                }
                            }
                        }
                    } else if !line.is_null() {
                        fulllinelength = (fulllinelength as libc::c_ulong).wrapping_add(
                            (strlen(line)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as size_t as size_t;
                        ptr = cli_realloc(fullline as *mut libc::c_void, fulllinelength)
                            as *mut libc::c_char;
                        if ptr.is_null() {
                            current_block_43 = 7746791466490516765;
                        } else {
                            fullline = ptr;
                            cli_strlcat(fullline, line, fulllinelength);
                            current_block_43 = 9353995356876505083;
                        }
                    } else {
                        lineAdded = 0 as libc::c_int != 0;
                        current_block_43 = 9353995356876505083;
                    }
                    match current_block_43 {
                        7746791466490516765 => {}
                        _ => {
                            if !fullline.is_null() {
                                if lineAdded {
                                    if haveTooManyHeaderBytes(
                                        fulllinelength,
                                        (*m).ctx,
                                        heuristicFound,
                                    ) {
                                        break;
                                    }
                                }
                                if !next_is_folded_header(t) {
                                    lineUnlink((*t).t_line);
                                    let ref mut fresh6 = (*t).t_line;
                                    *fresh6 = 0 as *mut line_t;
                                    if !(count_quotes(fullline) & 1 as libc::c_int != 0) {
                                        ptr = rfc822comments(fullline, 0 as *mut libc::c_char);
                                        if !ptr.is_null() {
                                            free(fullline as *mut libc::c_void);
                                            fullline = ptr;
                                        }
                                        totalHeaderCnt = totalHeaderCnt.wrapping_add(1);
                                        if haveTooManyEmailHeaders(
                                            totalHeaderCnt,
                                            (*m).ctx,
                                            heuristicFound,
                                        ) {
                                            break;
                                        }
                                        if !(parseEmailHeader(
                                            ret,
                                            fullline,
                                            rfc821_0,
                                            (*m).ctx,
                                            heuristicFound,
                                        ) < 0 as libc::c_int)
                                        {
                                            if *heuristicFound {
                                                break;
                                            }
                                            free(fullline as *mut libc::c_void);
                                            fullline = 0 as *mut libc::c_char;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                if bodyIsEmpty {
                    if line.is_null() {
                        current_block_43 = 7746791466490516765;
                    } else if newline_in_header(line) {
                        current_block_43 = 7746791466490516765;
                    } else {
                        bodyIsEmpty = 0 as libc::c_int != 0;
                        current_block_43 = 54079586644752974;
                    }
                } else {
                    current_block_43 = 54079586644752974;
                }
                match current_block_43 {
                    7746791466490516765 => {}
                    _ => {
                        cli_dbgmsg(
                            b"parseEmailHeaders: finished with headers, moving body\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                        messageMoveText(ret, t, m);
                        break;
                    }
                }
            }
        }
        t = (*t).t_next;
    }
    if !fullline.is_null() {
        if *fullline != 0 {
            match commandNumber {
                2 | 3 | 1 => {
                    cli_dbgmsg(
                        b"parseEmailHeaders: Fullline unparsed '%s'\n\0" as *const u8
                            as *const libc::c_char,
                        fullline,
                    );
                }
                _ => {}
            }
        }
        free(fullline as *mut libc::c_void);
    }
    if !anyHeadersFound {
        messageDestroy(ret);
        cli_dbgmsg(
            b"parseEmailHeaders: no headers found, assuming it isn't an email\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut message;
    }
    if *heuristicFound {
        messageDestroy(ret);
        cli_dbgmsg(
            b"parseEmailHeaders: found a heuristic, delete message and stop parsing.\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as *mut message;
    }
    cli_dbgmsg(b"parseEmailHeaders: return\n\0" as *const u8 as *const libc::c_char);
    return ret;
}
unsafe extern "C" fn parseEmailHeader(
    m: *mut message,
    line: *const libc::c_char,
    rfc821_0: *const table_t,
    ctx: *mut cli_ctx,
    heuristicFound: *mut bool,
) -> libc::c_int {
    let current_block: u64;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut strptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut separator: *const libc::c_char = 0 as *const libc::c_char;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tokenseparator: [libc::c_char; 2] = [0; 2];
    cli_dbgmsg(
        b"parseEmailHeader '%s'\n\0" as *const u8 as *const libc::c_char,
        line,
    );
    separator = b":= \0" as *const u8 as *const libc::c_char;
    while *separator != 0 {
        if !(strchr(line, *separator as libc::c_int)).is_null() {
            break;
        }
        separator = separator.offset(1);
    }
    if *separator as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    copy = rfc2047(line);
    if copy.is_null() {
        copy = cli_strdup(line);
        if copy.is_null() {
            current_block = 7532098228208462329;
        } else {
            current_block = 11050875288958768710;
        }
    } else {
        current_block = 11050875288958768710;
    }
    match current_block {
        11050875288958768710 => {
            tokenseparator[0 as libc::c_int as usize] = *separator;
            tokenseparator[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            cmd = strtok_r(copy, tokenseparator.as_mut_ptr(), &mut strptr);
            if !cmd.is_null() && strstrip(cmd) > 0 as libc::c_int as libc::c_ulong {
                let arg: *mut libc::c_char = strtok_r(
                    0 as *mut libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                    &mut strptr,
                );
                if !arg.is_null() {
                    ret = parseMimeHeader(m, cmd, rfc821_0, arg, ctx, heuristicFound);
                }
            }
        }
        _ => {}
    }
    if !copy.is_null() {
        free(copy as *mut libc::c_void);
        copy = 0 as *mut libc::c_char;
    }
    return ret;
}
static mut mhtml_keys: [key_entry; 5] = [
    {
        let init = key_entry {
            key: b"html\0" as *const u8 as *const libc::c_char,
            name: b"RootHTML\0" as *const u8 as *const libc::c_char,
            type_0: (0x10 as libc::c_int | 0x400 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let init = key_entry {
            key: b"head\0" as *const u8 as *const libc::c_char,
            name: b"Head\0" as *const u8 as *const libc::c_char,
            type_0: (0x20 as libc::c_int | 0x8 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let init = key_entry {
            key: b"meta\0" as *const u8 as *const libc::c_char,
            name: b"Meta\0" as *const u8 as *const libc::c_char,
            type_0: (0x20 as libc::c_int | 0x40 as libc::c_int | 0x400 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let init = key_entry {
            key: b"link\0" as *const u8 as *const libc::c_char,
            name: b"Link\0" as *const u8 as *const libc::c_char,
            type_0: (0x20 as libc::c_int | 0x40 as libc::c_int | 0x400 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let init = key_entry {
            key: b"script\0" as *const u8 as *const libc::c_char,
            name: b"Script\0" as *const u8 as *const libc::c_char,
            type_0: (0x20 as libc::c_int | 0x40 as libc::c_int | 0x200 as libc::c_int) as uint32_t,
        };
        init
    },
];
static mut num_mhtml_keys: size_t = 0;
static mut mhtml_comment_keys: [key_entry; 18] = [
    {
        let init = key_entry {
            key: b"o:documentproperties\0" as *const u8 as *const libc::c_char,
            name: b"DocumentProperties\0" as *const u8 as *const libc::c_char,
            type_0: (0x10 as libc::c_int | 0x400 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let init = key_entry {
            key: b"o:author\0" as *const u8 as *const libc::c_char,
            name: b"Author\0" as *const u8 as *const libc::c_char,
            type_0: (0x20 as libc::c_int | 0x200 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let init = key_entry {
            key: b"o:lastauthor\0" as *const u8 as *const libc::c_char,
            name: b"LastAuthor\0" as *const u8 as *const libc::c_char,
            type_0: (0x20 as libc::c_int | 0x200 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let init = key_entry {
            key: b"o:revision\0" as *const u8 as *const libc::c_char,
            name: b"Revision\0" as *const u8 as *const libc::c_char,
            type_0: (0x20 as libc::c_int | 0x200 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let init = key_entry {
            key: b"o:totaltime\0" as *const u8 as *const libc::c_char,
            name: b"TotalTime\0" as *const u8 as *const libc::c_char,
            type_0: (0x20 as libc::c_int | 0x200 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let init = key_entry {
            key: b"o:created\0" as *const u8 as *const libc::c_char,
            name: b"Created\0" as *const u8 as *const libc::c_char,
            type_0: (0x20 as libc::c_int | 0x200 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let init = key_entry {
            key: b"o:lastsaved\0" as *const u8 as *const libc::c_char,
            name: b"LastSaved\0" as *const u8 as *const libc::c_char,
            type_0: (0x20 as libc::c_int | 0x200 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let init = key_entry {
            key: b"o:pages\0" as *const u8 as *const libc::c_char,
            name: b"Pages\0" as *const u8 as *const libc::c_char,
            type_0: (0x20 as libc::c_int | 0x200 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let init = key_entry {
            key: b"o:words\0" as *const u8 as *const libc::c_char,
            name: b"Words\0" as *const u8 as *const libc::c_char,
            type_0: (0x20 as libc::c_int | 0x200 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let init = key_entry {
            key: b"o:characters\0" as *const u8 as *const libc::c_char,
            name: b"Characters\0" as *const u8 as *const libc::c_char,
            type_0: (0x20 as libc::c_int | 0x200 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let init = key_entry {
            key: b"o:company\0" as *const u8 as *const libc::c_char,
            name: b"Company\0" as *const u8 as *const libc::c_char,
            type_0: (0x20 as libc::c_int | 0x200 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let init = key_entry {
            key: b"o:lines\0" as *const u8 as *const libc::c_char,
            name: b"Lines\0" as *const u8 as *const libc::c_char,
            type_0: (0x20 as libc::c_int | 0x200 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let init = key_entry {
            key: b"o:paragraphs\0" as *const u8 as *const libc::c_char,
            name: b"Paragraphs\0" as *const u8 as *const libc::c_char,
            type_0: (0x20 as libc::c_int | 0x200 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let init = key_entry {
            key: b"o:characterswithspaces\0" as *const u8 as *const libc::c_char,
            name: b"CharactersWithSpaces\0" as *const u8 as *const libc::c_char,
            type_0: (0x20 as libc::c_int | 0x200 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let init = key_entry {
            key: b"o:version\0" as *const u8 as *const libc::c_char,
            name: b"Version\0" as *const u8 as *const libc::c_char,
            type_0: (0x20 as libc::c_int | 0x200 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let init = key_entry {
            key: b"o:officedocumentsettings\0" as *const u8 as *const libc::c_char,
            name: b"DocumentSettings\0" as *const u8 as *const libc::c_char,
            type_0: 0x1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let init = key_entry {
            key: b"w:worddocument\0" as *const u8 as *const libc::c_char,
            name: b"WordDocument\0" as *const u8 as *const libc::c_char,
            type_0: 0x1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let init = key_entry {
            key: b"w:latentstyles\0" as *const u8 as *const libc::c_char,
            name: b"LatentStyles\0" as *const u8 as *const libc::c_char,
            type_0: 0x1 as libc::c_int as uint32_t,
        };
        init
    },
];
static mut num_mhtml_comment_keys: size_t = 0;
unsafe extern "C" fn parseMHTMLComment(
    comment: *const libc::c_char,
    ctx: *mut cli_ctx,
    _wrkjobj: *mut libc::c_void,
    _cbdata: *mut libc::c_void,
) -> cl_error_t {
    let mut ret: cl_error_t = CL_SUCCESS;
    let mut xmlsrt: *const libc::c_char = 0 as *const libc::c_char;
    let mut xmlend: *const libc::c_char = 0 as *const libc::c_char;
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    xmlend = comment;
    loop {
        xmlsrt = strstr(xmlend, b"<xml>\0" as *const u8 as *const libc::c_char);
        if xmlsrt.is_null() {
            break;
        }
        xmlend = strstr(xmlsrt, b"</xml>\0" as *const u8 as *const libc::c_char);
        if xmlend.is_null() {
            cli_dbgmsg(
                b"parseMHTMLComment: unbounded xml tag\n\0" as *const u8 as *const libc::c_char,
            );
            break;
        } else {
            reader = xmlReaderForMemory(
                xmlsrt,
                (xmlend.offset_from(xmlsrt) as libc::c_long + 6 as libc::c_int as libc::c_long)
                    as libc::c_int,
                b"comment.xml\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                XML_PARSE_NOERROR as libc::c_int | XML_PARSE_NONET as libc::c_int,
            );
            if reader.is_null() {
                cli_dbgmsg(
                    b"parseMHTMLComment: cannot initialize xmlReader\n\0" as *const u8
                        as *const libc::c_char,
                );
                if !((*ctx).wrkproperty).is_null() {
                    ret = cli_json_parse_error(
                        (*ctx).wrkproperty,
                        b"MHTML_ERROR_XML_READER_MEM\0" as *const u8 as *const libc::c_char,
                    );
                }
                return ret;
            }
            ret = cli_msxml_parse_document(
                ctx,
                reader,
                mhtml_comment_keys.as_ptr(),
                num_mhtml_comment_keys,
                0x1 as libc::c_int as uint32_t,
                0 as *mut msxml_ctx,
            ) as cl_error_t;
            xmlTextReaderClose(reader);
            xmlFreeTextReader(reader);
            if ret as libc::c_uint != CL_SUCCESS as libc::c_int as libc::c_uint {
                return ret;
            }
        }
    }
    return ret;
}
unsafe extern "C" fn parseRootMHTML(
    mctx: *mut mbox_ctx,
    m: *mut message,
    t: *mut text,
) -> mbox_status {
    let ctx: *mut cli_ctx = (*mctx).ctx;
    let mut mxctx: msxml_ctx = msxml_ctx {
        scan_cb: None,
        scan_data: 0 as *mut libc::c_void,
        comment_cb: None,
        comment_data: 0 as *mut libc::c_void,
        ictx: 0 as *mut msxml_ictx,
    };
    let mut input: *mut blob = 0 as *mut blob;
    let mut htmlDoc: htmlDocPtr = 0 as *mut xmlDoc;
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    let mut ret: libc::c_int = CL_SUCCESS as libc::c_int;
    let mut rc: mbox_status = OK;
    let mut rhtml: *mut json_object = 0 as *mut json_object;
    cli_dbgmsg(b"in parseRootMHTML\n\0" as *const u8 as *const libc::c_char);
    if ctx.is_null() {
        return OK;
    }
    if m.is_null() && t.is_null() {
        return OK;
    }
    if !m.is_null() {
        input = messageToBlob(m, 0 as libc::c_int);
    } else {
        input = textToBlob(t, 0 as *mut blob, 0 as libc::c_int);
    }
    if input.is_null() {
        return OK;
    }
    htmlDoc = htmlReadMemory(
        (*input).data as *mut libc::c_char,
        (*input).len as libc::c_int,
        b"mhtml.html\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        XML_PARSE_NOERROR as libc::c_int
            | XML_PARSE_NONET as libc::c_int
            | HTML_PARSE_NOWARNING as libc::c_int,
    );
    if htmlDoc.is_null() {
        cli_dbgmsg(
            b"parseRootMHTML: cannot initialize read html document\n\0" as *const u8
                as *const libc::c_char,
        );
        if !((*ctx).wrkproperty).is_null() {
            ret = cli_json_parse_error(
                (*ctx).wrkproperty,
                b"MHTML_ERROR_HTML_READ\0" as *const u8 as *const libc::c_char,
            ) as libc::c_int;
        }
        if ret != CL_SUCCESS as libc::c_int {
            rc = FAIL;
        }
        blobDestroy(input);
        return rc;
    }
    if !((*mctx).wrkobj).is_null() {
        rhtml = cli_jsonobj(
            (*mctx).wrkobj,
            b"RootHTML\0" as *const u8 as *const libc::c_char,
        );
        if !rhtml.is_null() {
            cli_jsonstr(
                rhtml,
                b"Encoding\0" as *const u8 as *const libc::c_char,
                htmlGetMetaEncoding(htmlDoc) as *const libc::c_char,
            );
            cli_jsonint(
                rhtml,
                b"CompressMode\0" as *const u8 as *const libc::c_char,
                xmlGetDocCompressMode(htmlDoc as *const xmlDoc),
            );
        }
    }
    reader = xmlReaderWalker(htmlDoc);
    if reader.is_null() {
        cli_dbgmsg(
            b"parseRootMHTML: cannot initialize xmlTextReader\n\0" as *const u8
                as *const libc::c_char,
        );
        if !((*ctx).wrkproperty).is_null() {
            ret = cli_json_parse_error(
                (*ctx).wrkproperty,
                b"MHTML_ERROR_XML_READER_IO\0" as *const u8 as *const libc::c_char,
            ) as libc::c_int;
        }
        if ret != CL_SUCCESS as libc::c_int {
            rc = FAIL;
        }
        blobDestroy(input);
        return rc;
    }
    memset(
        &mut mxctx as *mut msxml_ctx as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<msxml_ctx>() as libc::c_ulong,
    );
    mxctx.comment_cb = Some(
        parseMHTMLComment
            as unsafe extern "C" fn(
                *const libc::c_char,
                *mut cli_ctx,
                *mut libc::c_void,
                *mut libc::c_void,
            ) -> cl_error_t,
    );
    ret = cli_msxml_parse_document(
        ctx,
        reader,
        mhtml_keys.as_ptr(),
        num_mhtml_keys,
        (0x1 as libc::c_int | 0x2 as libc::c_int) as uint32_t,
        &mut mxctx,
    );
    match ret {
        0 | 21 | 22 => {
            rc = OK;
        }
        23 => {
            rc = MAXREC;
        }
        25 => {
            rc = MAXFILES;
        }
        1 => {
            rc = VIRUS;
        }
        _ => {
            rc = FAIL;
        }
    }
    xmlTextReaderClose(reader);
    xmlFreeTextReader(reader);
    xmlFreeDoc(htmlDoc);
    blobDestroy(input);
    return rc;
}
unsafe extern "C" fn parseEmailBody(
    messageIn: *mut message,
    textIn: *mut text,
    mctx: *mut mbox_ctx,
    recursion_level: libc::c_uint,
) -> mbox_status {
    let mut rc: mbox_status = FAIL;
    let mut aText: *mut text = textIn;
    let mut mainMessage: *mut message = messageIn;
    let mut fb: *mut fileblob = 0 as *mut fileblob;
    let mut infected: bool = 0 as libc::c_int != 0;
    let engine: *const cl_engine = (*(*mctx).ctx).engine;
    let doPhishingScan: libc::c_int = ((*engine).dboptions & 0x8 as libc::c_int as libc::c_uint
        != 0
        && (*(*(*mctx).ctx).dconf).phishing & 0x1 as libc::c_int as libc::c_uint != 0)
        as libc::c_int;
    let saveobj: *mut json_object = (*mctx).wrkobj;
    let mut heuristicFound: bool = 0 as libc::c_int != 0;
    cli_dbgmsg(
        b"in parseEmailBody, %u files saved so far\n\0" as *const u8 as *const libc::c_char,
        (*mctx).files,
    );
    if (*engine).max_recursion_level != 0 {
        if recursion_level > (*engine).max_recursion_level {
            cli_dbgmsg(
                b"parseEmailBody: hit maximum recursion level (%u)\n\0" as *const u8
                    as *const libc::c_char,
                recursion_level,
            );
            return MAXREC;
        }
    }
    if (*engine).maxfiles != 0 && (*mctx).files >= (*engine).maxfiles {
        cli_dbgmsg(
            b"parseEmailBody: number of files exceeded %u\n\0" as *const u8 as *const libc::c_char,
            (*engine).maxfiles,
        );
        return MAXFILES;
    }
    rc = OK;
    if !mainMessage.is_null() && !(messageGetBody(mainMessage)).is_null() {
        let mut mimeType: mime_type = NOMIME;
        let mut subtype_0: libc::c_int = 0;
        let mut inhead: libc::c_int = 0;
        let mut htmltextPart: libc::c_int = 0;
        let mut inMimeHead: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut mimeSubtype: *const libc::c_char = 0 as *const libc::c_char;
        let mut boundary: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut t_line: *const text = 0 as *const text;
        let mut aMessage: *mut message = 0 as *mut message;
        let mut multiparts: libc::c_int = 0 as libc::c_int;
        let mut messages: *mut *mut message = 0 as *mut *mut message;
        cli_dbgmsg(b"Parsing mail file\n\0" as *const u8 as *const libc::c_char);
        mimeType = messageGetMimeType(mainMessage);
        mimeSubtype = messageGetMimeSubtype(mainMessage);
        if !((*mctx).wrkobj).is_null() {
            let ref mut fresh7 = (*mctx).wrkobj;
            *fresh7 = cli_jsonobj(
                (*mctx).wrkobj,
                b"Body\0" as *const u8 as *const libc::c_char,
            );
            cli_jsonstr(
                (*mctx).wrkobj,
                b"MimeType\0" as *const u8 as *const libc::c_char,
                getMimeTypeStr(mimeType),
            );
            cli_jsonstr(
                (*mctx).wrkobj,
                b"MimeSubtype\0" as *const u8 as *const libc::c_char,
                mimeSubtype,
            );
            cli_jsonstr(
                (*mctx).wrkobj,
                b"EncodingType\0" as *const u8 as *const libc::c_char,
                getEncTypeStr(messageGetEncoding(mainMessage)),
            );
            cli_jsonstr(
                (*mctx).wrkobj,
                b"Disposition\0" as *const u8 as *const libc::c_char,
                messageGetDispositionType(mainMessage),
            );
            if messageHasFilename(mainMessage) != 0 {
                let filename: *mut libc::c_char = messageGetFilename(mainMessage);
                cli_jsonstr(
                    (*mctx).wrkobj,
                    b"Filename\0" as *const u8 as *const libc::c_char,
                    filename,
                );
                free(filename as *mut libc::c_void);
            } else {
                cli_jsonstr(
                    (*mctx).wrkobj,
                    b"Filename\0" as *const u8 as *const libc::c_char,
                    b"(inline)\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        subtype_0 = tableFind((*mctx).subtypeTable, mimeSubtype);
        if mimeType as libc::c_uint == TEXT as libc::c_int as libc::c_uint
            && subtype_0 == 1 as libc::c_int
        {
            cli_dbgmsg(
                b"text/plain: Assume no attachments\n\0" as *const u8 as *const libc::c_char,
            );
            mimeType = NOMIME;
            messageSetMimeSubtype(mainMessage, b"\0" as *const u8 as *const libc::c_char);
        } else if mimeType as libc::c_uint == MESSAGE as libc::c_int as libc::c_uint
            && strcasecmp(
                mimeSubtype,
                b"rfc822-headers\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            cli_dbgmsg(
                b"Changing message/rfc822-headers to text/rfc822-headers\n\0" as *const u8
                    as *const libc::c_char,
            );
            mimeType = NOMIME;
            messageSetMimeSubtype(mainMessage, b"\0" as *const u8 as *const libc::c_char);
        } else {
            cli_dbgmsg(
                b"mimeType = %d\n\0" as *const u8 as *const libc::c_char,
                mimeType as libc::c_int,
            );
        }
        let current_block_306: u64;
        match mimeType as libc::c_uint {
            0 => {
                cli_dbgmsg(b"Not a mime encoded message\n\0" as *const u8 as *const libc::c_char);
                aText = textAddMessage(aText, mainMessage);
                if doPhishingScan == 0 {
                    current_block_306 = 3011375068465243373;
                } else {
                    current_block_306 = 13845630548962704447;
                }
            }
            6 => {
                current_block_306 = 13845630548962704447;
            }
            5 => {
                cli_dbgmsg(
                    b"Content-type 'multipart' handler\n\0" as *const u8 as *const libc::c_char,
                );
                boundary = messageFindArgument(
                    mainMessage,
                    b"boundary\0" as *const u8 as *const libc::c_char,
                );
                if !((*mctx).wrkobj).is_null() {
                    cli_jsonstr(
                        (*mctx).wrkobj,
                        b"Boundary\0" as *const u8 as *const libc::c_char,
                        boundary,
                    );
                }
                if boundary.is_null() {
                    cli_dbgmsg(
                        b"Multipart/%s MIME message contains no boundary header\n\0" as *const u8
                            as *const libc::c_char,
                        mimeSubtype,
                    );
                    mimeType = NOMIME;
                } else {
                    cli_chomp(boundary);
                    if *mimeSubtype.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
                    {
                        cli_dbgmsg(
                            b"Multipart has no subtype assuming alternative\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        mimeSubtype = b"alternative\0" as *const u8 as *const libc::c_char;
                        messageSetMimeSubtype(
                            mainMessage,
                            b"alternative\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    t_line = messageGetBody(mainMessage);
                    if t_line.is_null() {
                        cli_dbgmsg(
                            b"Multipart MIME message has no body\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        free(boundary as *mut libc::c_void);
                        mimeType = NOMIME;
                    } else {
                        loop {
                            if !((*t_line).t_line).is_null() {
                                if boundaryStart(lineGetData((*t_line).t_line), boundary) != 0 {
                                    break;
                                }
                                if binhexBegin(mainMessage) == t_line as *mut text {
                                    if exportBinhexMessage(mctx, mainMessage) {
                                        rc = VIRUS;
                                        infected = 1 as libc::c_int != 0;
                                        break;
                                    }
                                } else if !((*t_line).t_next).is_null()
                                    && encodingLine(mainMessage) == (*t_line).t_next
                                {
                                    cli_dbgmsg(
                                        b"Found MIME attachment before the first MIME section \"%s\"\n\0"
                                            as *const u8 as *const libc::c_char,
                                        lineGetData((*(*t_line).t_next).t_line),
                                    );
                                    if messageGetEncoding(mainMessage) as libc::c_uint
                                        == NOENCODING as libc::c_int as libc::c_uint
                                    {
                                        break;
                                    }
                                }
                            }
                            t_line = (*t_line).t_next;
                            if t_line.is_null() {
                                break;
                            }
                        }
                        if t_line.is_null() {
                            cli_dbgmsg(
                                b"Multipart MIME message contains no boundary lines (%s)\n\0"
                                    as *const u8
                                    as *const libc::c_char,
                                boundary,
                            );
                            free(boundary as *mut libc::c_void);
                            mimeType = NOMIME;
                        } else {
                            inhead = 1 as libc::c_int;
                            inMimeHead = 0 as libc::c_int;
                            subtype_0 = tableFind((*mctx).subtypeTable, mimeSubtype);
                            multiparts = 0 as libc::c_int;
                            while !t_line.is_null() && !infected {
                                let mut lines: libc::c_int = 0 as libc::c_int;
                                let mut m: *mut *mut message = 0 as *mut *mut message;
                                let mut old_rc: mbox_status = FAIL;
                                m = cli_realloc(
                                    messages as *mut libc::c_void,
                                    ((multiparts + 1 as libc::c_int) as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<*mut message>() as libc::c_ulong
                                        ),
                                ) as *mut *mut message;
                                if m.is_null() {
                                    break;
                                }
                                messages = m;
                                let ref mut fresh8 = *messages.offset(multiparts as isize);
                                *fresh8 = messageCreate();
                                aMessage = *fresh8;
                                if aMessage.is_null() {
                                    multiparts -= 1;
                                    break;
                                } else {
                                    messageSetCTX(aMessage, (*mctx).ctx);
                                    cli_dbgmsg(
                                        b"Now read in part %d\n\0" as *const u8
                                            as *const libc::c_char,
                                        multiparts,
                                    );
                                    loop {
                                        t_line = (*t_line).t_next;
                                        if t_line.is_null() {
                                            break;
                                        }
                                        if !((*t_line).t_line).is_null()
                                            && strlen(lineGetData((*t_line).t_line))
                                                > 0 as libc::c_int as libc::c_ulong
                                        {
                                            break;
                                        }
                                    }
                                    if t_line.is_null() {
                                        cli_dbgmsg(
                                            b"Empty part\n\0" as *const u8 as *const libc::c_char,
                                        );
                                        if !mainMessage.is_null()
                                            && (binhexBegin(mainMessage)).is_null()
                                        {
                                            messageDestroy(aMessage);
                                            multiparts -= 1;
                                        }
                                    } else {
                                        let mut current_block_124: u64;
                                        loop {
                                            let mut line: *const libc::c_char =
                                                lineGetData((*t_line).t_line);
                                            if inMimeHead != 0 {
                                                if line.is_null() {
                                                    inMimeHead = 0 as libc::c_int;
                                                } else {
                                                    cli_dbgmsg(
                                                        b"Multipart %d: About to add mime Argument '%s'\n\0"
                                                            as *const u8 as *const libc::c_char,
                                                        multiparts,
                                                        line,
                                                    );
                                                    parseEmailHeader(
                                                        aMessage,
                                                        line,
                                                        (*mctx).rfc821Table,
                                                        (*mctx).ctx,
                                                        &mut heuristicFound,
                                                    );
                                                    if heuristicFound {
                                                        rc = VIRUS;
                                                        break;
                                                    } else {
                                                        while *(*__ctype_b_loc())
                                                            .offset(*line as libc::c_int as isize)
                                                            as libc::c_int
                                                            & _ISspace as libc::c_int
                                                                as libc::c_ushort
                                                                as libc::c_int
                                                            != 0
                                                        {
                                                            line = line.offset(1);
                                                        }
                                                        if *line as libc::c_int == '\0' as i32 {
                                                            inMimeHead = 0 as libc::c_int;
                                                            inhead = inMimeHead;
                                                        } else {
                                                            inMimeHead = 0 as libc::c_int;
                                                            messageAddArgument(aMessage, line);
                                                        }
                                                    }
                                                }
                                            } else if inhead != 0 {
                                                let mut fullline: *mut libc::c_char =
                                                    0 as *mut libc::c_char;
                                                let mut ptr: *mut libc::c_char =
                                                    0 as *mut libc::c_char;
                                                if line.is_null() {
                                                    let next: *const text = (*t_line).t_next;
                                                    if !next.is_null()
                                                        && !((*next).t_line).is_null()
                                                    {
                                                        let data: *const libc::c_char =
                                                            lineGetData((*next).t_line);
                                                        if messageGetEncoding(aMessage)
                                                            as libc::c_uint
                                                            == NOENCODING as libc::c_int
                                                                as libc::c_uint
                                                            && messageGetMimeType(aMessage)
                                                                as libc::c_uint
                                                                == APPLICATION as libc::c_int
                                                                    as libc::c_uint
                                                            && !data.is_null()
                                                            && !(strstr(
                                                                data,
                                                                b"base64\0" as *const u8
                                                                    as *const libc::c_char,
                                                            ))
                                                            .is_null()
                                                        {
                                                            messageSetEncoding(
                                                                aMessage,
                                                                b"base64\0" as *const u8
                                                                    as *const libc::c_char,
                                                            );
                                                            cli_dbgmsg(
                                                                b"Ignoring fake end of headers\n\0"
                                                                    as *const u8
                                                                    as *const libc::c_char,
                                                            );
                                                            current_block_124 =
                                                                10109057886293123569;
                                                        } else if strncmp(
                                                            data,
                                                            b"Content\0" as *const u8
                                                                as *const libc::c_char,
                                                            7 as libc::c_int as libc::c_ulong,
                                                        ) == 0 as libc::c_int
                                                            || strncmp(
                                                                data,
                                                                b"filename=\0" as *const u8
                                                                    as *const libc::c_char,
                                                                9 as libc::c_int as libc::c_ulong,
                                                            ) == 0 as libc::c_int
                                                        {
                                                            cli_dbgmsg(
                                                                b"Ignoring fake end of headers\n\0"
                                                                    as *const u8
                                                                    as *const libc::c_char,
                                                            );
                                                            current_block_124 =
                                                                10109057886293123569;
                                                        } else {
                                                            current_block_124 =
                                                                16937825661756021828;
                                                        }
                                                    } else {
                                                        current_block_124 = 16937825661756021828;
                                                    }
                                                    match current_block_124 {
                                                        10109057886293123569 => {}
                                                        _ => {
                                                            cli_dbgmsg(
                                                                b"Multipart %d: End of header information\n\0" as *const u8
                                                                    as *const libc::c_char,
                                                                multiparts,
                                                            );
                                                            inhead = 0 as libc::c_int;
                                                        }
                                                    }
                                                } else if *(*__ctype_b_loc())
                                                    .offset(*line as libc::c_int as isize)
                                                    as libc::c_int
                                                    & _ISspace as libc::c_int as libc::c_ushort
                                                        as libc::c_int
                                                    != 0
                                                {
                                                    cli_dbgmsg(
                                                        b"Part %d starts with a continuation line\n\0" as *const u8
                                                            as *const libc::c_char,
                                                        multiparts,
                                                    );
                                                    messageAddArgument(aMessage, line);
                                                    if messageGetMimeType(aMessage) as libc::c_uint
                                                        == NOMIME as libc::c_int as libc::c_uint
                                                    {
                                                        messageSetMimeType(
                                                            aMessage,
                                                            b"application\0" as *const u8
                                                                as *const libc::c_char,
                                                        );
                                                    }
                                                } else {
                                                    inMimeHead = 0 as libc::c_int;
                                                    if strlen(line)
                                                        > 1000 as libc::c_int as libc::c_ulong
                                                    {
                                                        cli_dbgmsg(
                                                            b"parseEmailBody: line length exceds RFC2821 maximum length (1000)\n\0"
                                                                as *const u8 as *const libc::c_char,
                                                        );
                                                    } else {
                                                        fullline = rfc822comments(
                                                            line,
                                                            0 as *mut libc::c_char,
                                                        );
                                                        if fullline.is_null() {
                                                            fullline = cli_strdup(line);
                                                        }
                                                        while !t_line.is_null()
                                                            && next_is_folded_header(t_line)
                                                                as libc::c_int
                                                                != 0
                                                        {
                                                            let mut data_0: *const libc::c_char =
                                                                0 as *const libc::c_char;
                                                            let mut datasz: size_t = 0;
                                                            t_line = (*t_line).t_next;
                                                            data_0 = lineGetData((*t_line).t_line);
                                                            if *data_0
                                                                .offset(1 as libc::c_int as isize)
                                                                as libc::c_int
                                                                == '\0' as i32
                                                            {
                                                                cli_dbgmsg(
                                                                    b"Multipart %d: headers not terminated by blank line\n\0"
                                                                        as *const u8 as *const libc::c_char,
                                                                    multiparts,
                                                                );
                                                                inhead = 0 as libc::c_int;
                                                                break;
                                                            } else {
                                                                datasz = (strlen(fullline))
                                                                    .wrapping_add(strlen(data_0))
                                                                    .wrapping_add(
                                                                        1 as libc::c_int
                                                                            as libc::c_ulong,
                                                                    );
                                                                ptr = cli_realloc(
                                                                    fullline as *mut libc::c_void,
                                                                    datasz,
                                                                )
                                                                    as *mut libc::c_char;
                                                                if ptr.is_null() {
                                                                    break;
                                                                }
                                                                fullline = ptr;
                                                                cli_strlcat(
                                                                    fullline, data_0, datasz,
                                                                );
                                                            }
                                                        }
                                                        cli_dbgmsg(
                                                            b"Multipart %d: About to parse folded header '%s'\n\0"
                                                                as *const u8 as *const libc::c_char,
                                                            multiparts,
                                                            fullline,
                                                        );
                                                        parseEmailHeader(
                                                            aMessage,
                                                            fullline,
                                                            (*mctx).rfc821Table,
                                                            (*mctx).ctx,
                                                            &mut heuristicFound,
                                                        );
                                                        free(fullline as *mut libc::c_void);
                                                        if heuristicFound {
                                                            rc = VIRUS;
                                                        }
                                                    }
                                                }
                                            } else {
                                                if boundaryEnd(line, boundary) != 0 {
                                                    break;
                                                }
                                                if boundaryStart(line, boundary) != 0 {
                                                    inhead = 1 as libc::c_int;
                                                    break;
                                                } else {
                                                    if messageAddLine(aMessage, (*t_line).t_line)
                                                        < 0 as libc::c_int
                                                    {
                                                        break;
                                                    }
                                                    lines += 1;
                                                }
                                            }
                                            t_line = (*t_line).t_next;
                                            if t_line.is_null() {
                                                break;
                                            }
                                        }
                                        cli_dbgmsg(
                                            b"Part %d has %d lines, rc = %d\n\0" as *const u8
                                                as *const libc::c_char,
                                            multiparts,
                                            lines,
                                            rc as libc::c_int,
                                        );
                                        match subtype_0 {
                                            5 | 6 | 11 | 7 | 12 | 14 | -1 => {
                                                old_rc = rc;
                                                mainMessage = do_multipart(
                                                    mainMessage,
                                                    messages,
                                                    multiparts,
                                                    &mut rc,
                                                    mctx,
                                                    messageIn,
                                                    &mut aText,
                                                    recursion_level,
                                                );
                                                if rc as libc::c_uint
                                                    == OK_ATTACHMENTS_NOT_SAVED as libc::c_int
                                                        as libc::c_uint
                                                    && old_rc as libc::c_uint
                                                        == OK as libc::c_int as libc::c_uint
                                                {
                                                    rc = OK;
                                                }
                                                if !(*messages.offset(multiparts as isize))
                                                    .is_null()
                                                {
                                                    messageDestroy(
                                                        *messages.offset(multiparts as isize),
                                                    );
                                                    let ref mut fresh9 =
                                                        *messages.offset(multiparts as isize);
                                                    *fresh9 = 0 as *mut message;
                                                }
                                                multiparts -= 1;
                                                if rc as libc::c_uint
                                                    == VIRUS as libc::c_int as libc::c_uint
                                                {
                                                    infected = 1 as libc::c_int != 0;
                                                }
                                            }
                                            10 | 13 | 8 | 9 => {}
                                            _ => {
                                                if !(*messages.offset(multiparts as isize))
                                                    .is_null()
                                                {
                                                    messageDestroy(
                                                        *messages.offset(multiparts as isize),
                                                    );
                                                    let ref mut fresh10 =
                                                        *messages.offset(multiparts as isize);
                                                    *fresh10 = 0 as *mut message;
                                                }
                                                multiparts -= 1;
                                            }
                                        }
                                    }
                                    multiparts += 1;
                                }
                            }
                            free(boundary as *mut libc::c_void);
                            if haveTooManyMIMEPartsPerMessage(
                                multiparts as size_t,
                                (*mctx).ctx,
                                &mut rc,
                            ) {
                                if !messages.is_null() {
                                    i = 0 as libc::c_int;
                                    while i < multiparts {
                                        if !(*messages.offset(i as isize)).is_null() {
                                            messageDestroy(*messages.offset(i as isize));
                                        }
                                        i += 1;
                                    }
                                    free(messages as *mut libc::c_void);
                                    messages = 0 as *mut *mut message;
                                }
                            } else {
                                match subtype_0 {
                                    14 => {
                                        cli_dbgmsg(
                                            b"multipart/knowbot parsed as multipart/mixed for now\n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                        mimeSubtype =
                                            b"mixed\0" as *const u8 as *const libc::c_char;
                                    }
                                    -1 => {
                                        cli_dbgmsg(
                                            b"Unsupported multipart format `%s', parsed as mixed\n\0"
                                                as *const u8 as *const libc::c_char,
                                            mimeSubtype,
                                        );
                                        mimeSubtype =
                                            b"mixed\0" as *const u8 as *const libc::c_char;
                                    }
                                    _ => {}
                                }
                                if !mainMessage.is_null() && mainMessage != messageIn {
                                    messageDestroy(mainMessage);
                                    mainMessage = 0 as *mut message;
                                }
                                cli_dbgmsg(
                                    b"The message has %d parts\n\0" as *const u8
                                        as *const libc::c_char,
                                    multiparts,
                                );
                                if infected as libc::c_int != 0
                                    || multiparts == 0 as libc::c_int && aText.is_null()
                                {
                                    if !messages.is_null() {
                                        i = 0 as libc::c_int;
                                        while i < multiparts {
                                            if !(*messages.offset(i as isize)).is_null() {
                                                messageDestroy(*messages.offset(i as isize));
                                            }
                                            i += 1;
                                        }
                                        free(messages as *mut libc::c_void);
                                        messages = 0 as *mut *mut message;
                                    }
                                    if !aText.is_null() && textIn.is_null() {
                                        textDestroy(aText);
                                    }
                                    let ref mut fresh11 = (*mctx).wrkobj;
                                    *fresh11 = saveobj;
                                    match rc as libc::c_uint {
                                        3 => return VIRUS,
                                        4 => return MAXREC,
                                        _ => return OK_ATTACHMENTS_NOT_SAVED,
                                    }
                                }
                                cli_dbgmsg(
                                    b"Find out the multipart type (%s)\n\0" as *const u8
                                        as *const libc::c_char,
                                    mimeSubtype,
                                );
                                let mut current_block_224: u64;
                                match tableFind((*mctx).subtypeTable, mimeSubtype) {
                                    10 => {
                                        cli_dbgmsg(
                                            b"Multipart related handler\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        htmltextPart = getTextPart(
                                            messages as *const *mut message,
                                            multiparts as size_t,
                                        );
                                        if htmltextPart >= 0 as libc::c_int && !messages.is_null() {
                                            if !(messageGetBody(
                                                *messages.offset(htmltextPart as isize),
                                            ))
                                            .is_null()
                                            {
                                                aText = textAddMessage(
                                                    aText,
                                                    *messages.offset(htmltextPart as isize),
                                                );
                                            }
                                        } else {
                                            i = 0 as libc::c_int;
                                            while i < multiparts {
                                                if messageGetMimeType(*messages.offset(i as isize))
                                                    as libc::c_uint
                                                    == MULTIPART as libc::c_int as libc::c_uint
                                                {
                                                    htmltextPart = i;
                                                    break;
                                                } else {
                                                    i += 1;
                                                }
                                            }
                                        }
                                        if htmltextPart == -(1 as libc::c_int) {
                                            cli_dbgmsg(
                                                b"No HTML code found to be scanned\n\0" as *const u8
                                                    as *const libc::c_char,
                                            );
                                            current_block_224 = 5015379258158208590;
                                        } else {
                                            if !((*(*mctx).ctx).wrkproperty).is_null() {
                                                parseRootMHTML(
                                                    mctx,
                                                    *messages.offset(htmltextPart as isize),
                                                    aText,
                                                );
                                            }
                                            rc = parseEmailBody(
                                                *messages.offset(htmltextPart as isize),
                                                aText,
                                                mctx,
                                                recursion_level
                                                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                                            );
                                            if rc as libc::c_uint
                                                == OK as libc::c_int as libc::c_uint
                                                && !(*messages.offset(htmltextPart as isize))
                                                    .is_null()
                                            {
                                                messageDestroy(
                                                    *messages.offset(htmltextPart as isize),
                                                );
                                                let ref mut fresh12 =
                                                    *messages.offset(htmltextPart as isize);
                                                *fresh12 = 0 as *mut message;
                                                current_block_224 = 5015379258158208590;
                                            } else if rc as libc::c_uint
                                                == VIRUS as libc::c_int as libc::c_uint
                                            {
                                                infected = 1 as libc::c_int != 0;
                                                current_block_224 = 316278526493857137;
                                            } else {
                                                current_block_224 = 5015379258158208590;
                                            }
                                        }
                                        match current_block_224 {
                                            316278526493857137 => {}
                                            _ => {
                                                current_block_224 = 16940090585895101844;
                                            }
                                        }
                                    }
                                    7 | 6 => {
                                        current_block_224 = 16940090585895101844;
                                    }
                                    11 => {
                                        current_block_224 = 17140609492894984117;
                                    }
                                    13 | 5 | 12 => {
                                        current_block_224 = 14837708850602755115;
                                    }
                                    8 | 9 => {
                                        if !messages.is_null() {
                                            htmltextPart = getTextPart(
                                                messages as *const *mut message,
                                                multiparts as size_t,
                                            );
                                            if htmltextPart == -(1 as libc::c_int) {
                                                htmltextPart = 0 as libc::c_int;
                                            }
                                            rc = parseEmailBody(
                                                *messages.offset(htmltextPart as isize),
                                                aText,
                                                mctx,
                                                recursion_level
                                                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                                            );
                                        }
                                        current_block_224 = 316278526493857137;
                                    }
                                    _ => {
                                        cli_dbgmsg(
                                            b"Unepxected mime sub type\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        rc = CL_EFORMAT as libc::c_int as mbox_status;
                                        current_block_224 = 316278526493857137;
                                    }
                                }
                                match current_block_224 {
                                    16940090585895101844 => {
                                        cli_dbgmsg(
                                            b"Multipart alternative handler\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        current_block_224 = 17140609492894984117;
                                    }
                                    _ => {}
                                }
                                match current_block_224 {
                                    17140609492894984117 => {
                                        current_block_224 = 14837708850602755115;
                                    }
                                    _ => {}
                                }
                                match current_block_224 {
                                    14837708850602755115 => {
                                        if !aText.is_null() {
                                            if !mainMessage.is_null() && mainMessage != messageIn {
                                                messageDestroy(mainMessage);
                                            }
                                            mainMessage = 0 as *mut message;
                                        }
                                        cli_dbgmsg(
                                            b"Mixed message with %d parts\n\0" as *const u8
                                                as *const libc::c_char,
                                            multiparts,
                                        );
                                        i = 0 as libc::c_int;
                                        while i < multiparts {
                                            mainMessage = do_multipart(
                                                mainMessage,
                                                messages,
                                                i,
                                                &mut rc,
                                                mctx,
                                                messageIn,
                                                &mut aText,
                                                recursion_level
                                                    .wrapping_add(1 as libc::c_int as libc::c_uint),
                                            );
                                            if rc as libc::c_uint
                                                == VIRUS as libc::c_int as libc::c_uint
                                            {
                                                infected = 1 as libc::c_int != 0;
                                                break;
                                            } else {
                                                if rc as libc::c_uint
                                                    == MAXREC as libc::c_int as libc::c_uint
                                                {
                                                    break;
                                                }
                                                if rc as libc::c_uint
                                                    == OK_ATTACHMENTS_NOT_SAVED as libc::c_int
                                                        as libc::c_uint
                                                {
                                                    rc = OK;
                                                }
                                                i += 1;
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                if !mainMessage.is_null() && mainMessage != messageIn {
                                    messageDestroy(mainMessage);
                                }
                                if !aText.is_null() && textIn.is_null() {
                                    if !infected && {
                                        fb = fileblobCreate();
                                        !fb.is_null()
                                    } {
                                        cli_dbgmsg(
                                            b"Save non mime and/or text/plain part\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        fileblobSetFilename(
                                            fb,
                                            (*mctx).dir,
                                            b"textpart\0" as *const u8 as *const libc::c_char,
                                        );
                                        fileblobSetCTX(fb, (*mctx).ctx);
                                        textToFileblob(aText, fb, 1 as libc::c_int);
                                        fileblobDestroy(fb);
                                        let ref mut fresh13 = (*mctx).files;
                                        *fresh13 = (*fresh13).wrapping_add(1);
                                    }
                                    textDestroy(aText);
                                }
                                if !messages.is_null() {
                                    i = 0 as libc::c_int;
                                    while i < multiparts {
                                        if !(*messages.offset(i as isize)).is_null() {
                                            messageDestroy(*messages.offset(i as isize));
                                        }
                                        i += 1;
                                    }
                                    free(messages as *mut libc::c_void);
                                    messages = 0 as *mut *mut message;
                                }
                                let ref mut fresh14 = (*mctx).wrkobj;
                                *fresh14 = saveobj;
                                return rc;
                            }
                        }
                    }
                }
                current_block_306 = 3011375068465243373;
            }
            4 => {
                match messageGetEncoding(mainMessage) as libc::c_uint {
                    0 | 3 | 4 => {}
                    _ => {
                        cli_dbgmsg(
                            b"MIME type 'message' cannot be decoded\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
                rc = FAIL;
                if strcasecmp(mimeSubtype, b"rfc822\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                    || strcasecmp(
                        mimeSubtype,
                        b"delivery-status\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                {
                    let m_0: *mut message =
                        parseEmailHeaders(mainMessage, (*mctx).rfc821Table, &mut heuristicFound);
                    if !m_0.is_null() {
                        cli_dbgmsg(b"Decode rfc822\n\0" as *const u8 as *const libc::c_char);
                        messageSetCTX(m_0, (*mctx).ctx);
                        if !mainMessage.is_null() && mainMessage != messageIn {
                            messageDestroy(mainMessage);
                            mainMessage = 0 as *mut message;
                        } else {
                            messageReset(mainMessage);
                        }
                        if !(messageGetBody(m_0)).is_null() {
                            rc = parseEmailBody(
                                m_0,
                                0 as *mut text,
                                mctx,
                                recursion_level.wrapping_add(1 as libc::c_int as libc::c_uint),
                            );
                        }
                        messageDestroy(m_0);
                    } else if heuristicFound {
                        rc = VIRUS;
                    }
                } else if strcasecmp(
                    mimeSubtype,
                    b"disposition-notification\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    rc = OK;
                } else {
                    if strcasecmp(
                        mimeSubtype,
                        b"partial\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        if (*(*(*mctx).ctx).options).mail & 0x1 as libc::c_int as libc::c_uint != 0
                        {
                            if rfc1341(mctx, mainMessage) >= 0 as libc::c_int {
                                rc = OK;
                            }
                        } else {
                            cli_warnmsg(
                                b"Partial message received from MUA/MTA - message cannot be scanned\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                    } else if strcasecmp(
                        mimeSubtype,
                        b"external-body\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        cli_warnmsg(
                            b"Attempt to send Content-type message/external-body trapped\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                    } else {
                        cli_warnmsg(
                            b"Unsupported message format `%s' - if you believe this file contains a virus, submit it to www.clamav.net\n\0"
                                as *const u8 as *const libc::c_char,
                            mimeSubtype,
                        );
                    }
                    if !mainMessage.is_null() && mainMessage != messageIn {
                        messageDestroy(mainMessage);
                    }
                    if !messages.is_null() {
                        i = 0 as libc::c_int;
                        while i < multiparts {
                            if !(*messages.offset(i as isize)).is_null() {
                                messageDestroy(*messages.offset(i as isize));
                            }
                            i += 1;
                        }
                        free(messages as *mut libc::c_void);
                        messages = 0 as *mut *mut message;
                    }
                    let ref mut fresh15 = (*mctx).wrkobj;
                    *fresh15 = saveobj;
                    return rc;
                }
                current_block_306 = 3011375068465243373;
            }
            1 => {
                current_block_306 = 16449450280359929453;
            }
            2 | 7 | 3 => {
                current_block_306 = 3011375068465243373;
            }
            _ => {
                cli_dbgmsg(
                    b"Message received with unknown mime encoding - assume application\n\0"
                        as *const u8 as *const libc::c_char,
                );
                current_block_306 = 16449450280359929453;
            }
        }
        match current_block_306 {
            13845630548962704447 => {
                if doPhishingScan != 0 {
                    checkURLs(
                        mainMessage,
                        mctx,
                        &mut rc,
                        (subtype_0 == 3 as libc::c_int) as libc::c_int,
                    );
                    if rc as libc::c_uint == VIRUS as libc::c_int as libc::c_uint {
                        infected = 1 as libc::c_int != 0;
                    }
                }
            }
            16449450280359929453 => {
                fb = messageToFileblob(mainMessage, (*mctx).dir, 1 as libc::c_int);
                if !fb.is_null() {
                    cli_dbgmsg(
                        b"Saving main message as attachment\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    if fileblobScanAndDestroy(fb) == CL_VIRUS as libc::c_int {
                        rc = VIRUS;
                    }
                    let ref mut fresh16 = (*mctx).files;
                    *fresh16 = (*fresh16).wrapping_add(1);
                    if mainMessage != messageIn {
                        messageDestroy(mainMessage);
                        mainMessage = 0 as *mut message;
                    } else {
                        messageReset(mainMessage);
                    }
                }
            }
            _ => {}
        }
        if !messages.is_null() {
            cli_warnmsg(b"messages != NULL\n\0" as *const u8 as *const libc::c_char);
            i = 0 as libc::c_int;
            while i < multiparts {
                if !(*messages.offset(i as isize)).is_null() {
                    messageDestroy(*messages.offset(i as isize));
                }
                i += 1;
            }
            free(messages as *mut libc::c_void);
            messages = 0 as *mut *mut message;
        }
    }
    if !aText.is_null() && textIn.is_null() {
        let mut t: *const text = 0 as *const text;
        let mut lookahead_definately_is_bounce: bool = 0 as libc::c_int != 0;
        let mut current_block_361: u64;
        t = aText;
        while !t.is_null() && rc as libc::c_uint != VIRUS as libc::c_int as libc::c_uint {
            let mut l: *const line_t = (*t).t_line;
            let mut lookahead: *const text = 0 as *const text;
            let mut topofbounce: *const text = 0 as *const text;
            let mut s: *const libc::c_char = 0 as *const libc::c_char;
            let mut inheader: bool = false;
            if !l.is_null() {
                if lookahead_definately_is_bounce {
                    lookahead_definately_is_bounce = 0 as libc::c_int != 0;
                    current_block_361 = 15030729790988239748;
                } else if !isBounceStart(mctx, lineGetData(l)) {
                    current_block_361 = 1710542247040572269;
                } else {
                    current_block_361 = 15030729790988239748;
                }
                match current_block_361 {
                    1710542247040572269 => {}
                    _ => {
                        lookahead = (*t).t_next;
                        if lookahead.is_null() {
                            break;
                        }
                        if isBounceStart(mctx, lineGetData((*lookahead).t_line)) {
                            lookahead_definately_is_bounce = 1 as libc::c_int != 0;
                        } else {
                            while !lookahead.is_null() {
                                l = (*lookahead).t_line;
                                if l.is_null() {
                                    break;
                                }
                                s = lineGetData(l);
                                if strncasecmp(
                                    s,
                                    b"Content-Type:\0" as *const u8 as *const libc::c_char,
                                    13 as libc::c_int as libc::c_ulong,
                                ) == 0 as libc::c_int
                                {
                                    if (__cli_strcasestr(
                                        s,
                                        b"text/plain\0" as *const u8 as *const libc::c_char,
                                    ))
                                    .is_null()
                                    {
                                        if !(doPhishingScan == 0
                                            && !(__cli_strcasestr(
                                                s,
                                                b"text/html\0" as *const u8 as *const libc::c_char,
                                            ))
                                            .is_null())
                                        {
                                            break;
                                        }
                                    }
                                }
                                lookahead = (*lookahead).t_next;
                            }
                            if !lookahead.is_null() && ((*lookahead).t_line).is_null() {
                                cli_dbgmsg(
                                    b"Non mime part bounce message is not mime encoded, so it will not be scanned\n\0"
                                        as *const u8 as *const libc::c_char,
                                );
                                t = lookahead;
                            } else {
                                while !lookahead.is_null() {
                                    l = (*lookahead).t_line;
                                    if !l.is_null() {
                                        s = lineGetData(l);
                                        if strncasecmp(
                                            s,
                                            b"Content-Type:\0" as *const u8 as *const libc::c_char,
                                            13 as libc::c_int as libc::c_ulong,
                                        ) == 0 as libc::c_int
                                            && (strstr(
                                                s,
                                                b"multipart/\0" as *const u8 as *const libc::c_char,
                                            ))
                                            .is_null()
                                            && (strstr(
                                                s,
                                                b"message/rfc822\0" as *const u8
                                                    as *const libc::c_char,
                                            ))
                                            .is_null()
                                            && (strstr(
                                                s,
                                                b"text/plain\0" as *const u8 as *const libc::c_char,
                                            ))
                                            .is_null()
                                        {
                                            break;
                                        }
                                    }
                                    lookahead = (*lookahead).t_next;
                                }
                                if lookahead.is_null() {
                                    cli_dbgmsg(
                                        b"cli_mbox: I believe it's plain text which must be clean\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                    break;
                                } else {
                                    fb = fileblobCreate();
                                    if fb.is_null() {
                                        break;
                                    }
                                    cli_dbgmsg(
                                        b"Save non mime part bounce message\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    fileblobSetFilename(
                                        fb,
                                        (*mctx).dir,
                                        b"bounce\0" as *const u8 as *const libc::c_char,
                                    );
                                    fileblobAddData(
                                        fb,
                                        b"Received: by clamd (bounce)\n\0" as *const u8
                                            as *const libc::c_char
                                            as *const libc::c_uchar,
                                        28 as libc::c_int as size_t,
                                    );
                                    fileblobSetCTX(fb, (*mctx).ctx);
                                    inheader = 1 as libc::c_int != 0;
                                    topofbounce = 0 as *const text;
                                    loop {
                                        l = (*t).t_line;
                                        if l.is_null() {
                                            if inheader {
                                                inheader = 0 as libc::c_int != 0;
                                                topofbounce = t;
                                            }
                                        } else {
                                            s = lineGetData(l);
                                            fileblobAddData(
                                                fb,
                                                s as *const libc::c_uchar,
                                                strlen(s),
                                            );
                                        }
                                        fileblobAddData(
                                            fb,
                                            b"\n\0" as *const u8 as *const libc::c_char
                                                as *const libc::c_uchar,
                                            1 as libc::c_int as size_t,
                                        );
                                        lookahead = (*t).t_next;
                                        if lookahead.is_null() {
                                            break;
                                        }
                                        t = lookahead;
                                        l = (*t).t_line;
                                        if !inheader && !l.is_null() {
                                            s = lineGetData(l);
                                            if isBounceStart(mctx, s) {
                                                cli_dbgmsg(
                                                    b"Found the start of another bounce candidate (%s)\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                    s,
                                                );
                                                lookahead_definately_is_bounce =
                                                    1 as libc::c_int != 0;
                                                break;
                                            }
                                        }
                                        if !(fileblobInfected(fb) == 0) {
                                            break;
                                        }
                                    }
                                    if fileblobScanAndDestroy(fb) == CL_VIRUS as libc::c_int {
                                        rc = VIRUS;
                                    }
                                    let ref mut fresh17 = (*mctx).files;
                                    *fresh17 = (*fresh17).wrapping_add(1);
                                    if !topofbounce.is_null() {
                                        t = topofbounce;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            t = (*t).t_next;
        }
        textDestroy(aText);
        aText = 0 as *mut text;
    }
    if !mainMessage.is_null() && rc as libc::c_uint != VIRUS as libc::c_int as libc::c_uint {
        let mut t_line_0: *mut text = 0 as *mut text;
        if !((*mainMessage).body_first).is_null() && !(encodingLine(mainMessage)).is_null() && {
            t_line_0 = bounceBegin(mainMessage);
            !t_line_0.is_null()
        } {
            rc = (if exportBounceMessage(mctx, t_line_0) == CL_VIRUS as libc::c_int {
                VIRUS as libc::c_int
            } else {
                OK as libc::c_int
            }) as mbox_status;
        } else {
            let mut saveIt: bool = false;
            if messageGetMimeType(mainMessage) as libc::c_uint
                == MESSAGE as libc::c_int as libc::c_uint
            {
                saveIt = !(encodingLine(mainMessage)).is_null();
            } else if !((*mainMessage).body_last).is_null() && {
                t_line_0 = encodingLine(mainMessage);
                !t_line_0.is_null()
            } {
                fb = fileblobCreate();
                if !fb.is_null() {
                    cli_dbgmsg(
                        b"Found a bounce message with no header at '%s'\n\0" as *const u8
                            as *const libc::c_char,
                        lineGetData((*t_line_0).t_line),
                    );
                    fileblobSetFilename(
                        fb,
                        (*mctx).dir,
                        b"bounce\0" as *const u8 as *const libc::c_char,
                    );
                    fileblobAddData(
                        fb,
                        b"Received: by clamd (bounce)\n\0" as *const u8 as *const libc::c_char
                            as *const libc::c_uchar,
                        28 as libc::c_int as size_t,
                    );
                    fileblobSetCTX(fb, (*mctx).ctx);
                    if fileblobScanAndDestroy(textToFileblob(t_line_0, fb, 1 as libc::c_int))
                        == CL_VIRUS as libc::c_int
                    {
                        rc = VIRUS;
                    }
                    let ref mut fresh18 = (*mctx).files;
                    *fresh18 = (*fresh18).wrapping_add(1);
                }
                saveIt = 0 as libc::c_int != 0;
            } else {
                saveIt = 1 as libc::c_int != 0;
            }
            if saveIt {
                cli_dbgmsg(
                    b"Saving text part to scan, rc = %d\n\0" as *const u8 as *const libc::c_char,
                    rc as libc::c_int,
                );
                if saveTextPart(mctx, mainMessage, 1 as libc::c_int) == CL_VIRUS as libc::c_int {
                    rc = VIRUS;
                }
                if mainMessage != messageIn {
                    messageDestroy(mainMessage);
                    mainMessage = 0 as *mut message;
                } else {
                    messageReset(mainMessage);
                }
            }
        }
    }
    if !mainMessage.is_null() && mainMessage != messageIn {
        messageDestroy(mainMessage);
    }
    if rc as libc::c_uint != FAIL as libc::c_int as libc::c_uint && infected as libc::c_int != 0 {
        rc = VIRUS;
    }
    let ref mut fresh19 = (*mctx).wrkobj;
    *fresh19 = saveobj;
    cli_dbgmsg(
        b"parseEmailBody() returning %d\n\0" as *const u8 as *const libc::c_char,
        rc as libc::c_int,
    );
    return rc;
}
unsafe extern "C" fn boundaryStart(
    line: *const libc::c_char,
    boundary: *const libc::c_char,
) -> libc::c_int {
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rc: libc::c_int = 0;
    let mut buf: [libc::c_char; 1001] = [0; 1001];
    let mut newline: *mut libc::c_char = 0 as *mut libc::c_char;
    if line.is_null() || *line as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int;
    }
    if boundary.is_null() {
        return 0 as libc::c_int;
    }
    newline = strdup(line);
    if newline.is_null() {
        newline = line as *mut libc::c_char;
    }
    if newline != line as *mut libc::c_char && strlen(line) != 0 {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        p = newline
            .offset(strlen(line) as isize)
            .offset(-(1 as libc::c_int as isize));
        while p >= newline && *p as libc::c_int == ' ' as i32 {
            let fresh20 = p;
            p = p.offset(-1);
            *fresh20 = '\0' as i32 as libc::c_char;
        }
    }
    if newline != line as *mut libc::c_char {
        cli_chomp(newline);
    }
    if *newline as libc::c_int != '-' as i32 && *newline as libc::c_int != '(' as i32 {
        if newline != line as *mut libc::c_char {
            free(newline as *mut libc::c_void);
        }
        return 0 as libc::c_int;
    }
    if (strchr(newline, '-' as i32)).is_null() {
        if newline != line as *mut libc::c_char {
            free(newline as *mut libc::c_void);
        }
        return 0 as libc::c_int;
    }
    if strlen(newline) <= ::std::mem::size_of::<[libc::c_char; 1001]>() as libc::c_ulong {
        out = 0 as *mut libc::c_char;
        ptr = rfc822comments(newline, buf.as_mut_ptr());
    } else {
        out = rfc822comments(newline, 0 as *mut libc::c_char);
        ptr = out;
    }
    if ptr.is_null() {
        ptr = newline;
    }
    let fresh21 = ptr;
    ptr = ptr.offset(1);
    if *fresh21 as libc::c_int != '-' as i32 || *ptr as libc::c_int == '\0' as i32 {
        if !out.is_null() {
            free(out as *mut libc::c_void);
        }
        if newline != line as *mut libc::c_char {
            free(newline as *mut libc::c_void);
        }
        return 0 as libc::c_int;
    }
    if !(strstr(&*ptr.offset(1 as libc::c_int as isize), boundary)).is_null()
        || !(strstr(newline, boundary)).is_null()
    {
        let mut k: *const libc::c_char = ptr;
        rc = 0 as libc::c_int;
        loop {
            k = k.offset(1);
            if strcmp(k, boundary) == 0 as libc::c_int {
                rc = 1 as libc::c_int;
                break;
            } else if !(*k as libc::c_int == '-' as i32) {
                break;
            }
        }
        if rc == 0 as libc::c_int {
            k = &*line.offset(1 as libc::c_int as isize) as *const libc::c_char;
            loop {
                k = k.offset(1);
                if strcmp(k, boundary) == 0 as libc::c_int {
                    rc = 1 as libc::c_int;
                    break;
                } else if !(*k as libc::c_int == '-' as i32) {
                    break;
                }
            }
        }
    } else {
        let fresh22 = ptr;
        ptr = ptr.offset(1);
        if *fresh22 as libc::c_int != '-' as i32 {
            rc = 0 as libc::c_int;
        } else {
            rc = (strcasecmp(ptr, boundary) == 0 as libc::c_int) as libc::c_int;
        }
    }
    if !out.is_null() {
        free(out as *mut libc::c_void);
    }
    if rc == 1 as libc::c_int {
        cli_dbgmsg(
            b"boundaryStart: found %s in %s\n\0" as *const u8 as *const libc::c_char,
            boundary,
            line,
        );
    }
    if newline != line as *mut libc::c_char {
        free(newline as *mut libc::c_void);
    }
    return rc;
}
unsafe extern "C" fn boundaryEnd(
    line: *const libc::c_char,
    boundary: *const libc::c_char,
) -> libc::c_int {
    let mut len: size_t = 0;
    let mut newline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    if line.is_null() || *line as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int;
    }
    newline = strdup(line);
    p = newline;
    if newline.is_null() {
        p = line as *mut libc::c_char;
        newline = line as *mut libc::c_char;
    }
    if newline != line as *mut libc::c_char && strlen(line) != 0 {
        p2 = newline
            .offset(strlen(line) as isize)
            .offset(-(1 as libc::c_int as isize));
        while p2 >= newline && *p2 as libc::c_int == ' ' as i32 {
            let fresh23 = p2;
            p2 = p2.offset(-1);
            *fresh23 = '\0' as i32 as libc::c_char;
        }
    }
    let fresh24 = p;
    p = p.offset(1);
    if *fresh24 as libc::c_int != '-' as i32 {
        if newline != line as *mut libc::c_char {
            free(newline as *mut libc::c_void);
        }
        return 0 as libc::c_int;
    }
    let fresh25 = p;
    p = p.offset(1);
    if *fresh25 as libc::c_int != '-' as i32 {
        if newline != line as *mut libc::c_char {
            free(newline as *mut libc::c_void);
        }
        return 0 as libc::c_int;
    }
    len = strlen(boundary);
    if strncasecmp(p, boundary, len) != 0 as libc::c_int {
        if newline != line as *mut libc::c_char {
            free(newline as *mut libc::c_void);
        }
        return 0 as libc::c_int;
    }
    if strlen(p) < len.wrapping_add(2 as libc::c_int as libc::c_ulong) {
        if newline != line as *mut libc::c_char {
            free(newline as *mut libc::c_void);
        }
        return 0 as libc::c_int;
    }
    p = &mut *p.offset(len as isize) as *mut libc::c_char;
    let fresh26 = p;
    p = p.offset(1);
    if *fresh26 as libc::c_int != '-' as i32 {
        if newline != line as *mut libc::c_char {
            free(newline as *mut libc::c_void);
        }
        return 0 as libc::c_int;
    }
    if *p as libc::c_int == '-' as i32 {
        if newline != line as *mut libc::c_char {
            free(newline as *mut libc::c_void);
        }
        return 1 as libc::c_int;
    }
    if newline != line as *mut libc::c_char {
        free(newline as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn initialiseTables(
    rfc821Table: *mut *mut table_t,
    subtypeTable: *mut *mut table_t,
) -> libc::c_int {
    let mut tableinit: *const tableinit = 0 as *const tableinit;
    if (*rfc821Table).is_null() {
        *rfc821Table = tableCreate();
        if (*rfc821Table).is_null() {
            return -(1 as libc::c_int);
        }
        tableinit = rfc821headers.as_ptr();
        while !((*tableinit).key).is_null() {
            if tableInsert(*rfc821Table, (*tableinit).key, (*tableinit).value) < 0 as libc::c_int {
                tableDestroy(*rfc821Table);
                *rfc821Table = 0 as *mut table_t;
                return -(1 as libc::c_int);
            }
            tableinit = tableinit.offset(1);
        }
    }
    if (*subtypeTable).is_null() {
        *subtypeTable = tableCreate();
        if (*subtypeTable).is_null() {
            tableDestroy(*rfc821Table);
            *rfc821Table = 0 as *mut table_t;
            return -(1 as libc::c_int);
        }
        tableinit = mimeSubtypes.as_ptr();
        while !((*tableinit).key).is_null() {
            if tableInsert(*subtypeTable, (*tableinit).key, (*tableinit).value) < 0 as libc::c_int {
                tableDestroy(*rfc821Table);
                tableDestroy(*subtypeTable);
                *rfc821Table = 0 as *mut table_t;
                *subtypeTable = 0 as *mut table_t;
                return -(1 as libc::c_int);
            }
            tableinit = tableinit.offset(1);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn getTextPart(messages: *const *mut message, size: size_t) -> libc::c_int {
    let mut i: size_t = 0;
    let mut textpart: libc::c_int = -(1 as libc::c_int);
    i = 0 as libc::c_int as size_t;
    while i < size {
        if !(*messages.offset(i as isize)).is_null()
            && messageGetMimeType(*messages.offset(i as isize)) as libc::c_uint
                == TEXT as libc::c_int as libc::c_uint
        {
            if strcasecmp(
                messageGetMimeSubtype(*messages.offset(i as isize)),
                b"html\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                return i as libc::c_int;
            }
            textpart = i as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return textpart;
}
unsafe extern "C" fn strip(buf: *mut libc::c_char, mut len: libc::c_int) -> size_t {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: size_t = 0;
    if buf.is_null() || len <= 0 as libc::c_int {
        return 0 as libc::c_int as size_t;
    }
    i = strlen(buf);
    if len > i.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int {
        return i;
    }
    len -= 1;
    ptr = &mut *buf.offset(len as isize) as *mut libc::c_char;
    loop {
        if *ptr != 0 {
            *ptr = '\0' as i32 as libc::c_char;
        }
        len -= 1;
        if !(len >= 0 as libc::c_int
            && {
                ptr = ptr.offset(-1);
                *(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize) as libc::c_int
                    & _ISgraph as libc::c_int as libc::c_ushort as libc::c_int
                    == 0
            }
            && *ptr as libc::c_int != '\n' as i32
            && *ptr as libc::c_int != '\r' as i32)
        {
            break;
        }
    }
    return (len + 1 as libc::c_int) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn strstrip(s: *mut libc::c_char) -> size_t {
    if s.is_null() {
        return 0 as libc::c_int as size_t;
    }
    return strip(
        s,
        (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
}
unsafe extern "C" fn parseMimeHeader(
    m: *mut message,
    cmd: *const libc::c_char,
    rfc821Table: *const table_t,
    arg: *const libc::c_char,
    ctx: *mut cli_ctx,
    heuristicFound: *mut bool,
) -> libc::c_int {
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut commandNumber: libc::c_int = 0;
    let mut argCnt: size_t = 0 as libc::c_int as size_t;
    *heuristicFound = 0 as libc::c_int != 0;
    cli_dbgmsg(
        b"parseMimeHeader: cmd='%s', arg='%s'\n\0" as *const u8 as *const libc::c_char,
        cmd,
        arg,
    );
    copy = rfc822comments(cmd, 0 as *mut libc::c_char);
    if !copy.is_null() {
        commandNumber = tableFind(rfc821Table, copy);
        free(copy as *mut libc::c_void);
    } else {
        commandNumber = tableFind(rfc821Table, cmd);
    }
    copy = rfc822comments(arg, 0 as *mut libc::c_char);
    if !copy.is_null() {
        ptr = copy;
    } else {
        ptr = arg;
    }
    buf = 0 as *mut libc::c_char;
    match commandNumber {
        1 => {
            if arg.is_null() {
                cli_dbgmsg(
                    b"Empty content-type received, no subtype specified, assuming text/plain; charset=us-ascii\n\0"
                        as *const u8 as *const libc::c_char,
                );
            } else if (strchr(ptr, '/' as i32)).is_null() {
                cli_dbgmsg(
                    b"Invalid content-type '%s' received, no subtype specified, assuming text/plain; charset=us-ascii\n\0"
                        as *const u8 as *const libc::c_char,
                    ptr,
                );
            } else {
                let mut i: libc::c_int = 0;
                buf = cli_malloc((strlen(ptr)).wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as *mut libc::c_char;
                if buf.is_null() {
                    cli_errmsg(
                        b"parseMimeHeader: Unable to allocate memory for buf %llu\n\0" as *const u8
                            as *const libc::c_char,
                        (strlen(ptr)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as libc::c_ulonglong,
                    );
                    if !copy.is_null() {
                        free(copy as *mut libc::c_void);
                    }
                    return -(1 as libc::c_int);
                }
                if *arg as libc::c_int == '/' as i32 {
                    cli_dbgmsg(
                        b"Content-type '/' received, assuming application/octet-stream\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    messageSetMimeType(m, b"application\0" as *const u8 as *const libc::c_char);
                    messageSetMimeSubtype(m, b"octet-stream\0" as *const u8 as *const libc::c_char);
                } else {
                    while *(*__ctype_b_loc()).offset(*ptr as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                    {
                        ptr = ptr.offset(1);
                    }
                    if *ptr.offset(0 as libc::c_int as isize) as libc::c_int == '"' as i32 {
                        ptr = ptr.offset(1);
                    }
                    if *ptr.offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32 {
                        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut strptr: *mut libc::c_char = 0 as *mut libc::c_char;
                        s = cli_strtokbuf(
                            ptr,
                            0 as libc::c_int,
                            b";\0" as *const u8 as *const libc::c_char,
                            buf,
                        );
                        if !s.is_null() && *s as libc::c_int != 0 {
                            let buf2: *mut libc::c_char = cli_strdup(buf);
                            if buf2.is_null() {
                                if !copy.is_null() {
                                    free(copy as *mut libc::c_void);
                                }
                                free(buf as *mut libc::c_void);
                                return -(1 as libc::c_int);
                            }
                            loop {
                                let set: libc::c_int = messageSetMimeType(
                                    m,
                                    strtok_r(
                                        s,
                                        b"/\0" as *const u8 as *const libc::c_char,
                                        &mut strptr,
                                    ),
                                );
                                s = strtok_r(
                                    0 as *mut libc::c_char,
                                    b";\0" as *const u8 as *const libc::c_char,
                                    &mut strptr,
                                );
                                if s.is_null() {
                                    break;
                                }
                                if set != 0 {
                                    let mut len: size_t = (strstrip(s))
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                                    if *s.offset(len as isize) as libc::c_int == '"' as i32 {
                                        *s.offset(len as isize) = '\0' as i32 as libc::c_char;
                                        len = strstrip(s);
                                    }
                                    if len != 0 {
                                        if !(strchr(s, ' ' as i32)).is_null() {
                                            messageSetMimeSubtype(
                                                m,
                                                cli_strtokbuf(
                                                    s,
                                                    0 as libc::c_int,
                                                    b" \0" as *const u8 as *const libc::c_char,
                                                    buf2,
                                                ),
                                            );
                                        } else {
                                            messageSetMimeSubtype(m, s);
                                        }
                                    }
                                }
                                while *s as libc::c_int != 0
                                    && *(*__ctype_b_loc())
                                        .offset(*s as libc::c_uchar as libc::c_int as isize)
                                        as libc::c_int
                                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                        == 0
                                {
                                    s = s.offset(1);
                                }
                                let fresh27 = s;
                                s = s.offset(1);
                                if *fresh27 as libc::c_int == '\0' as i32 {
                                    break;
                                }
                                if *s as libc::c_int == '\0' as i32 {
                                    break;
                                }
                            }
                            free(buf2 as *mut libc::c_void);
                        }
                    }
                }
                i = 1 as libc::c_int;
                loop {
                    let fresh28 = i;
                    i = i + 1;
                    if (cli_strtokbuf(
                        ptr,
                        fresh28,
                        b";\0" as *const u8 as *const libc::c_char,
                        buf,
                    ))
                    .is_null()
                    {
                        break;
                    }
                    cli_dbgmsg(
                        b"mimeArgs = '%s'\n\0" as *const u8 as *const libc::c_char,
                        buf,
                    );
                    argCnt = argCnt.wrapping_add(1);
                    if haveTooManyMIMEArguments(argCnt, ctx, heuristicFound) {
                        break;
                    }
                    messageAddArguments(m, buf);
                }
            }
        }
        2 => {
            messageSetEncoding(m, ptr);
        }
        3 => {
            buf = cli_malloc((strlen(ptr)).wrapping_add(1 as libc::c_int as libc::c_ulong))
                as *mut libc::c_char;
            if buf.is_null() {
                cli_errmsg(
                    b"parseMimeHeader: Unable to allocate memory for buf %llu\n\0" as *const u8
                        as *const libc::c_char,
                    (strlen(ptr)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as libc::c_ulonglong,
                );
                if !copy.is_null() {
                    free(copy as *mut libc::c_void);
                }
                return -(1 as libc::c_int);
            }
            p = cli_strtokbuf(
                ptr,
                0 as libc::c_int,
                b";\0" as *const u8 as *const libc::c_char,
                buf,
            );
            if !p.is_null() && *p as libc::c_int != 0 {
                messageSetDispositionType(m, p);
                messageAddArgument(
                    m,
                    cli_strtokbuf(
                        ptr,
                        1 as libc::c_int,
                        b";\0" as *const u8 as *const libc::c_char,
                        buf,
                    ),
                );
            }
            if messageHasFilename(m) == 0 {
                messageAddArgument(m, b"filename=unknown\0" as *const u8 as *const libc::c_char);
            }
        }
        _ => {}
    }
    if !copy.is_null() {
        free(copy as *mut libc::c_void);
    }
    if !buf.is_null() {
        free(buf as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn saveTextPart(
    mctx: *mut mbox_ctx,
    m: *mut message,
    destroy_text: libc::c_int,
) -> libc::c_int {
    let mut fb: *mut fileblob = 0 as *mut fileblob;
    messageAddArgument(
        m,
        b"filename=textportion\0" as *const u8 as *const libc::c_char,
    );
    fb = messageToFileblob(m, (*mctx).dir, destroy_text);
    if !fb.is_null() {
        cli_dbgmsg(b"Saving main message\n\0" as *const u8 as *const libc::c_char);
        let ref mut fresh29 = (*mctx).files;
        *fresh29 = (*fresh29).wrapping_add(1);
        return fileblobScanAndDestroy(fb);
    }
    return CL_ETMPFILE as libc::c_int;
}
unsafe extern "C" fn rfc822comments(
    mut in_0: *const libc::c_char,
    mut out: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut iptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut optr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut backslash: libc::c_int = 0;
    let mut inquote: libc::c_int = 0;
    let mut commentlevel: libc::c_int = 0;
    if in_0.is_null() || out == in_0 as *mut libc::c_char {
        cli_errmsg(b"rfc822comments: Invalid parameters.n\0" as *const u8 as *const libc::c_char);
        return 0 as *mut libc::c_char;
    }
    if (strchr(in_0, '(' as i32)).is_null() {
        return 0 as *mut libc::c_char;
    }
    while *(*__ctype_b_loc()).offset(*in_0 as libc::c_uchar as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
        != 0
    {
        in_0 = in_0.offset(1);
    }
    if out.is_null() {
        out = cli_malloc((strlen(in_0)).wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        if out.is_null() {
            cli_errmsg(
                b"rfc822comments: Unable to allocate memory for out %llu\n\0" as *const u8
                    as *const libc::c_char,
                (strlen(in_0)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_ulonglong,
            );
            return 0 as *mut libc::c_char;
        }
    }
    inquote = 0 as libc::c_int;
    commentlevel = inquote;
    backslash = commentlevel;
    optr = out;
    cli_dbgmsg(b"rfc822comments: contains a comment\n\0" as *const u8 as *const libc::c_char);
    iptr = in_0;
    while *iptr != 0 {
        if backslash != 0 {
            if commentlevel == 0 as libc::c_int {
                let fresh30 = optr;
                optr = optr.offset(1);
                *fresh30 = *iptr;
            }
            backslash = 0 as libc::c_int;
        } else {
            match *iptr as libc::c_int {
                92 => {
                    backslash = 1 as libc::c_int;
                }
                34 => {
                    let fresh31 = optr;
                    optr = optr.offset(1);
                    *fresh31 = '"' as i32 as libc::c_char;
                    inquote = (inquote == 0) as libc::c_int;
                }
                40 => {
                    if inquote != 0 {
                        let fresh32 = optr;
                        optr = optr.offset(1);
                        *fresh32 = '(' as i32 as libc::c_char;
                    } else {
                        commentlevel += 1;
                    }
                }
                41 => {
                    if inquote != 0 {
                        let fresh33 = optr;
                        optr = optr.offset(1);
                        *fresh33 = ')' as i32 as libc::c_char;
                    } else if commentlevel > 0 as libc::c_int {
                        commentlevel -= 1;
                    }
                }
                _ => {
                    if commentlevel == 0 as libc::c_int {
                        let fresh34 = optr;
                        optr = optr.offset(1);
                        *fresh34 = *iptr;
                    }
                }
            }
        }
        iptr = iptr.offset(1);
    }
    if backslash != 0 {
        let fresh35 = optr;
        optr = optr.offset(1);
        *fresh35 = '\\' as i32 as libc::c_char;
    }
    *optr = '\0' as i32 as libc::c_char;
    cli_dbgmsg(
        b"rfc822comments '%s'=>'%s'\n\0" as *const u8 as *const libc::c_char,
        in_0,
        out,
    );
    return out;
}
unsafe extern "C" fn rfc2047(mut in_0: *const libc::c_char) -> *mut libc::c_char {
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pout: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    if (strstr(in_0, b"=?\0" as *const u8 as *const libc::c_char)).is_null()
        || (strstr(in_0, b"?=\0" as *const u8 as *const libc::c_char)).is_null()
    {
        return cli_strdup(in_0);
    }
    cli_dbgmsg(
        b"rfc2047 '%s'\n\0" as *const u8 as *const libc::c_char,
        in_0,
    );
    out = cli_malloc((strlen(in_0)).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    if out.is_null() {
        cli_errmsg(
            b"rfc2047: Unable to allocate memory for out %llu\n\0" as *const u8
                as *const libc::c_char,
            (strlen(in_0)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_ulonglong,
        );
        return 0 as *mut libc::c_char;
    }
    pout = out;
    while *in_0 != 0 {
        let mut encoding: libc::c_char = 0;
        let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut enctext: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut m: *mut message = 0 as *mut message;
        let mut b: *mut blob = 0 as *mut blob;
        while *in_0 != 0 {
            if *in_0 as libc::c_int == '=' as i32
                && *in_0.offset(1 as libc::c_int as isize) as libc::c_int == '?' as i32
            {
                in_0 = in_0.offset(2 as libc::c_int as isize);
                break;
            } else {
                let fresh36 = in_0;
                in_0 = in_0.offset(1);
                let fresh37 = pout;
                pout = pout.offset(1);
                *fresh37 = *fresh36;
            }
        }
        while *in_0 as libc::c_int != '?' as i32 && *in_0 as libc::c_int != 0 {
            in_0 = in_0.offset(1);
        }
        if *in_0 as libc::c_int == '\0' as i32 {
            break;
        }
        in_0 = in_0.offset(1);
        encoding = *in_0;
        encoding = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = encoding as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(encoding as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc()).offset(encoding as libc::c_int as isize);
            }
            __res
        }) as libc::c_char;
        if encoding as libc::c_int != 'q' as i32 && encoding as libc::c_int != 'b' as i32 {
            cli_warnmsg(
                b"Unsupported RFC2047 encoding type '%c' - if you believe this file contains a virus, submit it to www.clamav.net\n\0"
                    as *const u8 as *const libc::c_char,
                encoding as libc::c_int,
            );
            free(out as *mut libc::c_void);
            out = 0 as *mut libc::c_char;
            break;
        } else {
            in_0 = in_0.offset(1);
            if *in_0 as libc::c_int != '?' as i32 {
                break;
            }
            in_0 = in_0.offset(1);
            if *in_0 as libc::c_int == '\0' as i32 {
                break;
            }
            enctext = cli_strdup(in_0);
            if enctext.is_null() {
                free(out as *mut libc::c_void);
                out = 0 as *mut libc::c_char;
                break;
            } else {
                in_0 = strstr(in_0, b"?=\0" as *const u8 as *const libc::c_char);
                if in_0.is_null() {
                    free(enctext as *mut libc::c_void);
                    break;
                } else {
                    in_0 = in_0.offset(2 as libc::c_int as isize);
                    ptr = strstr(enctext, b"?=\0" as *const u8 as *const libc::c_char);
                    if ptr.is_null() {
                        free(enctext as *mut libc::c_void);
                        break;
                    } else {
                        *ptr = '\0' as i32 as libc::c_char;
                        m = messageCreate();
                        if m.is_null() {
                            free(enctext as *mut libc::c_void);
                            break;
                        } else {
                            messageAddStr(m, enctext);
                            free(enctext as *mut libc::c_void);
                            enctext = 0 as *mut libc::c_char;
                            match encoding as libc::c_int {
                                113 => {
                                    messageSetEncoding(
                                        m,
                                        b"quoted-printable\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                98 => {
                                    messageSetEncoding(
                                        m,
                                        b"base64\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                _ => {}
                            }
                            b = messageToBlob(m, 1 as libc::c_int);
                            if b.is_null() {
                                messageDestroy(m);
                                break;
                            } else {
                                len = blobGetDataSize(b);
                                cli_dbgmsg(
                                    b"Decoded as '%*.*s'\n\0" as *const u8 as *const libc::c_char,
                                    len as libc::c_int,
                                    len as libc::c_int,
                                    blobGetData(b) as *const libc::c_char,
                                );
                                memcpy(
                                    pout as *mut libc::c_void,
                                    blobGetData(b) as *const libc::c_void,
                                    len,
                                );
                                blobDestroy(b);
                                messageDestroy(m);
                                if len > 0 as libc::c_int as libc::c_ulong
                                    && *pout
                                        .offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize)
                                        as libc::c_int
                                        == '\n' as i32
                                {
                                    pout = pout.offset(
                                        len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    );
                                } else {
                                    pout = pout.offset(len as isize);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if out.is_null() {
        return 0 as *mut libc::c_char;
    }
    *pout = '\0' as i32 as libc::c_char;
    cli_dbgmsg(
        b"rfc2047 returns '%s'\n\0" as *const u8 as *const libc::c_char,
        out,
    );
    return out;
}
unsafe extern "C" fn rfc1341(mctx: *mut mbox_ctx, m: *mut message) -> libc::c_int {
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut id: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut number: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut total: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oldfilename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmpdir: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: libc::c_int = 0;
    let mut pdir: [libc::c_char; 4097] = [0; 4097];
    let mut md5_val: [libc::c_uchar; 16] = [0; 16];
    let mut md5_hex: *mut libc::c_char = 0 as *mut libc::c_char;
    if mctx.is_null() || m.is_null() {
        cli_dbgmsg(b"rfc1341: Invalid NULL arguments\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    id = messageFindArgument(m, b"id\0" as *const u8 as *const libc::c_char);
    if id.is_null() {
        return -(1 as libc::c_int);
    }
    if !((*mctx).ctx).is_null() {
        tmpdir = cl_engine_get_str(
            (*(*mctx).ctx).engine,
            CL_ENGINE_TMPDIR,
            0 as *mut libc::c_int,
        );
    }
    if tmpdir.is_null() {
        tmpdir = cli_gettmpdir();
    }
    snprintf(
        pdir.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_char; 4097]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"%s/clamav-partial\0" as *const u8 as *const libc::c_char,
        tmpdir,
    );
    if mkdir(
        pdir.as_mut_ptr(),
        (0o400 as libc::c_int | 0o200 as libc::c_int) as __mode_t,
    ) < 0 as libc::c_int
        && *__errno_location() != 17 as libc::c_int
    {
        cli_errmsg(
            b"Can't create the directory '%s'\n\0" as *const u8 as *const libc::c_char,
            pdir.as_mut_ptr(),
        );
        free(id as *mut libc::c_void);
        return -(1 as libc::c_int);
    } else {
        if *__errno_location() == 17 as libc::c_int {
            let mut statb: stat = stat {
                st_dev: 0,
                st_ino: 0,
                st_nlink: 0,
                st_mode: 0,
                st_uid: 0,
                st_gid: 0,
                __pad0: 0,
                st_rdev: 0,
                st_size: 0,
                st_blksize: 0,
                st_blocks: 0,
                st_atim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_mtim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_ctim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                __glibc_reserved: [0; 3],
            };
            if stat(pdir.as_mut_ptr(), &mut statb) < 0 as libc::c_int {
                let mut err: [libc::c_char; 128] = [0; 128];
                cli_errmsg(
                    b"Partial directory %s: %s\n\0" as *const u8 as *const libc::c_char,
                    pdir.as_mut_ptr(),
                    cli_strerror(
                        *__errno_location(),
                        err.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                    ),
                );
                free(id as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            if statb.st_mode & 0o77 as libc::c_int as libc::c_uint != 0 {
                cli_warnmsg(
                    b"Insecure partial directory %s (mode 0%o)\n\0" as *const u8
                        as *const libc::c_char,
                    pdir.as_mut_ptr(),
                    (statb.st_mode
                        & (0o400 as libc::c_int
                            | 0o200 as libc::c_int
                            | 0o100 as libc::c_int
                            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                                >> 3 as libc::c_int
                            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                                >> 3 as libc::c_int
                                >> 3 as libc::c_int) as libc::c_uint)
                        as libc::c_int,
                );
            }
        }
    }
    number = messageFindArgument(m, b"number\0" as *const u8 as *const libc::c_char);
    if number.is_null() {
        free(id as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    oldfilename = messageGetFilename(m);
    arg = cli_malloc(
        (10 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen(id))
            .wrapping_add(strlen(number)),
    ) as *mut libc::c_char;
    if !arg.is_null() {
        sprintf(
            arg,
            b"filename=%s%s\0" as *const u8 as *const libc::c_char,
            id,
            number,
        );
        messageAddArgument(m, arg);
        free(arg as *mut libc::c_void);
    }
    if !oldfilename.is_null() {
        cli_dbgmsg(
            b"Must reset to %s\n\0" as *const u8 as *const libc::c_char,
            oldfilename,
        );
        free(oldfilename as *mut libc::c_void);
    }
    n = atoi(number);
    cl_hash_data(
        b"md5\0" as *const u8 as *const libc::c_char,
        id as *const libc::c_void,
        strlen(id),
        md5_val.as_mut_ptr(),
        0 as *mut libc::c_uint,
    );
    md5_hex = cli_str2hex(
        md5_val.as_mut_ptr() as *const libc::c_char,
        16 as libc::c_int as libc::c_uint,
    );
    if md5_hex.is_null() {
        free(id as *mut libc::c_void);
        free(number as *mut libc::c_void);
        return CL_EMEM as libc::c_int;
    }
    if messageSavePartial(m, pdir.as_mut_ptr(), md5_hex, n as libc::c_uint) < 0 as libc::c_int {
        free(md5_hex as *mut libc::c_void);
        free(id as *mut libc::c_void);
        free(number as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    total = messageFindArgument(m, b"total\0" as *const u8 as *const libc::c_char);
    cli_dbgmsg(
        b"rfc1341: %s, %s of %s\n\0" as *const u8 as *const libc::c_char,
        id,
        number,
        if !total.is_null() {
            total as *const libc::c_char
        } else {
            b"?\0" as *const u8 as *const libc::c_char
        },
    );
    if !total.is_null() {
        let t: libc::c_int = atoi(total);
        let mut dd: *mut DIR = 0 as *mut DIR;
        free(total as *mut libc::c_void);
        if n == t && {
            dd = opendir(pdir.as_mut_ptr());
            !dd.is_null()
        } {
            let mut fout: *mut FILE = 0 as *mut FILE;
            let mut outname: [libc::c_char; 4097] = [0; 4097];
            let mut now: time_t = 0;
            sanitiseName(id);
            snprintf(
                outname.as_mut_ptr(),
                (::std::mem::size_of::<[libc::c_char; 4097]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                b"%s/%s\0" as *const u8 as *const libc::c_char,
                (*mctx).dir,
                id,
            );
            cli_dbgmsg(
                b"outname: %s\n\0" as *const u8 as *const libc::c_char,
                outname.as_mut_ptr(),
            );
            fout = fopen(
                outname.as_mut_ptr(),
                b"wb\0" as *const u8 as *const libc::c_char,
            );
            if fout.is_null() {
                cli_errmsg(
                    b"Can't open '%s' for writing\0" as *const u8 as *const libc::c_char,
                    outname.as_mut_ptr(),
                );
                free(id as *mut libc::c_void);
                free(number as *mut libc::c_void);
                free(md5_hex as *mut libc::c_void);
                closedir(dd);
                return -(1 as libc::c_int);
            }
            time(&mut now);
            n = 1 as libc::c_int;
            while n <= t {
                let mut filename: [libc::c_char; 257] = [0; 257];
                let mut dent: *mut dirent = 0 as *mut dirent;
                snprintf(
                    filename.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 257]>() as libc::c_ulong,
                    b"_%s-%u\0" as *const u8 as *const libc::c_char,
                    md5_hex,
                    n,
                );
                loop {
                    dent = readdir(dd);
                    if dent.is_null() {
                        break;
                    }
                    let mut fin: *mut FILE = 0 as *mut FILE;
                    let mut buffer: [libc::c_char; 8192] = [0; 8192];
                    let mut fullname: [libc::c_char; 4354] = [0; 4354];
                    let mut nblanks: libc::c_int = 0;
                    let mut statb_0: stat = stat {
                        st_dev: 0,
                        st_ino: 0,
                        st_nlink: 0,
                        st_mode: 0,
                        st_uid: 0,
                        st_gid: 0,
                        __pad0: 0,
                        st_rdev: 0,
                        st_size: 0,
                        st_blksize: 0,
                        st_blocks: 0,
                        st_atim: timespec {
                            tv_sec: 0,
                            tv_nsec: 0,
                        },
                        st_mtim: timespec {
                            tv_sec: 0,
                            tv_nsec: 0,
                        },
                        st_ctim: timespec {
                            tv_sec: 0,
                            tv_nsec: 0,
                        },
                        __glibc_reserved: [0; 3],
                    };
                    let mut dentry_idpart: *const libc::c_char = 0 as *const libc::c_char;
                    let mut test_fd: libc::c_int = 0;
                    if (*dent).d_ino == 0 as libc::c_int as libc::c_ulong {
                        continue;
                    }
                    if strcmp(
                        b".\0" as *const u8 as *const libc::c_char,
                        ((*dent).d_name).as_mut_ptr(),
                    ) == 0
                        || strcmp(
                            b"..\0" as *const u8 as *const libc::c_char,
                            ((*dent).d_name).as_mut_ptr(),
                        ) == 0
                    {
                        continue;
                    }
                    snprintf(
                        fullname.as_mut_ptr(),
                        (::std::mem::size_of::<[libc::c_char; 4354]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        b"%s/%s\0" as *const u8 as *const libc::c_char,
                        pdir.as_mut_ptr(),
                        ((*dent).d_name).as_mut_ptr(),
                    );
                    dentry_idpart = strchr(((*dent).d_name).as_mut_ptr(), '_' as i32);
                    if dentry_idpart.is_null()
                        || strcmp(filename.as_mut_ptr(), dentry_idpart) != 0 as libc::c_int
                    {
                        if (*(*(*m).ctx).engine).keeptmp == 0 {
                            continue;
                        }
                        test_fd = open(fullname.as_mut_ptr(), 0 as libc::c_int | 0 as libc::c_int);
                        if test_fd < 0 as libc::c_int {
                            continue;
                        }
                        if fstat(test_fd, &mut statb_0) < 0 as libc::c_int {
                            close(test_fd);
                        } else {
                            if now - statb_0.st_mtim.tv_sec
                                > (7 as libc::c_int * 24 as libc::c_int * 3600 as libc::c_int)
                                    as time_t
                            {
                                if cli_unlink(fullname.as_mut_ptr()) != 0 {
                                    cli_unlink(outname.as_mut_ptr());
                                    fclose(fout);
                                    free(md5_hex as *mut libc::c_void);
                                    free(id as *mut libc::c_void);
                                    free(number as *mut libc::c_void);
                                    closedir(dd);
                                    close(test_fd);
                                    return -(1 as libc::c_int);
                                }
                            }
                            close(test_fd);
                        }
                    } else {
                        fin = fopen(
                            fullname.as_mut_ptr(),
                            b"rb\0" as *const u8 as *const libc::c_char,
                        );
                        if fin.is_null() {
                            cli_errmsg(
                                b"Can't open '%s' for reading\0" as *const u8
                                    as *const libc::c_char,
                                fullname.as_mut_ptr(),
                            );
                            fclose(fout);
                            cli_unlink(outname.as_mut_ptr());
                            free(md5_hex as *mut libc::c_void);
                            free(id as *mut libc::c_void);
                            free(number as *mut libc::c_void);
                            closedir(dd);
                            return -(1 as libc::c_int);
                        }
                        nblanks = 0 as libc::c_int;
                        while !(fgets(
                            buffer.as_mut_ptr(),
                            (::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as libc::c_int,
                            fin,
                        ))
                        .is_null()
                        {
                            if buffer[0 as libc::c_int as usize] as libc::c_int == '\n' as i32 {
                                nblanks += 1;
                            } else {
                                if nblanks != 0 {
                                    while !(putc('\n' as i32, fout) == -(1 as libc::c_int)) {
                                        nblanks -= 1;
                                        if !(nblanks > 0 as libc::c_int) {
                                            break;
                                        }
                                    }
                                }
                                if nblanks != 0
                                    || fputs(buffer.as_mut_ptr(), fout) == -(1 as libc::c_int)
                                {
                                    fclose(fin);
                                    fclose(fout);
                                    cli_unlink(outname.as_mut_ptr());
                                    free(md5_hex as *mut libc::c_void);
                                    free(id as *mut libc::c_void);
                                    free(number as *mut libc::c_void);
                                    closedir(dd);
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                        fclose(fin);
                        if (*(*(*m).ctx).engine).keeptmp == 0 {
                            if cli_unlink(fullname.as_mut_ptr()) != 0 {
                                fclose(fout);
                                cli_unlink(outname.as_mut_ptr());
                                free(md5_hex as *mut libc::c_void);
                                free(id as *mut libc::c_void);
                                free(number as *mut libc::c_void);
                                closedir(dd);
                                return -(1 as libc::c_int);
                            }
                        }
                        break;
                    }
                }
                rewinddir(dd);
                n += 1;
            }
            closedir(dd);
            fclose(fout);
        }
    }
    free(number as *mut libc::c_void);
    free(id as *mut libc::c_void);
    free(md5_hex as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn hrefs_done(b: *mut blob, hrefs: *mut tag_arguments_t) {
    if !b.is_null() {
        blobDestroy(b);
    }
    html_tag_arg_free(hrefs);
}
unsafe extern "C" fn extract_text_urls(
    mem: *const libc::c_uchar,
    len: size_t,
    hrefs: *mut tag_arguments_t,
) {
    let mut url: [libc::c_char; 1024] = [0; 1024];
    let mut off: size_t = 0;
    off = 0 as libc::c_int as size_t;
    while off.wrapping_add(10 as libc::c_int as libc::c_ulong) < len {
        let mut proto: int32_t = (*(mem.offset(off as isize) as *const unaligned_32)).una_s32;
        proto |= 0x20202020 as libc::c_int;
        if proto == 0x70747468 as libc::c_int
            && (*mem.offset(off.wrapping_add(4 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == ':' as i32
                || *mem.offset(off.wrapping_add(5 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
                    == 's' as i32
                    && *mem.offset(off.wrapping_add(6 as libc::c_int as libc::c_ulong) as isize)
                        as libc::c_int
                        == ':' as i32)
            || proto == 0x3a707466 as libc::c_int
        {
            let mut url_len: size_t = 0;
            url_len = 4 as libc::c_int as size_t;
            while off.wrapping_add(url_len) < len
                && url_len
                    < (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            {
                let c: libc::c_uchar = *mem.offset(off.wrapping_add(url_len) as isize);
                if c as libc::c_int == ' ' as i32
                    || c as libc::c_int == '\n' as i32
                    || c as libc::c_int == '\t' as i32
                {
                    break;
                }
                url_len = url_len.wrapping_add(1);
            }
            memcpy(
                url.as_mut_ptr() as *mut libc::c_void,
                mem.offset(off as isize) as *const libc::c_void,
                url_len,
            );
            url[url_len as usize] = '\0' as i32 as libc::c_char;
            html_tag_arg_add(
                hrefs,
                b"href\0" as *const u8 as *const libc::c_char,
                url.as_mut_ptr(),
            );
            off = (off as libc::c_ulong).wrapping_add(url_len) as size_t as size_t;
        }
        off = off.wrapping_add(1);
    }
}
unsafe extern "C" fn getHrefs(m: *mut message, mut hrefs: *mut tag_arguments_t) -> *mut blob {
    let mut mem: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let b: *mut blob = messageToBlob(m, 0 as libc::c_int);
    let mut len: size_t = 0;
    if b.is_null() {
        return 0 as *mut blob;
    }
    len = blobGetDataSize(b);
    if len == 0 as libc::c_int as libc::c_ulong {
        blobDestroy(b);
        return 0 as *mut blob;
    }
    if len > (100 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong {
        cli_dbgmsg(
            b"Viruses pointed to by URLs not scanned in large message\n\0" as *const u8
                as *const libc::c_char,
        );
        blobDestroy(b);
        return 0 as *mut blob;
    }
    (*hrefs).count = 0 as libc::c_int;
    let ref mut fresh38 = (*hrefs).value;
    *fresh38 = 0 as *mut *mut libc::c_uchar;
    let ref mut fresh39 = (*hrefs).tag;
    *fresh39 = *fresh38;
    let ref mut fresh40 = (*hrefs).contents;
    *fresh40 = 0 as *mut *mut libc::c_uchar;
    cli_dbgmsg(b"getHrefs: calling html_normalise_mem\n\0" as *const u8 as *const libc::c_char);
    mem = blobGetData(b);
    if html_normalise_mem(
        mem,
        len as off_t,
        0 as *const libc::c_char,
        hrefs,
        (*(*m).ctx).dconf,
    ) == 0
    {
        blobDestroy(b);
        return 0 as *mut blob;
    }
    cli_dbgmsg(b"getHrefs: html_normalise_mem returned\n\0" as *const u8 as *const libc::c_char);
    if (*hrefs).count == 0 && (*hrefs).scanContents != 0 {
        extract_text_urls(mem, len, hrefs);
    }
    return b;
}
unsafe extern "C" fn checkURLs(
    mainMessage: *mut message,
    mctx: *mut mbox_ctx,
    rc: *mut mbox_status,
    _is_html: libc::c_int,
) {
    let mut b: *mut blob = 0 as *mut blob;
    let mut hrefs: tag_arguments_t = tag_arguments_t {
        count: 0,
        scanContents: 0,
        tag: 0 as *mut *mut libc::c_uchar,
        value: 0 as *mut *mut libc::c_uchar,
        contents: 0 as *mut *mut libc::c_uchar,
    };
    if *rc as libc::c_uint == VIRUS as libc::c_int as libc::c_uint {
        return;
    }
    hrefs.scanContents = ((*(*(*mctx).ctx).engine).dboptions & 0x8 as libc::c_int as libc::c_uint
        != 0
        && (*(*(*mctx).ctx).dconf).phishing & 0x1 as libc::c_int as libc::c_uint != 0)
        as libc::c_int;
    if hrefs.scanContents == 0 {
        return;
    }
    hrefs.count = 0 as libc::c_int;
    hrefs.value = 0 as *mut *mut libc::c_uchar;
    hrefs.tag = hrefs.value;
    hrefs.contents = 0 as *mut *mut libc::c_uchar;
    b = getHrefs(mainMessage, &mut hrefs);
    if !b.is_null() {
        if hrefs.scanContents != 0 {
            if phishingScan((*mctx).ctx, &mut hrefs) as libc::c_uint
                == CL_VIRUS as libc::c_int as libc::c_uint
            {
                (*mainMessage).set_isInfected(1 as libc::c_int as libc::c_uint);
                *rc = VIRUS;
                cli_dbgmsg(b"PH:Phishing found\n\0" as *const u8 as *const libc::c_char);
            }
        }
    }
    hrefs_done(b, &mut hrefs);
}
unsafe extern "C" fn usefulHeader(commandNumber: libc::c_int, cmd: *const libc::c_char) -> bool {
    match commandNumber {
        2 | 3 | 1 => return 1 as libc::c_int != 0,
        _ => {
            if strcasecmp(cmd, b"From\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                return 1 as libc::c_int != 0;
            }
            if strcasecmp(cmd, b"Received\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                return 1 as libc::c_int != 0;
            }
            if strcasecmp(cmd, b"De\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                return 1 as libc::c_int != 0;
            }
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn getline_from_mbox(
    buffer: *mut libc::c_char,
    buffer_len: size_t,
    map: *mut fmap_t,
    at: *mut size_t,
) -> *mut libc::c_char {
    let mut src: *const libc::c_char = 0 as *const libc::c_char;
    let mut cursrc: *const libc::c_char = 0 as *const libc::c_char;
    let mut curbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: size_t = 0;
    let mut input_len: size_t = if ((*map).len).wrapping_sub(*at)
        < buffer_len.wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        ((*map).len).wrapping_sub(*at)
    } else {
        buffer_len.wrapping_add(1 as libc::c_int as libc::c_ulong)
    };
    cursrc = fmap_need_off_once(map, *at, input_len) as *const libc::c_char;
    src = cursrc;
    if src.is_null() {
        cli_dbgmsg(b"getline_from_mbox: fmap need failed\n\0" as *const u8 as *const libc::c_char);
        return 0 as *mut libc::c_char;
    }
    if buffer_len == 0 as libc::c_int as libc::c_ulong || buffer.is_null() {
        cli_errmsg(
            b"Invalid call to getline_from_mbox(). Refer to https://docs.clamav.net/manual/Installing.html\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as *mut libc::c_char;
    }
    curbuf = buffer;
    i = 0 as libc::c_int as size_t;
    while i < buffer_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        let mut c: libc::c_char = 0;
        let fresh41 = input_len;
        input_len = input_len.wrapping_sub(1);
        if fresh41 == 0 {
            if curbuf == buffer {
                return 0 as *mut libc::c_char;
            }
            break;
        } else {
            let fresh42 = cursrc;
            cursrc = cursrc.offset(1);
            c = *fresh42;
            match c as libc::c_int {
                0 => {}
                10 => {
                    let fresh43 = curbuf;
                    curbuf = curbuf.offset(1);
                    *fresh43 = '\n' as i32 as libc::c_char;
                    if input_len != 0 && *cursrc as libc::c_int == '\r' as i32 {
                        i = i.wrapping_add(1);
                        cursrc = cursrc.offset(1);
                    }
                    break;
                }
                13 => {
                    let fresh44 = curbuf;
                    curbuf = curbuf.offset(1);
                    *fresh44 = '\r' as i32 as libc::c_char;
                    if input_len != 0 && *cursrc as libc::c_int == '\n' as i32 {
                        i = i.wrapping_add(1);
                        cursrc = cursrc.offset(1);
                    }
                    break;
                }
                _ => {
                    let fresh45 = curbuf;
                    curbuf = curbuf.offset(1);
                    *fresh45 = c;
                }
            }
            i = i.wrapping_add(1);
        }
    }
    *at = (*at as libc::c_ulong)
        .wrapping_add(cursrc.offset_from(src) as libc::c_long as libc::c_ulong) as size_t
        as size_t;
    *curbuf = '\0' as i32 as libc::c_char;
    return buffer;
}
unsafe extern "C" fn isBounceStart(mctx: *mut mbox_ctx, mut line: *const libc::c_char) -> bool {
    let mut len: size_t = 0;
    if line.is_null() {
        return 0 as libc::c_int != 0;
    }
    if *line as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int != 0;
    }
    len = strlen(line);
    if len < 6 as libc::c_int as libc::c_ulong || len >= 72 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int != 0;
    }
    if memcmp(
        line as *const libc::c_void,
        b"From \0" as *const u8 as *const libc::c_char as *const libc::c_void,
        5 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        || memcmp(
            line as *const libc::c_void,
            b">From \0" as *const u8 as *const libc::c_char as *const libc::c_void,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        let mut numSpaces: libc::c_int = 0 as libc::c_int;
        let mut numDigits: libc::c_int = 0 as libc::c_int;
        line = line.offset(4 as libc::c_int as isize);
        loop {
            if *line as libc::c_int == ' ' as i32 {
                numSpaces += 1;
            } else if *(*__ctype_b_loc())
                .offset((*line as libc::c_int & 0xff as libc::c_int) as isize)
                as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                numDigits += 1;
            }
            line = line.offset(1);
            if !(*line as libc::c_int != '\0' as i32) {
                break;
            }
        }
        if numSpaces < 6 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
        if numDigits < 11 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
        return 1 as libc::c_int != 0;
    }
    return cli_compare_ftm_file(line as *const libc::c_uchar, len, (*(*mctx).ctx).engine)
        as libc::c_uint
        == CL_TYPE_MAIL as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn exportBinhexMessage(mctx: *mut mbox_ctx, m: *mut message) -> bool {
    let mut infected: bool = 0 as libc::c_int != 0;
    let mut fb: *mut fileblob = 0 as *mut fileblob;
    if messageGetEncoding(m) as libc::c_uint == NOENCODING as libc::c_int as libc::c_uint {
        messageSetEncoding(m, b"x-binhex\0" as *const u8 as *const libc::c_char);
    }
    fb = messageToFileblob(m, (*mctx).dir, 0 as libc::c_int);
    if !fb.is_null() {
        cli_dbgmsg(
            b"Binhex file decoded to %s\n\0" as *const u8 as *const libc::c_char,
            fileblobGetFilename(fb),
        );
        if fileblobScanAndDestroy(fb) == CL_VIRUS as libc::c_int {
            infected = 1 as libc::c_int != 0;
        }
        let ref mut fresh46 = (*mctx).files;
        *fresh46 = (*fresh46).wrapping_add(1);
    } else {
        cli_errmsg(
            b"Couldn't decode binhex file to %s\n\0" as *const u8 as *const libc::c_char,
            (*mctx).dir,
        );
    }
    return infected;
}
unsafe extern "C" fn exportBounceMessage(mctx: *mut mbox_ctx, mut start: *mut text) -> libc::c_int {
    let mut rc: libc::c_int = CL_CLEAN as libc::c_int;
    let mut t: *mut text = 0 as *mut text;
    let mut fb: *mut fileblob = 0 as *mut fileblob;
    t = start;
    while !t.is_null() {
        let txt: *const libc::c_char = lineGetData((*t).t_line);
        let mut cmd: [libc::c_char; 1001] = [0; 1001];
        if !txt.is_null() {
            if !(cli_strtokbuf(
                txt,
                0 as libc::c_int,
                b":\0" as *const u8 as *const libc::c_char,
                cmd.as_mut_ptr(),
            ))
            .is_null()
            {
                match tableFind((*mctx).rfc821Table, cmd.as_mut_ptr()) {
                    2 => {
                        if (strstr(txt, b"7bit\0" as *const u8 as *const libc::c_char)).is_null()
                            && (strstr(txt, b"8bit\0" as *const u8 as *const libc::c_char))
                                .is_null()
                        {
                            break;
                        }
                    }
                    3 => {
                        break;
                    }
                    1 => {
                        if !(strstr(txt, b"text/plain\0" as *const u8 as *const libc::c_char))
                            .is_null()
                        {
                            t = 0 as *mut text;
                        }
                        break;
                    }
                    _ => {
                        if strcasecmp(
                            cmd.as_mut_ptr(),
                            b"From\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            start = t;
                        } else if strcasecmp(
                            cmd.as_mut_ptr(),
                            b"Received\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            start = t;
                        }
                    }
                }
            }
        }
        t = (*t).t_next;
    }
    if !t.is_null() && {
        fb = fileblobCreate();
        !fb.is_null()
    } {
        cli_dbgmsg(b"Found a bounce message\n\0" as *const u8 as *const libc::c_char);
        fileblobSetFilename(
            fb,
            (*mctx).dir,
            b"bounce\0" as *const u8 as *const libc::c_char,
        );
        fileblobSetCTX(fb, (*mctx).ctx);
        if (textToFileblob(start, fb, 1 as libc::c_int)).is_null() {
            cli_dbgmsg(
                b"Nothing new to save in the bounce message\n\0" as *const u8
                    as *const libc::c_char,
            );
            fileblobDestroy(fb);
        } else {
            rc = fileblobScanAndDestroy(fb);
        }
        let ref mut fresh47 = (*mctx).files;
        *fresh47 = (*fresh47).wrapping_add(1);
    } else {
        cli_dbgmsg(b"Not found a bounce message\n\0" as *const u8 as *const libc::c_char);
    }
    return rc;
}
unsafe extern "C" fn getMimeTypeStr(mimetype: mime_type) -> *const libc::c_char {
    let mut entry: *const tableinit = mimeTypeStr.as_ptr();
    while !((*entry).key).is_null() {
        if mimetype as libc::c_uint == (*entry).value as mime_type as libc::c_uint {
            return (*entry).key;
        }
        entry = entry.offset(1);
    }
    return b"UNKNOWN\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn getEncTypeStr(enctype: encoding_type) -> *const libc::c_char {
    let mut entry: *const tableinit = encTypeStr.as_ptr();
    while !((*entry).key).is_null() {
        if enctype as libc::c_uint == (*entry).value as encoding_type as libc::c_uint {
            return (*entry).key;
        }
        entry = entry.offset(1);
    }
    return b"UNKNOWN\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn do_multipart(
    mut mainMessage: *mut message,
    messages: *mut *mut message,
    i: libc::c_int,
    rc: *mut mbox_status,
    mctx: *mut mbox_ctx,
    messageIn: *mut message,
    tptr: *mut *mut text,
    recursion_level: libc::c_uint,
) -> *mut message {
    let mut addToText: bool = 0 as libc::c_int != 0;
    let mut dtype: *const libc::c_char = 0 as *const libc::c_char;
    let aMessage: *mut message = *messages.offset(i as isize);
    let doPhishingScan: libc::c_int =
        ((*(*(*mctx).ctx).engine).dboptions & 0x8 as libc::c_int as libc::c_uint != 0
            && (*(*(*mctx).ctx).dconf).phishing & 0x1 as libc::c_int as libc::c_uint != 0)
            as libc::c_int;
    let mut thisobj: *mut json_object = 0 as *mut json_object;
    let saveobj: *mut json_object = (*mctx).wrkobj;
    if !((*mctx).wrkobj).is_null() {
        let multiobj: *mut json_object = cli_jsonarray(
            (*mctx).wrkobj,
            b"Multipart\0" as *const u8 as *const libc::c_char,
        );
        if multiobj.is_null() {
            cli_errmsg(
                b"Cannot get multipart preclass array\n\0" as *const u8 as *const libc::c_char,
            );
        } else {
            thisobj = messageGetJObj(aMessage);
            if thisobj.is_null() {
                cli_dbgmsg(
                    b"Cannot get message preclass object\n\0" as *const u8 as *const libc::c_char,
                );
            } else if CL_SUCCESS as libc::c_int as libc::c_uint
                != cli_json_addowner(
                    multiobj,
                    thisobj,
                    0 as *const libc::c_char,
                    -(1 as libc::c_int),
                ) as libc::c_uint
            {
                cli_errmsg(
                    b"Cannot assign message preclass object to multipart preclass array\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    if aMessage.is_null() {
        if !thisobj.is_null() {
            cli_jsonstr(
                thisobj,
                b"MimeType\0" as *const u8 as *const libc::c_char,
                b"NULL\0" as *const u8 as *const libc::c_char,
            );
        }
        return mainMessage;
    }
    if *rc as libc::c_uint != OK as libc::c_int as libc::c_uint {
        return mainMessage;
    }
    cli_dbgmsg(
        b"Mixed message part %d is of type %d\n\0" as *const u8 as *const libc::c_char,
        i,
        messageGetMimeType(aMessage) as libc::c_uint,
    );
    if !thisobj.is_null() {
        cli_jsonstr(
            thisobj,
            b"MimeType\0" as *const u8 as *const libc::c_char,
            getMimeTypeStr(messageGetMimeType(aMessage)),
        );
        cli_jsonstr(
            thisobj,
            b"MimeSubtype\0" as *const u8 as *const libc::c_char,
            messageGetMimeSubtype(aMessage),
        );
        cli_jsonstr(
            thisobj,
            b"EncodingType\0" as *const u8 as *const libc::c_char,
            getEncTypeStr(messageGetEncoding(aMessage)),
        );
        cli_jsonstr(
            thisobj,
            b"Disposition\0" as *const u8 as *const libc::c_char,
            messageGetDispositionType(aMessage),
        );
        if messageHasFilename(aMessage) != 0 {
            let filename: *mut libc::c_char = messageGetFilename(aMessage);
            cli_jsonstr(
                thisobj,
                b"Filename\0" as *const u8 as *const libc::c_char,
                filename,
            );
            free(filename as *mut libc::c_void);
        } else {
            cli_jsonstr(
                thisobj,
                b"Filename\0" as *const u8 as *const libc::c_char,
                b"(inline)\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    match messageGetMimeType(aMessage) as libc::c_uint {
        1 | 2 | 3 | 7 => {}
        0 => {
            cli_dbgmsg(
                b"No mime headers found in multipart part %d\n\0" as *const u8
                    as *const libc::c_char,
                i,
            );
            if !mainMessage.is_null() {
                if !(binhexBegin(aMessage)).is_null() {
                    cli_dbgmsg(
                        b"Found binhex message in multipart/mixed mainMessage\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    if exportBinhexMessage(mctx, mainMessage) {
                        *rc = VIRUS;
                    }
                }
                if mainMessage != messageIn {
                    messageDestroy(mainMessage);
                }
                mainMessage = 0 as *mut message;
            } else if !aMessage.is_null() {
                if !(binhexBegin(aMessage)).is_null() {
                    cli_dbgmsg(
                        b"Found binhex message in multipart/mixed non mime part\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    if exportBinhexMessage(mctx, aMessage) {
                        *rc = VIRUS;
                    }
                    messageReset(*messages.offset(i as isize));
                }
            }
            addToText = 1 as libc::c_int != 0;
            if (messageGetBody(aMessage)).is_null() {
                cli_dbgmsg(b"No plain text alternative\n\0" as *const u8 as *const libc::c_char);
            }
        }
        6 => {
            dtype = messageGetDispositionType(aMessage);
            cli_dbgmsg(
                b"Mixed message text part disposition \"%s\"\n\0" as *const u8
                    as *const libc::c_char,
                dtype,
            );
            if !(strcasecmp(dtype, b"attachment\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int)
            {
                if *dtype as libc::c_int == '\0' as i32
                    || strcasecmp(dtype, b"inline\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                {
                    let mut cptr: *const libc::c_char = 0 as *const libc::c_char;
                    if !mainMessage.is_null() && mainMessage != messageIn {
                        messageDestroy(mainMessage);
                    }
                    mainMessage = 0 as *mut message;
                    cptr = messageGetMimeSubtype(aMessage);
                    cli_dbgmsg(
                        b"Mime subtype \"%s\"\n\0" as *const u8 as *const libc::c_char,
                        cptr,
                    );
                    if tableFind((*mctx).subtypeTable, cptr) == 1 as libc::c_int
                        && messageGetEncoding(aMessage) as libc::c_uint
                            == NOENCODING as libc::c_int as libc::c_uint
                    {
                        if messageHasFilename(aMessage) == 0 {
                            cli_dbgmsg(
                                b"Adding part to main message\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            addToText = 1 as libc::c_int != 0;
                        } else {
                            cli_dbgmsg(
                                b"Treating inline as attachment\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                    } else {
                        let is_html: libc::c_int = (tableFind((*mctx).subtypeTable, cptr)
                            == 3 as libc::c_int)
                            as libc::c_int;
                        if doPhishingScan != 0 {
                            checkURLs(aMessage, mctx, rc, is_html);
                        }
                        messageAddArgument(
                            aMessage,
                            b"filename=mixedtextportion\0" as *const u8 as *const libc::c_char,
                        );
                    }
                } else {
                    cli_dbgmsg(
                        b"Text type %s is not supported\n\0" as *const u8 as *const libc::c_char,
                        dtype,
                    );
                    return mainMessage;
                }
            }
        }
        4 => {
            cli_dbgmsg(
                b"Found message inside multipart (encoding type %d)\n\0" as *const u8
                    as *const libc::c_char,
                messageGetEncoding(aMessage) as libc::c_uint,
            );
            match messageGetEncoding(aMessage) as libc::c_uint {
                0 | 3 | 4 => {
                    if (encodingLine(aMessage)).is_null() {
                        cli_dbgmsg(
                            b"Unencoded multipart/message will not be scanned\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        messageDestroy(*messages.offset(i as isize));
                        let ref mut fresh48 = *messages.offset(i as isize);
                        *fresh48 = 0 as *mut message;
                        return mainMessage;
                    }
                }
                _ => {}
            }
            cli_dbgmsg(
                b"Encoded multipart/message will be scanned\n\0" as *const u8
                    as *const libc::c_char,
            );
            if saveTextPart(mctx, aMessage, 1 as libc::c_int) == CL_VIRUS as libc::c_int {
                *rc = VIRUS;
            }
            messageDestroy(*messages.offset(i as isize));
            let ref mut fresh49 = *messages.offset(i as isize);
            *fresh49 = 0 as *mut message;
            return mainMessage;
        }
        5 => {
            cli_dbgmsg(b"Found multipart inside multipart\n\0" as *const u8 as *const libc::c_char);
            let ref mut fresh50 = (*mctx).wrkobj;
            *fresh50 = thisobj;
            if !aMessage.is_null() {
                *rc = parseEmailBody(
                    aMessage,
                    *tptr,
                    mctx,
                    recursion_level.wrapping_add(1 as libc::c_int as libc::c_uint),
                );
                cli_dbgmsg(
                    b"Finished recursion, rc = %d\n\0" as *const u8 as *const libc::c_char,
                    *rc as libc::c_int,
                );
                messageDestroy(*messages.offset(i as isize));
                let ref mut fresh51 = *messages.offset(i as isize);
                *fresh51 = 0 as *mut message;
            } else {
                *rc = parseEmailBody(
                    0 as *mut message,
                    0 as *mut text,
                    mctx,
                    recursion_level.wrapping_add(1 as libc::c_int as libc::c_uint),
                );
                if !mainMessage.is_null() && mainMessage != messageIn {
                    messageDestroy(mainMessage);
                }
                mainMessage = 0 as *mut message;
            }
            let ref mut fresh52 = (*mctx).wrkobj;
            *fresh52 = saveobj;
            return mainMessage;
        }
        _ => {
            cli_dbgmsg(
                b"Only text and application attachments are fully supported, type = %d\n\0"
                    as *const u8 as *const libc::c_char,
                messageGetMimeType(aMessage) as libc::c_uint,
            );
        }
    }
    if *rc as libc::c_uint != VIRUS as libc::c_int as libc::c_uint {
        let fb: *mut fileblob = messageToFileblob(aMessage, (*mctx).dir, 1 as libc::c_int);
        let mut arrobj: *mut json_object = 0 as *mut json_object;
        let mut arrlen: size_t = 0 as libc::c_int as size_t;
        if !thisobj.is_null() {
            if json_object_object_get_ex(
                (*(*mctx).ctx).wrkproperty,
                b"ContainedObjects\0" as *const u8 as *const libc::c_char,
                &mut arrobj,
            ) != 0
            {
                arrlen = json_object_array_length(arrobj);
            }
        }
        if !fb.is_null() {
            fileblobSetCTX(fb, (*mctx).ctx);
            if fileblobScanAndDestroy(fb) == CL_VIRUS as libc::c_int {
                *rc = VIRUS;
            }
            if !addToText {
                let ref mut fresh53 = (*mctx).files;
                *fresh53 = (*fresh53).wrapping_add(1);
            }
        }
        if !thisobj.is_null() {
            let mut entry: *mut json_object = 0 as *mut json_object;
            let mut dtype_0: *const libc::c_char = 0 as *const libc::c_char;
            if json_object_object_get_ex(
                (*(*mctx).ctx).wrkproperty,
                b"ContainedObjects\0" as *const u8 as *const libc::c_char,
                &mut arrobj,
            ) != 0
            {
                if json_object_array_length(arrobj) > arrlen {
                    entry = json_object_array_get_idx(arrobj, arrlen);
                }
            }
            if !entry.is_null() {
                json_object_object_get_ex(
                    entry,
                    b"FileType\0" as *const u8 as *const libc::c_char,
                    &mut entry,
                );
                if !entry.is_null() {
                    dtype_0 = json_object_get_string(entry);
                }
            }
            cli_jsonint(
                thisobj,
                b"ContainedObjectsIndex\0" as *const u8 as *const libc::c_char,
                arrlen as int32_t,
            );
            cli_jsonstr(
                thisobj,
                b"ClamAVFileType\0" as *const u8 as *const libc::c_char,
                if !dtype_0.is_null() {
                    dtype_0
                } else {
                    b"UNKNOWN\0" as *const u8 as *const libc::c_char
                },
            );
        }
        if messageContainsVirus(aMessage) != 0 {
            *rc = VIRUS;
        }
    }
    messageDestroy(aMessage);
    let ref mut fresh54 = *messages.offset(i as isize);
    *fresh54 = 0 as *mut message;
    return mainMessage;
}
unsafe extern "C" fn count_quotes(mut buf: *const libc::c_char) -> libc::c_int {
    let mut quotes: libc::c_int = 0 as libc::c_int;
    while *buf != 0 {
        let fresh55 = buf;
        buf = buf.offset(1);
        if *fresh55 as libc::c_int == '"' as i32 {
            quotes += 1;
        }
    }
    return quotes;
}
unsafe extern "C" fn next_is_folded_header(t: *const text) -> bool {
    let next: *const text = (*t).t_next;
    let mut data: *const libc::c_char = 0 as *const libc::c_char;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    if next.is_null() {
        return 0 as libc::c_int != 0;
    }
    if ((*next).t_line).is_null() {
        return 0 as libc::c_int != 0;
    }
    data = lineGetData((*next).t_line);
    if *(*__ctype_b_loc()).offset(*data.offset(0 as libc::c_int as isize) as libc::c_int as isize)
        as libc::c_int
        & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
        != 0
    {
        return 1 as libc::c_int != 0;
    }
    if (strchr(data, '=' as i32)).is_null() {
        return 0 as libc::c_int != 0;
    }
    data = lineGetData((*t).t_line);
    ptr = strchr(data, '\0' as i32);
    loop {
        ptr = ptr.offset(-1);
        if !(ptr > data) {
            break;
        }
        match *ptr as libc::c_int {
            59 => return 1 as libc::c_int != 0,
            10 | 32 | 13 | 9 => {}
            _ => return 0 as libc::c_int != 0,
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn newline_in_header(line: *const libc::c_char) -> bool {
    cli_dbgmsg(
        b"newline_in_header, check \"%s\"\n\0" as *const u8 as *const libc::c_char,
        line,
    );
    if strncmp(
        line,
        b"Message-Id: \0" as *const u8 as *const libc::c_char,
        12 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return 1 as libc::c_int != 0;
    }
    if strncmp(
        line,
        b"Date: \0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return 1 as libc::c_int != 0;
    }
    cli_dbgmsg(
        b"newline_in_header, returning \"%s\"\n\0" as *const u8 as *const libc::c_char,
        line,
    );
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn run_static_initializers() {
    num_mhtml_keys = (::std::mem::size_of::<[key_entry; 5]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<key_entry>() as libc::c_ulong);
    num_mhtml_comment_keys = (::std::mem::size_of::<[key_entry; 18]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<key_entry>() as libc::c_ulong);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
