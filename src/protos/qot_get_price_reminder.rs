/// 提醒信息列表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceReminderItem {
    /// 每个提醒的唯一标识
    #[prost(int64, required, tag="1")]
    pub key: i64,
    /// Qot_Common::PriceReminderType 提醒类型
    #[prost(int32, required, tag="2")]
    pub r#type: i32,
    /// 提醒参数值
    #[prost(double, required, tag="3")]
    pub value: f64,
    /// 备注
    #[prost(string, required, tag="4")]
    pub note: ::prost::alloc::string::String,
    /// Qot_Common::PriceReminderFreq 提醒频率类型
    #[prost(int32, required, tag="5")]
    pub freq: i32,
    /// 该提醒设置是否生效。false不生效，true生效
    #[prost(bool, required, tag="6")]
    pub is_enable: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceReminder {
    /// 股票
    #[prost(message, required, tag="1")]
    pub security: super::qot_common::Security,
    /// 提醒信息列表
    #[prost(message, repeated, tag="2")]
    pub item_list: ::prost::alloc::vec::Vec<PriceReminderItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    /// 查询股票下的到价提醒项，security和market二选一，都存在的情况下security优先。
    #[prost(message, optional, tag="1")]
    pub security: ::core::option::Option<super::qot_common::Security>,
    ///Qot_Common::QotMarket 市场，查询市场下的到价提醒项，不区分沪深
    #[prost(int32, optional, tag="2")]
    pub market: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///到价提醒
    #[prost(message, repeated, tag="1")]
    pub price_reminder_list: ::prost::alloc::vec::Vec<PriceReminder>,
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
