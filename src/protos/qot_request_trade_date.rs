#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///Qot_Common.TradeDateMarket,要查询的市场
    #[prost(int32, required, tag="1")]
    pub market: i32,
    ///开始时间字符串
    #[prost(string, required, tag="2")]
    pub begin_time: ::prost::alloc::string::String,
    ///结束时间字符串
    #[prost(string, required, tag="3")]
    pub end_time: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeDate {
    ///时间字符串
    #[prost(string, required, tag="1")]
    pub time: ::prost::alloc::string::String,
    ///时间戳
    #[prost(double, optional, tag="2")]
    pub timestamp: ::core::option::Option<f64>,
    ///Qot_Common.TradeDateType,交易时间类型
    #[prost(int32, optional, tag="3")]
    pub trade_date_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///交易日,注意该交易日是通过自然日去除周末以及节假日得到，不包括临时休市数据。
    #[prost(message, repeated, tag="1")]
    pub trade_date_list: ::prost::alloc::vec::Vec<TradeDate>,
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
