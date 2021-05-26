#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///股票
    #[prost(message, required, tag="1")]
    pub security: super::qot_common::Security,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapitalFlowItem {
    ///净流入的资金额度
    #[prost(double, required, tag="1")]
    pub in_flow: f64,
    ///开始时间字符串,以分钟为单位
    #[prost(string, optional, tag="2")]
    pub time: ::core::option::Option<::prost::alloc::string::String>,
    ///开始时间戳
    #[prost(double, optional, tag="3")]
    pub timestamp: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///资金流向
    #[prost(message, repeated, tag="1")]
    pub flow_item_list: ::prost::alloc::vec::Vec<CapitalFlowItem>,
    ///数据最后有效时间字符串
    #[prost(string, optional, tag="2")]
    pub last_valid_time: ::core::option::Option<::prost::alloc::string::String>,
    ///数据最后有效时间戳
    #[prost(double, optional, tag="3")]
    pub last_valid_timestamp: ::core::option::Option<f64>,
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
