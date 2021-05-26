#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    ///数据起始点
    #[prost(int32, required, tag="1")]
    pub begin: i32,
    ///请求数据个数，最大200
    #[prost(int32, required, tag="2")]
    pub num: i32,
    ///Qot_Common.SortField，根据哪个字段排序
    #[prost(int32, required, tag="3")]
    pub sort_field: i32,
    ///升序ture，降序false
    #[prost(bool, required, tag="4")]
    pub ascend: bool,
    ///以下为筛选条件，可选字段，不填表示不过滤
    ///
    ///所属正股
    #[prost(message, optional, tag="5")]
    pub owner: ::core::option::Option<super::qot_common::Security>,
    ///Qot_Common.WarrantType，窝轮类型过滤列表
    #[prost(int32, repeated, packed="false", tag="6")]
    pub type_list: ::prost::alloc::vec::Vec<i32>,
    ///Qot_Common.Issuer，发行人过滤列表
    #[prost(int32, repeated, packed="false", tag="7")]
    pub issuer_list: ::prost::alloc::vec::Vec<i32>,
    ///到期日，到期日范围的开始时间戳
    #[prost(string, optional, tag="8")]
    pub maturity_time_min: ::core::option::Option<::prost::alloc::string::String>,
    ///到期日范围的结束时间戳
    #[prost(string, optional, tag="9")]
    pub maturity_time_max: ::core::option::Option<::prost::alloc::string::String>,
    ///Qot_Common.IpoPeriod，上市日
    #[prost(int32, optional, tag="10")]
    pub ipo_period: ::core::option::Option<i32>,
    ///Qot_Common.PriceType，价内/价外（暂不支持界内证的界内外筛选）
    #[prost(int32, optional, tag="11")]
    pub price_type: ::core::option::Option<i32>,
    ///Qot_Common.WarrantStatus，窝轮状态
    #[prost(int32, optional, tag="12")]
    pub status: ::core::option::Option<i32>,
    ///最新价的过滤下限（闭区间），不传代表下限为 -∞（精确到小数点后 3 位，超出部分会被舍弃）
    #[prost(double, optional, tag="13")]
    pub cur_price_min: ::core::option::Option<f64>,
    ///最新价的过滤上限（闭区间），不传代表上限为 +∞（精确到小数点后 3 位，超出部分会被舍弃） 	
    #[prost(double, optional, tag="14")]
    pub cur_price_max: ::core::option::Option<f64>,
    ///行使价的过滤下限（闭区间），不传代表下限为 -∞（精确到小数点后 3 位，超出部分会被舍弃）
    #[prost(double, optional, tag="15")]
    pub strike_price_min: ::core::option::Option<f64>,
    ///行使价的过滤上限（闭区间），不传代表上限为 +∞（精确到小数点后 3 位，超出部分会被舍弃） 
    #[prost(double, optional, tag="16")]
    pub strike_price_max: ::core::option::Option<f64>,
    ///街货占比的过滤下限（闭区间），该字段为百分比字段，默认不展示 %，如 20 实际对应 20%。不传代表下限为 -∞（精确到小数点后 3 位，超出部分会被舍弃）
    #[prost(double, optional, tag="17")]
    pub street_min: ::core::option::Option<f64>,
    ///街货占比的过滤上限（闭区间），该字段为百分比字段，默认不展示 %，如 20 实际对应 20%。不传代表上限为 +∞（精确到小数点后 3 位，超出部分会被舍弃）
    #[prost(double, optional, tag="18")]
    pub street_max: ::core::option::Option<f64>,
    ///换股比率的过滤下限（闭区间），不传代表下限为 -∞（精确到小数点后 3 位，超出部分会被舍弃）
    #[prost(double, optional, tag="19")]
    pub conversion_min: ::core::option::Option<f64>,
    ///换股比率的过滤上限（闭区间），不传代表上限为 +∞（精确到小数点后 3 位，超出部分会被舍弃）
    #[prost(double, optional, tag="20")]
    pub conversion_max: ::core::option::Option<f64>,
    ///成交量的过滤下限（闭区间），不传代表下限为 -∞
    #[prost(uint64, optional, tag="21")]
    pub vol_min: ::core::option::Option<u64>,
    ///成交量的过滤上限（闭区间），不传代表上限为 +∞
    #[prost(uint64, optional, tag="22")]
    pub vol_max: ::core::option::Option<u64>,
    ///溢价的过滤下限（闭区间），该字段为百分比字段，默认不展示 %，如 20 实际对应 20%。不传代表下限为 -∞（精确到小数点后 3 位，超出部分会被舍弃）
    #[prost(double, optional, tag="23")]
    pub premium_min: ::core::option::Option<f64>,
    ///溢价的过滤上限（闭区间），该字段为百分比字段，默认不展示 %，如 20 实际对应 20%。不传代表上限为 +∞（精确到小数点后 3 位，超出部分会被舍弃）
    #[prost(double, optional, tag="24")]
    pub premium_max: ::core::option::Option<f64>,
    ///杠杆比率的过滤下限（闭区间），不传代表下限为 -∞（精确到小数点后 3 位，超出部分会被舍弃）
    #[prost(double, optional, tag="25")]
    pub leverage_ratio_min: ::core::option::Option<f64>,
    ///杠杆比率的过滤上限（闭区间），不传代表上限为 +∞（精确到小数点后 3 位，超出部分会被舍弃）
    #[prost(double, optional, tag="26")]
    pub leverage_ratio_max: ::core::option::Option<f64>,
    ///对冲值的过滤下限（闭区间），仅认购认沽支持此字段过滤，不传代表下限为 -∞（精确到小数点后 3 位，超出部分会被舍弃）
    #[prost(double, optional, tag="27")]
    pub delta_min: ::core::option::Option<f64>,
    ///对冲值的过滤上限（闭区间），仅认购认沽支持此字段过滤，不传代表上限为 +∞（精确到小数点后 3 位，超出部分会被舍弃）
    #[prost(double, optional, tag="28")]
    pub delta_max: ::core::option::Option<f64>,
    ///引伸波幅的过滤下限（闭区间），仅认购认沽支持此字段过滤，不传代表下限为 -∞（精确到小数点后 3 位，超出部分会被舍弃）
    #[prost(double, optional, tag="29")]
    pub implied_min: ::core::option::Option<f64>,
    ///引伸波幅的过滤上限（闭区间），仅认购认沽支持此字段过滤，不传代表上限为 +∞（精确到小数点后 3 位，超出部分会被舍弃）	
    #[prost(double, optional, tag="30")]
    pub implied_max: ::core::option::Option<f64>,
    ///收回价的过滤下限（闭区间），仅牛熊证支持此字段过滤，不传代表下限为 -∞（精确到小数点后 3 位，超出部分会被舍弃）
    #[prost(double, optional, tag="31")]
    pub recovery_price_min: ::core::option::Option<f64>,
    ///收回价的过滤上限（闭区间），仅牛熊证支持此字段过滤，不传代表上限为 +∞（精确到小数点后 3 位，超出部分会被舍弃）
    #[prost(double, optional, tag="32")]
    pub recovery_price_max: ::core::option::Option<f64>,
    ///正股距收回价，的过滤下限（闭区间），仅牛熊证支持此字段过滤。该字段为百分比字段，默认不展示 %，如 20 实际对应 20%。不传代表下限为 -∞（精确到小数点后 3 位，超出部分会被舍弃）
    #[prost(double, optional, tag="33")]
    pub price_recovery_ratio_min: ::core::option::Option<f64>,
    ///正股距收回价，的过滤上限（闭区间），仅牛熊证支持此字段过滤。该字段为百分比字段，默认不展示 %，如 20 实际对应 20%。不传代表上限为 +∞（精确到小数点后 3 位，超出部分会被舍弃）	
    #[prost(double, optional, tag="34")]
    pub price_recovery_ratio_max: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WarrantData {
    ///静态数据项
    ///
    ///股票
    #[prost(message, required, tag="1")]
    pub stock: super::qot_common::Security,
    ///所属正股
    #[prost(message, required, tag="2")]
    pub owner: super::qot_common::Security,
    ///Qot_Common.WarrantType，窝轮类型
    #[prost(int32, required, tag="3")]
    pub r#type: i32,
    ///Qot_Common.Issuer，发行人
    #[prost(int32, required, tag="4")]
    pub issuer: i32,
    ///到期日
    #[prost(string, required, tag="5")]
    pub maturity_time: ::prost::alloc::string::String,
    ///到期日时间戳
    #[prost(double, optional, tag="6")]
    pub maturity_timestamp: ::core::option::Option<f64>,
    ///上市时间
    #[prost(string, required, tag="7")]
    pub list_time: ::prost::alloc::string::String,
    ///上市时间戳
    #[prost(double, optional, tag="8")]
    pub list_timestamp: ::core::option::Option<f64>,
    ///最后交易日
    #[prost(string, required, tag="9")]
    pub last_trade_time: ::prost::alloc::string::String,
    ///最后交易日时间戳
    #[prost(double, optional, tag="10")]
    pub last_trade_timestamp: ::core::option::Option<f64>,
    ///收回价，仅牛熊证支持此字段
    #[prost(double, optional, tag="11")]
    pub recovery_price: ::core::option::Option<f64>,
    ///换股比率
    #[prost(double, required, tag="12")]
    pub conversion_ratio: f64,
    ///每手数量
    #[prost(int32, required, tag="13")]
    pub lot_size: i32,
    ///行使价	
    #[prost(double, required, tag="14")]
    pub strike_price: f64,
    ///昨收价        
    #[prost(double, required, tag="15")]
    pub last_close_price: f64,
    ///名称	
    #[prost(string, required, tag="16")]
    pub name: ::prost::alloc::string::String,
    ///动态数据项
    ///
    ///当前价
    #[prost(double, required, tag="17")]
    pub cur_price: f64,
    ///涨跌额
    #[prost(double, required, tag="18")]
    pub price_change_val: f64,
    ///涨跌幅（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）	
    #[prost(double, required, tag="19")]
    pub change_rate: f64,
    ///Qot_Common.WarrantStatus，窝轮状态	
    #[prost(int32, required, tag="20")]
    pub status: i32,
    ///买入价	
    #[prost(double, required, tag="21")]
    pub bid_price: f64,
    ///卖出价
    #[prost(double, required, tag="22")]
    pub ask_price: f64,
    ///买量
    #[prost(int64, required, tag="23")]
    pub bid_vol: i64,
    ///卖量
    #[prost(int64, required, tag="24")]
    pub ask_vol: i64,
    ///成交量
    #[prost(int64, required, tag="25")]
    pub volume: i64,
    ///成交额	
    #[prost(double, required, tag="26")]
    pub turnover: f64,
    ///综合评分
    #[prost(double, required, tag="27")]
    pub score: f64,
    ///溢价（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    #[prost(double, required, tag="28")]
    pub premium: f64,
    ///打和点	
    #[prost(double, required, tag="29")]
    pub break_even_point: f64,
    ///杠杆比率（倍）
    #[prost(double, required, tag="30")]
    pub leverage: f64,
    ///价内/价外，正数表示价内，负数表示价外（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）        	
    #[prost(double, required, tag="31")]
    pub ipop: f64,
    ///正股距收回价，仅牛熊证支持此字段（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    #[prost(double, optional, tag="32")]
    pub price_recovery_ratio: ::core::option::Option<f64>,
    ///换股价
    #[prost(double, required, tag="33")]
    pub conversion_price: f64,
    ///街货占比（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）	
    #[prost(double, required, tag="34")]
    pub street_rate: f64,
    ///街货量
    #[prost(int64, required, tag="35")]
    pub street_vol: i64,
    ///振幅（该字段为百分比字段，默认不展示 %，如 20 实际对应 20%）
    #[prost(double, required, tag="36")]
    pub amplitude: f64,
    ///发行量	        
    #[prost(int64, required, tag="37")]
    pub issue_size: i64,
    ///最高价
    #[prost(double, required, tag="39")]
    pub high_price: f64,
    ///最低价	
    #[prost(double, required, tag="40")]
    pub low_price: f64,
    ///引申波幅，仅认购认沽支持此字段
    #[prost(double, optional, tag="41")]
    pub implied_volatility: ::core::option::Option<f64>,
    ///对冲值，仅认购认沽支持此字段
    #[prost(double, optional, tag="42")]
    pub delta: ::core::option::Option<f64>,
    ///有效杠杆
    #[prost(double, required, tag="43")]
    pub effective_leverage: f64,
    ///上限价，仅界内证支持此字段
    #[prost(double, optional, tag="44")]
    pub upper_strike_price: ::core::option::Option<f64>,
    ///下限价，仅界内证支持此字段
    #[prost(double, optional, tag="45")]
    pub lower_strike_price: ::core::option::Option<f64>,
    ///Qot_Common.PriceType，界内界外，仅界内证支持此字段
    #[prost(int32, optional, tag="46")]
    pub in_line_price_status: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    ///是否最后一页了，false:非最后一页，还有窝轮记录未返回; true:已是最后一页
    #[prost(bool, required, tag="1")]
    pub last_page: bool,
    ///该条件请求所有数据的个数
    #[prost(int32, required, tag="2")]
    pub all_count: i32,
    ///窝轮数据
    #[prost(message, repeated, tag="3")]
    pub warrant_data_list: ::prost::alloc::vec::Vec<WarrantData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(message, required, tag="1")]
    pub c2s: C2s,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    ///RetType，返回结果
    #[prost(int32, required, tag="1", default="-400")]
    pub ret_type: i32,
    #[prost(string, optional, tag="2")]
    pub ret_msg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="3")]
    pub err_code: ::core::option::Option<i32>,
    #[prost(message, optional, tag="4")]
    pub s2c: ::core::option::Option<S2c>,
}
