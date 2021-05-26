#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///交易写操作防重放攻击
    #[prost(message, required, tag="1")]
    pub packet_id: super::common::PacketId,
    ///交易公共参数头
    #[prost(message, required, tag="2")]
    pub header: super::trd_common::TrdHeader,
    ///订单号，forAll为true时，传0
    #[prost(uint64, required, tag="3")]
    pub order_id: u64,
    ///修改操作类型，参见Trd_Common.ModifyOrderOp的枚举定义
    #[prost(int32, required, tag="4")]
    pub modify_order_op: i32,
    ///是否对此业务账户的全部订单操作，true是，false否(对单个订单)，无此字段代表false，仅对单个订单
    #[prost(bool, optional, tag="5")]
    pub for_all: ::core::option::Option<bool>,
    ///下面的字段仅针对单个订单，且modifyOrderOp为ModifyOrderOp_Normal有效
    ///
    ///数量，期权单位是"张"（精确到小数点后 0 位，超出部分会被舍弃）
    #[prost(double, optional, tag="8")]
    pub qty: ::core::option::Option<f64>,
    ///价格，（证券账户精确到小数点后 3 位，期货账户精确到小数点后 9 位，超出部分会被舍弃）
    #[prost(double, optional, tag="9")]
    pub price: ::core::option::Option<f64>,
    ///以下为调整价格使用，都传才有效，对港、A股有意义，因为港股有价位，A股2位精度，美股可不传
    ///
    ///是否调整价格，如果价格不合法，是否调整到合法价位，true调整，false不调整
    #[prost(bool, optional, tag="10")]
    pub adjust_price: ::core::option::Option<bool>,
    ///调整方向和调整幅度百分比限制，正数代表向上调整，负数代表向下调整，具体值代表调整幅度限制，如：0.015代表向上调整且幅度不超过1.5%；-0.01代表向下调整且幅度不超过1%
    #[prost(double, optional, tag="11")]
    pub adjust_side_and_limit: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///交易公共参数头
    #[prost(message, required, tag="1")]
    pub header: super::trd_common::TrdHeader,
    ///订单号
    #[prost(uint64, required, tag="2")]
    pub order_id: u64,
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
