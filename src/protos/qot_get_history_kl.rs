#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///Qot_Common.RehabType,复权类型
    #[prost(int32, required, tag="1")]
    pub rehab_type: i32,
    ///Qot_Common.KLType,K线类型
    #[prost(int32, required, tag="2")]
    pub kl_type: i32,
    ///股票市场以及股票代码
    #[prost(message, required, tag="3")]
    pub security: super::qot_common::Security,
    ///开始时间字符串
    #[prost(string, required, tag="4")]
    pub begin_time: ::prost::alloc::string::String,
    ///结束时间字符串
    #[prost(string, required, tag="5")]
    pub end_time: ::prost::alloc::string::String,
    ///最多返回多少根K线，如果未指定表示不限制
    #[prost(int32, optional, tag="6")]
    pub max_ack_kl_num: ::core::option::Option<i32>,
    ///指定返回K线结构体特定某几项数据，KLFields枚举值或组合，如果未指定返回全部字段
    #[prost(int64, optional, tag="7")]
    pub need_kl_fields_flag: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    #[prost(message, required, tag="1")]
    pub security: super::qot_common::Security,
    ///K线数据
    #[prost(message, repeated, tag="2")]
    pub kl_list: ::prost::alloc::vec::Vec<super::qot_common::KLine>,
    ///如请求不指定maxAckKLNum值，则不会返回该字段，该字段表示超过指定限制的下一K线时间字符串
    #[prost(string, optional, tag="3")]
    pub next_kl_time: ::core::option::Option<::prost::alloc::string::String>,
    ///时间戳，如请求不指定maxAckKLNum值，则不会返回该字段，该字段表示超过指定限制的下一K线时间戳
    #[prost(double, optional, tag="4")]
    pub next_kl_timestamp: ::core::option::Option<f64>,
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
