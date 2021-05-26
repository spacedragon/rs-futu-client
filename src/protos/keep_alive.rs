#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///客户端发包时的格林威治时间戳，单位秒
    #[prost(int64, required, tag="1")]
    pub time: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///服务器回包时的格林威治时间戳，单位秒
    #[prost(int64, required, tag="1")]
    pub time: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(message, required, tag="1")]
    pub c2s: C2s,
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
