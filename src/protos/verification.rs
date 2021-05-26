///图形验证码下载之后会将其存至固定路径，请到该路径下查看验证码
///Windows平台：%appdata%/com.futunn.FutuOpenD/F3CNN/PicVerifyCode.png
///非Windows平台：~/.com.futunn.FutuOpenD/F3CNN/PicVerifyCode.png
///注意：只有最后一次请求验证码会生效，重复请求只有最后一次的验证码有效
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///验证码类型, VerificationType
    #[prost(int32, required, tag="1")]
    pub r#type: i32,
    ///操作, VerificationOp
    #[prost(int32, required, tag="2")]
    pub op: i32,
    ///验证码，请求验证码时忽略该字段，输入时必填
    #[prost(string, optional, tag="3")]
    pub code: ::core::option::Option<::prost::alloc::string::String>,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VerificationType {
    ///未知操作
    Unknow = 0,
    /// 图形验证码
    Picture = 1,
    /// 手机验证码
    Phone = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VerificationOp {
    ///未知操作
    Unknow = 0,
    ///请求验证码
    Request = 1,
    ///输入验证码并继续登录操作
    InputAndLogin = 2,
}
