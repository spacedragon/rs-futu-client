///包的唯一标识，用于回放攻击的识别和保护
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PacketId {
    ///当前TCP连接的连接ID，一条连接的唯一标识，InitConnect协议会返回
    #[prost(uint64, required, tag="1")]
    pub conn_id: u64,
    ///自增序列号
    #[prost(uint32, required, tag="2")]
    pub serial_no: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgramStatus {
    ///当前状态
    #[prost(enumeration="ProgramStatusType", required, tag="1")]
    pub r#type: i32,
    /// 额外描述
    #[prost(string, optional, tag="2")]
    pub str_ext_desc: ::core::option::Option<::prost::alloc::string::String>,
}
///返回结果
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RetType {
    ///成功
    Succeed = 0,
    ///失败
    Failed = -1,
    ///超时
    TimeOut = -100,
    ///连接断开
    DisConnect = -200,
    ///未知结果
    Unknown = -400,
    ///包内容非法
    Invalid = -500,
}
///包加密算法
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PacketEncAlgo {
    ///富途修改过的AES的ECB加密模式
    FtaesEcb = 0,
    ///不加密
    None = -1,
    ///标准的AES的ECB加密模式
    AesEcb = 1,
    ///标准的AES的CBC加密模式
    AesCbc = 2,
}
///协议格式，请求协议在请求头中指定，推送协议在Init时指定
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoFmt {
    ///Google Protobuf格式
    Protobuf = 0,
    ///Json格式
    Json = 1,
}
///用户注册归属地
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserAttribution {
    ///
    Unknown = 0,
    ///牛牛
    Nn = 1,
    ///MooMoo
    Mm = 2,
    ///新加坡
    Sg = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProgramStatusType {
    None = 0,
    ///已完成类似加载配置,启动服务器等操作,服务器启动之前的状态无需返回
    Loaded = 1,
    ///登录中
    Loging = 2,
    ///需要图形验证码
    NeedPicVerifyCode = 3,
    ///需要手机验证码
    NeedPhoneVerifyCode = 4,
    ///登录失败,详细原因在描述返回
    LoginFailed = 5,
    ///客户端版本过低
    ForceUpdate = 6,
    ///正在拉取类似免责声明等一些必要信息
    NessaryDataPreparing = 7,
    ///缺少必要信息
    NessaryDataMissing = 8,
    ///未同意免责声明
    UnAgreeDisclaimer = 9,
    ///可以接收业务协议收发,正常可用状态
    Ready = 10,
    ///OpenD登录后被强制退出登录，会导致连接全部断开,需要重连后才能得到以下该状态（并且需要在ui模式下）
    ///
    ///被强制退出登录,例如修改了登录密码,中途打开设备锁等,详细原因在描述返回
    ForceLogout = 11,
    ///拉取免责声明标志失败
    DisclaimerPullFailed = 12,
}
