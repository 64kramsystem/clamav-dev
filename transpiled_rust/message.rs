use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    pub type phishcheck;
    pub type regex_matcher;
    pub type re_guts;
    pub type json_object;
    pub type cli_events;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn cli_compare_ftm_file(
        buf: *const libc::c_uchar,
        buflen: size_t,
        engine: *const cl_engine,
    ) -> cli_file_t;
    fn json_object_put(obj: *mut json_object) -> libc::c_int;
    fn cli_chomp(string: *mut libc::c_char) -> libc::c_int;
    fn cli_strtok(
        line: *const libc::c_char,
        field: libc::c_int,
        delim: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn cli_strrcpy(
        dest: *mut libc::c_char,
        source: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn cli_strlcat(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        sz: size_t,
    ) -> size_t;
    fn cli_warnmsg(str: *const libc::c_char, _: ...);
    fn cli_errmsg(str: *const libc::c_char, _: ...);
    fn cli_dbgmsg(str: *const libc::c_char, _: ...);
    fn cli_malloc(nmemb: size_t) -> *mut libc::c_void;
    fn cli_calloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn cli_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn cli_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn blobCreate() -> *mut blob;
    fn blobDestroy(b: *mut blob);
    fn blobSetFilename(
        b: *mut blob,
        dir: *const libc::c_char,
        filename: *const libc::c_char,
    );
    fn blobAddData(b: *mut blob, data: *const libc::c_uchar, len: size_t) -> libc::c_int;
    fn tableFind(table: *const table_t, key: *const libc::c_char) -> libc::c_int;
    fn fileblobCreate() -> *mut fileblob;
    fn fileblobDestroy(fb: *mut fileblob);
    fn fileblobSetFilename(
        fb: *mut fileblob,
        dir: *const libc::c_char,
        filename: *const libc::c_char,
    );
    fn fileblobPartialSet(
        fb: *mut fileblob,
        fullname: *const libc::c_char,
        arg: *const libc::c_char,
    );
    fn fileblobSetCTX(fb: *mut fileblob, ctx: *mut cli_ctx);
    fn fileblobAddData(
        fb: *mut fileblob,
        data: *const libc::c_uchar,
        len: size_t,
    ) -> libc::c_int;
    fn lineCreate(data: *const libc::c_char) -> *mut line_t;
    fn lineLink(line: *mut line_t) -> *mut line_t;
    fn lineUnlink(line: *mut line_t) -> *mut line_t;
    fn lineGetData(line: *const line_t) -> *const libc::c_char;
    fn cli_jsonobj(obj: *mut json_object, key: *const libc::c_char) -> *mut json_object;
    fn tableInsert(
        table: *mut table_t,
        key: *const libc::c_char,
        value: libc::c_int,
    ) -> libc::c_int;
    fn textDestroy(t_head: *mut text);
    fn textToBlob(t: *mut text, b: *mut blob, destroy: libc::c_int) -> *mut blob;
    fn textToFileblob(
        t: *mut text,
        fb: *mut fileblob,
        destroy: libc::c_int,
    ) -> *mut fileblob;
    fn tableDestroy(table: *mut table_t);
    fn tableCreate() -> *mut table;
    fn textMove(t_head: *mut text, t: *mut text) -> *mut text;
    fn strstrip(s: *mut libc::c_char) -> size_t;
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type int8_t = __int8_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub type clcb_stats_get_hostid = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_char,
>;
pub type clcb_stats_get_size = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> size_t,
>;
pub type clcb_stats_get_num = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> size_t,
>;
pub type clcb_stats_flush = Option::<
    unsafe extern "C" fn(*mut cl_engine, *mut libc::c_void) -> (),
>;
pub type clcb_stats_submit = Option::<
    unsafe extern "C" fn(*mut cl_engine, *mut libc::c_void) -> (),
>;
pub type clcb_stats_decrement_count = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        *const libc::c_uchar,
        size_t,
        *mut libc::c_void,
    ) -> (),
>;
pub type clcb_stats_remove_sample = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        *const libc::c_uchar,
        size_t,
        *mut libc::c_void,
    ) -> (),
>;
pub type clcb_stats_add_sample = Option::<
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
pub type clcb_progress = Option::<
    unsafe extern "C" fn(size_t, size_t, *mut libc::c_void) -> cl_error_t,
>;
pub type clcb_file_props = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        libc::c_int,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type clcb_meta = Option::<
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
pub type clcb_hash = Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_ulonglong,
        *const libc::c_uchar,
        *const libc::c_char,
        *mut libc::c_void,
    ) -> (),
>;
pub type clcb_sigload = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        *const libc::c_char,
        libc::c_uint,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type clcb_virus_found = Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_char, *mut libc::c_void) -> (),
>;
pub type clcb_post_scan = Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        *const libc::c_char,
        *mut libc::c_void,
    ) -> cl_error_t,
>;
pub type clcb_pre_scan = Option::<
    unsafe extern "C" fn(
        libc::c_int,
        *const libc::c_char,
        *mut libc::c_void,
    ) -> cl_error_t,
>;
pub type clcb_pre_cache = Option::<
    unsafe extern "C" fn(
        libc::c_int,
        *const libc::c_char,
        *mut libc::c_void,
    ) -> cl_error_t,
>;
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
    pub unmap: Option::<unsafe extern "C" fn(*mut fmap_t) -> ()>,
    pub need: Option::<
        unsafe extern "C" fn(
            *mut fmap_t,
            size_t,
            size_t,
            libc::c_int,
        ) -> *const libc::c_void,
    >,
    pub need_offstr: Option::<
        unsafe extern "C" fn(*mut fmap_t, size_t, size_t) -> *const libc::c_void,
    >,
    pub gets: Option::<
        unsafe extern "C" fn(
            *mut fmap_t,
            *mut libc::c_char,
            *mut size_t,
            size_t,
        ) -> *const libc::c_void,
    >,
    pub unneed_off: Option::<unsafe extern "C" fn(*mut fmap_t, size_t, size_t) -> ()>,
    pub have_maphash: bool,
    pub maphash: [libc::c_uchar; 16],
    pub bitmap: *mut uint64_t,
    pub name: *mut libc::c_char,
}
pub type fmap_t = cl_fmap_t;
pub type cl_fmap_t = cl_fmap;
pub type clcb_pread = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, size_t, off_t) -> off_t,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mime_map {
    pub string: *const libc::c_char,
    pub type_0: mime_type,
}
pub type LINK1 = *mut ELEMENT1;
pub type ELEMENT1 = pstr_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pstr_list {
    pub d1: *mut libc::c_char,
    pub next: *mut pstr_list,
}
pub const CONTENTS: C2RustUnnamed_5 = 2;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const CHARSET: C2RustUnnamed_5 = 1;
pub const LANGUAGE: C2RustUnnamed_5 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct encoding_map {
    pub string: *const libc::c_char,
    pub type_0: encoding_type,
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
static mut encoding_map: [encoding_map; 13] = [
    {
        let mut init = encoding_map {
            string: b"7bit\0" as *const u8 as *const libc::c_char,
            type_0: NOENCODING,
        };
        init
    },
    {
        let mut init = encoding_map {
            string: b"text/plain\0" as *const u8 as *const libc::c_char,
            type_0: NOENCODING,
        };
        init
    },
    {
        let mut init = encoding_map {
            string: b"quoted-printable\0" as *const u8 as *const libc::c_char,
            type_0: QUOTEDPRINTABLE,
        };
        init
    },
    {
        let mut init = encoding_map {
            string: b"base64\0" as *const u8 as *const libc::c_char,
            type_0: BASE64,
        };
        init
    },
    {
        let mut init = encoding_map {
            string: b"8bit\0" as *const u8 as *const libc::c_char,
            type_0: EIGHTBIT,
        };
        init
    },
    {
        let mut init = encoding_map {
            string: b"binary\0" as *const u8 as *const libc::c_char,
            type_0: BINARY,
        };
        init
    },
    {
        let mut init = encoding_map {
            string: b"x-uuencode\0" as *const u8 as *const libc::c_char,
            type_0: UUENCODE,
        };
        init
    },
    {
        let mut init = encoding_map {
            string: b"x-yencode\0" as *const u8 as *const libc::c_char,
            type_0: YENCODE,
        };
        init
    },
    {
        let mut init = encoding_map {
            string: b"x-binhex\0" as *const u8 as *const libc::c_char,
            type_0: BINHEX,
        };
        init
    },
    {
        let mut init = encoding_map {
            string: b"us-ascii\0" as *const u8 as *const libc::c_char,
            type_0: NOENCODING,
        };
        init
    },
    {
        let mut init = encoding_map {
            string: b"x-uue\0" as *const u8 as *const libc::c_char,
            type_0: UUENCODE,
        };
        init
    },
    {
        let mut init = encoding_map {
            string: b"uuencode\0" as *const u8 as *const libc::c_char,
            type_0: UUENCODE,
        };
        init
    },
    {
        let mut init = encoding_map {
            string: 0 as *const libc::c_char,
            type_0: NOENCODING,
        };
        init
    },
];
static mut mime_map: [mime_map; 8] = [
    {
        let mut init = mime_map {
            string: b"text\0" as *const u8 as *const libc::c_char,
            type_0: TEXT,
        };
        init
    },
    {
        let mut init = mime_map {
            string: b"multipart\0" as *const u8 as *const libc::c_char,
            type_0: MULTIPART,
        };
        init
    },
    {
        let mut init = mime_map {
            string: b"application\0" as *const u8 as *const libc::c_char,
            type_0: APPLICATION,
        };
        init
    },
    {
        let mut init = mime_map {
            string: b"audio\0" as *const u8 as *const libc::c_char,
            type_0: AUDIO,
        };
        init
    },
    {
        let mut init = mime_map {
            string: b"image\0" as *const u8 as *const libc::c_char,
            type_0: IMAGE,
        };
        init
    },
    {
        let mut init = mime_map {
            string: b"message\0" as *const u8 as *const libc::c_char,
            type_0: MESSAGE,
        };
        init
    },
    {
        let mut init = mime_map {
            string: b"video\0" as *const u8 as *const libc::c_char,
            type_0: VIDEO,
        };
        init
    },
    {
        let mut init = mime_map {
            string: 0 as *const libc::c_char,
            type_0: TEXT,
        };
        init
    },
];
static mut base64Table: [libc::c_uchar; 256] = [
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    62 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    63 as libc::c_int as libc::c_uchar,
    52 as libc::c_int as libc::c_uchar,
    53 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    55 as libc::c_int as libc::c_uchar,
    56 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    58 as libc::c_int as libc::c_uchar,
    59 as libc::c_int as libc::c_uchar,
    60 as libc::c_int as libc::c_uchar,
    61 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    23 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    25 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    27 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    31 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    33 as libc::c_int as libc::c_uchar,
    34 as libc::c_int as libc::c_uchar,
    35 as libc::c_int as libc::c_uchar,
    36 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    40 as libc::c_int as libc::c_uchar,
    41 as libc::c_int as libc::c_uchar,
    42 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    46 as libc::c_int as libc::c_uchar,
    47 as libc::c_int as libc::c_uchar,
    48 as libc::c_int as libc::c_uchar,
    49 as libc::c_int as libc::c_uchar,
    50 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
];
#[no_mangle]
pub unsafe extern "C" fn messageCreate() -> *mut message {
    let mut m: *mut message = cli_calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<message>() as libc::c_ulong,
    ) as *mut message;
    if !m.is_null() {
        (*m).mimeType = NOMIME;
    }
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn messageDestroy(mut m: *mut message) {
    if m.is_null() {
        return;
    }
    messageReset(m);
    free(m as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn messageReset(mut m: *mut message) {
    let mut i: size_t = 0;
    if m.is_null() {
        return;
    }
    if !((*m).mimeSubtype).is_null() {
        free((*m).mimeSubtype as *mut libc::c_void);
    }
    if !((*m).mimeDispositionType).is_null() {
        free((*m).mimeDispositionType as *mut libc::c_void);
    }
    if !((*m).mimeArguments).is_null() {
        i = 0 as libc::c_int as size_t;
        while i < (*m).numberOfArguments {
            free(*((*m).mimeArguments).offset(i as isize) as *mut libc::c_void);
            i = i.wrapping_add(1);
        }
        free((*m).mimeArguments as *mut libc::c_void);
    }
    if !((*m).body_first).is_null() {
        textDestroy((*m).body_first);
    }
    if 0 as libc::c_int != (*m).base64chars {
        cli_errmsg(
            b"Internal email parse error: message base64chars should be 0 when resetting the message\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if !((*m).encodingTypes).is_null() {
        if 0 as libc::c_int == (*m).numberOfEncTypes {
            cli_errmsg(
                b"Internal email parse error: message numberOfEncTypes should be 0 if encoding types are set\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        free((*m).encodingTypes as *mut libc::c_void);
    }
    if !((*m).jobj).is_null() {
        json_object_put((*m).jobj);
    }
    memset(
        m as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<message>() as libc::c_ulong,
    );
    (*m).mimeType = NOMIME;
}
#[no_mangle]
pub unsafe extern "C" fn messageSetMimeType(
    mut mess: *mut message,
    mut type_0: *const libc::c_char,
) -> libc::c_int {
    static mut mime_mutex: pthread_mutex_t = pthread_mutex_t {
        __data: {
            let mut init = __pthread_mutex_s {
                __lock: 0 as libc::c_int,
                __count: 0 as libc::c_int as libc::c_uint,
                __owner: 0 as libc::c_int,
                __nusers: 0 as libc::c_int as libc::c_uint,
                __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
                __spins: 0 as libc::c_int as libc::c_short,
                __elision: 0 as libc::c_int as libc::c_short,
                __list: {
                    let mut init = __pthread_internal_list {
                        __prev: 0 as *const __pthread_internal_list
                            as *mut __pthread_internal_list,
                        __next: 0 as *const __pthread_internal_list
                            as *mut __pthread_internal_list,
                    };
                    init
                },
            };
            init
        },
    };
    let mut m: *const mime_map = 0 as *const mime_map;
    let mut typeval: libc::c_int = 0;
    static mut mime_table: *mut table_t = 0 as *const table_t as *mut table_t;
    if mess.is_null() {
        cli_dbgmsg(
            b"messageSetMimeType: NULL message pointer\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if type_0.is_null() {
        cli_dbgmsg(
            b"messageSetMimeType: Empty content-type field\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    cli_dbgmsg(
        b"messageSetMimeType: '%s'\n\0" as *const u8 as *const libc::c_char,
        type_0,
    );
    while *(*__ctype_b_loc()).offset(*type_0 as libc::c_int as isize) as libc::c_int
        & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        let fresh0 = type_0;
        type_0 = type_0.offset(1);
        if *fresh0 as libc::c_int == '\0' as i32 {
            return 0 as libc::c_int;
        }
    }
    pthread_mutex_lock(&mut mime_mutex);
    if mime_table.is_null() {
        mime_table = tableCreate();
        if mime_table.is_null() {
            pthread_mutex_unlock(&mut mime_mutex);
            return 0 as libc::c_int;
        }
        m = mime_map.as_ptr();
        while !((*m).string).is_null() {
            if tableInsert(mime_table, (*m).string, (*m).type_0 as libc::c_int) == 0 {
                tableDestroy(mime_table);
                mime_table = 0 as *mut table_t;
                pthread_mutex_unlock(&mut mime_mutex);
                return 0 as libc::c_int;
            }
            m = m.offset(1);
        }
    }
    pthread_mutex_unlock(&mut mime_mutex);
    typeval = tableFind(mime_table, type_0);
    if typeval != -(1 as libc::c_int) {
        (*mess).mimeType = typeval as mime_type;
        return 1 as libc::c_int;
    }
    if (*mess).mimeType as libc::c_uint == NOMIME as libc::c_int as libc::c_uint {
        if strncasecmp(
            type_0,
            b"x-\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            (*mess).mimeType = MEXTENSION;
        } else if strcasecmp(type_0, b"plain\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            cli_dbgmsg(
                b"Incorrect MIME type: `plain', set to Text\n\0" as *const u8
                    as *const libc::c_char,
            );
            (*mess).mimeType = TEXT;
        } else {
            let mut highestSimil: libc::c_int = 0 as libc::c_int;
            let mut t: libc::c_int = -(1 as libc::c_int);
            let mut closest: *const libc::c_char = 0 as *const libc::c_char;
            m = mime_map.as_ptr();
            while !((*m).string).is_null() {
                let s: libc::c_int = simil((*m).string, type_0);
                if s > highestSimil {
                    highestSimil = s;
                    closest = (*m).string;
                    t = (*m).type_0 as libc::c_int;
                }
                m = m.offset(1);
            }
            if highestSimil >= 50 as libc::c_int {
                cli_dbgmsg(
                    b"Unknown MIME type \"%s\" - guessing as %s (%d%% certainty)\n\0"
                        as *const u8 as *const libc::c_char,
                    type_0,
                    closest,
                    highestSimil,
                );
                (*mess).mimeType = t as mime_type;
            } else {
                cli_dbgmsg(
                    b"Unknown MIME type: `%s', set to Application - if you believe this file contains a virus, submit it to www.clamav.net\n\0"
                        as *const u8 as *const libc::c_char,
                    type_0,
                );
                (*mess).mimeType = APPLICATION;
            }
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn messageGetMimeType(mut m: *const message) -> mime_type {
    if m.is_null() {
        cli_errmsg(
            b"Internal email parser error: message is pointer is NULL when trying to get MIME type\n\0"
                as *const u8 as *const libc::c_char,
        );
        return NOMIME;
    }
    return (*m).mimeType;
}
#[no_mangle]
pub unsafe extern "C" fn messageSetMimeSubtype(
    mut m: *mut message,
    mut subtype: *const libc::c_char,
) {
    if m.is_null() {
        cli_errmsg(
            b"Internal email parser error: message is pointer is NULL when trying to set MIME sub-type\n\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    if subtype.is_null() {
        cli_dbgmsg(b"Empty content subtype\n\0" as *const u8 as *const libc::c_char);
        subtype = b"\0" as *const u8 as *const libc::c_char;
    }
    if !((*m).mimeSubtype).is_null() {
        free((*m).mimeSubtype as *mut libc::c_void);
    }
    let ref mut fresh1 = (*m).mimeSubtype;
    *fresh1 = cli_strdup(subtype);
}
#[no_mangle]
pub unsafe extern "C" fn messageGetMimeSubtype(
    mut m: *const message,
) -> *const libc::c_char {
    return if !((*m).mimeSubtype).is_null() {
        (*m).mimeSubtype as *const libc::c_char
    } else {
        b"\0" as *const u8 as *const libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn messageSetDispositionType(
    mut m: *mut message,
    mut disptype: *const libc::c_char,
) {
    if m.is_null() {
        cli_errmsg(
            b"Internal email parser error: message is pointer is NULL when trying to set disposition type\n\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    if !((*m).mimeDispositionType).is_null() {
        free((*m).mimeDispositionType as *mut libc::c_void);
    }
    if disptype.is_null() {
        let ref mut fresh2 = (*m).mimeDispositionType;
        *fresh2 = 0 as *mut libc::c_char;
        return;
    }
    while *disptype as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*disptype as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        disptype = disptype.offset(1);
    }
    if *disptype != 0 {
        let ref mut fresh3 = (*m).mimeDispositionType;
        *fresh3 = cli_strdup(disptype);
        if !((*m).mimeDispositionType).is_null() {
            strstrip((*m).mimeDispositionType);
        }
    } else {
        let ref mut fresh4 = (*m).mimeDispositionType;
        *fresh4 = 0 as *mut libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn messageGetDispositionType(
    mut m: *const message,
) -> *const libc::c_char {
    return if !((*m).mimeDispositionType).is_null() {
        (*m).mimeDispositionType as *const libc::c_char
    } else {
        b"\0" as *const u8 as *const libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn messageAddArgument(
    mut m: *mut message,
    mut arg: *const libc::c_char,
) {
    let mut offset: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if m.is_null() {
        cli_errmsg(
            b"Internal email parser error: message is pointer is NULL when trying to add an argument\n\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    if arg.is_null() {
        return;
    }
    while *(*__ctype_b_loc()).offset(*arg as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        arg = arg.offset(1);
    }
    if *arg as libc::c_int == '\0' as i32 {
        return;
    }
    cli_dbgmsg(
        b"messageAddArgument, arg='%s'\n\0" as *const u8 as *const libc::c_char,
        arg,
    );
    if usefulArg(arg) == 0 {
        return;
    }
    offset = 0 as libc::c_int as size_t;
    while offset < (*m).numberOfArguments {
        if (*((*m).mimeArguments).offset(offset as isize)).is_null() {
            break;
        }
        if strcasecmp(arg, *((*m).mimeArguments).offset(offset as isize))
            == 0 as libc::c_int
        {
            return;
        }
        offset = offset.wrapping_add(1);
    }
    if offset == (*m).numberOfArguments {
        let mut q: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let ref mut fresh5 = (*m).numberOfArguments;
        *fresh5 = (*fresh5).wrapping_add(1);
        q = cli_realloc(
            (*m).mimeArguments as *mut libc::c_void,
            ((*m).numberOfArguments)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
        if q.is_null() {
            let ref mut fresh6 = (*m).numberOfArguments;
            *fresh6 = (*fresh6).wrapping_sub(1);
            return;
        }
        let ref mut fresh7 = (*m).mimeArguments;
        *fresh7 = q;
    }
    let ref mut fresh8 = *((*m).mimeArguments).offset(offset as isize);
    *fresh8 = rfc2231(arg);
    p = *fresh8;
    if p.is_null() {
        cli_dbgmsg(
            b"messageAddArgument, error from rfc2231()\n\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if (strchr(p, '=' as i32)).is_null() {
        if strncmp(
            p,
            b"filename\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            if strlen(p) > 8 as libc::c_int as libc::c_ulong {
                cli_dbgmsg(
                    b"Possible data corruption fixed\n\0" as *const u8
                        as *const libc::c_char,
                );
                *p.offset(8 as libc::c_int as isize) = '=' as i32 as libc::c_char;
            } else {
                cli_dbgmsg(
                    b"Possible data corruption not fixed\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        } else {
            if *p != 0 {
                cli_dbgmsg(
                    b"messageAddArgument, '%s' contains no '='\n\0" as *const u8
                        as *const libc::c_char,
                    p,
                );
            }
            free(*((*m).mimeArguments).offset(offset as isize) as *mut libc::c_void);
            let ref mut fresh9 = *((*m).mimeArguments).offset(offset as isize);
            *fresh9 = 0 as *mut libc::c_char;
            return;
        }
    }
    if strncasecmp(
        p,
        b"filename=\0" as *const u8 as *const libc::c_char,
        9 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        || strncasecmp(
            p,
            b"name=\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        if messageGetMimeType(m) as libc::c_uint == NOMIME as libc::c_int as libc::c_uint
        {
            cli_dbgmsg(
                b"Force mime encoding to application\n\0" as *const u8
                    as *const libc::c_char,
            );
            messageSetMimeType(m, b"application\0" as *const u8 as *const libc::c_char);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn messageAddArguments(
    mut m: *mut message,
    mut s: *const libc::c_char,
) {
    let mut string: *const libc::c_char = s;
    cli_dbgmsg(b"Add arguments '%s'\n\0" as *const u8 as *const libc::c_char, string);
    if string.is_null() {
        cli_errmsg(
            b"Internal email parser error: message is pointer is NULL when trying to add message arguments\n\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    while *string != 0 {
        let mut key: *const libc::c_char = 0 as *const libc::c_char;
        let mut cptr: *const libc::c_char = 0 as *const libc::c_char;
        let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut field: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut datasz: size_t = 0 as libc::c_int as size_t;
        if *(*__ctype_b_loc())
            .offset((*string as libc::c_int & 0xff as libc::c_int) as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0 || *string as libc::c_int == ';' as i32
        {
            string = string.offset(1);
        } else {
            key = string;
            data = strchr(string, '=' as i32);
            if data.is_null() {
                data = strchr(string, ':' as i32);
            }
            if data.is_null() {
                cli_dbgmsg(
                    b"Can't parse header \"%s\"\n\0" as *const u8 as *const libc::c_char,
                    s,
                );
                return;
            }
            string = &mut *data.offset(1 as libc::c_int as isize) as *mut libc::c_char;
            while *(*__ctype_b_loc()).offset(*string as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0 && *string as libc::c_int != '\0' as i32
            {
                string = string.offset(1);
            }
            cptr = string;
            if *string != 0 {
                string = string.offset(1);
            }
            if *cptr as libc::c_int == '"' as i32 {
                let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut kcopy: *mut libc::c_char = 0 as *mut libc::c_char;
                kcopy = cli_strdup(key);
                if kcopy.is_null() {
                    return;
                }
                ptr = strchr(kcopy, '=' as i32);
                if ptr.is_null() {
                    ptr = strchr(kcopy, ':' as i32);
                    if ptr.is_null() {
                        cli_dbgmsg(
                            b"Can't parse header \"%s\"\n\0" as *const u8
                                as *const libc::c_char,
                            s,
                        );
                        free(kcopy as *mut libc::c_void);
                        return;
                    }
                }
                *ptr = '\0' as i32 as libc::c_char;
                cptr = cptr.offset(1);
                string = strchr(cptr, '"' as i32);
                if string.is_null() {
                    cli_dbgmsg(
                        b"Unbalanced quote character in \"%s\"\n\0" as *const u8
                            as *const libc::c_char,
                        s,
                    );
                    string = b"\0" as *const u8 as *const libc::c_char;
                } else {
                    string = string.offset(1);
                }
                if usefulArg(kcopy) == 0 {
                    free(kcopy as *mut libc::c_void);
                    continue;
                } else {
                    data = cli_strdup(cptr);
                    if data.is_null() {
                        cli_dbgmsg(
                            b"Can't parse header \"%s\" - if you believe this file contains a missed virus, report it to bugs@clamav.net\n\0"
                                as *const u8 as *const libc::c_char,
                            s,
                        );
                        free(kcopy as *mut libc::c_void);
                        return;
                    }
                    ptr = strchr(data, '"' as i32);
                    if !ptr.is_null() {
                        *ptr = '\0' as i32 as libc::c_char;
                    }
                    datasz = (strlen(kcopy))
                        .wrapping_add(strlen(data))
                        .wrapping_add(2 as libc::c_int as libc::c_ulong);
                    field = cli_realloc(
                        kcopy as *mut libc::c_void,
                        (strlen(kcopy))
                            .wrapping_add(strlen(data))
                            .wrapping_add(2 as libc::c_int as libc::c_ulong),
                    ) as *mut libc::c_char;
                    if !field.is_null() {
                        cli_strlcat(
                            field,
                            b"=\0" as *const u8 as *const libc::c_char,
                            datasz,
                        );
                        cli_strlcat(field, data, datasz);
                    } else {
                        free(kcopy as *mut libc::c_void);
                    }
                    free(data as *mut libc::c_void);
                }
            } else {
                let mut len: size_t = 0;
                if *cptr as libc::c_int == '\0' as i32 {
                    cli_dbgmsg(
                        b"Ignoring empty field in \"%s\"\n\0" as *const u8
                            as *const libc::c_char,
                        s,
                    );
                    return;
                }
                while *string as libc::c_int != '\0' as i32
                    && *(*__ctype_b_loc()).offset(*string as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    string = string.offset(1);
                }
                len = (string as size_t)
                    .wrapping_sub(key as size_t)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                field = cli_malloc(len) as *mut libc::c_char;
                if !field.is_null() {
                    memcpy(
                        field as *mut libc::c_void,
                        key as *const libc::c_void,
                        len.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    *field
                        .offset(
                            len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) = '\0' as i32 as libc::c_char;
                }
            }
            if !field.is_null() {
                messageAddArgument(m, field);
                free(field as *mut libc::c_void);
            }
        }
    }
}
unsafe extern "C" fn messageGetArgument(
    mut m: *const message,
    mut arg: size_t,
) -> *const libc::c_char {
    if m.is_null() {
        cli_errmsg(
            b"Internal email parse error: message pointer is NULL when trying to get a message argument\n\0"
                as *const u8 as *const libc::c_char,
        );
        return b"\0" as *const u8 as *const libc::c_char;
    }
    if arg >= (*m).numberOfArguments {
        return b"\0" as *const u8 as *const libc::c_char;
    }
    return if !(*((*m).mimeArguments).offset(arg as isize)).is_null() {
        *((*m).mimeArguments).offset(arg as isize) as *const libc::c_char
    } else {
        b"\0" as *const u8 as *const libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn messageFindArgument(
    mut m: *const message,
    mut variable: *const libc::c_char,
) -> *mut libc::c_char {
    let mut i: size_t = 0;
    let mut len: size_t = 0;
    if m.is_null() || variable.is_null() {
        cli_errmsg(
            b"Internal email parser error: invalid arguments when finding message arguments\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as *mut libc::c_char;
    }
    len = strlen(variable);
    i = 0 as libc::c_int as size_t;
    while i < (*m).numberOfArguments {
        let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
        ptr = messageGetArgument(m, i);
        if !(ptr.is_null() || *ptr as libc::c_int == '\0' as i32) {
            if strncasecmp(ptr, variable, len) == 0 as libc::c_int {
                ptr = &*ptr.offset(len as isize) as *const libc::c_char;
                while *(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    ptr = ptr.offset(1);
                }
                if *ptr as libc::c_int != '=' as i32 {
                    cli_dbgmsg(
                        b"messageFindArgument: no '=' sign found in MIME header '%s' (%s)\n\0"
                            as *const u8 as *const libc::c_char,
                        variable,
                        messageGetArgument(m, i),
                    );
                    return 0 as *mut libc::c_char;
                }
                ptr = ptr.offset(1);
                if strlen(ptr) > 1 as libc::c_int as libc::c_ulong
                    && *ptr as libc::c_int == '"' as i32
                    && !(strchr(&*ptr.offset(1 as libc::c_int as isize), '"' as i32))
                        .is_null()
                {
                    ptr = ptr.offset(1);
                    let mut ret: *mut libc::c_char = cli_strdup(ptr);
                    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
                    if ret.is_null() {
                        return 0 as *mut libc::c_char;
                    }
                    p = strchr(ret, '"' as i32);
                    if !p.is_null() {
                        *ret
                            .offset(
                                (strlen(ret))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) = '\0' as i32 as libc::c_char;
                        *p = '\0' as i32 as libc::c_char;
                    }
                    return ret;
                }
                return cli_strdup(ptr);
            }
        }
        i = i.wrapping_add(1);
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn messageGetFilename(mut m: *const message) -> *mut libc::c_char {
    let mut filename: *mut libc::c_char = messageFindArgument(
        m,
        b"filename\0" as *const u8 as *const libc::c_char,
    );
    if !filename.is_null() {
        return filename;
    }
    return messageFindArgument(m, b"name\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn messageHasArgument(
    mut m: *const message,
    mut variable: *const libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut len: size_t = 0;
    if m.is_null() || variable.is_null() {
        cli_errmsg(
            b"Internal email parser error: invalid arguments when checking if message has arguments\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    len = strlen(variable);
    i = 0 as libc::c_int as size_t;
    while i < (*m).numberOfArguments {
        let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
        ptr = messageGetArgument(m, i);
        if !(ptr.is_null() || *ptr as libc::c_int == '\0' as i32) {
            if strncasecmp(ptr, variable, len) == 0 as libc::c_int {
                ptr = &*ptr.offset(len as isize) as *const libc::c_char;
                while *(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    ptr = ptr.offset(1);
                }
                if *ptr as libc::c_int != '=' as i32 {
                    cli_dbgmsg(
                        b"messageHasArgument: no '=' sign found in MIME header '%s' (%s)\n\0"
                            as *const u8 as *const libc::c_char,
                        variable,
                        messageGetArgument(m, i),
                    );
                    return 0 as libc::c_int;
                }
                return 1 as libc::c_int;
            }
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn messageHasFilename(mut m: *const message) -> libc::c_int {
    return (messageHasArgument(m, b"filename\0" as *const u8 as *const libc::c_char) != 0
        || messageHasArgument(m, b"file\0" as *const u8 as *const libc::c_char) != 0)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn messageSetEncoding(
    mut m: *mut message,
    mut enctype: *const libc::c_char,
) {
    let mut e: *const encoding_map = 0 as *const encoding_map;
    let mut i: libc::c_int = 0;
    let mut type_0: *mut libc::c_char = 0 as *mut libc::c_char;
    if m.is_null() || enctype.is_null() {
        cli_errmsg(
            b"Internal email parser error: invalid arguments when setting message encoding type\n\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    while *(*__ctype_b_loc()).offset(*enctype as libc::c_int as isize) as libc::c_int
        & _ISblank as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        enctype = enctype.offset(1);
    }
    cli_dbgmsg(
        b"messageSetEncoding: '%s'\n\0" as *const u8 as *const libc::c_char,
        enctype,
    );
    if strcasecmp(enctype, b"8 bit\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        cli_dbgmsg(
            b"Broken content-transfer-encoding: '8 bit' changed to '8bit'\n\0"
                as *const u8 as *const libc::c_char,
        );
        enctype = b"8bit\0" as *const u8 as *const libc::c_char;
    }
    i = 0 as libc::c_int;
    loop {
        let fresh10 = i;
        i = i + 1;
        type_0 = cli_strtok(
            enctype,
            fresh10,
            b" \t\0" as *const u8 as *const libc::c_char,
        );
        if type_0.is_null() {
            break;
        }
        let mut highestSimil: libc::c_int = 0 as libc::c_int;
        let mut closest: *const libc::c_char = 0 as *const libc::c_char;
        e = encoding_map.as_ptr();
        while !((*e).string).is_null() {
            let mut sim: libc::c_int = 0;
            let lowertype: libc::c_char = ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *type_0
                            .offset(0 as libc::c_int as isize) as libc::c_int;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = tolower(
                            *type_0.offset(0 as libc::c_int as isize) as libc::c_int,
                        );
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(
                            *type_0.offset(0 as libc::c_int as isize) as libc::c_int
                                as isize,
                        );
                }
                __res
            }) as libc::c_char;
            if !(lowertype as libc::c_int
                != ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *((*e).string)
                                .offset(0 as libc::c_int as isize) as libc::c_int;
                            __res = (if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            });
                        } else {
                            __res = tolower(
                                *((*e).string).offset(0 as libc::c_int as isize)
                                    as libc::c_int,
                            );
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(
                                *((*e).string).offset(0 as libc::c_int as isize)
                                    as libc::c_int as isize,
                            );
                    }
                    __res
                }) && lowertype as libc::c_int != 'x' as i32)
            {
                if !(strcmp(
                    (*e).string,
                    b"uuencode\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int)
                {
                    sim = simil(type_0, (*e).string);
                    if sim == 100 as libc::c_int {
                        let mut j: libc::c_int = 0;
                        let mut et: *mut encoding_type = 0 as *mut encoding_type;
                        j = 0 as libc::c_int;
                        while j < (*m).numberOfEncTypes {
                            if *((*m).encodingTypes).offset(j as isize) as libc::c_uint
                                == (*e).type_0 as libc::c_uint
                            {
                                break;
                            }
                            j += 1;
                        }
                        if j < (*m).numberOfEncTypes {
                            cli_dbgmsg(
                                b"Ignoring duplicate encoding mechanism '%s'\n\0"
                                    as *const u8 as *const libc::c_char,
                                type_0,
                            );
                            break;
                        } else {
                            et = cli_realloc(
                                (*m).encodingTypes as *mut libc::c_void,
                                (((*m).numberOfEncTypes + 1 as libc::c_int)
                                    as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<encoding_type>() as libc::c_ulong,
                                    ),
                            ) as *mut encoding_type;
                            if et.is_null() {
                                break;
                            }
                            let ref mut fresh11 = (*m).encodingTypes;
                            *fresh11 = et;
                            let ref mut fresh12 = (*m).numberOfEncTypes;
                            let fresh13 = *fresh12;
                            *fresh12 = *fresh12 + 1;
                            *((*m).encodingTypes).offset(fresh13 as isize) = (*e).type_0;
                            cli_dbgmsg(
                                b"Encoding type %d is \"%s\"\n\0" as *const u8
                                    as *const libc::c_char,
                                (*m).numberOfEncTypes,
                                type_0,
                            );
                            break;
                        }
                    } else if sim > highestSimil {
                        closest = (*e).string;
                        highestSimil = sim;
                    }
                }
            }
            e = e.offset(1);
        }
        if ((*e).string).is_null() {
            if highestSimil >= 50 as libc::c_int {
                cli_dbgmsg(
                    b"Unknown encoding type \"%s\" - guessing as %s (%u%% certainty)\n\0"
                        as *const u8 as *const libc::c_char,
                    type_0,
                    closest,
                    highestSimil,
                );
                messageSetEncoding(m, closest);
            } else {
                cli_dbgmsg(
                    b"Unknown encoding type \"%s\" - if you believe this file contains a virus, submit it to www.clamav.net\n\0"
                        as *const u8 as *const libc::c_char,
                    type_0,
                );
                messageSetEncoding(m, b"base64\0" as *const u8 as *const libc::c_char);
                messageSetEncoding(
                    m,
                    b"quoted-printable\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        free(type_0 as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn messageGetEncoding(mut m: *const message) -> encoding_type {
    if m.is_null() {
        cli_errmsg(
            b"Internal email parser error: invalid arguments when checking message encoding type\n\0"
                as *const u8 as *const libc::c_char,
        );
        return NOENCODING;
    }
    if (*m).numberOfEncTypes == 0 as libc::c_int {
        return NOENCODING;
    }
    return *((*m).encodingTypes).offset(0 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn messageAddLine(
    mut m: *mut message,
    mut line: *mut line_t,
) -> libc::c_int {
    if m.is_null() {
        cli_errmsg(
            b"Internal email parser error: invalid arguments when adding line to message.\n\0"
                as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if ((*m).body_first).is_null() {
        let ref mut fresh14 = (*m).body_first;
        *fresh14 = cli_malloc(::std::mem::size_of::<text>() as libc::c_ulong)
            as *mut text;
        let ref mut fresh15 = (*m).body_last;
        *fresh15 = *fresh14;
    } else {
        let ref mut fresh16 = (*(*m).body_last).t_next;
        *fresh16 = cli_malloc(::std::mem::size_of::<text>() as libc::c_ulong)
            as *mut text;
        let ref mut fresh17 = (*m).body_last;
        *fresh17 = (*(*m).body_last).t_next;
    }
    if ((*m).body_last).is_null() {
        cli_errmsg(
            b"messageAddLine: out of memory for m->body_last\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let ref mut fresh18 = (*(*m).body_last).t_next;
    *fresh18 = 0 as *mut text;
    if !line.is_null() && !(lineGetData(line)).is_null() {
        let ref mut fresh19 = (*(*m).body_last).t_line;
        *fresh19 = lineLink(line);
        messageIsEncoding(m);
    } else {
        let ref mut fresh20 = (*(*m).body_last).t_line;
        *fresh20 = 0 as *mut line_t;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn messageAddStr(
    mut m: *mut message,
    mut data: *const libc::c_char,
) -> libc::c_int {
    let mut repeat: *mut line_t = 0 as *mut line_t;
    if m.is_null() {
        cli_errmsg(
            b"messageAddStr: invalid arguments\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if !data.is_null() {
        if *data as libc::c_int == '\0' as i32 {
            data = 0 as *const libc::c_char;
        } else {
            let mut iswhite: libc::c_int = 1 as libc::c_int;
            let mut p: *const libc::c_char = 0 as *const libc::c_char;
            p = data;
            while *p != 0 {
                if *p as libc::c_int & 0x80 as libc::c_int != 0
                    || *(*__ctype_b_loc()).offset(*p as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    iswhite = 0 as libc::c_int;
                    break;
                } else {
                    p = p.offset(1);
                }
            }
            if iswhite != 0 {
                data = b" \0" as *const u8 as *const libc::c_char;
            }
        }
    }
    if ((*m).body_first).is_null() {
        let ref mut fresh21 = (*m).body_first;
        *fresh21 = cli_malloc(::std::mem::size_of::<text>() as libc::c_ulong)
            as *mut text;
        let ref mut fresh22 = (*m).body_last;
        *fresh22 = *fresh21;
    } else if ((*m).body_last).is_null() {
        cli_errmsg(
            b"Internal email parser error: message 'body_last' pointer should not be NULL if 'body_first' is set.\n\0"
                as *const u8 as *const libc::c_char,
        );
    } else {
        if data.is_null() && ((*(*m).body_last).t_line).is_null() {
            if messageGetMimeType(m) as libc::c_uint
                != TEXT as libc::c_int as libc::c_uint
            {
                return 1 as libc::c_int;
            }
        }
        let ref mut fresh23 = (*(*m).body_last).t_next;
        *fresh23 = cli_malloc(::std::mem::size_of::<text>() as libc::c_ulong)
            as *mut text;
        if ((*(*m).body_last).t_next).is_null() {
            messageDedup(m);
            let ref mut fresh24 = (*(*m).body_last).t_next;
            *fresh24 = cli_malloc(::std::mem::size_of::<text>() as libc::c_ulong)
                as *mut text;
            if ((*(*m).body_last).t_next).is_null() {
                cli_errmsg(
                    b"messageAddStr: out of memory\n\0" as *const u8
                        as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
        }
        if !data.is_null() && !((*(*m).body_last).t_line).is_null()
            && strcmp(data, lineGetData((*(*m).body_last).t_line)) == 0 as libc::c_int
        {
            repeat = (*(*m).body_last).t_line;
        }
        let ref mut fresh25 = (*m).body_last;
        *fresh25 = (*(*m).body_last).t_next;
    }
    if ((*m).body_last).is_null() {
        cli_errmsg(
            b"messageAddStr: out of memory\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let ref mut fresh26 = (*(*m).body_last).t_next;
    *fresh26 = 0 as *mut text;
    if !data.is_null() && *data as libc::c_int != 0 {
        if !repeat.is_null() {
            let ref mut fresh27 = (*(*m).body_last).t_line;
            *fresh27 = lineLink(repeat);
        } else {
            let ref mut fresh28 = (*(*m).body_last).t_line;
            *fresh28 = lineCreate(data);
            if ((*(*m).body_last).t_line).is_null() {
                messageDedup(m);
                let ref mut fresh29 = (*(*m).body_last).t_line;
                *fresh29 = lineCreate(data);
                if ((*(*m).body_last).t_line).is_null() {
                    cli_errmsg(
                        b"messageAddStr: out of memory\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    return -(1 as libc::c_int);
                }
            }
            messageIsEncoding(m);
        }
    } else {
        let ref mut fresh30 = (*(*m).body_last).t_line;
        *fresh30 = 0 as *mut line_t;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn messageMoveText(
    mut m: *mut message,
    mut t: *mut text,
    mut old_message: *mut message,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    if ((*m).body_first).is_null() {
        if !old_message.is_null() && !((*old_message).body_first).is_null() {
            let mut u: *mut text = 0 as *mut text;
            let ref mut fresh31 = (*m).body_first;
            *fresh31 = t;
            u = (*old_message).body_first;
            while u != t {
                let mut next: *mut text = 0 as *mut text;
                if !((*u).t_line).is_null() {
                    lineUnlink((*u).t_line);
                    let ref mut fresh32 = (*u).t_line;
                    *fresh32 = 0 as *mut line_t;
                }
                next = (*u).t_next;
                free(u as *mut libc::c_void);
                u = next;
                if u.is_null() {
                    cli_dbgmsg(
                        b"messageMoveText sanity check: t not within old_message\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    return -(1 as libc::c_int);
                }
            }
            let ref mut fresh33 = (*m).body_last;
            *fresh33 = (*old_message).body_last;
            let ref mut fresh34 = (*old_message).body_last;
            *fresh34 = 0 as *mut text;
            let ref mut fresh35 = (*old_message).body_first;
            *fresh35 = *fresh34;
            if ((*old_message).bounce).is_null() && ((*old_message).encoding).is_null()
                && ((*old_message).binhex).is_null() && ((*old_message).yenc).is_null()
            {
                return 0 as libc::c_int;
            }
            let ref mut fresh36 = (*m).body_last;
            *fresh36 = (*m).body_first;
            rc = 0 as libc::c_int;
        } else {
            let ref mut fresh37 = (*m).body_first;
            *fresh37 = textMove(0 as *mut text, t);
            let ref mut fresh38 = (*m).body_last;
            *fresh38 = *fresh37;
            if ((*m).body_first).is_null() {
                return -(1 as libc::c_int)
            } else {
                rc = 0 as libc::c_int;
            }
        }
    } else {
        let ref mut fresh39 = (*m).body_last;
        *fresh39 = textMove((*m).body_last, t);
        if ((*m).body_last).is_null() {
            rc = -(1 as libc::c_int);
            let ref mut fresh40 = (*m).body_last;
            *fresh40 = (*m).body_first;
        } else {
            rc = 0 as libc::c_int;
        }
    }
    while !((*(*m).body_last).t_next).is_null() {
        let ref mut fresh41 = (*m).body_last;
        *fresh41 = (*(*m).body_last).t_next;
        if !((*(*m).body_last).t_line).is_null() {
            messageIsEncoding(m);
        }
    }
    return rc;
}
unsafe extern "C" fn messageIsEncoding(mut m: *mut message) {
    static mut encoding: [libc::c_char; 26] = unsafe {
        *::std::mem::transmute::<
            &[u8; 26],
            &[libc::c_char; 26],
        >(b"Content-Transfer-Encoding\0")
    };
    static mut binhex: [libc::c_char; 46] = unsafe {
        *::std::mem::transmute::<
            &[u8; 46],
            &[libc::c_char; 46],
        >(b"(This file must be converted with BinHex 4.0)\0")
    };
    let mut line: *const libc::c_char = lineGetData((*(*m).body_last).t_line);
    if ((*m).encoding).is_null()
        && strncasecmp(
            line,
            encoding.as_ptr(),
            (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
        && (strstr(line, b"7bit\0" as *const u8 as *const libc::c_char)).is_null()
    {
        let ref mut fresh42 = (*m).encoding;
        *fresh42 = (*m).body_last;
    } else if ((*m).bounce).is_null() && !((*m).ctx).is_null()
        && strncasecmp(
            line,
            b"Received: \0" as *const u8 as *const libc::c_char,
            10 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        && cli_compare_ftm_file(
            line as *const libc::c_uchar,
            strlen(line),
            (*(*m).ctx).engine,
        ) as libc::c_uint == CL_TYPE_MAIL as libc::c_int as libc::c_uint
    {
        let ref mut fresh43 = (*m).bounce;
        *fresh43 = (*m).body_last;
    } else if ((*m).binhex).is_null()
        && !(strstr(line, b"BinHex\0" as *const u8 as *const libc::c_char)).is_null()
        && simil(line, binhex.as_ptr()) > 90 as libc::c_int
    {
        let ref mut fresh44 = (*m).binhex;
        *fresh44 = (*m).body_last;
    } else if ((*m).yenc).is_null()
        && strncmp(
            line,
            b"=ybegin line=\0" as *const u8 as *const libc::c_char,
            13 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        let ref mut fresh45 = (*m).yenc;
        *fresh45 = (*m).body_last;
    }
}
#[no_mangle]
pub unsafe extern "C" fn messageGetBody(mut m: *mut message) -> *mut text {
    if m.is_null() {
        return 0 as *mut text;
    }
    return (*m).body_first;
}
unsafe extern "C" fn messageExport(
    mut m: *mut message,
    mut dir: *const libc::c_char,
    mut create: Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    mut destroy: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut setFilename: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            *const libc::c_char,
        ) -> (),
    >,
    mut addData: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_uchar,
            size_t,
        ) -> libc::c_int,
    >,
    mut exportText: Option::<
        unsafe extern "C" fn(
            *mut text,
            *mut libc::c_void,
            libc::c_int,
        ) -> *mut libc::c_void,
    >,
    mut setCTX: Option::<unsafe extern "C" fn(*mut libc::c_void, *mut cli_ctx) -> ()>,
    mut destroy_text: libc::c_int,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut t_line: *mut text = 0 as *mut text;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if m.is_null() {
        return 0 as *mut libc::c_void;
    }
    if (messageGetBody(m)).is_null() {
        return 0 as *mut libc::c_void;
    }
    ret = (Some(create.expect("non-null function pointer")))
        .expect("non-null function pointer")();
    if ret.is_null() {
        return 0 as *mut libc::c_void;
    }
    cli_dbgmsg(
        b"messageExport: numberOfEncTypes == %d\n\0" as *const u8 as *const libc::c_char,
        (*m).numberOfEncTypes,
    );
    if (*m).numberOfEncTypes == 0 as libc::c_int {
        cli_dbgmsg(
            b"messageExport: Entering fast copy mode\n\0" as *const u8
                as *const libc::c_char,
        );
        filename = messageFindArgument(
            m,
            b"filename\0" as *const u8 as *const libc::c_char,
        );
        if filename.is_null() {
            filename = messageFindArgument(
                m,
                b"name\0" as *const u8 as *const libc::c_char,
            );
            if filename.is_null() {
                cli_dbgmsg(
                    b"Unencoded attachment sent with no filename\n\0" as *const u8
                        as *const libc::c_char,
                );
                messageAddArgument(
                    m,
                    b"name=attachment\0" as *const u8 as *const libc::c_char,
                );
            } else {
                messageSetEncoding(m, b"7-bit\0" as *const u8 as *const libc::c_char);
            }
        }
        (Some(setFilename.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            ret,
            dir,
            if !filename.is_null() && *filename as libc::c_int != 0 {
                filename as *const libc::c_char
            } else {
                b"attachment\0" as *const u8 as *const libc::c_char
            },
        );
        if !filename.is_null() {
            free(filename as *mut libc::c_void);
        }
        if (*m).numberOfEncTypes == 0 as libc::c_int {
            return exportText
                .expect(
                    "non-null function pointer",
                )(messageGetBody(m), ret, destroy_text);
        }
    }
    if setCTX.is_some() && !((*m).ctx).is_null() {
        (Some(setCTX.expect("non-null function pointer")))
            .expect("non-null function pointer")(ret, (*m).ctx);
    }
    i = 0 as libc::c_int;
    while i < (*m).numberOfEncTypes {
        let mut enctype: encoding_type = *((*m).encodingTypes).offset(i as isize);
        let mut size: size_t = 0;
        if i > 0 as libc::c_int {
            let mut newret: *mut libc::c_void = 0 as *mut libc::c_void;
            newret = (Some(create.expect("non-null function pointer")))
                .expect("non-null function pointer")();
            if newret.is_null() {
                cli_dbgmsg(
                    b"Not all decoding algorithms were run\n\0" as *const u8
                        as *const libc::c_char,
                );
                return ret;
            }
            (Some(destroy.expect("non-null function pointer")))
                .expect("non-null function pointer")(ret);
            ret = newret;
        }
        cli_dbgmsg(
            b"messageExport: enctype %d is %d\n\0" as *const u8 as *const libc::c_char,
            i,
            enctype as libc::c_int,
        );
        if (enctype as libc::c_uint == YENCODE as libc::c_int as libc::c_uint
            || i == 0 as libc::c_int) && !(yEncBegin(m)).is_null()
        {
            let mut f: *const libc::c_char = 0 as *const libc::c_char;
            t_line = yEncBegin(m);
            f = lineGetData((*t_line).t_line);
            filename = strstr(f, b" name=\0" as *const u8 as *const libc::c_char);
            if !filename.is_null() {
                filename = cli_strdup(&mut *filename.offset(6 as libc::c_int as isize));
                if !filename.is_null() {
                    cli_chomp(filename);
                    strstrip(filename);
                    cli_dbgmsg(
                        b"Set yEnc filename to \"%s\"\n\0" as *const u8
                            as *const libc::c_char,
                        filename,
                    );
                }
            }
            (Some(setFilename.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                ret,
                dir,
                if !filename.is_null() && *filename as libc::c_int != 0 {
                    filename as *const libc::c_char
                } else {
                    b"attachment\0" as *const u8 as *const libc::c_char
                },
            );
            if !filename.is_null() {
                free(filename as *mut libc::c_void);
                filename = 0 as *mut libc::c_char;
            }
            t_line = (*t_line).t_next;
            enctype = YENCODE;
            let ref mut fresh46 = (*m).yenc;
            *fresh46 = 0 as *mut text;
        } else {
            if enctype as libc::c_uint == UUENCODE as libc::c_int as libc::c_uint {
                cli_dbgmsg(
                    b"messageExport: treat uuencode as text/plain\n\0" as *const u8
                        as *const libc::c_char,
                );
                let ref mut fresh47 = *((*m).encodingTypes).offset(i as isize);
                *fresh47 = NOENCODING;
                enctype = *fresh47;
            }
            filename = messageGetFilename(m);
            if filename.is_null() {
                cli_dbgmsg(
                    b"Attachment sent with no filename\n\0" as *const u8
                        as *const libc::c_char,
                );
                messageAddArgument(
                    m,
                    b"name=attachment\0" as *const u8 as *const libc::c_char,
                );
            } else if enctype as libc::c_uint
                == NOENCODING as libc::c_int as libc::c_uint
            {
                messageSetEncoding(m, b"base64\0" as *const u8 as *const libc::c_char);
            }
            (Some(setFilename.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                ret,
                dir,
                if !filename.is_null() && *filename as libc::c_int != 0 {
                    filename as *const libc::c_char
                } else {
                    b"attachment\0" as *const u8 as *const libc::c_char
                },
            );
            t_line = messageGetBody(m);
        }
        if !filename.is_null() {
            free(filename as *mut libc::c_void);
        }
        if t_line.is_null() {
            cli_dbgmsg(
                b"Empty attachment not saved\n\0" as *const u8 as *const libc::c_char,
            );
            (Some(destroy.expect("non-null function pointer")))
                .expect("non-null function pointer")(ret);
            return 0 as *mut libc::c_void;
        }
        if enctype as libc::c_uint == NOENCODING as libc::c_int as libc::c_uint {
            if i == (*m).numberOfEncTypes - 1 as libc::c_int {
                exportText
                    .expect("non-null function pointer")(t_line, ret, destroy_text);
                break;
            } else {
                exportText
                    .expect("non-null function pointer")(t_line, ret, 0 as libc::c_int);
            }
        } else {
            size = 0 as libc::c_int as size_t;
            let mut current_block_103: u64;
            loop {
                let mut smallbuf: [libc::c_uchar; 1024] = [0; 1024];
                let mut uptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                let mut line: *const libc::c_char = lineGetData((*t_line).t_line);
                let mut bigbuf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                let mut datasize: size_t = 0;
                if enctype as libc::c_uint == YENCODE as libc::c_int as libc::c_uint {
                    if line.is_null() {
                        current_block_103 = 993425571616822999;
                    } else {
                        if strncmp(
                            line,
                            b"=yend \0" as *const u8 as *const libc::c_char,
                            6 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            break;
                        }
                        current_block_103 = 2750570471926810434;
                    }
                } else {
                    current_block_103 = 2750570471926810434;
                }
                match current_block_103 {
                    2750570471926810434 => {
                        datasize = if !line.is_null() {
                            (strlen(line))
                                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                        } else {
                            0 as libc::c_int as libc::c_ulong
                        };
                        if datasize
                            >= ::std::mem::size_of::<[libc::c_uchar; 1024]>()
                                as libc::c_ulong
                        {
                            bigbuf = cli_malloc(datasize) as *mut libc::c_uchar;
                            data = bigbuf;
                            if data.is_null() {
                                cli_dbgmsg(
                                    b"Failed to allocate data buffer of size %zu\n\0"
                                        as *const u8 as *const libc::c_char,
                                    datasize,
                                );
                                break;
                            }
                        } else {
                            bigbuf = 0 as *mut libc::c_uchar;
                            data = smallbuf.as_mut_ptr();
                            datasize = ::std::mem::size_of::<[libc::c_uchar; 1024]>()
                                as libc::c_ulong;
                        }
                        uptr = decodeLine(m, enctype, line, data, datasize);
                        if uptr.is_null() {
                            if data == bigbuf {
                                free(data as *mut libc::c_void);
                            }
                            break;
                        } else {
                            if uptr != data {
                                (Some(addData.expect("non-null function pointer")))
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    ret,
                                    data,
                                    uptr.offset_from(data) as libc::c_long as size_t,
                                );
                                size = (size as libc::c_ulong)
                                    .wrapping_add(
                                        uptr.offset_from(data) as libc::c_long as size_t,
                                    ) as size_t as size_t;
                            }
                            if data == bigbuf {
                                free(data as *mut libc::c_void);
                            }
                            if !line.is_null() && destroy_text != 0
                                && i == (*m).numberOfEncTypes - 1 as libc::c_int
                            {
                                lineUnlink((*t_line).t_line);
                                let ref mut fresh48 = (*t_line).t_line;
                                *fresh48 = 0 as *mut line_t;
                            }
                        }
                    }
                    _ => {}
                }
                t_line = (*t_line).t_next;
                if t_line.is_null() {
                    break;
                }
            }
            cli_dbgmsg(
                b"Exported %lu bytes using enctype %d\n\0" as *const u8
                    as *const libc::c_char,
                size,
                enctype as libc::c_int,
            );
            if (*m).base64chars != 0 {
                let mut data_0: [libc::c_uchar; 4] = [0; 4];
                let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                ptr = base64Flush(m, data_0.as_mut_ptr());
                if !ptr.is_null() {
                    (Some(addData.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        ret,
                        data_0.as_mut_ptr(),
                        ptr.offset_from(data_0.as_mut_ptr()) as libc::c_long as size_t,
                    );
                }
            }
        }
        i += 1;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn base64Flush(
    mut m: *mut message,
    mut buf: *mut libc::c_uchar,
) -> *mut libc::c_uchar {
    cli_dbgmsg(
        b"%d trailing bytes to export\n\0" as *const u8 as *const libc::c_char,
        (*m).base64chars,
    );
    if (*m).base64chars != 0 {
        let mut ret: *mut libc::c_uchar = decode(
            m,
            0 as *const libc::c_char,
            buf,
            Some(base64 as unsafe extern "C" fn(libc::c_char) -> libc::c_uchar),
            0 as libc::c_int != 0,
        );
        (*m).base64chars = 0 as libc::c_int;
        return ret;
    }
    return 0 as *mut libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn messageSavePartial(
    mut m: *mut message,
    mut dir: *const libc::c_char,
    mut md5id: *const libc::c_char,
    mut part: libc::c_uint,
) -> libc::c_int {
    let mut fullname: [libc::c_char; 1024] = [0; 1024];
    let mut fb: *mut fileblob = 0 as *mut fileblob;
    let mut time_val: libc::c_ulong = 0;
    cli_dbgmsg(b"messageSavePartial\n\0" as *const u8 as *const libc::c_char);
    time_val = time(0 as *mut time_t) as libc::c_ulong;
    snprintf(
        fullname.as_mut_ptr(),
        1024 as libc::c_int as libc::c_ulong,
        b"%s/clamav-partial-%lu_%s-%u\0" as *const u8 as *const libc::c_char,
        dir,
        time_val,
        md5id,
        part,
    );
    fb = messageExport(
        m,
        fullname.as_mut_ptr(),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> *mut fileblob>,
            Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
        >(Some(fileblobCreate as unsafe extern "C" fn() -> *mut fileblob)),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut fileblob) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(fileblobDestroy as unsafe extern "C" fn(*mut fileblob) -> ())),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut fileblob,
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> (),
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> (),
            >,
        >(
            Some(
                fileblobPartialSet
                    as unsafe extern "C" fn(
                        *mut fileblob,
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> (),
            ),
        ),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut fileblob,
                    *const libc::c_uchar,
                    size_t,
                ) -> libc::c_int,
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_uchar,
                    size_t,
                ) -> libc::c_int,
            >,
        >(
            Some(
                fileblobAddData
                    as unsafe extern "C" fn(
                        *mut fileblob,
                        *const libc::c_uchar,
                        size_t,
                    ) -> libc::c_int,
            ),
        ),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut text,
                    *mut fileblob,
                    libc::c_int,
                ) -> *mut fileblob,
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut text,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> *mut libc::c_void,
            >,
        >(
            Some(
                textToFileblob
                    as unsafe extern "C" fn(
                        *mut text,
                        *mut fileblob,
                        libc::c_int,
                    ) -> *mut fileblob,
            ),
        ),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut fileblob, *mut cli_ctx) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void, *mut cli_ctx) -> ()>,
        >(
            Some(
                fileblobSetCTX as unsafe extern "C" fn(*mut fileblob, *mut cli_ctx) -> (),
            ),
        ),
        0 as libc::c_int,
    ) as *mut fileblob;
    if fb.is_null() {
        return CL_EFORMAT as libc::c_int;
    }
    fileblobDestroy(fb);
    return CL_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn messageToFileblob(
    mut m: *mut message,
    mut dir: *const libc::c_char,
    mut destroy: libc::c_int,
) -> *mut fileblob {
    let mut fb: *mut fileblob = 0 as *mut fileblob;
    cli_dbgmsg(b"messageToFileblob\n\0" as *const u8 as *const libc::c_char);
    fb = messageExport(
        m,
        dir,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> *mut fileblob>,
            Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
        >(Some(fileblobCreate as unsafe extern "C" fn() -> *mut fileblob)),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut fileblob) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(fileblobDestroy as unsafe extern "C" fn(*mut fileblob) -> ())),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut fileblob,
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> (),
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> (),
            >,
        >(
            Some(
                fileblobSetFilename
                    as unsafe extern "C" fn(
                        *mut fileblob,
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> (),
            ),
        ),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut fileblob,
                    *const libc::c_uchar,
                    size_t,
                ) -> libc::c_int,
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_uchar,
                    size_t,
                ) -> libc::c_int,
            >,
        >(
            Some(
                fileblobAddData
                    as unsafe extern "C" fn(
                        *mut fileblob,
                        *const libc::c_uchar,
                        size_t,
                    ) -> libc::c_int,
            ),
        ),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut text,
                    *mut fileblob,
                    libc::c_int,
                ) -> *mut fileblob,
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut text,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> *mut libc::c_void,
            >,
        >(
            Some(
                textToFileblob
                    as unsafe extern "C" fn(
                        *mut text,
                        *mut fileblob,
                        libc::c_int,
                    ) -> *mut fileblob,
            ),
        ),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut fileblob, *mut cli_ctx) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void, *mut cli_ctx) -> ()>,
        >(
            Some(
                fileblobSetCTX as unsafe extern "C" fn(*mut fileblob, *mut cli_ctx) -> (),
            ),
        ),
        destroy,
    ) as *mut fileblob;
    if destroy != 0 && !((*m).body_first).is_null() {
        textDestroy((*m).body_first);
        let ref mut fresh49 = (*m).body_last;
        *fresh49 = 0 as *mut text;
        let ref mut fresh50 = (*m).body_first;
        *fresh50 = *fresh49;
    }
    return fb;
}
#[no_mangle]
pub unsafe extern "C" fn messageToBlob(
    mut m: *mut message,
    mut destroy: libc::c_int,
) -> *mut blob {
    let mut b: *mut blob = 0 as *mut blob;
    cli_dbgmsg(b"messageToBlob\n\0" as *const u8 as *const libc::c_char);
    b = messageExport(
        m,
        0 as *const libc::c_char,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> *mut blob>,
            Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
        >(Some(blobCreate as unsafe extern "C" fn() -> *mut blob)),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut blob) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(blobDestroy as unsafe extern "C" fn(*mut blob) -> ())),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut blob,
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> (),
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> (),
            >,
        >(
            Some(
                blobSetFilename
                    as unsafe extern "C" fn(
                        *mut blob,
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> (),
            ),
        ),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut blob,
                    *const libc::c_uchar,
                    size_t,
                ) -> libc::c_int,
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_uchar,
                    size_t,
                ) -> libc::c_int,
            >,
        >(
            Some(
                blobAddData
                    as unsafe extern "C" fn(
                        *mut blob,
                        *const libc::c_uchar,
                        size_t,
                    ) -> libc::c_int,
            ),
        ),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut text, *mut blob, libc::c_int) -> *mut blob,
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut text,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> *mut libc::c_void,
            >,
        >(
            Some(
                textToBlob
                    as unsafe extern "C" fn(
                        *mut text,
                        *mut blob,
                        libc::c_int,
                    ) -> *mut blob,
            ),
        ),
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*mut libc::c_void, *mut cli_ctx) -> ()>,
        >(0 as *mut libc::c_void),
        destroy,
    ) as *mut blob;
    if destroy != 0 && !((*m).body_first).is_null() {
        textDestroy((*m).body_first);
        let ref mut fresh51 = (*m).body_last;
        *fresh51 = 0 as *mut text;
        let ref mut fresh52 = (*m).body_first;
        *fresh52 = *fresh51;
    }
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn messageToText(mut m: *mut message) -> *mut text {
    let mut i: libc::c_int = 0;
    let mut first: *mut text = 0 as *mut text;
    let mut last: *mut text = 0 as *mut text;
    let mut t_line: *const text = 0 as *const text;
    if m.is_null() {
        cli_errmsg(
            b"Internal email parser error: invalid arguments when converting message to text.\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as *mut text;
    }
    if (*m).numberOfEncTypes == 0 as libc::c_int {
        t_line = messageGetBody(m);
        while !t_line.is_null() {
            if first.is_null() {
                last = cli_malloc(::std::mem::size_of::<text>() as libc::c_ulong)
                    as *mut text;
                first = last;
            } else {
                let ref mut fresh53 = (*last).t_next;
                *fresh53 = cli_malloc(::std::mem::size_of::<text>() as libc::c_ulong)
                    as *mut text;
                last = (*last).t_next;
            }
            if last.is_null() {
                if !first.is_null() {
                    textDestroy(first);
                }
                return 0 as *mut text;
            }
            if !((*t_line).t_line).is_null() {
                let ref mut fresh54 = (*last).t_line;
                *fresh54 = lineLink((*t_line).t_line);
            } else {
                let ref mut fresh55 = (*last).t_line;
                *fresh55 = 0 as *mut line_t;
            }
            t_line = (*t_line).t_next;
        }
        if !last.is_null() {
            let ref mut fresh56 = (*last).t_next;
            *fresh56 = 0 as *mut text;
        }
        return first;
    }
    let mut current_block_94: u64;
    i = 0 as libc::c_int;
    while i < (*m).numberOfEncTypes {
        let enctype: encoding_type = *((*m).encodingTypes).offset(i as isize);
        cli_dbgmsg(
            b"messageToText: export transfer method %d = %d\n\0" as *const u8
                as *const libc::c_char,
            i,
            enctype as libc::c_int,
        );
        match enctype as libc::c_uint {
            0 | 4 | 3 => {
                t_line = messageGetBody(m);
                while !t_line.is_null() {
                    if first.is_null() {
                        last = cli_malloc(::std::mem::size_of::<text>() as libc::c_ulong)
                            as *mut text;
                        first = last;
                    } else if !last.is_null() {
                        let ref mut fresh57 = (*last).t_next;
                        *fresh57 = cli_malloc(
                            ::std::mem::size_of::<text>() as libc::c_ulong,
                        ) as *mut text;
                        last = (*last).t_next;
                    }
                    if last.is_null() {
                        if !first.is_null() {
                            textDestroy(first);
                        }
                        return 0 as *mut text;
                    }
                    if !((*t_line).t_line).is_null() {
                        let ref mut fresh58 = (*last).t_line;
                        *fresh58 = lineLink((*t_line).t_line);
                    } else {
                        let ref mut fresh59 = (*last).t_line;
                        *fresh59 = 0 as *mut line_t;
                    }
                    t_line = (*t_line).t_next;
                }
                current_block_94 = 13472856163611868459;
            }
            5 => {
                cli_warnmsg(
                    b"messageToText: Unexpected attempt to handle uuencoded file\n\0"
                        as *const u8 as *const libc::c_char,
                );
                if !first.is_null() {
                    if !last.is_null() {
                        let ref mut fresh60 = (*last).t_next;
                        *fresh60 = 0 as *mut text;
                    }
                    textDestroy(first);
                }
                return 0 as *mut text;
            }
            6 => {
                t_line = yEncBegin(m);
                if t_line.is_null() {
                    if !first.is_null() {
                        if !last.is_null() {
                            let ref mut fresh61 = (*last).t_next;
                            *fresh61 = 0 as *mut text;
                        }
                        textDestroy(first);
                    }
                    return 0 as *mut text;
                }
                t_line = (*t_line).t_next;
                current_block_94 = 15310302294506848511;
            }
            _ => {
                current_block_94 = 15310302294506848511;
            }
        }
        match current_block_94 {
            15310302294506848511 => {
                if i == 0 as libc::c_int && !(binhexBegin(m)).is_null() {
                    cli_warnmsg(
                        b"Binhex messages not supported yet.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                t_line = messageGetBody(m);
                let mut current_block_79: u64;
                while !t_line.is_null() {
                    let mut data: [libc::c_uchar; 1024] = [0; 1024];
                    let mut uptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                    let mut line: *const libc::c_char = lineGetData((*t_line).t_line);
                    if enctype as libc::c_uint == BASE64 as libc::c_int as libc::c_uint {
                        if line.is_null() {
                            current_block_79 = 6243635450180130569;
                        } else {
                            current_block_79 = 129780949503461575;
                        }
                    } else {
                        current_block_79 = 129780949503461575;
                    }
                    match current_block_79 {
                        129780949503461575 => {
                            if !line.is_null()
                                && strlen(line)
                                    > ::std::mem::size_of::<[libc::c_uchar; 1024]>()
                                        as libc::c_ulong
                            {
                                cli_errmsg(
                                    b"Internal email parser error: line size greater than size of receiving data buffer\n\0"
                                        as *const u8 as *const libc::c_char,
                                );
                                break;
                            } else {
                                uptr = decodeLine(
                                    m,
                                    enctype,
                                    line,
                                    data.as_mut_ptr(),
                                    ::std::mem::size_of::<[libc::c_uchar; 1024]>()
                                        as libc::c_ulong,
                                );
                                if uptr.is_null() {
                                    break;
                                }
                                if uptr.offset_from(data.as_mut_ptr()) as libc::c_long
                                    as size_t
                                    > ::std::mem::size_of::<[libc::c_uchar; 1024]>()
                                        as libc::c_ulong
                                {
                                    cli_errmsg(
                                        b"Internal email parser error: line size greater than size of receiving data buffer\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                    break;
                                } else {
                                    if first.is_null() {
                                        last = cli_malloc(
                                            ::std::mem::size_of::<text>() as libc::c_ulong,
                                        ) as *mut text;
                                        first = last;
                                    } else if !last.is_null() {
                                        let ref mut fresh62 = (*last).t_next;
                                        *fresh62 = cli_malloc(
                                            ::std::mem::size_of::<text>() as libc::c_ulong,
                                        ) as *mut text;
                                        last = (*last).t_next;
                                    }
                                    if last.is_null() {
                                        break;
                                    }
                                    if data[0 as libc::c_int as usize] as libc::c_int
                                        == '\n' as i32
                                        || data[0 as libc::c_int as usize] as libc::c_int
                                            == '\0' as i32
                                    {
                                        let ref mut fresh63 = (*last).t_line;
                                        *fresh63 = 0 as *mut line_t;
                                    } else if !line.is_null()
                                        && strncmp(
                                            data.as_mut_ptr() as *const libc::c_char,
                                            line,
                                            strlen(line),
                                        ) == 0 as libc::c_int
                                    {
                                        let ref mut fresh64 = (*last).t_line;
                                        *fresh64 = lineLink((*t_line).t_line);
                                    } else {
                                        let ref mut fresh65 = (*last).t_line;
                                        *fresh65 = lineCreate(
                                            data.as_mut_ptr() as *mut libc::c_char,
                                        );
                                    }
                                    if !line.is_null()
                                        && enctype as libc::c_uint
                                            == BASE64 as libc::c_int as libc::c_uint
                                    {
                                        if !(strchr(line, '=' as i32)).is_null() {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    t_line = (*t_line).t_next;
                }
                if (*m).base64chars != 0 {
                    let mut data_0: [libc::c_uchar; 4] = [0; 4];
                    memset(
                        data_0.as_mut_ptr() as *mut libc::c_void,
                        '\0' as i32,
                        ::std::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong,
                    );
                    if !(decode(
                        m,
                        0 as *const libc::c_char,
                        data_0.as_mut_ptr(),
                        Some(
                            base64 as unsafe extern "C" fn(libc::c_char) -> libc::c_uchar,
                        ),
                        0 as libc::c_int != 0,
                    ))
                        .is_null()
                        && data_0[0 as libc::c_int as usize] as libc::c_int != 0
                    {
                        if first.is_null() {
                            last = cli_malloc(
                                ::std::mem::size_of::<text>() as libc::c_ulong,
                            ) as *mut text;
                            first = last;
                        } else if !last.is_null() {
                            let ref mut fresh66 = (*last).t_next;
                            *fresh66 = cli_malloc(
                                ::std::mem::size_of::<text>() as libc::c_ulong,
                            ) as *mut text;
                            last = (*last).t_next;
                        }
                        if !last.is_null() {
                            let ref mut fresh67 = (*last).t_line;
                            *fresh67 = lineCreate(
                                data_0.as_mut_ptr() as *mut libc::c_char,
                            );
                        }
                    }
                    (*m).base64chars = 0 as libc::c_int;
                }
            }
            _ => {}
        }
        i += 1;
    }
    if !last.is_null() {
        let ref mut fresh68 = (*last).t_next;
        *fresh68 = 0 as *mut text;
    }
    return first;
}
#[no_mangle]
pub unsafe extern "C" fn yEncBegin(mut m: *mut message) -> *mut text {
    return (*m).yenc;
}
#[no_mangle]
pub unsafe extern "C" fn binhexBegin(mut m: *mut message) -> *mut text {
    return (*m).binhex;
}
#[no_mangle]
pub unsafe extern "C" fn bounceBegin(mut m: *mut message) -> *mut text {
    return (*m).bounce;
}
#[no_mangle]
pub unsafe extern "C" fn encodingLine(mut m: *mut message) -> *mut text {
    return (*m).encoding;
}
#[no_mangle]
pub unsafe extern "C" fn decodeLine(
    mut m: *mut message,
    mut et: encoding_type,
    mut line: *const libc::c_char,
    mut buf: *mut libc::c_uchar,
    mut buflen: size_t,
) -> *mut libc::c_uchar {
    let mut len: size_t = 0;
    let mut reallen: size_t = 0;
    let mut softbreak: bool = false;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut base64buf: [libc::c_char; 77] = [0; 77];
    if m.is_null() || buf.is_null() {
        cli_dbgmsg(
            b"decodeLine: invalid parameters\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut libc::c_uchar;
    }
    let mut current_block_45: u64;
    match et as libc::c_uint {
        1 => {
            if line.is_null() {
                let fresh69 = buf;
                buf = buf.offset(1);
                *fresh69 = '\n' as i32 as libc::c_uchar;
            } else {
                softbreak = 0 as libc::c_int != 0;
                while buflen != 0 && *line as libc::c_int != 0 {
                    if *line as libc::c_int == '=' as i32 {
                        let mut byte: libc::c_uchar = 0;
                        line = line.offset(1);
                        if *line as libc::c_int == '\0' as i32
                            || *line as libc::c_int == '\n' as i32
                        {
                            softbreak = 1 as libc::c_int != 0;
                            break;
                        } else {
                            byte = hex(*line);
                            line = line.offset(1);
                            if *line as libc::c_int == '\0' as i32
                                || *line as libc::c_int == '\n' as i32
                            {
                                let fresh70 = buf;
                                buf = buf.offset(1);
                                *fresh70 = byte;
                                break;
                            } else {
                                if byte as libc::c_int != '=' as i32 {
                                    byte = ((byte as libc::c_int) << 4 as libc::c_int
                                        | hex(*line) as libc::c_int) as libc::c_uchar;
                                } else {
                                    line = line.offset(-(2 as libc::c_int as isize));
                                }
                                let fresh71 = buf;
                                buf = buf.offset(1);
                                *fresh71 = byte;
                            }
                        }
                    } else {
                        let fresh72 = buf;
                        buf = buf.offset(1);
                        *fresh72 = *line as libc::c_uchar;
                    }
                    line = line.offset(1);
                    buflen = buflen.wrapping_sub(1);
                }
                if !softbreak {
                    let fresh73 = buf;
                    buf = buf.offset(1);
                    *fresh73 = '\n' as i32 as libc::c_uchar;
                }
            }
        }
        2 => {
            if !line.is_null() {
                if strlen(line)
                    < ::std::mem::size_of::<[libc::c_char; 77]>() as libc::c_ulong
                {
                    strcpy(base64buf.as_mut_ptr(), line);
                    copy = base64buf.as_mut_ptr();
                    current_block_45 = 4567019141635105728;
                } else {
                    copy = cli_strdup(line);
                    if copy.is_null() {
                        current_block_45 = 7385833325316299293;
                    } else {
                        current_block_45 = 4567019141635105728;
                    }
                }
                match current_block_45 {
                    7385833325316299293 => {}
                    _ => {
                        p2 = strchr(copy, '=' as i32);
                        if !p2.is_null() {
                            *p2 = '\0' as i32 as libc::c_char;
                        }
                        sanitiseBase64(copy);
                        buf = decode(
                            m,
                            copy,
                            buf,
                            Some(
                                base64
                                    as unsafe extern "C" fn(libc::c_char) -> libc::c_uchar,
                            ),
                            p2.is_null()
                                && strlen(copy) & 3 as libc::c_int as libc::c_ulong
                                    == 0 as libc::c_int as libc::c_ulong,
                        );
                        if copy != base64buf.as_mut_ptr() {
                            free(copy as *mut libc::c_void);
                        }
                    }
                }
            }
        }
        5 => {
            if !(0 as libc::c_int != (*m).base64chars) {
                if !(line.is_null() || *line as libc::c_int == '\0' as i32) {
                    if !(strcasecmp(line, b"end\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int)
                    {
                        if !(isuuencodebegin(line) != 0) {
                            if !(*line.offset(0 as libc::c_int as isize) as libc::c_int
                                & 0x3f as libc::c_int == ' ' as i32)
                            {
                                let fresh74 = line;
                                line = line.offset(1);
                                reallen = uudecode(*fresh74) as size_t;
                                if !(reallen <= 0 as libc::c_int as libc::c_ulong) {
                                    if !(reallen > 62 as libc::c_int as libc::c_ulong) {
                                        len = strlen(line);
                                        if len > buflen || reallen > len {
                                            cli_dbgmsg(
                                                b"uudecode: buffer overflow stopped, attempting to ignore but decoding may fail\n\0"
                                                    as *const u8 as *const libc::c_char,
                                            );
                                        } else {
                                            decode(
                                                m,
                                                line,
                                                buf,
                                                Some(
                                                    uudecode
                                                        as unsafe extern "C" fn(libc::c_char) -> libc::c_uchar,
                                                ),
                                                len & 3 as libc::c_int as libc::c_ulong
                                                    == 0 as libc::c_int as libc::c_ulong,
                                            );
                                            buf = &mut *buf.offset(reallen as isize)
                                                as *mut libc::c_uchar;
                                        }
                                        (*m).base64chars = 0 as libc::c_int;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        6 => {
            if !(line.is_null() || *line as libc::c_int == '\0' as i32) {
                if !(strncmp(
                    line,
                    b"=yend \0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int)
                {
                    while *line != 0 {
                        if *line as libc::c_int == '=' as i32 {
                            line = line.offset(1);
                            if *line as libc::c_int == '\0' as i32 {
                                break;
                            }
                            let fresh75 = line;
                            line = line.offset(1);
                            let fresh76 = buf;
                            buf = buf.offset(1);
                            *fresh76 = (*fresh75 as libc::c_int - 64 as libc::c_int
                                & 255 as libc::c_int) as libc::c_uchar;
                        } else {
                            let fresh77 = line;
                            line = line.offset(1);
                            let fresh78 = buf;
                            buf = buf.offset(1);
                            *fresh78 = (*fresh77 as libc::c_int - 42 as libc::c_int
                                & 255 as libc::c_int) as libc::c_uchar;
                        }
                    }
                }
            }
        }
        4 | 0 | 3 | _ => {
            if !line.is_null() {
                buf = cli_strrcpy(buf as *mut libc::c_char, line) as *mut libc::c_uchar;
            }
            return cli_strrcpy(
                buf as *mut libc::c_char,
                b"\n\0" as *const u8 as *const libc::c_char,
            ) as *mut libc::c_uchar;
        }
    }
    *buf = '\0' as i32 as libc::c_uchar;
    return buf;
}
unsafe extern "C" fn sanitiseBase64(mut s: *mut libc::c_char) {
    cli_dbgmsg(b"sanitiseBase64 '%s'\n\0" as *const u8 as *const libc::c_char, s);
    while *s != 0 {
        if base64Table[(*s as libc::c_int & 0xff as libc::c_int) as libc::c_uint
            as usize] as libc::c_int == 255 as libc::c_int
        {
            let mut p1: *mut libc::c_char = 0 as *mut libc::c_char;
            p1 = s;
            while *p1.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
                *p1
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *p1.offset(1 as libc::c_int as isize);
                p1 = p1.offset(1);
            }
        } else {
            s = s.offset(1);
        }
    }
}
unsafe extern "C" fn decode(
    mut m: *mut message,
    mut in_0: *const libc::c_char,
    mut out: *mut libc::c_uchar,
    mut decoder: Option::<unsafe extern "C" fn(libc::c_char) -> libc::c_uchar>,
    mut isFast: bool,
) -> *mut libc::c_uchar {
    let mut b1: libc::c_uchar = 0;
    let mut b2: libc::c_uchar = 0;
    let mut b3: libc::c_uchar = 0;
    let mut b4: libc::c_uchar = 0;
    let mut cb1: libc::c_uchar = 0;
    let mut cb2: libc::c_uchar = 0;
    let mut cb3: libc::c_uchar = 0;
    cb3 = '\0' as i32 as libc::c_uchar;
    cb2 = cb3;
    cb1 = cb2;
    let mut current_block_8: u64;
    match (*m).base64chars {
        3 => {
            cb3 = (*m).base64_3 as libc::c_uchar;
            current_block_8 = 8302290019059933921;
        }
        2 => {
            current_block_8 = 8302290019059933921;
        }
        1 => {
            current_block_8 = 2364459056544639896;
        }
        _ => {
            if (3 as libc::c_int) < (*m).base64chars {
                cli_errmsg(
                    b"email message decode error: invalid base64chars value: %d\n\0"
                        as *const u8 as *const libc::c_char,
                    (*m).base64chars,
                );
                return out;
            }
            current_block_8 = 2968425633554183086;
        }
    }
    match current_block_8 {
        8302290019059933921 => {
            cb2 = (*m).base64_2 as libc::c_uchar;
            current_block_8 = 2364459056544639896;
        }
        _ => {}
    }
    match current_block_8 {
        2364459056544639896 => {
            cb1 = (*m).base64_1 as libc::c_uchar;
            isFast = 0 as libc::c_int != 0;
        }
        _ => {}
    }
    if isFast {
        while *in_0 != 0 {
            let fresh79 = in_0;
            in_0 = in_0.offset(1);
            b1 = (Some(decoder.expect("non-null function pointer")))
                .expect("non-null function pointer")(*fresh79);
            let fresh80 = in_0;
            in_0 = in_0.offset(1);
            b2 = (Some(decoder.expect("non-null function pointer")))
                .expect("non-null function pointer")(*fresh80);
            let fresh81 = in_0;
            in_0 = in_0.offset(1);
            b3 = (Some(decoder.expect("non-null function pointer")))
                .expect("non-null function pointer")(*fresh81);
            let fresh82 = out;
            out = out.offset(1);
            *fresh82 = ((b1 as libc::c_int) << 2 as libc::c_int
                | b2 as libc::c_int >> 4 as libc::c_int & 0x3 as libc::c_int)
                as libc::c_uchar;
            let fresh83 = in_0;
            in_0 = in_0.offset(1);
            b4 = (Some(decoder.expect("non-null function pointer")))
                .expect("non-null function pointer")(*fresh83);
            let fresh84 = out;
            out = out.offset(1);
            *fresh84 = ((b2 as libc::c_int) << 4 as libc::c_int
                | b3 as libc::c_int >> 2 as libc::c_int & 0xf as libc::c_int)
                as libc::c_uchar;
            let fresh85 = out;
            out = out.offset(1);
            *fresh85 = ((b3 as libc::c_int) << 6 as libc::c_int
                | b4 as libc::c_int & 0x3f as libc::c_int) as libc::c_uchar;
        }
    } else if in_0.is_null() {
        let mut nbytes: libc::c_int = 0;
        if (*m).base64chars == 0 as libc::c_int {
            return out;
        }
        cli_dbgmsg(
            b"base64chars = %d (%c %c %c)\n\0" as *const u8 as *const libc::c_char,
            (*m).base64chars,
            if *(*__ctype_b_loc()).offset(cb1 as libc::c_int as isize) as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                cb1 as libc::c_int
            } else {
                '@' as i32
            },
            if *(*__ctype_b_loc()).offset(cb2 as libc::c_int as isize) as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                cb2 as libc::c_int
            } else {
                '@' as i32
            },
            if *(*__ctype_b_loc()).offset(cb3 as libc::c_int as isize) as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                cb3 as libc::c_int
            } else {
                '@' as i32
            },
        );
        let ref mut fresh86 = (*m).base64chars;
        *fresh86 -= 1;
        b1 = cb1;
        nbytes = 1 as libc::c_int;
        if (*m).base64chars != 0 {
            let ref mut fresh87 = (*m).base64chars;
            *fresh87 -= 1;
            b2 = cb2;
            if (*m).base64chars != 0 {
                nbytes = 2 as libc::c_int;
                let ref mut fresh88 = (*m).base64chars;
                *fresh88 -= 1;
                b3 = cb3;
                nbytes = 3 as libc::c_int;
            } else if b2 != 0 {
                nbytes = 2 as libc::c_int;
            }
        }
        let mut current_block_47: u64;
        match nbytes {
            3 => {
                b4 = '\0' as i32 as libc::c_uchar;
                current_block_47 = 4087181708114818847;
            }
            4 => {
                current_block_47 = 4087181708114818847;
            }
            2 => {
                let fresh92 = out;
                out = out.offset(1);
                *fresh92 = ((b1 as libc::c_int) << 2 as libc::c_int
                    | b2 as libc::c_int >> 4 as libc::c_int & 0x3 as libc::c_int)
                    as libc::c_uchar;
                if (b2 as libc::c_int) << 4 as libc::c_int & 0xff as libc::c_int != 0 {
                    let fresh93 = out;
                    out = out.offset(1);
                    *fresh93 = ((b2 as libc::c_int) << 4 as libc::c_int)
                        as libc::c_uchar;
                }
                current_block_47 = 5891011138178424807;
            }
            1 => {
                let fresh94 = out;
                out = out.offset(1);
                *fresh94 = ((b1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar;
                current_block_47 = 5891011138178424807;
            }
            _ => {
                cli_errmsg(
                    b"email message decode error: invalid nbytes value: %d\n\0"
                        as *const u8 as *const libc::c_char,
                    nbytes,
                );
                return out;
            }
        }
        match current_block_47 {
            4087181708114818847 => {
                let fresh89 = out;
                out = out.offset(1);
                *fresh89 = ((b1 as libc::c_int) << 2 as libc::c_int
                    | b2 as libc::c_int >> 4 as libc::c_int & 0x3 as libc::c_int)
                    as libc::c_uchar;
                let fresh90 = out;
                out = out.offset(1);
                *fresh90 = ((b2 as libc::c_int) << 4 as libc::c_int
                    | b3 as libc::c_int >> 2 as libc::c_int & 0xf as libc::c_int)
                    as libc::c_uchar;
                if nbytes == 4 as libc::c_int
                    || b3 as libc::c_int & 0x3 as libc::c_int != 0
                {
                    let fresh91 = out;
                    out = out.offset(1);
                    *fresh91 = ((b3 as libc::c_int) << 6 as libc::c_int
                        | b4 as libc::c_int & 0x3f as libc::c_int) as libc::c_uchar;
                }
            }
            _ => {}
        }
    } else {
        let mut current_block_90: u64;
        while *in_0 != 0 {
            let mut nbytes_0: libc::c_int = 0;
            if (*m).base64chars != 0 {
                let ref mut fresh95 = (*m).base64chars;
                *fresh95 -= 1;
                b1 = cb1;
            } else {
                let fresh96 = in_0;
                in_0 = in_0.offset(1);
                b1 = (Some(decoder.expect("non-null function pointer")))
                    .expect("non-null function pointer")(*fresh96);
            }
            if *in_0 as libc::c_int == '\0' as i32 {
                b2 = '\0' as i32 as libc::c_uchar;
                nbytes_0 = 1 as libc::c_int;
            } else {
                if (*m).base64chars != 0 {
                    let ref mut fresh97 = (*m).base64chars;
                    *fresh97 -= 1;
                    b2 = cb2;
                } else {
                    let fresh98 = in_0;
                    in_0 = in_0.offset(1);
                    b2 = (Some(decoder.expect("non-null function pointer")))
                        .expect("non-null function pointer")(*fresh98);
                }
                if *in_0 as libc::c_int == '\0' as i32 {
                    b3 = '\0' as i32 as libc::c_uchar;
                    nbytes_0 = 2 as libc::c_int;
                } else {
                    if (*m).base64chars != 0 {
                        let ref mut fresh99 = (*m).base64chars;
                        *fresh99 -= 1;
                        b3 = cb3;
                    } else {
                        let fresh100 = in_0;
                        in_0 = in_0.offset(1);
                        b3 = (Some(decoder.expect("non-null function pointer")))
                            .expect("non-null function pointer")(*fresh100);
                    }
                    if *in_0 as libc::c_int == '\0' as i32 {
                        b4 = '\0' as i32 as libc::c_uchar;
                        nbytes_0 = 3 as libc::c_int;
                    } else {
                        let fresh101 = in_0;
                        in_0 = in_0.offset(1);
                        b4 = (Some(decoder.expect("non-null function pointer")))
                            .expect("non-null function pointer")(*fresh101);
                        nbytes_0 = 4 as libc::c_int;
                    }
                }
            }
            match nbytes_0 {
                4 => {
                    let fresh102 = out;
                    out = out.offset(1);
                    *fresh102 = ((b1 as libc::c_int) << 2 as libc::c_int
                        | b2 as libc::c_int >> 4 as libc::c_int & 0x3 as libc::c_int)
                        as libc::c_uchar;
                    let fresh103 = out;
                    out = out.offset(1);
                    *fresh103 = ((b2 as libc::c_int) << 4 as libc::c_int
                        | b3 as libc::c_int >> 2 as libc::c_int & 0xf as libc::c_int)
                        as libc::c_uchar;
                    let fresh104 = out;
                    out = out.offset(1);
                    *fresh104 = ((b3 as libc::c_int) << 6 as libc::c_int
                        | b4 as libc::c_int & 0x3f as libc::c_int) as libc::c_uchar;
                    continue;
                }
                3 => {
                    (*m).base64_3 = b3 as libc::c_char;
                    current_block_90 = 12451867549002202088;
                }
                2 => {
                    current_block_90 = 12451867549002202088;
                }
                1 => {
                    current_block_90 = 13650677630343994444;
                }
                _ => {
                    cli_errmsg(
                        b"email message decode error: invalid nbytes value: %d\n\0"
                            as *const u8 as *const libc::c_char,
                        nbytes_0,
                    );
                    return out;
                }
            }
            match current_block_90 {
                12451867549002202088 => {
                    (*m).base64_2 = b2 as libc::c_char;
                }
                _ => {}
            }
            (*m).base64_1 = b1 as libc::c_char;
            (*m).base64chars = nbytes_0;
            break;
        }
    }
    return out;
}
unsafe extern "C" fn hex(mut c: libc::c_char) -> libc::c_uchar {
    if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        return (c as libc::c_int - '0' as i32) as libc::c_uchar;
    }
    if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'F' as i32 {
        return (c as libc::c_int - 'A' as i32 + 10 as libc::c_int) as libc::c_uchar;
    }
    if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'f' as i32 {
        return (c as libc::c_int - 'a' as i32 + 10 as libc::c_int) as libc::c_uchar;
    }
    cli_dbgmsg(
        b"Illegal hex character '%c'\n\0" as *const u8 as *const libc::c_char,
        c as libc::c_int,
    );
    return '=' as i32 as libc::c_uchar;
}
unsafe extern "C" fn base64(mut c: libc::c_char) -> libc::c_uchar {
    let ret: libc::c_uchar = base64Table[(c as libc::c_int & 0xff as libc::c_int)
        as libc::c_uint as usize];
    if ret as libc::c_int == 255 as libc::c_int {
        return 63 as libc::c_int as libc::c_uchar;
    }
    return ret;
}
unsafe extern "C" fn uudecode(mut c: libc::c_char) -> libc::c_uchar {
    return (c as libc::c_int - ' ' as i32) as libc::c_uchar;
}
unsafe extern "C" fn usefulArg(mut arg: *const libc::c_char) -> libc::c_int {
    if strncasecmp(
        arg,
        b"name\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
        && strncasecmp(
            arg,
            b"filename\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
        && strncasecmp(
            arg,
            b"boundary\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
        && strncasecmp(
            arg,
            b"protocol\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
        && strncasecmp(
            arg,
            b"id\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
        && strncasecmp(
            arg,
            b"number\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
        && strncasecmp(
            arg,
            b"total\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
        && strncasecmp(
            arg,
            b"type\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
    {
        cli_dbgmsg(
            b"Discarding unwanted argument '%s'\n\0" as *const u8 as *const libc::c_char,
            arg,
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn messageSetCTX(mut m: *mut message, mut ctx: *mut cli_ctx) {
    let ref mut fresh105 = (*m).ctx;
    *fresh105 = ctx;
}
#[no_mangle]
pub unsafe extern "C" fn messageContainsVirus(mut m: *const message) -> libc::c_int {
    return if (*m).isInfected() as libc::c_int != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn messageDedup(mut m: *mut message) {
    let mut t1: *const text = 0 as *const text;
    let mut saved: size_t = 0 as libc::c_int as size_t;
    cli_dbgmsg(b"messageDedup\n\0" as *const u8 as *const libc::c_char);
    t1 = if !((*m).dedupedThisFar).is_null() {
        (*m).dedupedThisFar
    } else {
        (*m).body_first as *const text
    };
    t1 = (*m).body_first;
    while !t1.is_null() {
        let mut d1: *const libc::c_char = 0 as *const libc::c_char;
        let mut t2: *mut text = 0 as *mut text;
        let mut l1: *mut line_t = 0 as *mut line_t;
        let mut r1: libc::c_uint = 0;
        if saved >= (100 as libc::c_int * 1000 as libc::c_int) as libc::c_ulong {
            break;
        }
        l1 = (*t1).t_line;
        if !l1.is_null() {
            d1 = lineGetData(l1);
            if !(strlen(d1) < 8 as libc::c_int as libc::c_ulong) {
                r1 = *l1.offset(0 as libc::c_int as isize) as libc::c_uchar
                    as libc::c_uint;
                if !(r1 == 255 as libc::c_int as libc::c_uint) {
                    if !(t1 == (*m).encoding as *const text) {
                        if !(t1 == (*m).bounce as *const text) {
                            if !(t1 == (*m).binhex as *const text) {
                                if !(t1 == (*m).yenc as *const text) {
                                    t2 = (*t1).t_next;
                                    while !t2.is_null() {
                                        let mut d2: *const libc::c_char = 0 as *const libc::c_char;
                                        let mut l2: *mut line_t = (*t2).t_line;
                                        if !l2.is_null() {
                                            d2 = lineGetData(l2);
                                            if !(d1 == d2) {
                                                if strcmp(d1, d2) == 0 as libc::c_int {
                                                    if (lineUnlink(l2)).is_null() {
                                                        saved = (saved as libc::c_ulong)
                                                            .wrapping_add(
                                                                (strlen(d1)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                                                            ) as size_t as size_t;
                                                    }
                                                    let ref mut fresh106 = (*t2).t_line;
                                                    *fresh106 = lineLink(l1);
                                                    if ((*t2).t_line).is_null() {
                                                        cli_errmsg(
                                                            b"messageDedup: out of memory\n\0" as *const u8
                                                                as *const libc::c_char,
                                                        );
                                                        return;
                                                    }
                                                    r1 = r1.wrapping_add(1);
                                                    if r1 == 255 as libc::c_int as libc::c_uint {
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                        t2 = (*t2).t_next;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        t1 = (*t1).t_next;
    }
    cli_dbgmsg(
        b"messageDedup reclaimed %lu bytes\n\0" as *const u8 as *const libc::c_char,
        saved,
    );
    let ref mut fresh107 = (*m).dedupedThisFar;
    *fresh107 = t1;
}
unsafe extern "C" fn rfc2231(mut in_0: *const libc::c_char) -> *mut libc::c_char {
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut field: C2RustUnnamed_5 = LANGUAGE;
    if !(strstr(in_0, b"*0*=\0" as *const u8 as *const libc::c_char)).is_null() {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        ret = cli_malloc((strlen(in_0)).wrapping_add(16 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        p = ret;
        if ret.is_null() {
            cli_errmsg(
                b"rfc2331: out of memory, unable to proceed\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as *mut libc::c_char;
        }
        loop {
            match *in_0 as libc::c_int {
                42 => {
                    loop {
                        in_0 = in_0.offset(1);
                        if !(*in_0 as libc::c_int != '*' as i32
                            && *in_0 as libc::c_int != 0)
                        {
                            break;
                        }
                    }
                    if !(*in_0 != 0) {
                        break;
                    }
                    in_0 = in_0.offset(1);
                }
                61 => {
                    strcpy(p, b"=rfc2231failure\0" as *const u8 as *const libc::c_char);
                    p = p
                        .offset(
                            strlen(
                                b"=rfc2231failure\0" as *const u8 as *const libc::c_char,
                            ) as isize,
                        );
                    break;
                }
                _ => {
                    let fresh108 = in_0;
                    in_0 = in_0.offset(1);
                    let fresh109 = p;
                    p = p.offset(1);
                    *fresh109 = *fresh108;
                }
            }
            if !(*in_0 != 0) {
                break;
            }
        }
        *p = '\0' as i32 as libc::c_char;
        cli_dbgmsg(
            b"RFC2231 parameter continuations are not yet handled, returning \"%s\"\n\0"
                as *const u8 as *const libc::c_char,
            ret,
        );
        return ret;
    }
    ptr = strstr(in_0, b"*0=\0" as *const u8 as *const libc::c_char);
    if !ptr.is_null() {
        field = CONTENTS;
    } else {
        ptr = strstr(in_0, b"*=\0" as *const u8 as *const libc::c_char);
        field = LANGUAGE;
    }
    if ptr.is_null() {
        ret = cli_strdup(in_0);
        out = ret;
        while *out != 0 {
            let fresh110 = out;
            out = out.offset(1);
            *fresh110 = (*fresh110 as libc::c_int & 0x7f as libc::c_int) as libc::c_char;
        }
        return ret;
    }
    cli_dbgmsg(b"rfc2231 '%s'\n\0" as *const u8 as *const libc::c_char, in_0);
    ret = cli_malloc((strlen(in_0)).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    if ret.is_null() {
        cli_errmsg(
            b"rfc2331: out of memory for ret\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut libc::c_char;
    }
    out = ret;
    while in_0 != ptr {
        let fresh111 = in_0;
        in_0 = in_0.offset(1);
        let fresh112 = out;
        out = out.offset(1);
        *fresh112 = *fresh111;
    }
    let fresh113 = out;
    out = out.offset(1);
    *fresh113 = '=' as i32 as libc::c_char;
    loop {
        let fresh114 = ptr;
        ptr = ptr.offset(1);
        if !(*fresh114 as libc::c_int != '=' as i32) {
            break;
        }
    }
    while *ptr != 0 {
        match field as libc::c_uint {
            0 => {
                if *ptr as libc::c_int == '\'' as i32 {
                    field = CHARSET;
                }
            }
            1 => {
                if *ptr as libc::c_int == '\'' as i32 {
                    field = CONTENTS;
                }
            }
            2 => {
                if *ptr as libc::c_int == '%' as i32 {
                    let mut byte: libc::c_uchar = 0;
                    ptr = ptr.offset(1);
                    if !(*ptr as libc::c_int == '\0' as i32
                        || *ptr as libc::c_int == '\n' as i32)
                    {
                        byte = hex(*ptr);
                        ptr = ptr.offset(1);
                        if *ptr as libc::c_int == '\0' as i32
                            || *ptr as libc::c_int == '\n' as i32
                        {
                            let fresh115 = out;
                            out = out.offset(1);
                            *fresh115 = byte as libc::c_char;
                        } else {
                            byte = ((byte as libc::c_int) << 4 as libc::c_int)
                                as libc::c_uchar;
                            byte = (byte as libc::c_int + hex(*ptr) as libc::c_int)
                                as libc::c_uchar;
                            let fresh116 = out;
                            out = out.offset(1);
                            *fresh116 = byte as libc::c_char;
                        }
                    }
                } else {
                    let fresh117 = out;
                    out = out.offset(1);
                    *fresh117 = *ptr;
                }
            }
            _ => {}
        }
        let fresh118 = ptr;
        ptr = ptr.offset(1);
        if *fresh118 as libc::c_int == '\0' as i32 {
            break;
        }
    }
    if field as libc::c_uint != CONTENTS as libc::c_int as libc::c_uint {
        free(ret as *mut libc::c_void);
        cli_dbgmsg(
            b"Invalid RFC2231 header: '%s'\n\0" as *const u8 as *const libc::c_char,
            in_0,
        );
        return cli_strdup(b"\0" as *const u8 as *const libc::c_char);
    }
    *out = '\0' as i32 as libc::c_char;
    cli_dbgmsg(b"rfc2231 returns '%s'\n\0" as *const u8 as *const libc::c_char, ret);
    return ret;
}
unsafe extern "C" fn simil(
    mut str1: *const libc::c_char,
    mut str2: *const libc::c_char,
) -> libc::c_int {
    let mut top: LINK1 = 0 as LINK1;
    let mut score: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut common: size_t = 0;
    let mut total: size_t = 0;
    let mut len1: size_t = 0;
    let mut len2: size_t = 0;
    let mut rs1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rs2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ls1: [libc::c_char; 50] = [0; 50];
    let mut ls2: [libc::c_char; 50] = [0; 50];
    if strcasecmp(str1, str2) == 0 as libc::c_int {
        return 100 as libc::c_int;
    }
    s1 = cli_strdup(str1);
    if s1.is_null() {
        return -(2 as libc::c_int);
    }
    s2 = cli_strdup(str2);
    if s2.is_null() {
        free(s1 as *mut libc::c_void);
        return -(2 as libc::c_int);
    }
    total = strstrip(s1);
    if total > (50 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
        || {
            len2 = strstrip(s2);
            len2 > (50 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
        }
    {
        free(s1 as *mut libc::c_void);
        free(s2 as *mut libc::c_void);
        return -(5 as libc::c_int);
    }
    total = (total as libc::c_ulong).wrapping_add(len2) as size_t as size_t;
    if push(&mut top, s1) == -(2 as libc::c_int)
        || push(&mut top, s2) == -(2 as libc::c_int)
    {
        free(s1 as *mut libc::c_void);
        free(s2 as *mut libc::c_void);
        return -(2 as libc::c_int);
    }
    while pop(&mut top, ls2.as_mut_ptr()) == -(4 as libc::c_int) {
        pop(&mut top, ls1.as_mut_ptr());
        common = compare(ls1.as_mut_ptr(), &mut rs1, ls2.as_mut_ptr(), &mut rs2)
            as size_t;
        if common > 0 as libc::c_int as libc::c_ulong {
            score = score.wrapping_add(common as libc::c_uint);
            len1 = strlen(ls1.as_mut_ptr());
            len2 = strlen(ls2.as_mut_ptr());
            if len1 > 1 as libc::c_int as libc::c_ulong
                && len2 >= 1 as libc::c_int as libc::c_ulong
                || len2 > 1 as libc::c_int as libc::c_ulong
                    && len1 >= 1 as libc::c_int as libc::c_ulong
            {
                if push(&mut top, ls1.as_mut_ptr()) == -(2 as libc::c_int)
                    || push(&mut top, ls2.as_mut_ptr()) == -(2 as libc::c_int)
                {
                    free(s1 as *mut libc::c_void);
                    free(s2 as *mut libc::c_void);
                    return -(2 as libc::c_int);
                }
            }
            len1 = strlen(rs1);
            len2 = strlen(rs2);
            if len1 > 1 as libc::c_int as libc::c_ulong
                && len2 >= 1 as libc::c_int as libc::c_ulong
                || len2 > 1 as libc::c_int as libc::c_ulong
                    && len1 >= 1 as libc::c_int as libc::c_ulong
            {
                if push(&mut top, rs1) == -(2 as libc::c_int)
                    || push(&mut top, rs2) == -(2 as libc::c_int)
                {
                    free(s1 as *mut libc::c_void);
                    free(s2 as *mut libc::c_void);
                    return -(2 as libc::c_int);
                }
            }
        }
    }
    free(s1 as *mut libc::c_void);
    free(s2 as *mut libc::c_void);
    return (if total > 0 as libc::c_int as libc::c_ulong {
        (score.wrapping_mul(200 as libc::c_int as libc::c_uint) as libc::c_ulong)
            .wrapping_div(total)
    } else {
        0 as libc::c_int as libc::c_ulong
    }) as libc::c_int;
}
unsafe extern "C" fn compare(
    mut ls1: *mut libc::c_char,
    mut rs1: *mut *mut libc::c_char,
    mut ls2: *mut libc::c_char,
    mut rs2: *mut *mut libc::c_char,
) -> libc::c_uint {
    let mut common: libc::c_uint = 0;
    let mut maxchars: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut some_similarity: bool = 0 as libc::c_int != 0;
    let mut s1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut maxs1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut maxs2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut maxe1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut maxe2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cs1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cs2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end2: *mut libc::c_char = 0 as *mut libc::c_char;
    end1 = ls1.offset(strlen(ls1) as isize);
    end2 = ls2.offset(strlen(ls2) as isize);
    start1 = ls1;
    loop {
        s1 = start1;
        s2 = ls2;
        if !(s1 < end1) {
            break;
        }
        while s1 < end1 && s2 < end2 {
            if ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *s1 as libc::c_int;
                        __res = (if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = tolower(*s1 as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(*s1 as libc::c_int as isize);
                }
                __res
            })
                == ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *s2 as libc::c_int;
                            __res = (if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            });
                        } else {
                            __res = tolower(*s2 as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(*s2 as libc::c_int as isize);
                    }
                    __res
                })
            {
                some_similarity = 1 as libc::c_int != 0;
                cs1 = s1;
                cs2 = s2;
                common = 0 as libc::c_int as libc::c_uint;
                while !(s1 == end1 || s2 == end2) {
                    s1 = s1.offset(1);
                    s2 = s2.offset(1);
                    common = common.wrapping_add(1);
                    if !(({
                        let mut __res: libc::c_int = 0;
                        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int = *s1 as libc::c_int;
                                __res = (if __c < -(128 as libc::c_int)
                                    || __c > 255 as libc::c_int
                                {
                                    __c
                                } else {
                                    *(*__ctype_tolower_loc()).offset(__c as isize)
                                });
                            } else {
                                __res = tolower(*s1 as libc::c_int);
                            }
                        } else {
                            __res = *(*__ctype_tolower_loc())
                                .offset(*s1 as libc::c_int as isize);
                        }
                        __res
                    })
                        == ({
                            let mut __res: libc::c_int = 0;
                            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                > 1 as libc::c_int as libc::c_ulong
                            {
                                if 0 != 0 {
                                    let mut __c: libc::c_int = *s2 as libc::c_int;
                                    __res = (if __c < -(128 as libc::c_int)
                                        || __c > 255 as libc::c_int
                                    {
                                        __c
                                    } else {
                                        *(*__ctype_tolower_loc()).offset(__c as isize)
                                    });
                                } else {
                                    __res = tolower(*s2 as libc::c_int);
                                }
                            } else {
                                __res = *(*__ctype_tolower_loc())
                                    .offset(*s2 as libc::c_int as isize);
                            }
                            __res
                        }))
                    {
                        break;
                    }
                }
                if common > maxchars {
                    let mut diff: libc::c_uint = common.wrapping_sub(maxchars);
                    maxchars = common;
                    maxs1 = cs1;
                    maxs2 = cs2;
                    maxe1 = s1;
                    maxe2 = s2;
                    end1 = end1.offset(-(diff as isize));
                    end2 = end2.offset(-(diff as isize));
                } else {
                    s1 = s1.offset(-(common as isize));
                }
            } else {
                s2 = s2.offset(1);
            }
        }
        start1 = start1.offset(1);
    }
    if some_similarity {
        *maxs1 = '\0' as i32 as libc::c_char;
        *maxs2 = '\0' as i32 as libc::c_char;
        *rs1 = maxe1;
        *rs2 = maxe2;
    }
    return maxchars;
}
unsafe extern "C" fn push(
    mut top: *mut LINK1,
    mut string: *const libc::c_char,
) -> libc::c_int {
    let mut element: LINK1 = 0 as *mut ELEMENT1;
    element = cli_malloc(::std::mem::size_of::<ELEMENT1>() as libc::c_ulong) as LINK1;
    if element.is_null() {
        return -(2 as libc::c_int);
    }
    let ref mut fresh119 = (*element).d1;
    *fresh119 = cli_strdup(string);
    if (*fresh119).is_null() {
        free(element as *mut libc::c_void);
        return -(2 as libc::c_int);
    }
    let ref mut fresh120 = (*element).next;
    *fresh120 = *top;
    *top = element;
    return -(4 as libc::c_int);
}
unsafe extern "C" fn pop(
    mut top: *mut LINK1,
    mut buffer: *mut libc::c_char,
) -> libc::c_int {
    let mut t1: LINK1 = 0 as *mut ELEMENT1;
    t1 = *top;
    if !t1.is_null() {
        strcpy(buffer, (*t1).d1);
        *top = (*t1).next;
        free((*t1).d1 as *mut libc::c_void);
        free(t1 as *mut libc::c_char as *mut libc::c_void);
        return -(4 as libc::c_int);
    }
    return -(3 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn isuuencodebegin(mut line: *const libc::c_char) -> libc::c_int {
    if *line.offset(0 as libc::c_int as isize) as libc::c_int != 'b' as i32 {
        return 0 as libc::c_int;
    }
    if strlen(line) < 10 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    return (strncasecmp(
        line,
        b"begin \0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        && *(*__ctype_b_loc())
            .offset(*line.offset(6 as libc::c_int as isize) as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        && *(*__ctype_b_loc())
            .offset(*line.offset(7 as libc::c_int as isize) as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        && *(*__ctype_b_loc())
            .offset(*line.offset(8 as libc::c_int as isize) as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0 && *line.offset(9 as libc::c_int as isize) as libc::c_int == ' ' as i32)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn messageGetJObj(mut m: *mut message) -> *mut json_object {
    if m.is_null() {
        return 0 as *mut json_object;
    }
    if ((*m).jobj).is_null() {
        let ref mut fresh121 = (*m).jobj;
        *fresh121 = cli_jsonobj(0 as *mut json_object, 0 as *const libc::c_char);
    }
    return (*m).jobj;
}
