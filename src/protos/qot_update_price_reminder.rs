#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///股票
    #[prost(message, required, tag="1")]
    pub security: super::qot_common::Security,
    ///价格
    #[prost(double, required, tag="2")]
    pub price: f64,
    ///涨跌幅
    #[prost(double, required, tag="3")]
    pub change_rate: f64,
    ///市场状态
    #[prost(int32, required, tag="4")]
    pub market_status: i32,
    ///内容
    #[prost(string, required, tag="5")]
    pub content: ::prost::alloc::string::String,
    ///备注
    #[prost(string, required, tag="6")]
    pub note: ::prost::alloc::string::String,
    /// 到价提醒的标识
    #[prost(int64, optional, tag="7")]
    pub key: ::core::option::Option<i64>,
    /// Qot_Common::PriceReminderType，提醒类型
    #[prost(int32, optional, tag="8")]
    pub r#type: ::core::option::Option<i32>,
    /// 设置的提醒值
    #[prost(double, optional, tag="9")]
    pub set_value: ::core::option::Option<f64>,
    /// 设置的提醒类型触发时当前值
    #[prost(double, optional, tag="10")]
    pub cur_value: ::core::option::Option<f64>,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MarketStatus {
    Unknow = 0,
    /// 盘中
    Open = 1,
    /// 美股盘前
    UsPre = 2,
    /// 美股盘后
    UsAfter = 3,
}
