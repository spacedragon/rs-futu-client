#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///交易写操作防重放攻击
    #[prost(message, required, tag="1")]
    pub packet_id: super::common::PacketId,
    ///交易公共参数头
    #[prost(message, required, tag="2")]
    pub header: super::trd_common::TrdHeader,
    ///交易方向, 参见Trd_Common.TrdSide的枚举定义
    #[prost(int32, required, tag="3")]
    pub trd_side: i32,
    ///订单类型, 参见Trd_Common.OrderType的枚举定义
    #[prost(int32, required, tag="4")]
    pub order_type: i32,
    ///代码，港股必须是5位数字，A股必须是6位数字，美股没限制
    #[prost(string, required, tag="5")]
    pub code: ::prost::alloc::string::String,
    ///数量，期权单位是"张"（精确到小数点后 0 位，超出部分会被舍弃。期权期货单位是"张"）
    #[prost(double, required, tag="6")]
    pub qty: f64,
    ///价格，（证券账户精确到小数点后 3 位，期货账户精确到小数点后 9 位，超出部分会被舍弃）
    #[prost(double, optional, tag="7")]
    pub price: ::core::option::Option<f64>,
    ///以下2个为调整价格使用，都传才有效，对港、A股有意义，因为港股有价位，A股2位精度，美股可不传
    ///
    ///是否调整价格，如果价格不合法，是否调整到合法价位，true调整，false不调整
    #[prost(bool, optional, tag="8")]
    pub adjust_price: ::core::option::Option<bool>,
    ///调整方向和调整幅度百分比限制，正数代表向上调整，负数代表向下调整，具体值代表调整幅度限制，如：0.015代表向上调整且幅度不超过1.5%；-0.01代表向下调整且幅度不超过1%
    #[prost(double, optional, tag="9")]
    pub adjust_side_and_limit: ::core::option::Option<f64>,
    ///证券所属市场，参见TrdSecMarket的枚举定义
    #[prost(int32, optional, tag="10")]
    pub sec_market: ::core::option::Option<i32>,
    ///用户备注字符串，最多只能传64字节。可用于标识订单唯一信息等，下单填上，订单结构就会带上。
    #[prost(string, optional, tag="11")]
    pub remark: ::core::option::Option<::prost::alloc::string::String>,
    ///订单有效期限，参见TrdCommon_TimeInForce的枚举定义
    #[prost(int32, optional, tag="12")]
    pub time_in_force: ::core::option::Option<i32>,
    ///是否允许盘前盘后成交。仅适用于美股限价单。默认false
    #[prost(bool, optional, tag="13")]
    pub fill_outside_rth: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///交易公共参数头
    #[prost(message, required, tag="1")]
    pub header: super::trd_common::TrdHeader,
    ///订单号
    #[prost(uint64, optional, tag="2")]
    pub order_id: ::core::option::Option<u64>,
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
