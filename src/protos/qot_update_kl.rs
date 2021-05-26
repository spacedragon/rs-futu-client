#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///Qot_Common.RehabType,复权类型
    #[prost(int32, required, tag="1")]
    pub rehab_type: i32,
    ///Qot_Common.KLType,K线类型
    #[prost(int32, required, tag="2")]
    pub kl_type: i32,
    ///股票
    #[prost(message, required, tag="3")]
    pub security: super::qot_common::Security,
    ///推送的k线点
    #[prost(message, repeated, tag="4")]
    pub kl_list: ::prost::alloc::vec::Vec<super::qot_common::KLine>,
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
