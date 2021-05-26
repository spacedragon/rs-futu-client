#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///Qot_Common.RehabType,复权类型
    #[prost(int32, required, tag="1")]
    pub rehab_type: i32,
    ///Qot_Common.KLType,K线类型
    #[prost(int32, required, tag="2")]
    pub kl_type: i32,
    ///NoDataMode,当请求时间点数据为空时，如何返回数据
    #[prost(int32, required, tag="3")]
    pub no_data_mode: i32,
    ///股票市场以及股票代码
    #[prost(message, repeated, tag="4")]
    pub security_list: ::prost::alloc::vec::Vec<super::qot_common::Security>,
    ///时间字符串
    #[prost(string, repeated, tag="5")]
    pub time_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///最多返回多少只股票的数据，如果未指定表示不限制
    #[prost(int32, optional, tag="6")]
    pub max_req_security_num: ::core::option::Option<i32>,
    ///指定返回K线结构体特定某几项数据，KLFields枚举值或组合，如果未指定返回全部字段
    #[prost(int64, optional, tag="7")]
    pub need_kl_fields_flag: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistoryPointsKl {
    ///DataStatus,数据状态
    #[prost(int32, required, tag="1")]
    pub status: i32,
    ///请求的时间
    #[prost(string, required, tag="2")]
    pub req_time: ::prost::alloc::string::String,
    ///K线数据
    #[prost(message, required, tag="3")]
    pub kl: super::qot_common::KLine,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityHistoryKlPoints {
    ///股票	
    #[prost(message, required, tag="1")]
    pub security: super::qot_common::Security,
    ///K线数据
    #[prost(message, repeated, tag="2")]
    pub kl_list: ::prost::alloc::vec::Vec<HistoryPointsKl>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///多只股票的多点历史K线点
    #[prost(message, repeated, tag="1")]
    pub kl_point_list: ::prost::alloc::vec::Vec<SecurityHistoryKlPoints>,
    ///如请求不指定maxReqSecurityNum值，则不会返回该字段，该字段表示请求是否还有超过指定限制的数据
    #[prost(bool, optional, tag="2")]
    pub has_next: ::core::option::Option<bool>,
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
///当请求时间点数据为空时，如何返回数据
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NoDataMode {
    ///直接返回空数据
    Null = 0,
    ///往前取值，返回前一个时间点数据
    Forward = 1,
    ///向后取值，返回后一个时间点数据
    Backward = 2,
}
///这个时间点返回数据的状态以及来源
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataStatus {
    ///空数据
    Null = 0,
    ///当前时间点数据
    Current = 1,
    ///前一个时间点数据
    Previous = 2,
    ///后一个时间点数据
    Back = 3,
}
