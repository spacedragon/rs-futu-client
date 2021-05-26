#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeChangeInfo {
    ///CodeChangeType,代码变化或者新增临时代码的事件类型
    #[prost(int32, required, tag="1")]
    pub r#type: i32,
    ///主代码，在创业板转主板中表示主板
    #[prost(message, required, tag="2")]
    pub security: super::qot_common::Security,
    ///关联代码，在创业板转主板中表示创业板，在剩余事件中表示临时代码
    #[prost(message, required, tag="3")]
    pub related_security: super::qot_common::Security,
    ///公布时间
    #[prost(string, optional, tag="4")]
    pub public_time: ::core::option::Option<::prost::alloc::string::String>,
    ///公布时间戳
    #[prost(double, optional, tag="5")]
    pub public_timestamp: ::core::option::Option<f64>,
    ///生效时间
    #[prost(string, optional, tag="6")]
    pub effective_time: ::core::option::Option<::prost::alloc::string::String>,
    ///生效时间戳
    #[prost(double, optional, tag="7")]
    pub effective_timestamp: ::core::option::Option<f64>,
    ///结束时间，在创业板转主板事件不存在该字段，在剩余事件表示临时代码交易结束时间
    #[prost(string, optional, tag="8")]
    pub end_time: ::core::option::Option<::prost::alloc::string::String>,
    ///结束时间戳，在创业板转主板事件不存在该字段，在剩余事件表示临时代码交易结束时间
    #[prost(double, optional, tag="9")]
    pub end_timestamp: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeFilter {
    ///TimeFilterType, 过滤类型
    #[prost(int32, required, tag="1")]
    pub r#type: i32,
    ///开始时间点
    #[prost(string, optional, tag="2")]
    pub begin_time: ::core::option::Option<::prost::alloc::string::String>,
    ///结束时间点
    #[prost(string, optional, tag="3")]
    pub end_time: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///占位
    #[prost(int32, optional, tag="1")]
    pub place_holder: ::core::option::Option<i32>,
    ///根据股票筛选
    #[prost(message, repeated, tag="2")]
    pub security_list: ::prost::alloc::vec::Vec<super::qot_common::Security>,
    ///根据时间筛选
    #[prost(message, repeated, tag="3")]
    pub time_filter_list: ::prost::alloc::vec::Vec<TimeFilter>,
    ///CodeChangeType，根据类型筛选
    #[prost(int32, repeated, packed="false", tag="4")]
    pub type_list: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///股票代码更换信息，目前仅有港股数据
    #[prost(message, repeated, tag="1")]
    pub code_change_list: ::prost::alloc::vec::Vec<CodeChangeInfo>,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CodeChangeType {
    ///未知
    Unkown = 0,
    ///创业板转主板
    GemToMain = 1,
    ///买卖未缴款供股权
    Unpaid = 2,
    ///更改买卖单位
    ChangeLot = 3,
    ///拆股
    Split = 4,
    ///合股
    Joint = 5,
    ///股份先并后拆
    JointSplit = 6,
    ///股份先拆后并
    SplitJoint = 7,
    ///其他
    Other = 8,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TimeFilterType {
    Unknow = 0,
    ///根据公布时间过滤
    Public = 1,
    ///根据生效时间过滤
    Effective = 2,
    ///根据结束时间过滤
    End = 3,
}
