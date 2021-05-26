#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///交易公共参数头
    #[prost(message, required, tag="1")]
    pub header: super::trd_common::TrdHeader,
    ///立即刷新OpenD缓存的此数据，默认不填。true向服务器获取最新数据更新缓存并返回；flase或没填则返回OpenD缓存的数据，不会向服务器请求。
    #[prost(bool, optional, tag="2")]
    pub refresh_cache: ::core::option::Option<bool>,
    ///正常情况下，服务器有更新就会立即推送到OpenD，OpenD缓存着数据，API请求过来，返回同步的缓存数据，一般不需要指定刷新缓存，保证快速返回且减少对服务器的压力
    ///如果遇到丢包等情况，可能出现缓存数据与服务器不一致，用户如果发现数据更新有异样，可指定刷新缓存，解决数据同步的问题。
    ///
    ///货币种类，参见Trd_Common.Currency。期货账户必填，其它账户忽略 
    #[prost(int32, optional, tag="3")]
    pub currency: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///交易公共参数头
    #[prost(message, required, tag="1")]
    pub header: super::trd_common::TrdHeader,
    ///账户资金
    #[prost(message, optional, tag="2")]
    pub funds: ::core::option::Option<super::trd_common::Funds>,
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
