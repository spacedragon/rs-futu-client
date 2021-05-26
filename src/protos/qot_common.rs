///两个字段确定一支股票
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Security {
    ///QotMarket,股票市场
    #[prost(int32, required, tag="1")]
    pub market: i32,
    ///股票代码
    #[prost(string, required, tag="2")]
    pub code: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KLine {
    ///时间戳字符串
    #[prost(string, required, tag="1")]
    pub time: ::prost::alloc::string::String,
    ///是否是空内容的点,若为ture则只有时间信息
    #[prost(bool, required, tag="2")]
    pub is_blank: bool,
    ///最高价
    #[prost(double, optional, tag="3")]
    pub high_price: ::core::option::Option<f64>,
    ///开盘价
    #[prost(double, optional, tag="4")]
    pub open_price: ::core::option::Option<f64>,
    ///最低价
    #[prost(double, optional, tag="5")]
    pub low_price: ::core::option::Option<f64>,
    ///收盘价
    #[prost(double, optional, tag="6")]
    pub close_price: ::core::option::Option<f64>,
    ///昨收价
    #[prost(double, optional, tag="7")]
    pub last_close_price: ::core::option::Option<f64>,
    ///成交量
    #[prost(int64, optional, tag="8")]
    pub volume: ::core::option::Option<i64>,
    ///成交额
    #[prost(double, optional, tag="9")]
    pub turnover: ::core::option::Option<f64>,
    ///换手率（该字段为百分比字段，展示为小数表示）
    #[prost(double, optional, tag="10")]
    pub turnover_rate: ::core::option::Option<f64>,
    ///市盈率
    #[prost(double, optional, tag="11")]
    pub pe: ::core::option::Option<f64>,
    ///涨跌幅（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, optional, tag="12")]
    pub change_rate: ::core::option::Option<f64>,
    ///时间戳
    #[prost(double, optional, tag="13")]
    pub timestamp: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionBasicQotExData {
    ///行权价
    #[prost(double, required, tag="1")]
    pub strike_price: f64,
    ///每份合约数(整型数据)
    #[prost(int32, required, tag="2")]
    pub contract_size: i32,
    ///每份合约数（浮点型数据）
    #[prost(double, optional, tag="17")]
    pub contract_size_float: ::core::option::Option<f64>,
    ///未平仓合约数
    #[prost(int32, required, tag="3")]
    pub open_interest: i32,
    ///隐含波动率（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, required, tag="4")]
    pub implied_volatility: f64,
    ///溢价（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, required, tag="5")]
    pub premium: f64,
    ///希腊值 Delta
    #[prost(double, required, tag="6")]
    pub delta: f64,
    ///希腊值 Gamma
    #[prost(double, required, tag="7")]
    pub gamma: f64,
    ///希腊值 Vega
    #[prost(double, required, tag="8")]
    pub vega: f64,
    ///希腊值 Theta
    #[prost(double, required, tag="9")]
    pub theta: f64,
    ///希腊值 Rho
    #[prost(double, required, tag="10")]
    pub rho: f64,
    ///净未平仓合约数，仅港股期权适用
    #[prost(int32, optional, tag="11")]
    pub net_open_interest: ::core::option::Option<i32>,
    ///距离到期日天数，负数表示已过期
    #[prost(int32, optional, tag="12")]
    pub expiry_date_distance: ::core::option::Option<i32>,
    ///合约名义金额，仅港股期权适用
    #[prost(double, optional, tag="13")]
    pub contract_nominal_value: ::core::option::Option<f64>,
    ///相等正股手数，指数期权无该字段，仅港股期权适用
    #[prost(double, optional, tag="14")]
    pub owner_lot_multiplier: ::core::option::Option<f64>,
    ///OptionAreaType，期权类型（按行权时间）
    #[prost(int32, optional, tag="15")]
    pub option_area_type: ::core::option::Option<i32>,
    ///合约乘数
    #[prost(double, optional, tag="16")]
    pub contract_multiplier: ::core::option::Option<f64>,
    ///IndexOptionType，指数期权类型
    #[prost(int32, optional, tag="18")]
    pub index_option_type: ::core::option::Option<i32>,
}
///美股支持盘前盘后数据
///科创板仅支持盘后数据：成交量，成交额
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreAfterMarketData {
    /// 盘前或盘后 - 价格
    #[prost(double, optional, tag="1")]
    pub price: ::core::option::Option<f64>,
    /// 盘前或盘后 - 最高价
    #[prost(double, optional, tag="2")]
    pub high_price: ::core::option::Option<f64>,
    /// 盘前或盘后 - 最低价
    #[prost(double, optional, tag="3")]
    pub low_price: ::core::option::Option<f64>,
    /// 盘前或盘后 - 成交量
    #[prost(int64, optional, tag="4")]
    pub volume: ::core::option::Option<i64>,
    /// 盘前或盘后 - 成交额
    #[prost(double, optional, tag="5")]
    pub turnover: ::core::option::Option<f64>,
    /// 盘前或盘后 - 涨跌额
    #[prost(double, optional, tag="6")]
    pub change_val: ::core::option::Option<f64>,
    /// 盘前或盘后 - 涨跌幅（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, optional, tag="7")]
    pub change_rate: ::core::option::Option<f64>,
    /// 盘前或盘后 - 振幅（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, optional, tag="8")]
    pub amplitude: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FutureBasicQotExData {
    ///昨结
    #[prost(double, required, tag="1")]
    pub last_settle_price: f64,
    ///持仓量
    #[prost(int32, required, tag="2")]
    pub position: i32,
    ///日增仓
    #[prost(int32, required, tag="3")]
    pub position_change: i32,
    ///距离到期日天数
    #[prost(int32, optional, tag="4")]
    pub expiry_date_distance: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WarrantBasicQotExData {
    ///对冲值,仅认购认沽支持该字段
    #[prost(double, optional, tag="1")]
    pub delta: ::core::option::Option<f64>,
    ///引申波幅,仅认购认沽支持该字段
    #[prost(double, optional, tag="2")]
    pub implied_volatility: ::core::option::Option<f64>,
    ///溢价（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, required, tag="3")]
    pub premium: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasicQot {
    ///股票
    #[prost(message, required, tag="1")]
    pub security: Security,
    ///是否停牌
    #[prost(bool, required, tag="2")]
    pub is_suspended: bool,
    ///上市日期字符串
    #[prost(string, required, tag="3")]
    pub list_time: ::prost::alloc::string::String,
    ///价差
    #[prost(double, required, tag="4")]
    pub price_spread: f64,
    ///最新价的更新时间字符串，对其他字段不适用
    #[prost(string, required, tag="5")]
    pub update_time: ::prost::alloc::string::String,
    ///最高价
    #[prost(double, required, tag="6")]
    pub high_price: f64,
    ///开盘价
    #[prost(double, required, tag="7")]
    pub open_price: f64,
    ///最低价
    #[prost(double, required, tag="8")]
    pub low_price: f64,
    ///最新价
    #[prost(double, required, tag="9")]
    pub cur_price: f64,
    ///昨收价
    #[prost(double, required, tag="10")]
    pub last_close_price: f64,
    ///成交量
    #[prost(int64, required, tag="11")]
    pub volume: i64,
    ///成交额
    #[prost(double, required, tag="12")]
    pub turnover: f64,
    ///换手率（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, required, tag="13")]
    pub turnover_rate: f64,
    ///振幅（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, required, tag="14")]
    pub amplitude: f64,
    ///DarkStatus, 暗盘交易状态	
    #[prost(int32, optional, tag="15")]
    pub dark_status: ::core::option::Option<i32>,
    ///期权特有字段
    #[prost(message, optional, tag="16")]
    pub option_ex_data: ::core::option::Option<OptionBasicQotExData>,
    ///上市日期时间戳
    #[prost(double, optional, tag="17")]
    pub list_timestamp: ::core::option::Option<f64>,
    ///最新价的更新时间戳，对其他字段不适用
    #[prost(double, optional, tag="18")]
    pub update_timestamp: ::core::option::Option<f64>,
    ///盘前数据
    #[prost(message, optional, tag="19")]
    pub pre_market: ::core::option::Option<PreAfterMarketData>,
    ///盘后数据
    #[prost(message, optional, tag="20")]
    pub after_market: ::core::option::Option<PreAfterMarketData>,
    ///SecurityStatus, 股票状态
    #[prost(int32, optional, tag="21")]
    pub sec_status: ::core::option::Option<i32>,
    ///期货特有字段
    #[prost(message, optional, tag="22")]
    pub future_ex_data: ::core::option::Option<FutureBasicQotExData>,
    ///窝轮特有字段
    #[prost(message, optional, tag="23")]
    pub warrant_ex_data: ::core::option::Option<WarrantBasicQotExData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeShare {
    ///时间字符串
    #[prost(string, required, tag="1")]
    pub time: ::prost::alloc::string::String,
    ///距离0点过了多少分钟
    #[prost(int32, required, tag="2")]
    pub minute: i32,
    ///是否是空内容的点,若为ture则只有时间信息
    #[prost(bool, required, tag="3")]
    pub is_blank: bool,
    ///当前价
    #[prost(double, optional, tag="4")]
    pub price: ::core::option::Option<f64>,
    ///昨收价
    #[prost(double, optional, tag="5")]
    pub last_close_price: ::core::option::Option<f64>,
    ///均价
    #[prost(double, optional, tag="6")]
    pub avg_price: ::core::option::Option<f64>,
    ///成交量
    #[prost(int64, optional, tag="7")]
    pub volume: ::core::option::Option<i64>,
    ///成交额
    #[prost(double, optional, tag="8")]
    pub turnover: ::core::option::Option<f64>,
    ///时间戳
    #[prost(double, optional, tag="9")]
    pub timestamp: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityStaticBasic {
    ///股票
    #[prost(message, required, tag="1")]
    pub security: Security,
    ///股票ID
    #[prost(int64, required, tag="2")]
    pub id: i64,
    ///每手数量,期权以及期货类型表示合约乘数
    #[prost(int32, required, tag="3")]
    pub lot_size: i32,
    ///Qot_Common.SecurityType,股票类型
    #[prost(int32, required, tag="4")]
    pub sec_type: i32,
    ///股票名字
    #[prost(string, required, tag="5")]
    pub name: ::prost::alloc::string::String,
    ///上市时间字符串
    #[prost(string, required, tag="6")]
    pub list_time: ::prost::alloc::string::String,
    ///是否退市
    #[prost(bool, optional, tag="7")]
    pub delisting: ::core::option::Option<bool>,
    ///上市时间戳
    #[prost(double, optional, tag="8")]
    pub list_timestamp: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WarrantStaticExData {
    ///Qot_Common.WarrantType,窝轮类型
    #[prost(int32, required, tag="1")]
    pub r#type: i32,
    ///所属正股
    #[prost(message, required, tag="2")]
    pub owner: Security,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionStaticExData {
    ///Qot_Common.OptionType,期权
    #[prost(int32, required, tag="1")]
    pub r#type: i32,
    ///标的股
    #[prost(message, required, tag="2")]
    pub owner: Security,
    ///行权日
    #[prost(string, required, tag="3")]
    pub strike_time: ::prost::alloc::string::String,
    ///行权价
    #[prost(double, required, tag="4")]
    pub strike_price: f64,
    ///是否停牌
    #[prost(bool, required, tag="5")]
    pub suspend: bool,
    ///发行市场名字
    #[prost(string, required, tag="6")]
    pub market: ::prost::alloc::string::String,
    ///行权日时间戳
    #[prost(double, optional, tag="7")]
    pub strike_timestamp: ::core::option::Option<f64>,
    ///Qot_Common.IndexOptionType, 指数期权的类型，仅在指数期权有效
    #[prost(int32, optional, tag="8")]
    pub index_option_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FutureStaticExData {
    ///最后交易日，只有非主连期货合约才有该字段
    #[prost(string, required, tag="1")]
    pub last_trade_time: ::prost::alloc::string::String,
    ///最后交易日时间戳，只有非主连期货合约才有该字段
    #[prost(double, optional, tag="2")]
    pub last_trade_timestamp: ::core::option::Option<f64>,
    ///是否主连合约
    #[prost(bool, required, tag="3")]
    pub is_main_contract: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityStaticInfo {
    ///基本股票静态信息
    #[prost(message, required, tag="1")]
    pub basic: SecurityStaticBasic,
    ///窝轮额外股票静态信息
    #[prost(message, optional, tag="2")]
    pub warrant_ex_data: ::core::option::Option<WarrantStaticExData>,
    ///期权额外股票静态信息
    #[prost(message, optional, tag="3")]
    pub option_ex_data: ::core::option::Option<OptionStaticExData>,
    ///期货额外股票静态信息
    #[prost(message, optional, tag="4")]
    pub future_ex_data: ::core::option::Option<FutureStaticExData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Broker {
    ///经纪ID
    #[prost(int64, required, tag="1")]
    pub id: i64,
    ///经纪名称
    #[prost(string, required, tag="2")]
    pub name: ::prost::alloc::string::String,
    ///经纪档位
    #[prost(int32, required, tag="3")]
    pub pos: i32,
    ///以下为SF行情特有字段
    ///
    ///交易所订单ID，与交易接口返回的订单ID并不一样
    #[prost(int64, optional, tag="4")]
    pub order_id: ::core::option::Option<i64>,
    ///订单股数
    #[prost(int64, optional, tag="5")]
    pub volume: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ticker {
    ///时间字符串
    #[prost(string, required, tag="1")]
    pub time: ::prost::alloc::string::String,
    /// 唯一标识
    #[prost(int64, required, tag="2")]
    pub sequence: i64,
    ///TickerDirection, 买卖方向
    #[prost(int32, required, tag="3")]
    pub dir: i32,
    ///价格
    #[prost(double, required, tag="4")]
    pub price: f64,
    ///成交量
    #[prost(int64, required, tag="5")]
    pub volume: i64,
    ///成交额
    #[prost(double, required, tag="6")]
    pub turnover: f64,
    ///收到推送数据的本地时间戳，用于定位延迟
    #[prost(double, optional, tag="7")]
    pub recv_time: ::core::option::Option<f64>,
    ///TickerType, 逐笔类型
    #[prost(int32, optional, tag="8")]
    pub r#type: ::core::option::Option<i32>,
    ///逐笔类型符号
    #[prost(int32, optional, tag="9")]
    pub type_sign: ::core::option::Option<i32>,
    ///用于区分推送情况
    #[prost(int32, optional, tag="10")]
    pub push_data_type: ::core::option::Option<i32>,
    ///时间戳
    #[prost(double, optional, tag="11")]
    pub timestamp: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBookDetail {
    ///交易所订单ID，与交易接口返回的订单ID并不一样
    #[prost(int64, required, tag="1")]
    pub order_id: i64,
    ///订单股数
    #[prost(int64, required, tag="2")]
    pub volume: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBook {
    ///委托价格
    #[prost(double, required, tag="1")]
    pub price: f64,
    ///委托数量
    #[prost(int64, required, tag="2")]
    pub volume: i64,
    ///委托订单个数
    #[prost(int32, required, tag="3")]
    pub oreder_count: i32,
    ///订单信息，SF行情特有
    #[prost(message, repeated, tag="4")]
    pub detail_list: ::prost::alloc::vec::Vec<OrderBookDetail>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderDetail {
    ///委托订单个数
    #[prost(int32, required, tag="1")]
    pub order_count: i32,
    ///每笔委托的委托量，注意：当前只会返回最多前50笔委托的委托数量
    #[prost(double, repeated, packed="false", tag="2")]
    pub order_vol: ::prost::alloc::vec::Vec<f64>,
}
///持股变动
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareHoldingChange {
    ///持有者名称（机构名称 或 基金名称 或 高管姓名）
    #[prost(string, required, tag="1")]
    pub holder_name: ::prost::alloc::string::String,
    ///当前持股数量
    #[prost(double, required, tag="2")]
    pub holding_qty: f64,
    ///当前持股百分比（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, required, tag="3")]
    pub holding_ratio: f64,
    ///较上一次变动数量
    #[prost(double, required, tag="4")]
    pub change_qty: f64,
    ///较上一次变动百分比（该字段为百分比字段，默认不展示%，如20实际对应20%。是相对于自身的比例，而不是总的。如总股本1万股，持有100股，持股百分比是1%，卖掉50股，变动比例是50%，而不是0.5%）
    #[prost(double, required, tag="5")]
    pub change_ratio: f64,
    ///发布时间(YYYY-MM-DD HH:MM:SS字符串)
    #[prost(string, required, tag="6")]
    pub time: ::prost::alloc::string::String,
    ///时间戳
    #[prost(double, optional, tag="7")]
    pub timestamp: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubInfo {
    ///Qot_Common.SubType,订阅类型
    #[prost(int32, required, tag="1")]
    pub sub_type: i32,
    ///订阅该类型行情的股票
    #[prost(message, repeated, tag="2")]
    pub security_list: ::prost::alloc::vec::Vec<Security>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnSubInfo {
    ///该连接订阅信息
    #[prost(message, repeated, tag="1")]
    pub sub_info_list: ::prost::alloc::vec::Vec<SubInfo>,
    ///该连接已经使用的订阅额度
    #[prost(int32, required, tag="2")]
    pub used_quota: i32,
    ///用于区分是否是自己连接的数据
    #[prost(bool, required, tag="3")]
    pub is_own_conn_data: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlateInfo {
    ///板块
    #[prost(message, required, tag="1")]
    pub plate: Security,
    ///板块名字
    #[prost(string, required, tag="2")]
    pub name: ::prost::alloc::string::String,
    ///PlateSetType 板块类型, 仅3207（获取股票所属板块）协议返回该字段
    #[prost(int32, optional, tag="3")]
    pub plate_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rehab {
    ///时间字符串
    #[prost(string, required, tag="1")]
    pub time: ::prost::alloc::string::String,
    ///公司行动(CompanyAct)组合标志位,指定某些字段值是否有效
    #[prost(int64, required, tag="2")]
    pub company_act_flag: i64,
    ///前复权因子A
    #[prost(double, required, tag="3")]
    pub fwd_factor_a: f64,
    ///前复权因子B
    #[prost(double, required, tag="4")]
    pub fwd_factor_b: f64,
    ///后复权因子A
    #[prost(double, required, tag="5")]
    pub bwd_factor_a: f64,
    ///后复权因子B
    #[prost(double, required, tag="6")]
    pub bwd_factor_b: f64,
    ///拆股(例如，1拆5，Base为1，Ert为5)
    #[prost(int32, optional, tag="7")]
    pub split_base: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="8")]
    pub split_ert: ::core::option::Option<i32>,
    ///合股(例如，50合1，Base为50，Ert为1)
    #[prost(int32, optional, tag="9")]
    pub join_base: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="10")]
    pub join_ert: ::core::option::Option<i32>,
    ///送股(例如，10送3, Base为10,Ert为3)
    #[prost(int32, optional, tag="11")]
    pub bonus_base: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="12")]
    pub bonus_ert: ::core::option::Option<i32>,
    ///转赠股(例如，10转3, Base为10,Ert为3)
    #[prost(int32, optional, tag="13")]
    pub transfer_base: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="14")]
    pub transfer_ert: ::core::option::Option<i32>,
    ///配股(例如，10送2, 配股价为6.3元, Base为10, Ert为2, Price为6.3)
    #[prost(int32, optional, tag="15")]
    pub allot_base: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="16")]
    pub allot_ert: ::core::option::Option<i32>,
    #[prost(double, optional, tag="17")]
    pub allot_price: ::core::option::Option<f64>,
    ///增发股(例如，10送2, 增发股价为6.3元, Base为10, Ert为2, Price为6.3)
    #[prost(int32, optional, tag="18")]
    pub add_base: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="19")]
    pub add_ert: ::core::option::Option<i32>,
    #[prost(double, optional, tag="20")]
    pub add_price: ::core::option::Option<f64>,
    ///现金分红(例如，每10股派现0.5元,则该字段值为0.05)
    #[prost(double, optional, tag="21")]
    pub dividend: ::core::option::Option<f64>,
    ///特别股息(例如，每10股派特别股息0.5元,则该字段值为0.05)
    #[prost(double, optional, tag="22")]
    pub sp_dividend: ::core::option::Option<f64>,
    ///时间戳
    #[prost(double, optional, tag="23")]
    pub timestamp: ::core::option::Option<f64>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum QotMarket {
    ///未知市场
    Unknown = 0,
    ///香港市场
    HkSecurity = 1,
    ///港期货(已废弃，使用QotMarket_HK_Security即可)
    HkFuture = 2,
    ///美国市场
    UsSecurity = 11,
    ///沪股市场
    CnshSecurity = 21,
    ///深股市场
    CnszSecurity = 22,
    ///新加坡市场
    SgSecurity = 31,
    ///日本市场
    JpSecurity = 41,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SecurityType {
    ///未知
    Unknown = 0,
    ///债券
    Bond = 1,
    ///一揽子权证
    Bwrt = 2,
    ///正股
    Eqty = 3,
    ///信托,基金
    Trust = 4,
    ///窝轮
    Warrant = 5,
    ///指数
    Index = 6,
    ///板块
    Plate = 7,
    ///期权
    Drvt = 8,
    ///板块集
    PlateSet = 9,
    ///期货
    Future = 10,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlateSetType {
    ///所有板块
    All = 0,
    ///行业板块
    Industry = 1,
    ///地域板块,港美股市场的地域分类数据暂为空
    Region = 2,
    ///概念板块
    Concept = 3,
    ///其他板块, 仅用于3207（获取股票所属板块）协议返回,不可作为其他协议的请求参数
    Other = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WarrantType {
    ///未知
    Unknown = 0,
    ///认购
    Buy = 1,
    ///认沽
    Sell = 2,
    ///牛
    Bull = 3,
    ///熊
    Bear = 4,
    ///界内证
    InLine = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OptionType {
    ///未知
    Unknown = 0,
    ///涨
    Call = 1,
    ///跌
    Put = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IndexOptionType {
    ///未知
    Unknown = 0,
    ///正常普通的指数期权
    Normal = 1,
    ///小型指数期权
    Small = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OptionAreaType {
    ///未知
    Unknown = 0,
    ///美式
    American = 1,
    ///欧式
    European = 2,
    ///百慕大
    Bermuda = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum QotMarketState {
    /// 无交易,美股未开盘
    None = 0,
    /// 竞价 
    Auction = 1,
    /// 早盘前等待开盘
    WaitingOpen = 2,
    /// 早盘 
    Morning = 3,
    /// 午间休市 
    Rest = 4,
    /// 午盘 
    Afternoon = 5,
    /// 收盘
    Closed = 6,
    /// 盘前
    PreMarketBegin = 8,
    /// 盘前结束 
    PreMarketEnd = 9,
    /// 盘后
    AfterHoursBegin = 10,
    /// 盘后结束 
    AfterHoursEnd = 11,
    /// 夜市开盘 
    NightOpen = 13,
    /// 夜市收盘 
    NightEnd = 14,
    /// 期货日市开盘 
    FutureDayOpen = 15,
    /// 期货日市休市 
    FutureDayBreak = 16,
    /// 期货日市收盘 
    FutureDayClose = 17,
    /// 期货日市等待开盘 
    FutureDayWaitForOpen = 18,
    /// 盘后竞价,港股市场增加CAS机制对应的市场状态	
    HkCas = 19,
    /// 夜市等待开盘
    FutureNightWait = 20,
    /// 期货下午开盘
    FutureAfternoon = 21,
    ///美国期货新增加状态
    ///
    /// 期货切交易日
    FutureSwitchDate = 22,
    /// 期货开盘
    FutureOpen = 23,
    /// 期货中盘休息
    FutureBreak = 24,
    /// 期货休息后开盘
    FutureBreakOver = 25,
    /// 期货收盘
    FutureClose = 26,
    ///科创板新增状态
    ///
    /// 科创板的盘后撮合时段
    StibAfterHoursWait = 27,
    /// 科创板的盘后交易开始
    StibAfterHoursBegin = 28,
    /// 科创板的盘后交易结束
    StibAfterHoursEnd = 29,
}
///交易日查询市场
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradeDateMarket {
    ///未知
    Unknown = 0,
    ///港股市场
    Hk = 1,
    ///美股市场
    Us = 2,
    ///A股市场
    Cn = 3,
    ///深（沪）股通
    Nt = 4,
    ///港股通（深、沪）
    St = 5,
}
///交易日类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradeDateType {
    ///全天交易
    Whole = 0,
    ///上午交易，下午休市
    Morning = 1,
    ///下午交易，上午休市
    Afternoon = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RehabType {
    ///不复权
    None = 0,
    ///前复权
    Forward = 1,
    ///后复权
    Backward = 2,
}
///枚举值兼容旧协议定义
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KlType {
    ///未知
    Unknown = 0,
    ///1分K
    KlType1min = 1,
    ///日K
    Day = 2,
    ///周K
    Week = 3,
    ///月K	
    Month = 4,
    ///年K
    Year = 5,
    ///5分K
    KlType5min = 6,
    ///15分K
    KlType15min = 7,
    ///30分K
    KlType30min = 8,
    ///60分K		
    KlType60min = 9,
    ///3分K
    KlType3min = 10,
    ///季K
    Quarter = 11,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KlFields {
    ///
    None = 0,
    ///最高价
    High = 1,
    ///开盘价
    Open = 2,
    ///最低价
    Low = 4,
    ///收盘价
    Close = 8,
    ///昨收价
    LastClose = 16,
    ///成交量
    Volume = 32,
    ///成交额
    Turnover = 64,
    ///换手率
    TurnoverRate = 128,
    ///市盈率
    Pe = 256,
    ///涨跌幅
    ChangeRate = 512,
}
///订阅类型
///枚举值兼容旧协议定义
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubType {
    None = 0,
    ///基础报价
    Basic = 1,
    ///摆盘
    OrderBook = 2,
    ///逐笔
    Ticker = 4,
    ///分时
    Rt = 5,
    ///日K
    KlDay = 6,
    ///5分K
    Kl5min = 7,
    ///15分K
    Kl15min = 8,
    ///30分K
    Kl30min = 9,
    ///60分K
    Kl60min = 10,
    ///1分K
    Kl1min = 11,
    ///周K
    KlWeek = 12,
    ///月K
    KlMonth = 13,
    ///经纪队列
    Broker = 14,
    ///季K
    KlQurater = 15,
    ///年K
    KlYear = 16,
    ///3分K
    Kl3min = 17,
    ///委托明细
    OrderDetail = 18,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TickerDirection {
    ///未知
    Unknown = 0,
    ///外盘
    Bid = 1,
    ///内盘
    Ask = 2,
    ///中性盘
    Neutral = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TickerType {
    ///未知
    Unknown = 0,
    ///自动对盘
    Automatch = 1,
    ///开市前成交盘
    Late = 2,
    ///非自动对盘
    NoneAutomatch = 3,
    ///同一证券商自动对盘
    InterAutomatch = 4,
    ///同一证券商非自动对盘
    InterNoneAutomatch = 5,
    ///碎股交易
    OddLot = 6,
    ///竞价交易	
    Auction = 7,
    ///批量交易
    Bulk = 8,
    ///现金交易
    Crash = 9,
    ///跨市场交易
    CrossMarket = 10,
    ///批量卖出
    BulkSold = 11,
    ///离价交易
    FreeOnBoard = 12,
    ///第127条交易（纽交所规则）或第155条交易
    Rule127Or155 = 13,
    ///延迟交易
    Delay = 14,
    ///中央收市价
    MarketCenterClosePrice = 15,
    ///隔日交易
    NextDay = 16,
    ///中央开盘价交易
    MarketCenterOpening = 17,
    ///前参考价
    PriorReferencePrice = 18,
    ///中央开盘价
    MarketCenterOpenPrice = 19,
    ///卖方
    Seller = 20,
    ///T类交易(盘前和盘后交易)
    T = 21,
    ///延长交易时段
    ExtendedTradingHours = 22,
    ///合单交易
    Contingent = 23,
    ///平均价成交
    AvgPrice = 24,
    ///场外售出
    OtcSold = 25,
    ///碎股跨市场交易
    OddLotCrossMarket = 26,
    ///衍生工具定价
    DerivativelyPriced = 27,
    ///再开盘定价
    ReOpeningPriced = 28,
    ///收盘定价
    ClosingPriced = 29,
    ///综合延迟价格
    ComprehensiveDelayPrice = 30,
    ///交易的一方不是香港交易所的成员，属于场外交易
    Overseas = 31,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DarkStatus {
    ///无暗盘交易
    None = 0,
    ///暗盘交易中
    Trading = 1,
    ///暗盘交易结束
    End = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SecurityStatus {
    ///未知
    Unknown = 0,
    ///正常状态
    Normal = 1,
    ///待上市
    Listing = 2,
    ///申购中
    Purchasing = 3,
    ///认购中
    Subscribing = 4,
    ///暗盘开盘前
    BeforeDrakTradeOpening = 5,
    ///暗盘交易中
    DrakTrading = 6,
    ///暗盘已收盘
    DrakTradeEnd = 7,
    ///待开盘
    ToBeOpen = 8,
    ///停牌
    Suspended = 9,
    ///已收回
    Called = 10,
    ///已过最后交易日
    ExpiredLastTradingDate = 11,
    ///已过期
    Expired = 12,
    ///已退市
    Delisted = 13,
    ///公司行动中，交易关闭，转至临时代码交易
    ChangeToTemporaryCode = 14,
    ///临时买卖结束，交易关闭
    TemporaryCodeTradeEnd = 15,
    ///已转板，旧代码交易关闭
    ChangedPlateTradeEnd = 16,
    ///已换代码，旧代码交易关闭
    ChangedCodeTradeEnd = 17,
    ///可恢复性熔断
    RecoverableCircuitBreaker = 18,
    ///不可恢复性熔断
    UnRecoverableCircuitBreaker = 19,
    ///盘后撮合
    AfterCombination = 20,
    ///盘后交易
    AfterTransation = 21,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HolderCategory {
    ///未知
    Unknow = 0,
    ///机构
    Agency = 1,
    ///基金
    Fund = 2,
    ///高管
    SeniorManager = 3,
}
///推送数据的分类，目前只有逐笔在使用
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PushDataType {
    Unknow = 0,
    ///实时推送的数据
    Realtime = 1,
    ///对后台行情连接断开期间拉取补充的数据 最多50个
    ByDisConn = 2,
    ///非实时非连接断开补充数据
    Cache = 3,
}
///窝轮排序
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SortField {
    Unknow = 0,
    ///代码
    Code = 1,
    ///最新价
    CurPrice = 2,
    ///涨跌额
    PriceChangeVal = 3,
    ///涨跌幅%
    ChangeRate = 4,
    ///状态
    Status = 5,
    ///买入价
    BidPrice = 6,
    ///卖出价
    AskPrice = 7,
    ///买量
    BidVol = 8,
    ///卖量
    AskVol = 9,
    ///成交量
    Volume = 10,
    ///成交额
    Turnover = 11,
    ///振幅%
    Amplitude = 30,
    ///以下排序字段只支持用于Qot_GetWarrant协议
    ///
    ///综合评分
    Score = 12,
    ///溢价%
    Premium = 13,
    ///有效杠杆
    EffectiveLeverage = 14,
    ///对冲值,仅认购认沽支持该字段
    Delta = 15,
    ///引伸波幅,仅认购认沽支持该字段
    ImpliedVolatility = 16,
    ///类型
    Type = 17,
    ///行权价
    StrikePrice = 18,
    ///打和点
    BreakEvenPoint = 19,
    ///到期日
    MaturityTime = 20,
    ///上市日期
    ListTime = 21,
    ///最后交易日
    LastTradeTime = 22,
    ///杠杆比率
    Leverage = 23,
    ///价内/价外%
    InOutMoney = 24,
    ///收回价,仅牛熊证支持该字段
    RecoveryPrice = 25,
    /// 换股价
    ChangePrice = 26,
    ///换股比率
    Change = 27,
    ///街货比%
    StreetRate = 28,
    ///街货量
    StreetVol = 29,
    /// 窝轮名称
    WarrantName = 31,
    ///发行人
    Issuer = 32,
    /// 每手
    LotSize = 33,
    ///发行量
    IssueSize = 34,
    ///上限价，仅用于界内证
    UpperStrikePrice = 45,
    ///下限价，仅用于界内证
    LowerStrikePrice = 46,
    ///界内界外，仅用于界内证
    InLinePriceStatus = 47,
    ///以下排序字段只支持用于Qot_GetPlateSecurity协议，并仅支持美股
    ///
    ///盘前最新价
    PreCurPrice = 35,
    ///盘后最新价
    AfterCurPrice = 36,
    ///盘前涨跌额
    PrePriceChangeVal = 37,
    ///盘后涨跌额
    AfterPriceChangeVal = 38,
    ///盘前涨跌幅%
    PreChangeRate = 39,
    ///盘后涨跌幅%
    AfterChangeRate = 40,
    ///盘前振幅%
    PreAmplitude = 41,
    ///盘后振幅%
    AfterAmplitude = 42,
    ///盘前成交额
    PreTurnover = 43,
    ///盘后成交额
    AfterTurnover = 44,
    ///以下排序字段只支持用于Qot_GetPlateSecurity协议，并仅支持期货
    ///
    ///昨结
    LastSettlePrice = 48,
    ///持仓量
    Position = 49,
    ///日增仓
    PositionChange = 50,
}
///窝轮发行人
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Issuer {
    ///未知
    Unknow = 0,
    ///法兴
    Sg = 1,
    ///法巴
    Bp = 2,
    ///瑞信
    Cs = 3,
    ///花旗	
    Ct = 4,
    ///东亚
    Ea = 5,
    ///高盛
    Gs = 6,
    ///汇丰
    Hs = 7,
    ///摩通	
    Jp = 8,
    ///麦银	
    Mb = 9,
    ///渣打
    Sc = 10,
    ///瑞银
    Ub = 11,
    ///中银
    Bi = 12,
    ///德银
    Db = 13,
    ///大和
    Dc = 14,
    ///美林
    Ml = 15,
    ///野村
    Nm = 16,
    ///荷合
    Rb = 17,
    ///苏皇	
    Rs = 18,
    ///巴克莱
    Bc = 19,
    ///海通
    Ht = 20,
    ///瑞通
    Vt = 21,
    ///比联
    Kc = 22,
    ///摩利
    Ms = 23,
    ///国君
    Gj = 24,
}
///窝轮上市日
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IpoPeriod {
    ///未知
    Unknow = 0,
    ///今日上市
    Today = 1,
    ///明日上市
    Tomorrow = 2,
    ///未来一周上市
    Nextweek = 3,
    ///过去一周上市
    Lastweek = 4,
    ///过去一月上市
    Lastmonth = 5,
}
///窝轮价外/内,界内证表示界内界外
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PriceType {
    Unknow = 0,
    ///价外，界内证表示界外
    Outside = 1,
    ///价内，界内证表示界内
    WithIn = 2,
}
///窝轮状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WarrantStatus {
    ///未知
    Unknow = 0,
    ///正常状态
    Normal = 1,
    ///停牌
    Suspend = 2,
    ///终止交易
    StopTrade = 3,
    ///等待上市
    PendingListing = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CompanyAct {
    ///无
    None = 0,
    ///拆股		
    Split = 1,
    ///合股
    Join = 2,
    ///送股
    Bonus = 4,
    ///转赠股
    Transfer = 8,
    ///配股	
    Allot = 16,
    ///增发股
    Add = 32,
    ///现金分红
    Dividend = 64,
    ///特别股息	
    SpDividend = 128,
}
///行情权限
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum QotRight {
    ///未知
    Unknow = 0,
    ///Bmp，无法订阅
    Bmp = 1,
    ///Level1
    Level1 = 2,
    ///Level2
    Level2 = 3,
    ///SF高级行情
    Sf = 4,
    ///无权限
    No = 5,
}
/// 提醒类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PriceReminderType {
    /// 未知
    Unknown = 0,
    /// 价格涨到
    PriceUp = 1,
    /// 价格跌到
    PriceDown = 2,
    /// 日涨幅超（该字段为百分比字段，设置时填 20 表示 20%）
    ChangeRateUp = 3,
    /// 日跌幅超（该字段为百分比字段，设置时填 20 表示 20%）
    ChangeRateDown = 4,
    /// 5 分钟涨幅超（该字段为百分比字段，设置时填 20 表示 20%）
    PriceReminderType5minChangeRateUp = 5,
    /// 5 分钟跌幅超（该字段为百分比字段，设置时填 20 表示 20%）
    PriceReminderType5minChangeRateDown = 6,
    /// 成交量超过
    VolumeUp = 7,
    /// 成交额超过
    TurnoverUp = 8,
    /// 换手率超过（该字段为百分比字段，设置时填 20 表示 20%）
    TurnoverRateUp = 9,
    /// 买一价高于
    BidPriceUp = 10,
    /// 卖一价低于
    AskPriceDown = 11,
    /// 买一量高于    
    BidVolUp = 12,
    /// 卖一量高于
    AskVolUp = 13,
    /// 3 分钟涨幅超（该字段为百分比字段，设置时填 20 表示 20%）
    PriceReminderType3minChangeRateUp = 14,
    /// 3 分钟跌幅超（该字段为百分比字段，设置时填 20 表示 20%）
    PriceReminderType3minChangeRateDown = 15,
}
/// 提醒频率
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PriceReminderFreq {
    /// 未知
    Unknown = 0,
    /// 持续提醒
    Always = 1,
    /// 每日一次
    OnceADay = 2,
    /// 仅提醒一次
    OnlyOnce = 3,
}
/// 资产类别
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AssetClass {
    ///未知
    Unknow = 0,
    ///股票
    Stock = 1,
    ///债券
    Bond = 2,
    ///商品
    Commodity = 3,
    ///货币市场
    CurrencyMarket = 4,
    ///期货
    Future = 5,
    ///掉期
    Swap = 6,
}
