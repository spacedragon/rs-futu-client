#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///交易公共参数头
    #[prost(message, required, tag="1")]
    pub header: super::trd_common::TrdHeader,
    ///过滤条件
    #[prost(message, optional, tag="2")]
    pub filter_conditions: ::core::option::Option<super::trd_common::TrdFilterConditions>,
    ///立即刷新OpenD缓存的此数据，默认不填。true向服务器获取最新数据更新缓存并返回；flase或没填则返回OpenD缓存的数据，不会向服务器请求。
    #[prost(bool, optional, tag="3")]
    pub refresh_cache: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///交易公共参数头
    #[prost(message, required, tag="1")]
    pub header: super::trd_common::TrdHeader,
    ///成交列表
    #[prost(message, repeated, tag="2")]
    pub order_fill_list: ::prost::alloc::vec::Vec<super::trd_common::OrderFill>,
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
