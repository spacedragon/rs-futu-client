#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///客户端版本号，clientVer = "."以前的数 * 100 + "."以后的，举例：1.1版本的clientVer为1 * 100 + 1 = 101，2.21版本为2 * 100 + 21 = 221
    #[prost(int32, required, tag="1")]
    pub client_ver: i32,
    ///客户端唯一标识，无生具体生成规则，客户端自己保证唯一性即可
    #[prost(string, required, tag="2")]
    pub client_id: ::prost::alloc::string::String,
    ///此连接是否接收市场状态、交易需要重新解锁等等事件通知，true代表接收，FutuOpenD就会向此连接推送这些通知，反之false代表不接收不推送
    #[prost(bool, optional, tag="3")]
    pub recv_notify: ::core::option::Option<bool>,
    ///如果通信要加密，首先得在FutuOpenD和客户端都配置RSA密钥，不配置始终不加密
    ///如果配置了RSA密钥且指定的加密算法不为PacketEncAlgo_None则加密(即便这里不设置，配置了RSA密钥，也会采用默认加密方式)，默认采用FTAES_ECB算法
    ///
    ///指定包加密算法，参见Common.PacketEncAlgo的枚举定义
    #[prost(int32, optional, tag="4")]
    pub packet_enc_algo: ::core::option::Option<i32>,
    ///指定这条连接上的推送协议格式，若不指定则使用push_proto_type配置项
    #[prost(int32, optional, tag="5")]
    pub push_proto_fmt: ::core::option::Option<i32>,
    ///接口编程语言，用于统计语言偏好
    #[prost(string, optional, tag="6")]
    pub programming_language: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///FutuOpenD的版本号
    #[prost(int32, required, tag="1")]
    pub server_ver: i32,
    ///FutuOpenD登陆的牛牛用户ID
    #[prost(uint64, required, tag="2")]
    pub login_user_id: u64,
    ///此连接的连接ID，连接的唯一标识
    #[prost(uint64, required, tag="3")]
    pub conn_id: u64,
    ///此连接后续AES加密通信的Key，固定为16字节长字符串
    #[prost(string, required, tag="4")]
    pub conn_aes_key: ::prost::alloc::string::String,
    ///心跳保活间隔
    #[prost(int32, required, tag="5")]
    pub keep_alive_interval: i32,
    ///AES加密通信CBC加密模式的iv，固定为16字节长字符串
    #[prost(string, optional, tag="6")]
    pub aes_cb_civ: ::core::option::Option<::prost::alloc::string::String>,
    ///用户类型，牛牛用户或MooMoo用户
    #[prost(int32, optional, tag="7")]
    pub user_attribution: ::core::option::Option<i32>,
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
