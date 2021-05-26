///内部使用
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    #[prost(string, required, tag="1")]
    pub cmd: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub params: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    #[prost(string, required, tag="1")]
    pub cmd: ::prost::alloc::string::String,
    #[prost(string, required, tag="2")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(message, required, tag="1")]
    pub c2s: C2s,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    ///返回结果，参见Common.RetType的枚举定义
    #[prost(int32, required, tag="1", default="-400")]
    pub ret_type: i32,
    ///返回结果描述
    #[prost(string, optional, tag="2")]
    pub ret_msg: ::core::option::Option<::prost::alloc::string::String>,
    ///错误码，客户端一般通过retType和retMsg来判断结果和详情，errCode只做日志记录，仅在个别协议失败时对账用
    #[prost(int32, optional, tag="3")]
    pub err_code: ::core::option::Option<i32>,
    #[prost(message, optional, tag="4")]
    pub s2c: ::core::option::Option<S2c>,
}
