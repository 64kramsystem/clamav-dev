use c2rust_bitfields;
use libc;

use crate::sys::{cl_error_t, image_fuzzy_hash_t};

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
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_0 = 0;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub const CL_EFORMAT: cl_error_t = 26;
pub const CL_SUCCESS: cl_error_t = 0;
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
pub type bytecode_security = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub logic: *mut libc::c_char,
    pub code_start: *mut uint8_t,
}
pub type lsig_type_t = lsig_type;
pub type lsig_type = libc::c_uint;
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
pub const CL_TYPE_MAIL: cli_file = 558;
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

pub type cli_ctx = cli_ctx_tag;

pub type mime_type_C = libc::c_uint;
pub(crate) const MEXTENSION: mime_type_C = 8;
pub(crate) const VIDEO: mime_type_C = 7;
pub(crate) const TEXT: mime_type_C = 6;
pub(crate) const MULTIPART: mime_type_C = 5;
pub(crate) const MESSAGE: mime_type_C = 4;
pub(crate) const IMAGE: mime_type_C = 3;
pub(crate) const AUDIO: mime_type_C = 2;
pub(crate) const APPLICATION: mime_type_C = 1;
pub(crate) const NOMIME: mime_type_C = 0;

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum mime_type_R {
    MEXTENSION = 8,
    VIDEO = 7,
    TEXT = 6,
    MULTIPART = 5,
    MESSAGE = 4,
    IMAGE = 3,
    AUDIO = 2,
    APPLICATION = 1,
    NOMIME = 0,
}

pub type encoding_type_C = libc::c_uint;
pub(crate) const BINHEX: encoding_type_C = 8;
pub(crate) const YENCODE: encoding_type_C = 6;
pub(crate) const UUENCODE: encoding_type_C = 5;
pub(crate) const BINARY: encoding_type_C = 4;
pub(crate) const EIGHTBIT: encoding_type_C = 3;
pub(crate) const BASE64: encoding_type_C = 2;
pub(crate) const QUOTEDPRINTABLE: encoding_type_C = 1;
pub(crate) const NOENCODING: encoding_type_C = 0;

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum encoding_type_R {
    BINHEX = 8,
    YENCODE = 6,
    UUENCODE = 5,
    BINARY = 4,
    EIGHTBIT = 3,
    BASE64 = 2,
    QUOTEDPRINTABLE = 1,
    NOENCODING = 0,
}

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
pub struct message_C;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct message {
    pub encodingTypes: *mut encoding_type_R,
    pub mimeType: mime_type_R,
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
    pub type_0: mime_type_R,
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
    pub type_0: encoding_type_R,
}
