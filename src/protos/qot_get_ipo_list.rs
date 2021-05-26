/// Ipo基本数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasicIpoData {
    /// Qot_Common::QotMarket 股票市场，支持港股、美股和 A股。其中，A 股整体返回，不区分沪股和深股。
    #[prost(message, required, tag="1")]
    pub security: super::qot_common::Security,
    /// 股票名称
    #[prost(string, required, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// 上市日期字符串
    #[prost(string, optional, tag="3")]
    pub list_time: ::core::option::Option<::prost::alloc::string::String>,
    /// 上市日期时间戳
    #[prost(double, optional, tag="4")]
    pub list_timestamp: ::core::option::Option<f64>,
}
/// A股Ipo列表额外数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CnIpoExData {
    /// 申购代码
    #[prost(string, required, tag="1")]
    pub apply_code: ::prost::alloc::string::String,
    /// 发行总数
    #[prost(int64, required, tag="2")]
    pub issue_size: i64,
    /// 网上发行量
    #[prost(int64, required, tag="3")]
    pub online_issue_size: i64,
    /// 申购上限
    #[prost(int64, required, tag="4")]
    pub apply_upper_limit: i64,
    /// 顶格申购需配市值
    #[prost(int64, required, tag="5")]
    pub apply_limit_market_value: i64,
    /// 是否预估发行价
    #[prost(bool, required, tag="6")]
    pub is_estimate_ipo_price: bool,
    /// 发行价 预估值会因为募集资金、发行数量、发行费用等数据变动而变动，仅供参考。实际数据公布后会第一时间更新。
    #[prost(double, required, tag="7")]
    pub ipo_price: f64,
    /// 行业市盈率
    #[prost(double, required, tag="8")]
    pub industry_pe_rate: f64,
    /// 是否预估中签率
    #[prost(bool, required, tag="9")]
    pub is_estimate_winning_ratio: bool,
    /// 中签率 该字段为百分比字段，默认不展示%，如20实际对应20%。预估值会因为募集资金、发行数量、发行费用等数据变动而变动，仅供参考。实际数据公布后会第一时间更新。
    #[prost(double, required, tag="10")]
    pub winning_ratio: f64,
    /// 发行市盈率
    #[prost(double, required, tag="11")]
    pub issue_pe_rate: f64,
    /// 申购日期字符串
    #[prost(string, optional, tag="12")]
    pub apply_time: ::core::option::Option<::prost::alloc::string::String>,
    /// 申购日期时间戳
    #[prost(double, optional, tag="13")]
    pub apply_timestamp: ::core::option::Option<f64>,
    /// 公布中签日期字符串
    #[prost(string, optional, tag="14")]
    pub winning_time: ::core::option::Option<::prost::alloc::string::String>,
    /// 公布中签日期时间戳
    #[prost(double, optional, tag="15")]
    pub winning_timestamp: ::core::option::Option<f64>,
    /// 是否已经公布中签号
    #[prost(bool, required, tag="16")]
    pub is_has_won: bool,
    /// Qot_GetIpoList::WinningNumData 中签号数据，对应PC中"公布中签日期的已公布"
    #[prost(message, repeated, tag="17")]
    pub winning_num_data: ::prost::alloc::vec::Vec<WinningNumData>,
}
/// 中签号数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WinningNumData {
    /// 分组名
    #[prost(string, required, tag="1")]
    pub winning_name: ::prost::alloc::string::String,
    /// 中签号信息
    #[prost(string, required, tag="2")]
    pub winning_info: ::prost::alloc::string::String,
}
/// 港股Ipo列表额外数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HkIpoExData {
    /// 最低发售价
    #[prost(double, required, tag="1")]
    pub ipo_price_min: f64,
    /// 最高发售价
    #[prost(double, required, tag="2")]
    pub ipo_price_max: f64,
    /// 上市价
    #[prost(double, required, tag="3")]
    pub list_price: f64,
    /// 每手股数
    #[prost(int32, required, tag="4")]
    pub lot_size: i32,
    /// 入场费
    #[prost(double, required, tag="5")]
    pub entrance_price: f64,
    /// 是否为认购状态，True-认购中，False-待上市
    #[prost(bool, required, tag="6")]
    pub is_subscribe_status: bool,
    /// 截止认购日期字符串
    #[prost(string, optional, tag="7")]
    pub apply_end_time: ::core::option::Option<::prost::alloc::string::String>,
    /// 截止认购日期时间戳 因需处理认购手续，富途认购截止时间会早于交易所公布的日期。
    #[prost(double, optional, tag="8")]
    pub apply_end_timestamp: ::core::option::Option<f64>,
}
/// 美股Ipo列表额外数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsIpoExData {
    /// 最低发行价
    #[prost(double, required, tag="1")]
    pub ipo_price_min: f64,
    /// 最高发行价
    #[prost(double, required, tag="2")]
    pub ipo_price_max: f64,
    /// 发行量
    #[prost(int64, required, tag="3")]
    pub issue_size: i64,
}
/// 新股Ipo数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpoData {
    /// IPO基本数据	
    #[prost(message, required, tag="1")]
    pub basic: BasicIpoData,
    /// A股IPO额外数据
    #[prost(message, optional, tag="2")]
    pub cn_ex_data: ::core::option::Option<CnIpoExData>,
    /// 港股IPO额外数据
    #[prost(message, optional, tag="3")]
    pub hk_ex_data: ::core::option::Option<HkIpoExData>,
    /// 美股IPO额外数据
    #[prost(message, optional, tag="4")]
    pub us_ex_data: ::core::option::Option<UsIpoExData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    /// Qot_Common::QotMarket股票市场，支持沪股和深股，且沪股和深股不做区分都代表A股市场。
    #[prost(int32, required, tag="1")]
    pub market: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    /// 新股IPO数据
    #[prost(message, repeated, tag="1")]
    pub ipo_list: ::prost::alloc::vec::Vec<IpoData>,
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
