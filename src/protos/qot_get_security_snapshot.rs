#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///股票
    #[prost(message, repeated, tag="1")]
    pub security_list: ::prost::alloc::vec::Vec<super::qot_common::Security>,
}
/// 正股类型额外数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquitySnapshotExData {
    /// 发行股本,即总股本
    #[prost(int64, required, tag="1")]
    pub issued_shares: i64,
    /// 总市值 =总股本*当前价格（单位：元）
    #[prost(double, required, tag="2")]
    pub issued_market_val: f64,
    /// 资产净值
    #[prost(double, required, tag="3")]
    pub net_asset: f64,
    /// 盈利（亏损）
    #[prost(double, required, tag="4")]
    pub net_profit: f64,
    /// 每股盈利
    #[prost(double, required, tag="5")]
    pub earnings_pershare: f64,
    /// 流通股本
    #[prost(int64, required, tag="6")]
    pub outstanding_shares: i64,
    /// 流通市值 =流通股本*当前价格（单位：元）
    #[prost(double, required, tag="7")]
    pub outstanding_market_val: f64,
    /// 每股净资产
    #[prost(double, required, tag="8")]
    pub net_asset_pershare: f64,
    /// 收益率（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, required, tag="9")]
    pub ey_rate: f64,
    /// 市盈率
    #[prost(double, required, tag="10")]
    pub pe_rate: f64,
    /// 市净率
    #[prost(double, required, tag="11")]
    pub pb_rate: f64,
    /// 市盈率TTM
    #[prost(double, required, tag="12")]
    pub pe_ttm_rate: f64,
    /// 股息TTM，派息
    #[prost(double, optional, tag="13")]
    pub dividend_ttm: ::core::option::Option<f64>,
    /// 股息率TTM（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, optional, tag="14")]
    pub dividend_ratio_ttm: ::core::option::Option<f64>,
    /// 股息LFY，上一年度派息
    #[prost(double, optional, tag="15")]
    pub dividend_lfy: ::core::option::Option<f64>,
    /// 股息率LFY（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, optional, tag="16")]
    pub dividend_lfy_ratio: ::core::option::Option<f64>,
}
/// 窝轮类型额外数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WarrantSnapshotExData {
    ///换股比率
    #[prost(double, required, tag="1")]
    pub conversion_rate: f64,
    ///Qot_Common.WarrantType,窝轮类型
    #[prost(int32, required, tag="2")]
    pub warrant_type: i32,
    ///行使价
    #[prost(double, required, tag="3")]
    pub strike_price: f64,
    ///到期日时间字符串
    #[prost(string, required, tag="4")]
    pub maturity_time: ::prost::alloc::string::String,
    ///最后交易日时间字符串
    #[prost(string, required, tag="5")]
    pub end_trade_time: ::prost::alloc::string::String,
    ///所属正股 
    #[prost(message, required, tag="6")]
    pub owner: super::qot_common::Security,
    ///收回价,仅牛熊证支持该字段
    #[prost(double, required, tag="7")]
    pub recovery_price: f64,
    ///街货量
    #[prost(int64, required, tag="8")]
    pub street_volumn: i64,
    ///发行量
    #[prost(int64, required, tag="9")]
    pub issue_volumn: i64,
    ///街货占比（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, required, tag="10")]
    pub street_rate: f64,
    ///对冲值,仅认购认沽支持该字段
    #[prost(double, required, tag="11")]
    pub delta: f64,
    ///引申波幅,仅认购认沽支持该字段
    #[prost(double, required, tag="12")]
    pub implied_volatility: f64,
    ///溢价（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, required, tag="13")]
    pub premium: f64,
    ///到期日时间戳
    #[prost(double, optional, tag="14")]
    pub maturity_timestamp: ::core::option::Option<f64>,
    ///最后交易日时间戳
    #[prost(double, optional, tag="15")]
    pub end_trade_timestamp: ::core::option::Option<f64>,
    /// 杠杆比率（倍）
    #[prost(double, optional, tag="16")]
    pub leverage: ::core::option::Option<f64>,
    /// 价内/价外（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, optional, tag="17")]
    pub ipop: ::core::option::Option<f64>,
    /// 打和点
    #[prost(double, optional, tag="18")]
    pub break_even_point: ::core::option::Option<f64>,
    /// 换股价
    #[prost(double, optional, tag="19")]
    pub conversion_price: ::core::option::Option<f64>,
    /// 正股距收回价（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, optional, tag="20")]
    pub price_recovery_ratio: ::core::option::Option<f64>,
    /// 综合评分
    #[prost(double, optional, tag="21")]
    pub score: ::core::option::Option<f64>,
    ///上限价，仅界内证支持该字段
    #[prost(double, optional, tag="22")]
    pub upper_strike_price: ::core::option::Option<f64>,
    ///下限价，仅界内证支持该字段
    #[prost(double, optional, tag="23")]
    pub lower_strike_price: ::core::option::Option<f64>,
    ///Qot_Common.PriceType, 界内界外，仅界内证支持该字段
    #[prost(int32, optional, tag="24")]
    pub in_line_price_status: ::core::option::Option<i32>,
    ///发行人代码
    #[prost(string, optional, tag="25")]
    pub issuer_code: ::core::option::Option<::prost::alloc::string::String>,
}
/// 期权类型额外数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionSnapshotExData {
    ///Qot_Common.OptionType,期权
    #[prost(int32, required, tag="1")]
    pub r#type: i32,
    ///标的股
    #[prost(message, required, tag="2")]
    pub owner: super::qot_common::Security,
    ///行权日
    #[prost(string, required, tag="3")]
    pub strike_time: ::prost::alloc::string::String,
    ///行权价
    #[prost(double, required, tag="4")]
    pub strike_price: f64,
    ///每份合约数(整型数据)
    #[prost(int32, required, tag="5")]
    pub contract_size: i32,
    ///每份合约数（浮点型数据）
    #[prost(double, optional, tag="22")]
    pub contract_size_float: ::core::option::Option<f64>,
    ///未平仓合约数
    #[prost(int32, required, tag="6")]
    pub open_interest: i32,
    ///隐含波动率（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, required, tag="7")]
    pub implied_volatility: f64,
    ///溢价（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, required, tag="8")]
    pub premium: f64,
    ///希腊值 Delta
    #[prost(double, required, tag="9")]
    pub delta: f64,
    ///希腊值 Gamma
    #[prost(double, required, tag="10")]
    pub gamma: f64,
    ///希腊值 Vega
    #[prost(double, required, tag="11")]
    pub vega: f64,
    ///希腊值 Theta
    #[prost(double, required, tag="12")]
    pub theta: f64,
    ///希腊值 Rho
    #[prost(double, required, tag="13")]
    pub rho: f64,
    ///行权日时间戳			
    #[prost(double, optional, tag="14")]
    pub strike_timestamp: ::core::option::Option<f64>,
    ///Qot_Common.IndexOptionType，指数期权类型
    #[prost(int32, optional, tag="15")]
    pub index_option_type: ::core::option::Option<i32>,
    ///净未平仓合约数，仅港股期权适用	
    #[prost(int32, optional, tag="16")]
    pub net_open_interest: ::core::option::Option<i32>,
    ///距离到期日天数，负数表示已过期
    #[prost(int32, optional, tag="17")]
    pub expiry_date_distance: ::core::option::Option<i32>,
    ///合约名义金额，仅港股期权适用
    #[prost(double, optional, tag="18")]
    pub contract_nominal_value: ::core::option::Option<f64>,
    ///相等正股手数，指数期权无该字段，仅港股期权适用
    #[prost(double, optional, tag="19")]
    pub owner_lot_multiplier: ::core::option::Option<f64>,
    ///Qot_Common.OptionAreaType，期权类型（按行权时间）
    #[prost(int32, optional, tag="20")]
    pub option_area_type: ::core::option::Option<i32>,
    ///合约乘数
    #[prost(double, optional, tag="21")]
    pub contract_multiplier: ::core::option::Option<f64>,
}
/// 指数类型额外数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexSnapshotExData {
    /// 上涨支数
    #[prost(int32, required, tag="1")]
    pub raise_count: i32,
    /// 下跌支数
    #[prost(int32, required, tag="2")]
    pub fall_count: i32,
    /// 平盘支数
    #[prost(int32, required, tag="3")]
    pub equal_count: i32,
}
/// 板块类型额外数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlateSnapshotExData {
    /// 上涨支数
    #[prost(int32, required, tag="1")]
    pub raise_count: i32,
    /// 下跌支数
    #[prost(int32, required, tag="2")]
    pub fall_count: i32,
    /// 平盘支数
    #[prost(int32, required, tag="3")]
    pub equal_count: i32,
}
///期货类型额外数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FutureSnapshotExData {
    ///昨结
    #[prost(double, required, tag="1")]
    pub last_settle_price: f64,
    ///持仓量
    #[prost(int32, required, tag="2")]
    pub position: i32,
    ///日增仓
    #[prost(int32, required, tag="3")]
    pub position_change: i32,
    ///最后交易日，只有非主连期货合约才有该字段
    #[prost(string, required, tag="4")]
    pub last_trade_time: ::prost::alloc::string::String,
    ///最后交易日时间戳，只有非主连期货合约才有该字段
    #[prost(double, optional, tag="5")]
    pub last_trade_timestamp: ::core::option::Option<f64>,
    ///是否主连合约
    #[prost(bool, required, tag="6")]
    pub is_main_contract: bool,
}
///基金类型额外数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrustSnapshotExData {
    ///股息率（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, required, tag="1")]
    pub dividend_yield: f64,
    ///资产规模（单位：元）
    #[prost(double, required, tag="2")]
    pub aum: f64,
    ///总发行量
    #[prost(int64, required, tag="3")]
    pub outstanding_units: i64,
    ///单位净值
    #[prost(double, required, tag="4")]
    pub net_asset_value: f64,
    ///溢价（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, required, tag="5")]
    pub premium: f64,
    ///Qot_Common.AssetClass，资产类别
    #[prost(int32, required, tag="6")]
    pub asset_class: i32,
}
///基本快照数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotBasicData {
    ///股票
    #[prost(message, required, tag="1")]
    pub security: super::qot_common::Security,
    ///Qot_Common.SecurityType,股票类型
    #[prost(int32, required, tag="2")]
    pub r#type: i32,
    ///是否停牌
    #[prost(bool, required, tag="3")]
    pub is_suspend: bool,
    ///上市时间字符串
    #[prost(string, required, tag="4")]
    pub list_time: ::prost::alloc::string::String,
    ///每手数量
    #[prost(int32, required, tag="5")]
    pub lot_size: i32,
    ///价差
    #[prost(double, required, tag="6")]
    pub price_spread: f64,
    ///更新时间字符串
    #[prost(string, required, tag="7")]
    pub update_time: ::prost::alloc::string::String,
    ///最高价
    #[prost(double, required, tag="8")]
    pub high_price: f64,
    ///开盘价
    #[prost(double, required, tag="9")]
    pub open_price: f64,
    ///最低价
    #[prost(double, required, tag="10")]
    pub low_price: f64,
    ///昨收价
    #[prost(double, required, tag="11")]
    pub last_close_price: f64,
    ///最新价
    #[prost(double, required, tag="12")]
    pub cur_price: f64,
    ///成交量
    #[prost(int64, required, tag="13")]
    pub volume: i64,
    ///成交额
    #[prost(double, required, tag="14")]
    pub turnover: f64,
    ///换手率（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, required, tag="15")]
    pub turnover_rate: f64,
    ///上市时间戳
    #[prost(double, optional, tag="16")]
    pub list_timestamp: ::core::option::Option<f64>,
    ///更新时间戳
    #[prost(double, optional, tag="17")]
    pub update_timestamp: ::core::option::Option<f64>,
    ///卖价
    #[prost(double, optional, tag="18")]
    pub ask_price: ::core::option::Option<f64>,
    ///买价
    #[prost(double, optional, tag="19")]
    pub bid_price: ::core::option::Option<f64>,
    ///卖量
    #[prost(int64, optional, tag="20")]
    pub ask_vol: ::core::option::Option<i64>,
    ///买量
    #[prost(int64, optional, tag="21")]
    pub bid_vol: ::core::option::Option<i64>,
    /// 是否可融资，如果为true，后两个字段才有意义
    #[prost(bool, optional, tag="22")]
    pub enable_margin: ::core::option::Option<bool>,
    /// 股票抵押率（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, optional, tag="23")]
    pub mortgage_ratio: ::core::option::Option<f64>,
    /// 融资初始保证金率（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, optional, tag="24")]
    pub long_margin_initial_ratio: ::core::option::Option<f64>,
    /// 是否可卖空，如果为true，后三个字段才有意义
    #[prost(bool, optional, tag="25")]
    pub enable_short_sell: ::core::option::Option<bool>,
    /// 卖空参考利率（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, optional, tag="26")]
    pub short_sell_rate: ::core::option::Option<f64>,
    /// 剩余可卖空数量（股）
    #[prost(int64, optional, tag="27")]
    pub short_available_volume: ::core::option::Option<i64>,
    /// 卖空（融券）初始保证金率（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, optional, tag="28")]
    pub short_margin_initial_ratio: ::core::option::Option<f64>,
    /// 振幅（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, optional, tag="29")]
    pub amplitude: ::core::option::Option<f64>,
    /// 平均价
    #[prost(double, optional, tag="30")]
    pub avg_price: ::core::option::Option<f64>,
    /// 委比（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, optional, tag="31")]
    pub bid_ask_ratio: ::core::option::Option<f64>,
    /// 量比
    #[prost(double, optional, tag="32")]
    pub volume_ratio: ::core::option::Option<f64>,
    /// 52周最高价
    #[prost(double, optional, tag="33")]
    pub highest52_weeks_price: ::core::option::Option<f64>,
    /// 52周最低价
    #[prost(double, optional, tag="34")]
    pub lowest52_weeks_price: ::core::option::Option<f64>,
    /// 历史最高价
    #[prost(double, optional, tag="35")]
    pub highest_history_price: ::core::option::Option<f64>,
    /// 历史最低价
    #[prost(double, optional, tag="36")]
    pub lowest_history_price: ::core::option::Option<f64>,
    ///Qot_Common::PreAfterMarketData 盘前数据
    #[prost(message, optional, tag="37")]
    pub pre_market: ::core::option::Option<super::qot_common::PreAfterMarketData>,
    ///Qot_Common::PreAfterMarketData 盘后数据
    #[prost(message, optional, tag="38")]
    pub after_market: ::core::option::Option<super::qot_common::PreAfterMarketData>,
    ///Qot_Common::SecurityStatus 股票状态
    #[prost(int32, optional, tag="39")]
    pub sec_status: ::core::option::Option<i32>,
    ///5分钟收盘价
    #[prost(double, optional, tag="40")]
    pub close_price5_minute: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Snapshot {
    ///快照基本数据
    #[prost(message, required, tag="1")]
    pub basic: SnapshotBasicData,
    ///正股快照额外数据
    #[prost(message, optional, tag="2")]
    pub equity_ex_data: ::core::option::Option<EquitySnapshotExData>,
    ///窝轮快照额外数据
    #[prost(message, optional, tag="3")]
    pub warrant_ex_data: ::core::option::Option<WarrantSnapshotExData>,
    ///期权快照额外数据
    #[prost(message, optional, tag="4")]
    pub option_ex_data: ::core::option::Option<OptionSnapshotExData>,
    ///指数快照额外数据
    #[prost(message, optional, tag="5")]
    pub index_ex_data: ::core::option::Option<IndexSnapshotExData>,
    ///板块快照额外数据
    #[prost(message, optional, tag="6")]
    pub plate_ex_data: ::core::option::Option<PlateSnapshotExData>,
    ///期货类型额外数据
    #[prost(message, optional, tag="7")]
    pub future_ex_data: ::core::option::Option<FutureSnapshotExData>,
    ///基金类型额外数据
    #[prost(message, optional, tag="8")]
    pub trust_ex_data: ::core::option::Option<TrustSnapshotExData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///股票快照
    #[prost(message, repeated, tag="1")]
    pub snapshot_list: ::prost::alloc::vec::Vec<Snapshot>,
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
