#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///股票
    #[prost(message, repeated, tag="1")]
    pub security_list: ::prost::alloc::vec::Vec<super::qot_common::Security>,
    ///Qot_Common.SubType,要注册到该连接的订阅类型
    #[prost(int32, repeated, packed="false", tag="2")]
    pub sub_type_list: ::prost::alloc::vec::Vec<i32>,
    ///Qot_Common.RehabType,复权类型,注册K线类型才生效,其他订阅类型忽略该参数,注册K线时该参数不指定默认前复权
    #[prost(int32, repeated, packed="false", tag="3")]
    pub rehab_type_list: ::prost::alloc::vec::Vec<i32>,
    ///注册或取消
    #[prost(bool, required, tag="4")]
    pub is_reg_or_un_reg: bool,
    ///注册后如果本地已有数据是否首推一次已存在数据,该参数不指定则默认true
    #[prost(bool, optional, tag="5")]
    pub is_first_push: ::core::option::Option<bool>,
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
