#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///股票
    #[prost(message, required, tag="1")]
    pub security: super::qot_common::Security,
    ///最多返回的逐笔个数,实际返回数量不一定会返回这么多,最多返回1000个
    #[prost(int32, required, tag="2")]
    pub max_ret_num: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///股票
    #[prost(message, required, tag="1")]
    pub security: super::qot_common::Security,
    ///逐笔
    #[prost(message, repeated, tag="2")]
    pub ticker_list: ::prost::alloc::vec::Vec<super::qot_common::Ticker>,
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
