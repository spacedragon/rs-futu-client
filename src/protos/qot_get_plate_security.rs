#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///板块
    #[prost(message, required, tag="1")]
    pub plate: super::qot_common::Security,
    ///Qot_Common.SortField,根据哪个字段排序,不填默认Code排序
    #[prost(int32, optional, tag="2")]
    pub sort_field: ::core::option::Option<i32>,
    ///升序ture, 降序false, 不填默认升序
    #[prost(bool, optional, tag="3")]
    pub ascend: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///板块下的股票静态信息
    #[prost(message, repeated, tag="1")]
    pub static_info_list: ::prost::alloc::vec::Vec<super::qot_common::SecurityStaticInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(message, required, tag="1")]
    pub c2s: C2s,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    ///RetType,返回结果
    #[prost(int32, required, tag="1", default="-400")]
    pub ret_type: i32,
    #[prost(string, optional, tag="2")]
    pub ret_msg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="3")]
    pub err_code: ::core::option::Option<i32>,
    #[prost(message, optional, tag="4")]
    pub s2c: ::core::option::Option<S2c>,
}
