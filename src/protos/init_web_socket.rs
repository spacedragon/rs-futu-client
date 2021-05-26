#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///opend的ip
    #[prost(string, optional, tag="1")]
    pub ip: ::core::option::Option<::prost::alloc::string::String>,
    ///opend的port
    #[prost(int32, optional, tag="2")]
    pub port: ::core::option::Option<i32>,
    ///与opend连接的密钥（正文而非路径）
    #[prost(string, optional, tag="3")]
    pub rsa_private_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub websocket_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub client_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub programming_language: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///服务器回包时的格林威治时间戳，单位秒
    #[prost(int64, optional, tag="1")]
    pub server_time: ::core::option::Option<i64>,
    ///连接ID
    #[prost(uint64, optional, tag="2")]
    pub conn_id: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(message, optional, tag="1")]
    pub c2s: ::core::option::Option<C2s>,
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
