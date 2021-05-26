/// 简单属性筛选
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseFilter {
    /// StockField 简单属性
    #[prost(int32, required, tag="1")]
    pub field_name: i32,
    /// 区间下限（闭区间），不传代表下限为 -∞
    #[prost(double, optional, tag="2")]
    pub filter_min: ::core::option::Option<f64>,
    /// 区间上限（闭区间），不传代表上限为 +∞
    #[prost(double, optional, tag="3")]
    pub filter_max: ::core::option::Option<f64>,
    /// 该字段是否不需要筛选，True：不筛选，False：筛选。不传默认不筛选
    #[prost(bool, optional, tag="4")]
    pub is_no_filter: ::core::option::Option<bool>,
    /// SortDir 排序方向，默认不排序。
    #[prost(int32, optional, tag="5")]
    pub sort_dir: ::core::option::Option<i32>,
}
/// 累积属性筛选
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccumulateFilter {
    /// AccumulateField 累积属性
    #[prost(int32, required, tag="1")]
    pub field_name: i32,
    /// 区间下限（闭区间），不传代表下限为 -∞
    #[prost(double, optional, tag="2")]
    pub filter_min: ::core::option::Option<f64>,
    /// 区间上限（闭区间），不传代表上限为 +∞
    #[prost(double, optional, tag="3")]
    pub filter_max: ::core::option::Option<f64>,
    /// 该字段是否不需要筛选，True：不筛选，False：筛选。不传默认不筛选
    #[prost(bool, optional, tag="4")]
    pub is_no_filter: ::core::option::Option<bool>,
    /// SortDir 排序方向，默认不排序。
    #[prost(int32, optional, tag="5")]
    pub sort_dir: ::core::option::Option<i32>,
    /// 近几日，累积时间
    #[prost(int32, required, tag="6")]
    pub days: i32,
}
/// 财务属性筛选
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinancialFilter {
    /// FinancialField 财务属性
    #[prost(int32, required, tag="1")]
    pub field_name: i32,
    /// 区间下限（闭区间），不传代表下限为 -∞
    #[prost(double, optional, tag="2")]
    pub filter_min: ::core::option::Option<f64>,
    /// 区间上限（闭区间），不传代表上限为 +∞
    #[prost(double, optional, tag="3")]
    pub filter_max: ::core::option::Option<f64>,
    /// 该字段是否不需要筛选，True：不筛选，False：筛选。不传默认不筛选
    #[prost(bool, optional, tag="4")]
    pub is_no_filter: ::core::option::Option<bool>,
    /// SortDir 排序方向，默认不排序。
    #[prost(int32, optional, tag="5")]
    pub sort_dir: ::core::option::Option<i32>,
    /// FinancialQuarter 财报累积时间
    #[prost(int32, required, tag="6")]
    pub quarter: i32,
}
/// 形态技术指标属性筛选
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatternFilter {
    /// PatternField 形态技术指标属性
    #[prost(int32, required, tag="1")]
    pub field_name: i32,
    /// Qot_Common.KLType，K线类型，仅支持K_60M，K_DAY，K_WEEK，K_MON 四种时间周期
    #[prost(int32, required, tag="2")]
    pub kl_type: i32,
    /// 该字段是否不需要筛选，True代表不筛选，False代表筛选。不传默认为不筛选
    #[prost(bool, optional, tag="3")]
    pub is_no_filter: ::core::option::Option<bool>,
}
/// 自定义技术指标属性筛选
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomIndicatorFilter {
    /// CustomIndicatorField 自定义技术指标属性
    #[prost(int32, required, tag="1")]
    pub first_field_name: i32,
    /// CustomIndicatorField 自定义技术指标属性
    #[prost(int32, required, tag="2")]
    pub second_field_name: i32,
    /// RelativePosition 相对位置,主要用于MA，EMA，RSI指标做比较
    #[prost(int32, required, tag="3")]
    pub relative_position: i32,
    /// 自定义数值，用于与RSI进行比较
    #[prost(double, optional, tag="4")]
    pub field_value: ::core::option::Option<f64>,
    /// Qot_Common.KLType，K线类型，仅支持K_60M，K_DAY，K_WEEK，K_MON 四种时间周期	
    #[prost(int32, required, tag="5")]
    pub kl_type: i32,
    /// 该字段是否不需要筛选，True代表不筛选，False代表筛选。不传默认为不筛选
    #[prost(bool, optional, tag="6")]
    pub is_no_filter: ::core::option::Option<bool>,
}
/// 简单属性数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseData {
    /// StockField 简单属性
    #[prost(int32, required, tag="1")]
    pub field_name: i32,
    #[prost(double, required, tag="2")]
    pub value: f64,
}
/// 累积属性数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccumulateData {
    /// AccumulateField 累积属性
    #[prost(int32, required, tag="1")]
    pub field_name: i32,
    #[prost(double, required, tag="2")]
    pub value: f64,
    /// 近几日，累积时间
    #[prost(int32, required, tag="3")]
    pub days: i32,
}
/// 财务属性数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinancialData {
    /// FinancialField 财务属性
    #[prost(int32, required, tag="1")]
    pub field_name: i32,
    #[prost(double, required, tag="2")]
    pub value: f64,
    /// FinancialQuarter 财报累积时间
    #[prost(int32, required, tag="3")]
    pub quarter: i32,
}
/// 自定义技术指标属性数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomIndicatorData {
    /// CustomIndicatorField 自定义技术指标属性
    #[prost(int32, required, tag="1")]
    pub field_name: i32,
    #[prost(double, required, tag="2")]
    pub value: f64,
    /// Qot_Common.KLType，K线类型，仅支持K_60M，K_DAY，K_WEEK，K_MON 四种时间周期
    #[prost(int32, required, tag="3")]
    pub kl_type: i32,
}
/// 返回的股票数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StockData {
    /// 股票
    #[prost(message, required, tag="1")]
    pub security: super::qot_common::Security,
    /// 股票名称
    #[prost(string, required, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// 筛选后的简单指标属性数据
    #[prost(message, repeated, tag="3")]
    pub base_data_list: ::prost::alloc::vec::Vec<BaseData>,
    /// 筛选后的累积指标属性数据
    #[prost(message, repeated, tag="4")]
    pub accumulate_data_list: ::prost::alloc::vec::Vec<AccumulateData>,
    /// 筛选后的财务指标属性数据
    #[prost(message, repeated, tag="5")]
    pub financial_data_list: ::prost::alloc::vec::Vec<FinancialData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    /// 数据起始点
    #[prost(int32, required, tag="1")]
    pub begin: i32,
    /// 请求数据个数，最大200        
    #[prost(int32, required, tag="2")]
    pub num: i32,
    /// Qot_Common::QotMarket股票市场，支持沪股和深股，且沪股和深股不做区分都代表A股市场。
    #[prost(int32, required, tag="3")]
    pub market: i32,
    /// 以下为筛选条件，可选字段，不填表示不过滤
    ///
    /// 板块
    #[prost(message, optional, tag="4")]
    pub plate: ::core::option::Option<super::qot_common::Security>,
    /// 简单指标过滤器
    #[prost(message, repeated, tag="5")]
    pub base_filter_list: ::prost::alloc::vec::Vec<BaseFilter>,
    /// 累积指标过滤器
    #[prost(message, repeated, tag="6")]
    pub accumulate_filter_list: ::prost::alloc::vec::Vec<AccumulateFilter>,
    /// 财务指标过滤器
    #[prost(message, repeated, tag="7")]
    pub financial_filter_list: ::prost::alloc::vec::Vec<FinancialFilter>,
    /// 形态技术指标过滤器
    #[prost(message, repeated, tag="8")]
    pub pattern_filter_list: ::prost::alloc::vec::Vec<PatternFilter>,
    /// 自定义技术指标过滤器	
    #[prost(message, repeated, tag="9")]
    pub custom_indicator_filter_list: ::prost::alloc::vec::Vec<CustomIndicatorFilter>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    /// 是否最后一页了,false:非最后一页,还有窝轮记录未返回; true:已是最后一页
    #[prost(bool, required, tag="1")]
    pub last_page: bool,
    /// 该条件请求所有数据的个数
    #[prost(int32, required, tag="2")]
    pub all_count: i32,
    /// 返回的股票数据列表
    #[prost(message, repeated, tag="3")]
    pub data_list: ::prost::alloc::vec::Vec<StockData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(message, required, tag="1")]
    pub c2s: C2s,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    /// RetType,返回结果
    #[prost(int32, required, tag="1", default="-400")]
    pub ret_type: i32,
    #[prost(string, optional, tag="2")]
    pub ret_msg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="3")]
    pub err_code: ::core::option::Option<i32>,
    #[prost(message, optional, tag="4")]
    pub s2c: ::core::option::Option<S2c>,
}
// 使用到以下 6 个结构体（BaseFilter，AccumulateFilter，FinancialFilter，BaseData，AccumulateData，FinancialData）的用户请注意，由于属性字段名“field”与 C # 的 protobuf 保留函数名产生冲突，Futu API 将从 3.18 版本开始将这一字段统一更名为“fieldName”，请注意修改使用到对应接口的字段名。

/// 简单属性
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StockField {
    /// 未知
    Unknown = 0,
    /// 股票代码，不能填区间上下限值。
    StockCode = 1,
    /// 股票名称，不能填区间上下限值。
    StockName = 2,
    /// 最新价（精确到小数点后 3 位，超出部分会被舍弃）例如填写[10,20]值区间
    CurPrice = 3,
    /// (现价 - 52周最高)/52周最高，对应PC端离52周高点百分比（精确到小数点后 3 位，超出部分会被舍弃）例如填写[-30,-10]值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%，如20实际对应20%）
    CurPriceToHighest52WeeksRatio = 4,
    /// (现价 - 52周最低)/52周最低，对应PC端离52周低点百分比（精确到小数点后 3 位，超出部分会被舍弃）例如填写[20,40]值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    CurPriceToLowest52WeeksRatio = 5,
    /// (今日最高 - 52周最高)/52周最高（精确到小数点后 3 位，超出部分会被舍弃）例如填写[-3,-1]值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    HighPriceToHighest52WeeksRatio = 6,
    /// (今日最低 - 52周最低)/52周最低（精确到小数点后 3 位，超出部分会被舍弃）例如填写[10,70]值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    LowPriceToLowest52WeeksRatio = 7,
    /// 量比（精确到小数点后 3 位，超出部分会被舍弃）例如填写[0.5,30]值区间
    VolumeRatio = 8,
    /// 委比（精确到小数点后 3 位，超出部分会被舍弃）例如填写[-20,80.5]值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    BidAskRatio = 9,
    /// 每手价格（精确到小数点后 3 位，超出部分会被舍弃）例如填写[40,100]值区间
    LotPrice = 10,
    /// 市值（精确到小数点后 3 位，超出部分会被舍弃）例如填写[50000000,3000000000]值区间
    MarketVal = 11,
    /// 市盈率(静态)（精确到小数点后 3 位，超出部分会被舍弃）例如填写[-8,65.3]值区间
    PeAnnual = 12,
    /// 市盈率 TTM（精确到小数点后 3 位，超出部分会被舍弃）例如填写[-10,20.5]值区间
    PeTtm = 13,
    /// 市净率（精确到小数点后 3 位，超出部分会被舍弃）例如填写[0.5,20]值区间
    PbRate = 14,
    /// 五分钟价格涨跌幅（精确到小数点后 3 位，超出部分会被舍弃）例如填写[-5,6.3]值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    ChangeRate5min = 15,
    /// 年初至今价格涨跌幅（精确到小数点后 3 位，超出部分会被舍弃）例如填写[-50.1,400.7]值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    ChangeRateBeginYear = 16,
    /// 基础量价属性
    ///
    /// 市销率 TTM（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [100, 500] 值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%） 
    Psttm = 17,
    /// 市现率 TTM（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [100, 1000] 值区间 （该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    Pcfttm = 18,
    /// 总股数（精确到小数点后 0 位，超出部分会被舍弃）例如填写 [1000000000,1000000000] 值区间 (单位：股)（精确到小数点后 0 位，超出部分会被舍弃）
    TotalShare = 19,
    /// 流通股数（精确到小数点后 0 位，超出部分会被舍弃）例如填写 [1000000000,1000000000] 值区间 (单位：股)（精确到小数点后 0 位，超出部分会被舍弃）
    FloatShare = 20,
    /// 流通市值（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1000000000,1000000000] 值区间 (单位：元)
    FloatMarketVal = 21,
}
/// 累积属性
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccumulateField {
    /// 未知
    Unknown = 0,
    /// 涨跌幅（精确到小数点后 3 位，超出部分会被舍弃）例如填写[-10.2,20.4]值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    ChangeRate = 1,
    /// 振幅（精确到小数点后 3 位，超出部分会被舍弃）例如填写[0.5,20.6]值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    Amplitude = 2,
    /// 日均成交量（精确到小数点后 0 位，超出部分会被舍弃）例如填写[2000,70000]值区间（精确到小数点后 0 位，超出部分会被舍弃）
    Volume = 3,
    /// 日均成交额（精确到小数点后 3 位，超出部分会被舍弃）例如填写[1400,890000]值区间
    Turnover = 4,
    /// 换手率（精确到小数点后 3 位，超出部分会被舍弃）例如填写[2,30]值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    TurnoverRate = 5,
}
/// 财务属性
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FinancialField {
    /// 基础财务属性
    ///
    /// 未知
    Unknown = 0,
    /// 净利润（精确到小数点后 3 位，超出部分会被舍弃）例如填写[100000000,2500000000]值区间
    NetProfit = 1,
    /// 净利润增长率（精确到小数点后 3 位，超出部分会被舍弃）例如填写[-10,300]值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    NetProfitGrowth = 2,
    /// 营业收入（精确到小数点后 3 位，超出部分会被舍弃）例如填写[100000000,6400000000]值区间
    SumOfBusiness = 3,
    /// 营收同比增长率（精确到小数点后 3 位，超出部分会被舍弃）例如填写[-5,200]值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    SumOfBusinessGrowth = 4,
    /// 净利率（精确到小数点后 3 位，超出部分会被舍弃）例如填写[10,113]值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    NetProfitRate = 5,
    /// 毛利率（精确到小数点后 3 位，超出部分会被舍弃）例如填写[4,65]值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    GrossProfitRate = 6,
    /// 资产负债率（精确到小数点后 3 位，超出部分会被舍弃）例如填写[5,470]值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    DebtAssetsRate = 7,
    /// 净资产收益率（精确到小数点后 3 位，超出部分会被舍弃）例如填写[20,230]值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    ReturnOnEquityRate = 8,
    /// 盈利能力属性
    ///
    /// 投入资本回报率（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1.0,10.0] 值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    Roic = 9,
    /// 资产回报率 TTM（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1.0,10.0] 值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%。仅适用于年报。）
    Roattm = 10,
    /// 息税前利润 TTM（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1000000000,1000000000] 值区间（单位：元。仅适用于年报。）
    Ebitttm = 11,
    /// 税息折旧及摊销前利润（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1000000000,1000000000] 值区间（单位：元）
    Ebitda = 12,
    /// 营业利润率 TTM（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1.0,10.0] 值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%。仅适用于年报。）
    OperatingMarginTtm = 13,
    /// EBIT利润率（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1.0,10.0] 值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    EbitMargin = 14,
    /// EBITDA利润率（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1.0,10.0] 值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    EbitdaMargin = 15,
    /// 财务成本率（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1.0,10.0] 值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    FinancialCostRate = 16,
    /// 营业利润 TTM（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1000000000,1000000000] 值区间 （单位：元。仅适用于年报。）
    OperatingProfitTtm = 17,
    /// 归属于母公司的净利润（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1000000000,1000000000] 值区间 （单位：元。仅适用于年报。）
    ShareholderNetProfitTtm = 18,
    /// 盈利中的现金收入比例（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1.0,60.0] 值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%。仅适用于年报。）
    NetProfitCashCoverTtm = 19,
    /// 偿债能力属性
    ///
    /// 流动比率（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [100,250] 值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    CurrentRatio = 20,
    /// 速动比率（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [100,250] 值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）	
    QuickRatio = 21,
    /// 清债能力属性
    ///
    /// 流动资产率（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [10,100] 值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    CurrentAssetRatio = 22,
    /// 流动负债率（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [10,100] 值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    CurrentDebtRatio = 23,
    /// 权益乘数（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [100,180] 值区间
    EquityMultiplier = 24,
    /// 产权比率（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [50,100] 值区间 （该字段为百分比字段，默认不展示 %，如 20 实际对应 20%） 
    PropertyRatio = 25,
    /// 现金和现金等价（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1000000000,1000000000] 值区间（单位：元）	
    CashAndCashEquivalents = 26,
    /// 运营能力属性
    ///
    /// 总资产周转率（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [50,100] 值区间 （该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    TotalAssetTurnover = 27,
    /// 固定资产周转率（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [50,100] 值区间 （该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    FixedAssetTurnover = 28,
    /// 存货周转率（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [50,100] 值区间 （该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    InventoryTurnover = 29,
    /// 经营活动现金流 TTM（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1000000000,1000000000] 值区间（单位：元。仅适用于年报。）
    OperatingCashFlowTtm = 30,
    /// 应收账款净额（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1000000000,1000000000] 值区间 例如填写 [1000000000,1000000000] 值区间 （单位：元）	
    AccountsReceivable = 31,
    /// 成长能力属性
    ///
    /// EBIT同比增长率（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1.0,10.0] 值区间 （该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    EbitGrowthRate = 32,
    /// 营业利润同比增长率（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1.0,10.0] 值区间 （该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    OperatingProfitGrowthRate = 33,
    /// 总资产同比增长率（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1.0,10.0] 值区间 （该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    TotalAssetsGrowthRate = 34,
    /// 归母净利润同比增长率（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1.0,10.0] 值区间 （该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    ProfitToShareholdersGrowthRate = 35,
    /// 总利润同比增长率（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1.0,10.0] 值区间 （该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    ProfitBeforeTaxGrowthRate = 36,
    /// EPS同比增长率（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1.0,10.0] 值区间 （该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    EpsGrowthRate = 37,
    /// ROE同比增长率（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1.0,10.0] 值区间 （该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    RoeGrowthRate = 38,
    /// ROIC同比增长率（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1.0,10.0] 值区间 （该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    RoicGrowthRate = 39,
    /// 经营现金流同比增长率（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1.0,10.0] 值区间 （该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    NocfGrowthRate = 40,
    /// 每股经营现金流同比增长率（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [1.0,10.0] 值区间 （该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    NocfPerShareGrowthRate = 41,
    /// 现金流属性
    ///
    /// 经营现金收入比（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [10,100] 值区间（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    OperatingRevenueCashCover = 42,
    /// 营业利润占比（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [10,100] 值区间 （该字段为百分比字段，默认不展示 %，如 20 实际对应 20%） 	
    OperatingProfitToTotalProfit = 43,
    /// 市场表现属性
    ///
    /// 基本每股收益（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [0.1,10] 值区间 (单位：元)
    BasicEps = 44,
    /// 稀释每股收益（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [0.1,10] 值区间 (单位：元)
    DilutedEps = 45,
    /// 每股经营现金净流量（精确到小数点后 3 位，超出部分会被舍弃）例如填写 [0.1,10] 值区间 (单位：元)	
    NocfPerShare = 46,
}
/// 自定义技术指标属性
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CustomIndicatorField {
    /// 未知
    Unknown = 0,
    /// 最新价格 
    Price = 1,
    /// 5日简单均线 
    Ma5 = 2,
    /// 10日简单均线 
    Ma10 = 3,
    /// 20日简单均线 
    Ma20 = 4,
    /// 30日简单均线 
    Ma30 = 5,
    /// 60日简单均线 
    Ma60 = 6,
    /// 120日简单均线
    Ma120 = 7,
    /// 250日简单均线
    Ma250 = 8,
    /// RSI(12)
    Rsi = 9,
    /// 5日指数移动均线 
    Ema5 = 10,
    /// 10日指数移动均线 
    Ema10 = 11,
    /// 20日指数移动均线 
    Ema20 = 12,
    /// 30日指数移动均线 
    Ema30 = 13,
    /// 60日指数移动均线 
    Ema60 = 14,
    /// 120日指数移动均线
    Ema120 = 15,
    /// 250日指数移动均线
    Ema250 = 16,
    /// 自定义数值，用于与 RSI 进行比较（stock_field1不支持此字段）
    Value = 17,
}
/// 形态技术指标属性
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PatternField {
    /// 未知
    Unknown = 0,
    /// MA多头排列（连续两天MA5>MA10>MA20>MA30>MA60，且当日收盘价大于前一天收盘价）
    MaAlignmentLong = 1,
    /// MA空头排列（连续两天MA5 <MA10 <MA20 <MA30 <MA60，且当日收盘价小于前一天收盘价）
    MaAlignmentShort = 2,
    /// EMA多头排列（连续两天EMA5>EMA10>EMA20>EMA30>EMA60，且当日收盘价大于前一天收盘价）
    EmaAlignmentLong = 3,
    /// EMA空头排列（连续两天EMA5 <EMA10 <EMA20 <EMA30 <EMA60，且当日收盘价小于前一天收盘价）
    EmaAlignmentShort = 4,
    /// RSI低位金叉（50以下，短线RSI上穿长线RSI（前一日短线RSI小于长线RSI，当日短线RSI大于长线RSI）） 
    RsiGoldCrossLow = 5,
    /// RSI高位死叉（50以上，短线RSI下穿长线RSI（前一日短线RSI大于长线RSI，当日短线RSI小于长线RSI）） 
    RsiDeathCrossHigh = 6,
    /// RSI顶背离（相邻的两个K线波峰，后面的波峰对应的CLOSE>前面的波峰对应的CLOSE，后面波峰的RSI12值 <前面波峰的RSI12值）
    RsiTopDivergence = 7,
    /// RSI底背离（相邻的两个K线波谷，后面的波谷对应的CLOSE <前面的波谷对应的CLOSE，后面波谷的RSI12值>前面波谷的RSI12值） 
    RsiBottomDivergence = 8,
    /// KDJ低位金叉（KDJ的值都小于或等于30，且前一日K,J值分别小于D值，当日K,J值分别大于D值）
    KdjGoldCrossLow = 9,
    /// KDJ高位死叉（KDJ的值都大于或等于70，且前一日K,J值分别大于D值，当日K,J值分别小于D值）
    KdjDeathCrossHigh = 10,
    /// KDJ顶背离（相邻的两个K线波峰，后面的波峰对应的CLOSE>前面的波峰对应的CLOSE，后面波峰的J值 <前面波峰的J值）
    KdjTopDivergence = 11,
    /// KDJ底背离（相邻的两个K线波谷，后面的波谷对应的CLOSE <前面的波谷对应的CLOSE，后面波谷的J值>前面波谷的J值）
    KdjBottomDivergence = 12,
    /// MACD低位金叉（DIFF上穿DEA（前一日DIFF小于DEA，当日DIFF大于DEA））
    MacdGoldCrossLow = 13,
    /// MACD高位死叉（DIFF下穿DEA（前一日DIFF大于DEA，当日DIFF小于DEA））
    MacdDeathCrossHigh = 14,
    /// MACD顶背离（相邻的两个K线波峰，后面的波峰对应的CLOSE>前面的波峰对应的CLOSE，后面波峰的macd值 <前面波峰的macd值）
    MacdTopDivergence = 15,
    /// MACD底背离（相邻的两个K线波谷，后面的波谷对应的CLOSE <前面的波谷对应的CLOSE，后面波谷的macd值>前面波谷的macd值）
    MacdBottomDivergence = 16,
    /// BOLL突破上轨（前一日股价低于上轨值，当日股价大于上轨值） 
    BollBreakUpper = 17,
    /// BOLL突破下轨（前一日股价高于下轨值，当日股价小于下轨值）
    BollLower = 18,
    /// BOLL向上破中轨（前一日股价低于中轨值，当日股价大于中轨值）
    BollCrossMiddleUp = 19,
    /// BOLL向下破中轨（前一日股价大于中轨值，当日股价小于中轨值）
    BollCrossMiddleDown = 20,
}
/// 财务时间周期
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FinancialQuarter {
    /// 未知
    Unknown = 0,
    /// 年报
    Annual = 1,
    /// 一季报
    FirstQuarter = 2,
    /// 中报
    Interim = 3,
    /// 三季报
    ThirdQuarter = 4,
    /// 最近季报
    MostRecentQuarter = 5,
}
/// 相对位置比较
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RelativePosition {
    /// 未知
    Unknown = 0,
    /// 大于，first位于second的上方
    More = 1,
    /// 小于，first位于second的下方
    Less = 2,
    /// 升穿，first从下往上穿second
    CrossUp = 3,
    /// 跌穿，first从上往下穿second
    CrossDown = 4,
}
/// 排序方向
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SortDir {
    /// 不排序
    No = 0,
    /// 升序
    Ascend = 1,
    /// 降序
    Descend = 2,
}
