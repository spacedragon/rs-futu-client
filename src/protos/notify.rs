#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GtwEvent {
    ///GtwEventType,事件类型
    #[prost(int32, required, tag="1")]
    pub event_type: i32,
    ///事件描述
    #[prost(string, required, tag="2")]
    pub desc: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgramStatus {
    ///当前程序状态
    #[prost(message, required, tag="1")]
    pub program_status: super::common::ProgramStatus,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectStatus {
    ///是否登陆行情服务器
    #[prost(bool, required, tag="1")]
    pub qot_logined: bool,
    ///是否登陆交易服务器
    #[prost(bool, required, tag="2")]
    pub trd_logined: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QotRight {
    ///港股行情权限, Qot_Common.QotRight
    #[prost(int32, required, tag="4")]
    pub hk_qot_right: i32,
    ///美股行情权限, Qot_Common.QotRight
    #[prost(int32, required, tag="5")]
    pub us_qot_right: i32,
    ///A股行情权限, Qot_Common.QotRight
    #[prost(int32, required, tag="6")]
    pub cn_qot_right: i32,
    ///港股期权行情权限, Qot_Common.QotRight
    #[prost(int32, optional, tag="7")]
    pub hk_option_qot_right: ::core::option::Option<i32>,
    ///是否有美股期权行情权限
    #[prost(bool, optional, tag="8")]
    pub has_us_option_qot_right: ::core::option::Option<bool>,
    ///港股期货行情权限, Qot_Common.QotRight
    #[prost(int32, optional, tag="9")]
    pub hk_future_qot_right: ::core::option::Option<i32>,
    ///美股期货行情权限, Qot_Common.QotRight
    #[prost(int32, optional, tag="10")]
    pub us_future_qot_right: ::core::option::Option<i32>,
    ///美股期货行情权限, Qot_Common.QotRight
    #[prost(int32, optional, tag="11")]
    pub us_option_qot_right: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiLevel {
    ///api用户等级描述，已在2.10版本之后废弃
    #[prost(string, required, tag="3")]
    pub api_level: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiQuota {
    ///订阅额度
    #[prost(int32, required, tag="1")]
    pub sub_quota: i32,
    ///历史K线额度
    #[prost(int32, required, tag="2")]
    pub history_kl_quota: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///通知类型
    #[prost(int32, required, tag="1")]
    pub r#type: i32,
    ///事件通息
    #[prost(message, optional, tag="2")]
    pub event: ::core::option::Option<GtwEvent>,
    ///程序状态
    #[prost(message, optional, tag="3")]
    pub program_status: ::core::option::Option<ProgramStatus>,
    ///连接状态
    #[prost(message, optional, tag="4")]
    pub connect_status: ::core::option::Option<ConnectStatus>,
    ///行情权限
    #[prost(message, optional, tag="5")]
    pub qot_right: ::core::option::Option<QotRight>,
    ///用户等级，已在2.10版本之后废弃
    #[prost(message, optional, tag="6")]
    pub api_level: ::core::option::Option<ApiLevel>,
    ///API额度
    #[prost(message, optional, tag="7")]
    pub api_quota: ::core::option::Option<ApiQuota>,
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
pub enum NotifyType {
    ///无
    None = 0,
    ///OpenD运行事件通知
    GtwEvent = 1,
    ///程序状态
    ProgramStatus = 2,
    ///连接状态
    ConnStatus = 3,
    ///行情权限
    QotRight = 4,
    ///用户等级，已在2.10版本之后废弃
    ApiLevel = 5,
    ///API额度
    ApiQuota = 6,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GtwEventType {
    ///正常无错
    None = 0,
    ///加载本地配置失败
    LocalCfgLoadFailed = 1,
    ///服务器启动失败
    ApiSvrRunFailed = 2,
    ///客户端版本过低
    ForceUpdate = 3,
    ///登录失败
    LoginFailed = 4,
    ///未同意免责声明
    UnAgreeDisclaimer = 5,
    ///缺少必要网络配置信息;例如控制订阅额度 //已优化，不会再出现该情况
    NetCfgMissing = 6,
    ///牛牛帐号在别处登录
    KickedOut = 7,
    ///登录密码被修改
    LoginPwdChanged = 8,
    ///用户被禁止登录
    BanLogin = 9,
    ///需要图形验证码
    NeedPicVerifyCode = 10,
    ///需要手机验证码
    NeedPhoneVerifyCode = 11,
    ///程序自带数据不存在
    AppDataNotExist = 12,
    ///缺少必要数据
    NessaryDataMissing = 13,
    ///交易密码被修改
    TradePwdChanged = 14,
    ///启用设备锁
    EnableDeviceLock = 15,
}
