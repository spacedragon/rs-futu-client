///账户现金信息，目前仅用于期货账户
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccCashInfo {
    /// 货币类型，取值参考 Currency
    #[prost(int32, optional, tag="1")]
    pub currency: ::core::option::Option<i32>,
    /// 现金结余
    #[prost(double, optional, tag="2")]
    pub cash: ::core::option::Option<f64>,
    /// 现金可提金额
    #[prost(double, optional, tag="3")]
    pub available_balance: ::core::option::Option<f64>,
}
///交易协议公共参数头
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrdHeader {
    ///交易环境, 参见TrdEnv的枚举定义
    #[prost(int32, required, tag="1")]
    pub trd_env: i32,
    ///业务账号, 业务账号与交易环境、市场权限需要匹配，否则会返回错误
    #[prost(uint64, required, tag="2")]
    pub acc_id: u64,
    ///交易市场, 参见TrdMarket的枚举定义
    #[prost(int32, required, tag="3")]
    pub trd_market: i32,
}
///交易业务账户结构
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrdAcc {
    ///交易环境，参见TrdEnv的枚举定义
    #[prost(int32, required, tag="1")]
    pub trd_env: i32,
    ///业务账号
    #[prost(uint64, required, tag="2")]
    pub acc_id: u64,
    ///业务账户支持的交易市场权限，即此账户能交易那些市场, 可拥有多个交易市场权限，目前仅单个，取值参见TrdMarket的枚举定义
    #[prost(int32, repeated, packed="false", tag="3")]
    pub trd_market_auth_list: ::prost::alloc::vec::Vec<i32>,
    ///账户类型，取值见TrdAccType
    #[prost(int32, optional, tag="4")]
    pub acc_type: ::core::option::Option<i32>,
    ///卡号
    #[prost(string, optional, tag="5")]
    pub card_num: ::core::option::Option<::prost::alloc::string::String>,
    ///所属券商，取值见SecurityFirm
    #[prost(int32, optional, tag="6")]
    pub security_firm: ::core::option::Option<i32>,
    ///模拟交易账号类型，取值见SimAccType
    #[prost(int32, optional, tag="7")]
    pub sim_acc_type: ::core::option::Option<i32>,
}
///账户资金结构
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Funds {
    ///最大购买力（做多），3位精度，下同。
    #[prost(double, required, tag="1")]
    pub power: f64,
    ///资产净值
    #[prost(double, required, tag="2")]
    pub total_assets: f64,
    ///现金
    #[prost(double, required, tag="3")]
    pub cash: f64,
    ///证券市值, 仅证券账户适用
    #[prost(double, required, tag="4")]
    pub market_val: f64,
    ///冻结资金
    #[prost(double, required, tag="5")]
    pub frozen_cash: f64,
    ///计息金额
    #[prost(double, required, tag="6")]
    pub debt_cash: f64,
    ///现金可提，仅证券账户适用 
    #[prost(double, required, tag="7")]
    pub avl_withdrawal_cash: f64,
    ///币种，本结构体资金相关的货币类型，取值参见 Currency，期货适用
    #[prost(int32, optional, tag="8")]
    pub currency: ::core::option::Option<i32>,
    ///可用资金，期货适用
    #[prost(double, optional, tag="9")]
    pub available_funds: ::core::option::Option<f64>,
    ///未实现盈亏，期货适用
    #[prost(double, optional, tag="10")]
    pub unrealized_pl: ::core::option::Option<f64>,
    ///已实现盈亏，期货适用
    #[prost(double, optional, tag="11")]
    pub realized_pl: ::core::option::Option<f64>,
    ///风控状态，参见 CltRiskLevel, 期货适用
    #[prost(int32, optional, tag="12")]
    pub risk_level: ::core::option::Option<i32>,
    ///初始保证金
    #[prost(double, optional, tag="13")]
    pub initial_margin: ::core::option::Option<f64>,
    ///维持保证金
    #[prost(double, optional, tag="14")]
    pub maintenance_margin: ::core::option::Option<f64>,
    ///分币种的现金信息，期货适用
    #[prost(message, repeated, tag="15")]
    pub cash_info_list: ::prost::alloc::vec::Vec<AccCashInfo>,
    ///卖空购买力
    #[prost(double, optional, tag="16")]
    pub max_power_short: ::core::option::Option<f64>,
    ///现金购买力
    #[prost(double, optional, tag="17")]
    pub net_cash_power: ::core::option::Option<f64>,
    ///多头市值
    #[prost(double, optional, tag="18")]
    pub long_mv: ::core::option::Option<f64>,
    ///空头市值
    #[prost(double, optional, tag="19")]
    pub short_mv: ::core::option::Option<f64>,
    ///在途资产
    #[prost(double, optional, tag="20")]
    pub pending_asset: ::core::option::Option<f64>,
    ///融资可提，仅证券账户适用
    #[prost(double, optional, tag="21")]
    pub max_withdrawal: ::core::option::Option<f64>,
    ///风险状态，参见 [CltRiskStatus]，证券账户适用，共分 9 个等级，LEVEL1是最安全，LEVEL9是最危险
    #[prost(int32, optional, tag="22")]
    pub risk_status: ::core::option::Option<i32>,
    ///	Margin Call 保证金
    #[prost(double, optional, tag="23")]
    pub margin_call_margin: ::core::option::Option<f64>,
}
///账户持仓结构
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    ///持仓ID，一条持仓的唯一标识
    #[prost(uint64, required, tag="1")]
    pub position_id: u64,
    ///持仓方向，参见PositionSide的枚举定义
    #[prost(int32, required, tag="2")]
    pub position_side: i32,
    ///代码
    #[prost(string, required, tag="3")]
    pub code: ::prost::alloc::string::String,
    ///名称
    #[prost(string, required, tag="4")]
    pub name: ::prost::alloc::string::String,
    ///持有数量，2位精度，期权单位是"张"，下同
    #[prost(double, required, tag="5")]
    pub qty: f64,
    ///可卖数量
    #[prost(double, required, tag="6")]
    pub can_sell_qty: f64,
    ///市价，3位精度，期货为2位精度
    #[prost(double, required, tag="7")]
    pub price: f64,
    ///成本价，无精度限制，期货为2位精度，如果没传，代表此时此值无效,
    #[prost(double, optional, tag="8")]
    pub cost_price: ::core::option::Option<f64>,
    ///市值，3位精度, 期货此字段值为0
    #[prost(double, required, tag="9")]
    pub val: f64,
    ///盈亏金额，3位精度，期货为2位精度
    #[prost(double, required, tag="10")]
    pub pl_val: f64,
    ///盈亏百分比(如plRatio等于8.8代表涨8.8%)，无精度限制，如果没传，代表此时此值无效
    #[prost(double, optional, tag="11")]
    pub pl_ratio: ::core::option::Option<f64>,
    ///证券所属市场，参见TrdSecMarket的枚举定义
    #[prost(int32, optional, tag="12")]
    pub sec_market: ::core::option::Option<i32>,
    ///以下是此持仓今日统计
    ///
    ///今日盈亏金额，3位精度，下同, 期货为2位精度
    #[prost(double, optional, tag="21")]
    pub td_pl_val: ::core::option::Option<f64>,
    ///今日交易额，期货不适用
    #[prost(double, optional, tag="22")]
    pub td_trd_val: ::core::option::Option<f64>,
    ///今日买入总额，期货不适用
    #[prost(double, optional, tag="23")]
    pub td_buy_val: ::core::option::Option<f64>,
    ///今日买入总量，期货不适用
    #[prost(double, optional, tag="24")]
    pub td_buy_qty: ::core::option::Option<f64>,
    ///今日卖出总额，期货不适用
    #[prost(double, optional, tag="25")]
    pub td_sell_val: ::core::option::Option<f64>,
    ///今日卖出总量，期货不适用
    #[prost(double, optional, tag="26")]
    pub td_sell_qty: ::core::option::Option<f64>,
    ///未实现盈亏，期货适用
    #[prost(double, optional, tag="28")]
    pub unrealized_pl: ::core::option::Option<f64>,
    ///已实现盈亏，期货适用
    #[prost(double, optional, tag="29")]
    pub realized_pl: ::core::option::Option<f64>,
}
///订单结构
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    ///交易方向, 参见TrdSide的枚举定义
    #[prost(int32, required, tag="1")]
    pub trd_side: i32,
    ///订单类型, 参见OrderType的枚举定义
    #[prost(int32, required, tag="2")]
    pub order_type: i32,
    ///订单状态, 参见OrderStatus的枚举定义
    #[prost(int32, required, tag="3")]
    pub order_status: i32,
    ///订单号
    #[prost(uint64, required, tag="4")]
    pub order_id: u64,
    ///扩展订单号(仅查问题时备用)
    #[prost(string, required, tag="5")]
    pub order_id_ex: ::prost::alloc::string::String,
    ///代码
    #[prost(string, required, tag="6")]
    pub code: ::prost::alloc::string::String,
    ///名称
    #[prost(string, required, tag="7")]
    pub name: ::prost::alloc::string::String,
    ///订单数量，2位精度，期权单位是"张"
    #[prost(double, required, tag="8")]
    pub qty: f64,
    ///订单价格，3位精度
    #[prost(double, optional, tag="9")]
    pub price: ::core::option::Option<f64>,
    ///创建时间，严格按YYYY-MM-DD HH:MM:SS或YYYY-MM-DD HH:MM:SS.MS格式传
    #[prost(string, required, tag="10")]
    pub create_time: ::prost::alloc::string::String,
    ///最后更新时间，严格按YYYY-MM-DD HH:MM:SS或YYYY-MM-DD HH:MM:SS.MS格式传
    #[prost(string, required, tag="11")]
    pub update_time: ::prost::alloc::string::String,
    ///成交数量，2位精度，期权单位是"张"
    #[prost(double, optional, tag="12")]
    pub fill_qty: ::core::option::Option<f64>,
    ///成交均价，无精度限制
    #[prost(double, optional, tag="13")]
    pub fill_avg_price: ::core::option::Option<f64>,
    ///最后的错误描述，如果有错误，会有此描述最后一次错误的原因，无错误为空
    #[prost(string, optional, tag="14")]
    pub last_err_msg: ::core::option::Option<::prost::alloc::string::String>,
    ///证券所属市场，参见TrdSecMarket的枚举定义
    #[prost(int32, optional, tag="15")]
    pub sec_market: ::core::option::Option<i32>,
    ///创建时间戳
    #[prost(double, optional, tag="16")]
    pub create_timestamp: ::core::option::Option<f64>,
    ///最后更新时间戳
    #[prost(double, optional, tag="17")]
    pub update_timestamp: ::core::option::Option<f64>,
    ///用户备注字符串，最大长度64字节
    #[prost(string, optional, tag="18")]
    pub remark: ::core::option::Option<::prost::alloc::string::String>,
    ///订单期限，参考 TimeInForce 类的定义
    #[prost(int32, optional, tag="19")]
    pub time_in_force: ::core::option::Option<i32>,
    ///是否允许美股订单盘前盘后成交
    #[prost(bool, optional, tag="20")]
    pub fill_outside_rth: ::core::option::Option<bool>,
}
///成交结构
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderFill {
    ///交易方向, 参见TrdSide的枚举定义
    #[prost(int32, required, tag="1")]
    pub trd_side: i32,
    ///成交号
    #[prost(uint64, required, tag="2")]
    pub fill_id: u64,
    ///扩展成交号(仅查问题时备用)
    #[prost(string, required, tag="3")]
    pub fill_id_ex: ::prost::alloc::string::String,
    ///订单号
    #[prost(uint64, optional, tag="4")]
    pub order_id: ::core::option::Option<u64>,
    ///扩展订单号(仅查问题时备用)
    #[prost(string, optional, tag="5")]
    pub order_id_ex: ::core::option::Option<::prost::alloc::string::String>,
    ///代码
    #[prost(string, required, tag="6")]
    pub code: ::prost::alloc::string::String,
    ///名称
    #[prost(string, required, tag="7")]
    pub name: ::prost::alloc::string::String,
    ///成交数量，2位精度，期权单位是"张"
    #[prost(double, required, tag="8")]
    pub qty: f64,
    ///成交价格，3位精度
    #[prost(double, required, tag="9")]
    pub price: f64,
    ///创建时间（成交时间），严格按YYYY-MM-DD HH:MM:SS或YYYY-MM-DD HH:MM:SS.MS格式传
    #[prost(string, required, tag="10")]
    pub create_time: ::prost::alloc::string::String,
    ///对手经纪号，港股有效
    #[prost(int32, optional, tag="11")]
    pub counter_broker_id: ::core::option::Option<i32>,
    ///对手经纪名称，港股有效
    #[prost(string, optional, tag="12")]
    pub counter_broker_name: ::core::option::Option<::prost::alloc::string::String>,
    ///证券所属市场，参见TrdSecMarket的枚举定义
    #[prost(int32, optional, tag="13")]
    pub sec_market: ::core::option::Option<i32>,
    ///创建时间戳
    #[prost(double, optional, tag="14")]
    pub create_timestamp: ::core::option::Option<f64>,
    ///最后更新时间戳
    #[prost(double, optional, tag="15")]
    pub update_timestamp: ::core::option::Option<f64>,
    ///成交状态, 参见OrderFillStatus的枚举定义
    #[prost(int32, optional, tag="16")]
    pub status: ::core::option::Option<i32>,
}
///最大可交易数量
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaxTrdQtys {
    ///因目前服务器实现的问题，卖空需要先卖掉持仓才能再卖空，是分开两步卖的，买回来同样是逆向两步；而看多的买是可以现金加融资一起一步买的，请注意这个差异
    ///
    ///不使用融资，仅自己的现金最大可买整手股数，期货此字段值为0
    #[prost(double, required, tag="1")]
    pub max_cash_buy: f64,
    ///使用融资，自己的现金 + 融资资金总共的最大可买整手股数，期货不适用
    #[prost(double, optional, tag="2")]
    pub max_cash_and_margin_buy: ::core::option::Option<f64>,
    ///不使用融券(卖空)，仅自己的持仓最大可卖整手股数
    #[prost(double, required, tag="3")]
    pub max_position_sell: f64,
    ///使用融券(卖空)，最大可卖空整手股数，不包括多仓，期货不适用
    #[prost(double, optional, tag="4")]
    pub max_sell_short: ::core::option::Option<f64>,
    ///卖空后，需要买回的最大整手股数。因为卖空后，必须先买回已卖空的股数，还掉股票，才能再继续买多。期货不适用
    #[prost(double, optional, tag="5")]
    pub max_buy_back: ::core::option::Option<f64>,
    ///开多仓每张合约初始保证金。当前仅期货和期权适用（最低 FutuOpenD 版本要求：5.0.1310）
    #[prost(double, optional, tag="6")]
    pub long_required_im: ::core::option::Option<f64>,
    ///开空仓每张合约初始保证金。当前仅期货和期权适用（最低 FutuOpenD 版本要求：5.0.1310）
    #[prost(double, optional, tag="7")]
    pub short_required_im: ::core::option::Option<f64>,
}
///过滤条件，条件组合是"与"不是"或"，用于获取订单、成交、持仓等时二次过滤
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrdFilterConditions {
    ///代码过滤，只返回包含这些代码的数据，没传不过滤
    #[prost(string, repeated, tag="1")]
    pub code_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///ID主键过滤，只返回包含这些ID的数据，没传不过滤，订单是orderID、成交是fillID、持仓是positionID
    #[prost(uint64, repeated, packed="false", tag="2")]
    pub id_list: ::prost::alloc::vec::Vec<u64>,
    ///开始时间，严格按YYYY-MM-DD HH:MM:SS或YYYY-MM-DD HH:MM:SS.MS格式传，对持仓无效，拉历史数据必须填
    #[prost(string, optional, tag="3")]
    pub begin_time: ::core::option::Option<::prost::alloc::string::String>,
    ///结束时间，严格按YYYY-MM-DD HH:MM:SS或YYYY-MM-DD HH:MM:SS.MS格式传，对持仓无效，拉历史数据必须填
    #[prost(string, optional, tag="4")]
    pub end_time: ::core::option::Option<::prost::alloc::string::String>,
}
///交易环境
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrdEnv {
    ///仿真环境(模拟环境)
    Simulate = 0,
    ///真实环境
    Real = 1,
}
///交易市场，是大的市场，不是具体品种
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrdMarket {
    ///未知市场
    Unknown = 0,
    ///香港市场
    Hk = 1,
    ///美国市场
    Us = 2,
    ///大陆市场
    Cn = 3,
    ///香港A股通市场
    Hkcc = 4,
    ///期货市场
    Futures = 5,
}
///可交易证券所属市场，目前主要是区分A股的沪市和深市，香港和美国暂不需要细分
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrdSecMarket {
    ///未知市场
    Unknown = 0,
    ///香港市场(股票、窝轮、牛熊、期权、期货等)
    Hk = 1,
    ///美国市场(股票、期权、期货等)
    Us = 2,
    ///沪股市场(股票)
    CnSh = 31,
    ///深股市场(股票)
    CnSz = 32,
    ///新加坡市场(期货)
    Sg = 41,
    ///日本市场(期货)
    Jp = 51,
}
///交易方向
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrdSide {
    ///客户端下单只传Buy或Sell即可，SellShort是美股订单时服务器返回有此方向，BuyBack目前不存在，但也不排除服务器会传
    ///
    ///未知方向
    Unknown = 0,
    ///买入
    Buy = 1,
    ///卖出
    Sell = 2,
    ///卖空
    SellShort = 3,
    ///买回
    BuyBack = 4,
}
///订单类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderType {
    ///未知类型
    Unknown = 0,
    ///普通订单(港股的增强限价单、港股期权的限价单，A股限价委托、美股的限价单，港股期货的限价单，CME期货的限价单)。目前港股期权只能指定此订单类型。
    Normal = 1,
    ///市价订单(目前支持美股、港股正股、涡轮、牛熊、界内证)
    Market = 2,
    ///绝对限价订单(目前仅港股)，只有价格完全匹配才成交，否则下单失败，比如你下价格为5元的买单，卖单价格必须也要是5元才能成交，低于5元也不能成交，下单失败。卖出同理
    AbsoluteLimit = 5,
    ///竞价订单(目前仅港股)，仅港股早盘竞价和收盘竞价有效，A股的早盘竞价订单类型不变还是OrderType_Normal
    Auction = 6,
    ///竞价限价订单(目前仅港股)，仅早盘竞价和收盘竞价有效，参与竞价，且要求满足指定价格才会成交
    AuctionLimit = 7,
    ///特别限价订单(目前仅港股)，成交规则同增强限价订单，且部分成交后，交易所自动撤销订单
    SpecialLimit = 8,
    ///特别限价且要求全部成交订单(目前仅港股)，要么全部成交，要么自动撤单
    SpecialLimitAll = 9,
}
///订单状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderStatus {
    ///未提交
    Unsubmitted = 0,
    ///未知状态
    Unknown = -1,
    ///等待提交
    WaitingSubmit = 1,
    ///提交中
    Submitting = 2,
    ///提交失败，下单失败
    SubmitFailed = 3,
    ///处理超时，结果未知
    TimeOut = 4,
    ///已提交，等待成交
    Submitted = 5,
    ///部分成交
    FilledPart = 10,
    ///全部已成
    FilledAll = 11,
    ///正在撤单_部分(部分已成交，正在撤销剩余部分)
    CancellingPart = 12,
    ///正在撤单_全部
    CancellingAll = 13,
    ///部分成交，剩余部分已撤单
    CancelledPart = 14,
    ///全部已撤单，无成交
    CancelledAll = 15,
    ///下单失败，服务拒绝
    Failed = 21,
    ///已失效
    Disabled = 22,
    ///已删除，无成交的订单才能删除
    Deleted = 23,
    ///成交被撤销，一般遇不到，意思是已经成交的订单被回滚撤销，成交无效变为废单
    FillCancelled = 24,
}
///一笔成交的状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderFillStatus {
    ///正常
    Ok = 0,
    ///成交被取消
    Cancelled = 1,
    ///成交被更改
    Changed = 2,
}
///持仓方向类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PositionSide {
    ///多仓，默认情况是多仓
    Long = 0,
    ///未知方向
    Unknown = -1,
    ///空仓
    Short = 1,
}
///修改订单的操作类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ModifyOrderOp {
    ///港股支持全部操作，美股目前仅支持ModifyOrderOp_Normal和ModifyOrderOp_Cancel
    ///
    ///未知操作
    Unknown = 0,
    ///修改订单的价格、数量等，即以前的改单
    Normal = 1,
    ///撤单
    Cancel = 2,
    ///失效
    Disable = 3,
    ///生效
    Enable = 4,
    ///删除
    Delete = 5,
}
///交易账户类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrdAccType {
    ///未知类型
    Unknown = 0,
    ///现金账户
    Cash = 1,
    ///保证金账户
    Margin = 2,
}
///货币种类
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Currency {
    ///未知货币
    Unknown = 0,
    /// 港币
    Hkd = 1,
    /// 美元
    Usd = 2,
    /// 离岸人民币
    Cnh = 3,
    /// 日元
    Jpy = 4,
}
///账户风险控制等级
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CltRiskLevel {
    /// 未知
    Unknown = -1,
    /// 安全
    Safe = 0,
    /// 预警
    Warning = 1,
    /// 危险
    Danger = 2,
    /// 绝对安全
    AbsoluteSafe = 3,
    /// 危险, 期权相关
    OptDanger = 4,
}
///订单有效期
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TimeInForce {
    /// 当日有效
    Day = 0,
    /// 撤单前有效，最多持续90自然日。
    Gtc = 1,
}
///券商
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SecurityFirm {
    ///未知
    Unknown = 0,
    ///富途证券（香港）
    FutuSecurities = 1,
    ///富途证券（美国）
    FutuInc = 2,
    ///富途证券（新加坡）
    FutuSg = 3,
}
///模拟交易账户类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SimAccType {
    ///未知
    Unknown = 0,
    ///股票模拟账户（仅用于交易证券类产品，不支持交易期权）
    Stock = 1,
    ///期权模拟账户（仅用于交易期权，不支持交易股票证券类产品）
    Option = 2,
}
///风险状态，共分 9 个等级，LEVEL1是最安全，LEVEL9是最危险
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CltRiskStatus {
    ///未知
    Unknown = 0,
    ///非常安全
    Level1 = 1,
    ///安全
    Level2 = 2,
    ///较安全
    Level3 = 3,
    ///较低风险
    Level4 = 4,
    ///中等风险
    Level5 = 5,
    ///较高风险
    Level6 = 6,
    ///预警
    Level7 = 7,
    ///预警
    Level8 = 8,
    ///预警
    Level9 = 9,
}
