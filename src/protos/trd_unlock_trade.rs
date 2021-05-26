#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///true解锁交易，false锁定交易
    #[prost(bool, required, tag="1")]
    pub unlock: bool,
    ///交易密码的MD5转16进制(全小写)，解锁交易必须要填密码，锁定交易不需要验证密码，可不填
    #[prost(string, optional, tag="2")]
    pub pwd_md5: ::core::option::Option<::prost::alloc::string::String>,
    ///券商标识，取值见Trd_Common.SecurityFirm
    #[prost(int32, optional, tag="3")]
    pub security_firm: ::core::option::Option<i32>,
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
