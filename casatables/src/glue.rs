#![allow(non_camel_case_types, non_snake_case, unused)]
/* automatically generated by rust-bindgen 0.69.4 */

#[repr(u32)]
#[doc = "Different data types supported by the CASA tables format."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GlueDataType {
    #[doc = "A boolean value."]
    TpBool = 0,
    #[doc = "A signed 8-bit integer value."]
    TpChar = 1,
    #[doc = "An unsigned 8-bit integer value."]
    TpUChar = 2,
    #[doc = "A signed 16-bit integer value."]
    TpShort = 3,
    #[doc = "An unsigned 16-bit integer value."]
    TpUShort = 4,
    #[doc = "A signed 32-bit integer value."]
    TpInt = 5,
    #[doc = "An unsigned 32-bit integer value."]
    TpUInt = 6,
    #[doc = "A 32-bit IEEE754 floating-point value."]
    TpFloat = 7,
    #[doc = "A 64-bit IEEE754 double-precision floating-point value."]
    TpDouble = 8,
    #[doc = "A complex number composed of two single-precision floating-point values."]
    TpComplex = 9,
    #[doc = "A complex number composed of two double-precision floating-point values."]
    TpDComplex = 10,
    #[doc = "A string value. **Todo:** encoding???"]
    TpString = 11,
    #[doc = "A value that is its own CASA table."]
    TpTable = 12,
    #[doc = "A value that is an array of booleans."]
    TpArrayBool = 13,
    #[doc = "A value that is an array of signed 8-bit integers."]
    TpArrayChar = 14,
    #[doc = "A value that is an array of unsigned 8-bit integers."]
    TpArrayUChar = 15,
    #[doc = "A value that is an array of signed 16-bit integers."]
    TpArrayShort = 16,
    #[doc = "A value that is an array of unsigned 16-bit integers."]
    TpArrayUShort = 17,
    #[doc = "A value that is an array of signed 32-bit integers."]
    TpArrayInt = 18,
    #[doc = "A value that is an array of unsigned 32-bit integers."]
    TpArrayUInt = 19,
    #[doc = "A value that is an array of 32-bit single-precision floating-point numbers."]
    TpArrayFloat = 20,
    #[doc = "A value that is an array of 64-bit double-precision floating-point numbers."]
    TpArrayDouble = 21,
    #[doc = "A value that is an array of complex numbers with single-precision components."]
    TpArrayComplex = 22,
    #[doc = "A value that is an array of complex numbers with double-precision components."]
    TpArrayDComplex = 23,
    #[doc = "A value that is an array of strings. **Todo:** encoding???"]
    TpArrayString = 24,
    #[doc = "A value that is a dictionary of name-value pairs."]
    TpRecord = 25,
    #[doc = "A value of some other type."]
    TpOther = 26,
    #[doc = "A value that is a physical quantity with associated dimensions."]
    TpQuantity = 27,
    #[doc = "A value that is an array of physical quantities with associated dimensions."]
    TpArrayQuantity = 28,
    #[doc = "A signed 64-bit integer value."]
    TpInt64 = 29,
    #[doc = "A value that is an array of unsigned 8-bit integers."]
    TpArrayInt64 = 30,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GlueTable {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GlueTableRow {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GlueTableDesc {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GlueTableRecord {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StringBridge {
    pub data: *const ::std::os::raw::c_void,
    pub n_bytes: ::std::os::raw::c_ulong,
}
#[test]
fn bindgen_test_layout_StringBridge() {
    const UNINIT: ::std::mem::MaybeUninit<StringBridge> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
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
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(StringBridge),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).n_bytes) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(StringBridge),
            "::",
            stringify!(n_bytes)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ExcInfo {
    pub message: [::std::os::raw::c_char; 512usize],
}
#[test]
fn bindgen_test_layout_ExcInfo() {
    const UNINIT: ::std::mem::MaybeUninit<ExcInfo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
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
        unsafe { ::std::ptr::addr_of!((*ptr).message) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ExcInfo),
            "::",
            stringify!(message)
        )
    );
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
pub type KeywordReprCallback = ::std::option::Option<
    unsafe extern "C" fn(
        name: *const StringBridge,
        dtype: GlueDataType,
        repr: *const StringBridge,
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
#[doc = "Different modes for creating a CASA table description."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TableDescCreateMode {
    #[doc = " Create a new table description file."]
    TDM_NEW = 0,
    #[doc = " Create a new file, raising an error if it already exists."]
    TDM_NEW_NO_REPLACE = 1,
    #[doc = " Create a table description without an associated file on disk."]
    TDM_SCRATCH = 2,
}
#[doc = "Different modes for creating a CASA table description."]
pub use self::TableDescCreateMode as TableDescOption;
extern "C" {
    pub fn data_type_get_element_size(ty: GlueDataType) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tablerec_create(exc: *mut ExcInfo) -> *mut GlueTableRecord;
}
extern "C" {
    pub fn tablerec_copy(other: *const GlueTableRecord, exc: *mut ExcInfo) -> *mut GlueTableRecord;
}
extern "C" {
    pub fn tablerec_eq(rec: *const GlueTableRecord, other: *const GlueTableRecord) -> bool;
}
extern "C" {
    pub fn tablerec_get_keyword_info(
        rec: *const GlueTableRecord,
        callback: KeywordInfoCallback,
        ctxt: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tablerec_get_keyword_repr(
        rec: *const GlueTableRecord,
        callback: KeywordReprCallback,
        ctxt: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tablerec_get_field_info(
        rec: *const GlueTableRecord,
        col_name: *const StringBridge,
        data_type: *mut GlueDataType,
        n_dim: *mut ::std::os::raw::c_int,
        dims: *mut ::std::os::raw::c_ulong,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tablerec_get_field(
        rec: *const GlueTableRecord,
        field_name: *const StringBridge,
        data: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tablerec_get_field_string(
        rec: *const GlueTableRecord,
        col_name: *const StringBridge,
        callback: StringBridgeCallback,
        ctxt: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tablerec_get_field_string_array(
        rec: *const GlueTableRecord,
        col_name: *const StringBridge,
        callback: StringBridgeCallback,
        ctxt: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tablerec_get_field_subrecord(
        rec: *const GlueTableRecord,
        col_name: *const StringBridge,
        sub_rec: *mut GlueTableRecord,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tablerec_put_field(
        rec: *mut GlueTableRecord,
        field_name: *const StringBridge,
        data_type: GlueDataType,
        n_dims: ::std::os::raw::c_ulong,
        dims: *const ::std::os::raw::c_ulong,
        data: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tablerec_free(rec: *mut GlueTableRecord, exc: *mut ExcInfo) -> ::std::os::raw::c_int;
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
    ) -> ::std::os::raw::c_int;
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
    ) -> ::std::os::raw::c_int;
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
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tabledesc_set_ndims(
        table_desc: *mut GlueTableDesc,
        col_name: *const StringBridge,
        n_dims: ::std::os::raw::c_ulong,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tabledesc_get_keywords(
        table_desc: *mut GlueTableDesc,
        exc: *mut ExcInfo,
    ) -> *const GlueTableRecord;
}
extern "C" {
    pub fn tabledesc_get_column_keywords(
        table_desc: *mut GlueTableDesc,
        col_name: *const StringBridge,
        exc: *mut ExcInfo,
    ) -> *const GlueTableRecord;
}
extern "C" {
    pub fn tabledesc_put_keyword(
        table_desc: *mut GlueTableDesc,
        kw_name: *const StringBridge,
        data_type: GlueDataType,
        n_dims: ::std::os::raw::c_ulong,
        dims: *const ::std::os::raw::c_ulong,
        data: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tabledesc_put_column_keyword(
        table_desc: *mut GlueTableDesc,
        col_name: *const StringBridge,
        kw_name: *const StringBridge,
        data_type: GlueDataType,
        n_dims: ::std::os::raw::c_ulong,
        dims: *const ::std::os::raw::c_ulong,
        data: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
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
    pub fn table_get_file_name(
        table: *const GlueTable,
        callback: StringBridgeCallback,
        ctxt: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
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
    pub fn table_get_column_keyword_info(
        table: *const GlueTable,
        col_name: *const StringBridge,
        callback: KeywordInfoCallback,
        ctxt: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_get_keywords(table: *mut GlueTable, exc: *mut ExcInfo) -> *const GlueTableRecord;
}
extern "C" {
    pub fn table_get_column_keywords(
        table: *mut GlueTable,
        col_name: *const StringBridge,
        exc: *mut ExcInfo,
    ) -> *const GlueTableRecord;
}
extern "C" {
    pub fn table_put_keyword(
        table: *mut GlueTable,
        kw_name: *const StringBridge,
        data_type: GlueDataType,
        n_dims: ::std::os::raw::c_ulong,
        dims: *const ::std::os::raw::c_ulong,
        data: *mut ::std::os::raw::c_void,
        exc: *mut ExcInfo,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn table_put_column_keyword(
        table: *mut GlueTable,
        col_name: *const StringBridge,
        kw_name: *const StringBridge,
        data_type: GlueDataType,
        n_dims: ::std::os::raw::c_ulong,
        dims: *const ::std::os::raw::c_ulong,
        data: *mut ::std::os::raw::c_void,
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
