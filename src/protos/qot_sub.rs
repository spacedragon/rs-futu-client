#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///股票
    #[prost(message, repeated, tag="1")]
    pub security_list: ::prost::alloc::vec::Vec<super::qot_common::Security>,
    ///Qot_Common.SubType,订阅数据类型
    #[prost(int32, repeated, packed="false", tag="2")]
    pub sub_type_list: ::prost::alloc::vec::Vec<i32>,
    ///ture表示订阅,false表示反订阅
    #[prost(bool, required, tag="3")]
    pub is_sub_or_un_sub: bool,
    ///是否注册或反注册该连接上面行情的推送,该参数不指定不做注册反注册操作
    #[prost(bool, optional, tag="4")]
    pub is_reg_or_un_reg_push: ::core::option::Option<bool>,
    ///Qot_Common.RehabType,复权类型,注册推送并且是K线类型才生效,其他订阅类型忽略该参数,注册K线推送时该参数不指定默认前复权
    #[prost(int32, repeated, packed="false", tag="5")]
    pub reg_push_rehab_type_list: ::prost::alloc::vec::Vec<i32>,
    ///注册后如果本地已有数据是否首推一次已存在数据,该参数不指定则默认true
    #[prost(bool, optional, tag="6")]
    pub is_first_push: ::core::option::Option<bool>,
    ///当被设置为True时忽略其他参数，取消当前连接的所有订阅，并且反注册推送。
    #[prost(bool, optional, tag="7")]
    pub is_unsub_all: ::core::option::Option<bool>,
    ///订阅摆盘可用,是否订阅摆盘明细,仅支持SF行情,该参数不指定则默认false
    #[prost(bool, optional, tag="8")]
    pub is_sub_order_book_detail: ::core::option::Option<bool>,
    /// 是否允许美股盘前盘后数据（仅用于订阅美股的实时K线、实时分时、实时逐笔）
    #[prost(bool, optional, tag="9")]
    pub extended_time: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
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
