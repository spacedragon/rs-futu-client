///交易时间
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeTime {
    /// 开始时间,以分钟为单位
    #[prost(double, optional, tag="1")]
    pub begin: ::core::option::Option<f64>,
    /// 结束时间,以分钟为单位
    #[prost(double, optional, tag="2")]
    pub end: ::core::option::Option<f64>,
}
///期货合约资料的列表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FutureInfo {
    /// 合约名称
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// 合约代码
    #[prost(message, required, tag="2")]
    pub security: super::qot_common::Security,
    ///最后交易日，只有非主连期货合约才有该字段
    #[prost(string, required, tag="3")]
    pub last_trade_time: ::prost::alloc::string::String,
    ///最后交易日时间戳，只有非主连期货合约才有该字段
    #[prost(double, optional, tag="4")]
    pub last_trade_timestamp: ::core::option::Option<f64>,
    ///标的股 股票期货和股指期货才有该字段
    #[prost(message, optional, tag="5")]
    pub owner: ::core::option::Option<super::qot_common::Security>,
    ///标的 
    #[prost(string, required, tag="6")]
    pub owner_other: ::prost::alloc::string::String,
    ///交易所
    #[prost(string, required, tag="7")]
    pub exchange: ::prost::alloc::string::String,
    ///合约类型
    #[prost(string, required, tag="8")]
    pub contract_type: ::prost::alloc::string::String,
    ///合约规模
    #[prost(double, required, tag="9")]
    pub contract_size: f64,
    ///合约规模的单位
    #[prost(string, required, tag="10")]
    pub contract_size_unit: ::prost::alloc::string::String,
    ///报价货币
    #[prost(string, required, tag="11")]
    pub quote_currency: ::prost::alloc::string::String,
    ///最小变动单位
    #[prost(double, required, tag="12")]
    pub min_var: f64,
    ///最小变动单位的单位
    #[prost(string, required, tag="13")]
    pub min_var_unit: ::prost::alloc::string::String,
    ///报价单位
    #[prost(string, optional, tag="14")]
    pub quote_unit: ::core::option::Option<::prost::alloc::string::String>,
    ///交易时间
    #[prost(message, repeated, tag="15")]
    pub trade_time: ::prost::alloc::vec::Vec<TradeTime>,
    ///所在时区
    #[prost(string, required, tag="16")]
    pub time_zone: ::prost::alloc::string::String,
    ///交易所规格
    #[prost(string, required, tag="17")]
    pub exchange_format_url: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///股票列表
    #[prost(message, repeated, tag="1")]
    pub security_list: ::prost::alloc::vec::Vec<super::qot_common::Security>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///期货合约资料的列表
    #[prost(message, repeated, tag="1")]
    pub future_info_list: ::prost::alloc::vec::Vec<FutureInfo>,
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
