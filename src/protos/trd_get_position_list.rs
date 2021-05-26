#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///交易公共参数头
    #[prost(message, required, tag="1")]
    pub header: super::trd_common::TrdHeader,
    ///过滤条件
    #[prost(message, optional, tag="2")]
    pub filter_conditions: ::core::option::Option<super::trd_common::TrdFilterConditions>,
    ///过滤盈亏百分比下限，高于此比例的会返回，比如传10.0，返回盈亏比例大于10%的持仓
    #[prost(double, optional, tag="3")]
    pub filter_pl_ratio_min: ::core::option::Option<f64>,
    ///过滤盈亏百分比上限，低于此比例的会返回，比如传20.0，返回盈亏比例小于20%的持仓
    #[prost(double, optional, tag="4")]
    pub filter_pl_ratio_max: ::core::option::Option<f64>,
    ///立即刷新OpenD缓存的此数据，默认不填。true向服务器获取最新数据更新缓存并返回；flase或没填则返回OpenD缓存的数据，不会向服务器请求。
    #[prost(bool, optional, tag="5")]
    pub refresh_cache: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///交易公共参数头
    #[prost(message, required, tag="1")]
    pub header: super::trd_common::TrdHeader,
    ///持仓列表
    #[prost(message, repeated, tag="2")]
    pub position_list: ::prost::alloc::vec::Vec<super::trd_common::Position>,
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
