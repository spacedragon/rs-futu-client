#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///交易公共参数头
    #[prost(message, required, tag="1")]
    pub header: super::trd_common::TrdHeader,
    ///过滤条件
    #[prost(message, required, tag="2")]
    pub filter_conditions: super::trd_common::TrdFilterConditions,
    ///需要过滤的订单状态列表
    #[prost(int32, repeated, packed="false", tag="3")]
    pub filter_status_list: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///交易公共参数头
    #[prost(message, required, tag="1")]
    pub header: super::trd_common::TrdHeader,
    ///历史订单列表
    #[prost(message, repeated, tag="2")]
    pub order_list: ::prost::alloc::vec::Vec<super::trd_common::Order>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(message, required, tag="1")]
    pub c2s: C2s,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    ///以下3个字段每条协议都有，注释说明在InitConnect.proto中
    #[prost(int32, required, tag="1", default="-400")]
    pub ret_type: i32,
    #[prost(string, optional, tag="2")]
    pub ret_msg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="3")]
    pub err_code: ::core::option::Option<i32>,
    #[prost(message, optional, tag="4")]
    pub s2c: ::core::option::Option<S2c>,
}
