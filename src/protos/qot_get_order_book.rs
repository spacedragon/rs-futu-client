#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///股票
    #[prost(message, required, tag="1")]
    pub security: super::qot_common::Security,
    ///请求的摆盘个数
    #[prost(int32, required, tag="2")]
    pub num: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///股票
    #[prost(message, required, tag="1")]
    pub security: super::qot_common::Security,
    ///卖盘
    #[prost(message, repeated, tag="2")]
    pub order_book_ask_list: ::prost::alloc::vec::Vec<super::qot_common::OrderBook>,
    ///买盘
    #[prost(message, repeated, tag="3")]
    pub order_book_bid_list: ::prost::alloc::vec::Vec<super::qot_common::OrderBook>,
    /// 富途服务器从交易所收到数据的时间(for bid)部分数据的接收时间为零，例如服务器重启或第一次推送的缓存数据。该字段暂时只支持港股。
    #[prost(string, optional, tag="4")]
    pub svr_recv_time_bid: ::core::option::Option<::prost::alloc::string::String>,
    /// 富途服务器从交易所收到数据的时间戳(for bid)
    #[prost(double, optional, tag="5")]
    pub svr_recv_time_bid_timestamp: ::core::option::Option<f64>,
    /// 富途服务器从交易所收到数据的时间(for ask)
    #[prost(string, optional, tag="6")]
    pub svr_recv_time_ask: ::core::option::Option<::prost::alloc::string::String>,
    /// 富途服务器从交易所收到数据的时间戳(for ask)
    #[prost(double, optional, tag="7")]
    pub svr_recv_time_ask_timestamp: ::core::option::Option<f64>,
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
