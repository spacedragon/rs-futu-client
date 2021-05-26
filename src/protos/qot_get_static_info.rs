#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///Qot_Common.QotMarket,股票市场
    #[prost(int32, optional, tag="1")]
    pub market: ::core::option::Option<i32>,
    ///Qot_Common.SecurityType,股票类型
    #[prost(int32, optional, tag="2")]
    pub sec_type: ::core::option::Option<i32>,
    ///股票，若该字段存在，忽略其他字段，只返回该字段股票的静态信息
    #[prost(message, repeated, tag="3")]
    pub security_list: ::prost::alloc::vec::Vec<super::qot_common::Security>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///静态信息
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
