#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///股票
    #[prost(message, required, tag="1")]
    pub security: super::qot_common::Security,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///股票
    #[prost(message, required, tag="1")]
    pub security: super::qot_common::Security,
    ///卖盘
    #[prost(message, required, tag="2")]
    pub order_detail_ask: super::qot_common::OrderDetail,
    ///买盘
    #[prost(message, required, tag="3")]
    pub order_detail_bid: super::qot_common::OrderDetail,
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
