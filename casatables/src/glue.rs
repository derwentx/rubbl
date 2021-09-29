#![allow(non_camel_case_types, non_snake_case, unused)]
/* automatically generated by rust-bindgen 0.59.1 */

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GlueDataType {
    TpBool = 0,
    TpChar = 1,
    TpUChar = 2,
    TpShort = 3,
    TpUShort = 4,
    TpInt = 5,
    TpUInt = 6,
    TpFloat = 7,
    TpDouble = 8,
    TpComplex = 9,
    TpDComplex = 10,
    TpString = 11,
    TpTable = 12,
    TpArrayBool = 13,
    TpArrayChar = 14,
    TpArrayUChar = 15,
    TpArrayShort = 16,
    TpArrayUShort = 17,
    TpArrayInt = 18,
    TpArrayUInt = 19,
    TpArrayFloat = 20,
    TpArrayDouble = 21,
    TpArrayComplex = 22,
    TpArrayDComplex = 23,
    TpArrayString = 24,
    TpRecord = 25,
    TpOther = 26,
    TpQuantity = 27,
    TpArrayQuantity = 28,
    TpInt64 = 29,
    TpArrayInt64 = 30,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct GlueTable {
    _unused: [u8; 0],
}
impl Clone for GlueTable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct GlueTableRow {
    _unused: [u8; 0],
}
impl Clone for GlueTableRow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct GlueTableDesc {
    _unused: [u8; 0],
}
impl Clone for GlueTableDesc {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct StringBridge {
    pub data: *const ::std::os::raw::c_void,
    pub n_bytes: ::std::os::raw::c_ulong,
}
#[test]
fn bindgen_test_layout_StringBridge() {
    assert_eq!(
        ::std::mem::size_of::<StringBridge>(),
        16usize,
        concat!("Size of: ", stringify!(StringBridge))
    );
    assert_eq!(
        ::std::mem::align_of::<StringBridge>(),
        8usize,
        concat!("Alignment of ", stringify!(StringBridge))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<StringBridge>())).data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(StringBridge),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<StringBridge>())).n_bytes as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(StringBridge),
            "::",
            stringify!(n_bytes)
        )
    );
}
impl Clone for StringBridge {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct ExcInfo {
    pub message: [::std::os::raw::c_char; 512usize],
}
#[test]
fn bindgen_test_layout_ExcInfo() {
    assert_eq!(
        ::std::mem::size_of::<ExcInfo>(),
        512usize,
        concat!("Size of: ", stringify!(ExcInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<ExcInfo>(),
        1usize,
        concat!("Alignment of ", stringify!(ExcInfo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ExcInfo>())).message as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ExcInfo),
            "::",
            stringify!(message)
        )
    );
}
impl Clone for ExcInfo {
    fn clone(&self) -> Self {
        *self
    }
}
pub type StringBridgeCallback = ::std::option::Option<
    unsafe extern "C" fn(name: *const StringBridge, ctxt: *mut ::std::os::raw::c_void),
>;
pub type KeywordInfoCallback = ::std::option::Option<
    unsafe extern "C" fn(
        name: *const StringBridge,
        dtype: GlueDataType,
        ctxt: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TableOpenMode {
    TOM_OPEN_READONLY = 1,
    TOM_OPEN_RW = 2,
    TOM_CREATE = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TableCreateMode {
    TCM_NEW = 1,
    TCM_NEW_NO_REPLACE = 2,
    TCM_SCRATCH = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TableDescCreateMode {
    TDM_NEW = 0,
    TDM_NEW_NO_REPLACE = 1,
    TDM_SCRATCH = 2,
}
pub use self::TableDescCreateMode as TableDescOption;
extern "C" {
    pub fn data_type_get_element_size(ty: GlueDataType) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tabledesc_create(
        type_: *const StringBridge,
        mode: TableDescCreateMode,
        exc: *mut ExcInfo,
    ) -> *mut GlueTableDesc;
}
extern "C" {
    pub fn tabledesc_add_scalar_column(
        table_desc: *mut GlueTableDesc,
        data_type: GlueDataType,
        col_name: *const StringBridge,
        comment: *const StringBridge,
        direct: bool,
        undefined: bool,
        exc: *mut ExcInfo,
    ) -> *mut GlueTableDesc;
}
extern "C" {
    pub fn tabledesc_add_array_column(
        table_desc: *mut GlueTableDesc,
        data_type: GlueDataType,
        col_name: *const StringBridge,
        comment: *const StringBridge,
        direct: bool,
        undefined: bool,
        exc: *mut ExcInfo,
    ) -> *mut GlueTableDesc;
}
extern "C" {
    pub fn tabledesc_add_fixed_array_column(
        table_desc: *mut GlueTableDesc,
        data_type: GlueDataType,
        col_name: *const StringBridge,
        comment: *const StringBridge,
        n_dims: ::std::os::raw::c_ulong,
        dims: *const ::std::os::raw::c_ulong,
        direct: bool,
        undefined: bool,
        exc: *mut ExcInfo,
    ) -> *mut GlueTableDesc;
}
extern "C" {
    pub fn table_create(
        path: *const StringBridge,
        table_desc: *mut GlueTableDesc,
        n_rows: ::std::os::raw::c_ulong,
        mode: TableCreateMode,
        exc: *mut ExcInfo,
    ) -> *mut GlueTable;
}
extern "C" {
    pub fn table_alloc_and_open(
        path: *const StringBridge,
        mode: TableOpenMode,
        exc: *mut ExcInfo,
    ) -> *mut GlueTable;
}
extern "C" {
    pub fn table_close_and_free(table: *mut GlueTable, exc: *mut ExcInfo);
}
extern "C" {
    pub fn table_n_rows(table: *const GlueTable) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn table_n_columns(table: *const GlueTable) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn table_get_column_names(
        table: *const GlueTable,
        callback: StringBridgeCallback,
        ctxt: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_n_keywords(table: *const GlueTable) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn table_get_keyword_info(
        table: *const GlueTable,
        callback: KeywordInfoCallback,
        ctxt: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_copy_rows(
        source: *const GlueTable,
        dest: *mut GlueTable,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_deep_copy_no_rows(
        table: *const GlueTable,
        dest_path: *const StringBridge,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_get_column_info(
        table: *const GlueTable,
        col_name: *const StringBridge,
        n_rows: *mut ::std::os::raw::c_ulong,
        data_type: *mut GlueDataType,
        is_scalar: *mut ::std::os::raw::c_int,
        is_fixed_shape: *mut ::std::os::raw::c_int,
        n_dim: *mut ::std::os::raw::c_int,
        dims: *mut ::std::os::raw::c_ulong,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_remove_column(
        table: *mut GlueTable,
        col_name: *const StringBridge,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_add_scalar_column(
        table: *mut GlueTable,
        data_type: GlueDataType,
        col_name: *const StringBridge,
        comment: *const StringBridge,
        direct: bool,
        undefined: bool,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_add_array_column(
        table: *mut GlueTable,
        data_type: GlueDataType,
        col_name: *const StringBridge,
        comment: *const StringBridge,
        direct: bool,
        undefined: bool,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_add_fixed_array_column(
        table: *mut GlueTable,
        data_type: GlueDataType,
        col_name: *const StringBridge,
        comment: *const StringBridge,
        n_dims: ::std::os::raw::c_ulong,
        dims: *const ::std::os::raw::c_ulong,
        direct: bool,
        undefined: bool,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_get_scalar_column_data(
        table: *const GlueTable,
        col_name: *const StringBridge,
        data: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_get_scalar_column_data_string(
        table: *const GlueTable,
        col_name: *const StringBridge,
        callback: StringBridgeCallback,
        ctxt: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_get_cell_info(
        table: *const GlueTable,
        col_name: *const StringBridge,
        row_number: ::std::os::raw::c_ulong,
        data_type: *mut GlueDataType,
        n_dim: *mut ::std::os::raw::c_int,
        dims: *mut ::std::os::raw::c_ulong,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_get_cell(
        table: *const GlueTable,
        col_name: *const StringBridge,
        row_number: ::std::os::raw::c_ulong,
        data: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_get_cell_string(
        table: *const GlueTable,
        col_name: *const StringBridge,
        row_number: ::std::os::raw::c_ulong,
        callback: StringBridgeCallback,
        ctxt: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_get_cell_string_array(
        table: *const GlueTable,
        col_name: *const StringBridge,
        row_number: ::std::os::raw::c_ulong,
        callback: StringBridgeCallback,
        ctxt: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_put_cell(
        table: *mut GlueTable,
        col_name: *const StringBridge,
        row_number: ::std::os::raw::c_ulong,
        data_type: GlueDataType,
        n_dims: ::std::os::raw::c_ulong,
        dims: *const ::std::os::raw::c_ulong,
        data: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_add_rows(
        table: *mut GlueTable,
        n_rows: ::std::os::raw::c_ulong,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_row_alloc(
        table: *const GlueTable,
        is_read_only: ::std::os::raw::c_uchar,
        exc: *mut ExcInfo,
    ) -> *mut GlueTableRow;
}
extern "C" {
    pub fn table_row_free(row: *mut GlueTableRow, exc: *mut ExcInfo) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_row_read(
        row: *mut GlueTableRow,
        row_number: ::std::os::raw::c_ulong,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_row_copy_and_put(
        src_row: *mut GlueTableRow,
        dest_row_number: ::std::os::raw::c_ulong,
        dest_row: *mut GlueTableRow,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_row_get_cell_info(
        row: *const GlueTableRow,
        col_name: *const StringBridge,
        data_type: *mut GlueDataType,
        n_dim: *mut ::std::os::raw::c_int,
        dims: *mut ::std::os::raw::c_ulong,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_row_get_cell(
        row: *const GlueTableRow,
        col_name: *const StringBridge,
        data: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_row_get_cell_string(
        row: *const GlueTableRow,
        col_name: *const StringBridge,
        callback: StringBridgeCallback,
        ctxt: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_row_get_cell_string_array(
        row: *const GlueTableRow,
        col_name: *const StringBridge,
        callback: StringBridgeCallback,
        ctxt: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_row_put_cell(
        row: *mut GlueTableRow,
        col_name: *const StringBridge,
        data_type: GlueDataType,
        n_dims: ::std::os::raw::c_ulong,
        dims: *const ::std::os::raw::c_ulong,
        data: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_row_write(
        row: *mut GlueTableRow,
        dest_row_number: ::std::os::raw::c_ulong,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
