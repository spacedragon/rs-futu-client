#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///股票
    #[prost(message, required, tag="1")]
    pub security: super::qot_common::Security,
    /// ReferenceType, 相关类型
    #[prost(int32, required, tag="2")]
    pub reference_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///相关股票列表
    #[prost(message, repeated, tag="2")]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReferenceType {
    Unknow = 0,
    ///正股相关的窝轮
    Warrant = 1,
    ///期货主连的相关合约
    Future = 2,
}
