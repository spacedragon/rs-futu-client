#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///分组名,有同名的返回排序的首个
    #[prost(string, required, tag="1")]
    pub group_name: ::prost::alloc::string::String,
    ///ModifyUserSecurityOp,操作类型
    #[prost(int32, required, tag="2")]
    pub op: i32,
    ///新增、删除或移出该分组下的股票
    #[prost(message, repeated, tag="3")]
    pub security_list: ::prost::alloc::vec::Vec<super::qot_common::Security>,
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
pub enum ModifyUserSecurityOp {
    Unknown = 0,
    ///新增
    Add = 1,
    ///删除自选
    Del = 2,
    ///移出分组
    MoveOut = 3,
}
