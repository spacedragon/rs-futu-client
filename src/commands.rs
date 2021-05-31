#[allow(dead_code)]

use crate::protos::*;

pub trait ProtoCommand
    where Self: prost::Message + Sized
{
    type REQ: prost::Message;
    type RESP: prost::Message + Default;
    const PROTO_ID: ProtoId;
    fn to_request(self) -> Self::REQ;
}

macro_rules! define_command {
    ($id:ident ,$l:ident) => {
        pub type $id = crate::protos::$l::C2s;
        impl ProtoCommand for crate::protos::$l::C2s {
            type REQ = crate::protos::$l::Request;
            type RESP = crate::protos::$l::Response;
            const PROTO_ID: ProtoId = ProtoId::$id;
            fn to_request(self) -> Self::REQ {
                $l::Request {
                    c2s: self
                }
            }
        }
    }
}

#[derive(FromPrimitive, ToPrimitive)]
pub enum ProtoId {
    InitConnect = 1001,
    GetGlobalState = 1002,
    Notify= 1003,
    KeepAlive = 1004,
	QotSub = 3001,//	订阅或者反订阅
	QotGetSubInfo = 3003,//	获取订阅信息
	QotGetBasicQot = 3004,//	获取股票基本报价
	QotGetKL = 3006,//	获取 K 线
	QotGetRT = 3008,//	获取分时
	QotGetTicker = 3010,//	获取逐笔
	QotGetOrderBook = 3012,//	获取买卖盘
	QotGetBroker = 3014,//	获取经纪队列
	QotRequestHistoryKL = 3103,//	在线获取单只股票一段历史 K 线
	QotRequestHistoryKLQuota = 3104,//	获取历史 K 线额度
	QotRequestRehab = 3105,//	在线获取单只股票复权信息
	QotGetStaticInfo = 3202,//	获取股票静态信息
	QotGetSecuritySnapshot = 3203,//	获取股票快照
	QotGetPlateSet = 3204,//	获取板块集合下的板块
	QotGetPlateSecurity = 3205,//	获取板块下的股票
	QotGetReference = 3206,//	获取正股相关股票
	QotGetOwnerPlate = 3207,//	获取股票所属板块
	QotGetHoldingChangeList = 3208,//	获取持股变化列表
	QotGetOptionChain = 3209,//	获取期权链
	QotGetWarrant = 3210,//	获取窝轮
	QotGetCapitalFlow = 3211,//	获取资金流向
	QotGetCapitalDistribution = 3212,//	获取资金分布
	QotGetUserSecurity = 3213,//	获取自选股分组下的股票
	QotModifyUserSecurity = 3214,//	修改自选股分组下的股票
	QotStockFilter = 3215,//	获取条件选股
	QotGetIpoList = 3217,//	获取新股
	QotGetFutureInfo = 3218,//	获取期货合约资料
	QotRequestTradeDate = 3219,//	获取市场交易日,在线拉取不在本地计算
	QotSetPriceReminder = 3220,//	设置到价提醒
	QotGetPriceReminder = 3221,//	获取到价提醒
	QotUpdatePriceReminder = 3019,//	到价提醒通知
	QotGetUserSecurityGroup = 3222,//	获取自选股分组列表
	QotGetMarketState = 3223,//	获取指定品种的市场状态
}

define_command!(InitConnect, init_connect);
define_command!(KeepAlive, keep_alive);

pub type Notify = crate::protos::notify::S2c;

define_command!(GetGlobalState, get_global_state);
define_command!(QotSub, qot_sub);
define_command!(QotGetSubInfo, qot_get_sub_info);
define_command!(QotRequestHistoryKL, qot_request_history_kl);
define_command!(QotRequestHistoryKLQuota, qot_request_history_kl_quota);
define_command!(QotRequestRehab, qot_request_rehab);
define_command!(QotGetStaticInfo, qot_get_static_info);
define_command!(QotGetSecuritySnapshot, qot_get_security_snapshot);
define_command!(QotGetPlateSet, qot_get_plate_set);
define_command!(QotGetPlateSecurity, qot_get_plate_security);
define_command!(QotGetReference, qot_get_reference);
define_command!(QotGetOwnerPlate, qot_get_owner_plate);
define_command!(QotGetHoldingChangeList, qot_get_holding_change_list);
define_command!(QotGetOptionChain, qot_get_option_chain);
define_command!(QotGetWarrant, qot_get_warrant);
define_command!(QotGetCapitalFlow, qot_get_capital_flow);
define_command!(QotGetCapitalDistribution, qot_get_capital_distribution);
define_command!(QotGetUserSecurity, qot_get_user_security);
define_command!(QotModifyUserSecurity, qot_modify_user_security);
define_command!(QotStockFilter, qot_stock_filter);
define_command!(QotGetIpoList, qot_get_ipo_list);
define_command!(QotGetFutureInfo, qot_get_future_info);
define_command!(QotRequestTradeDate, qot_request_trade_date);
define_command!(QotSetPriceReminder, qot_set_price_reminder);
define_command!(QotGetPriceReminder, qot_get_price_reminder);
// define_command!(QotUpdatePriceReminder, qot_update_price_reminder);
define_command!(QotGetUserSecurityGroup, qot_get_user_security_group);
define_command!(QotGetMarketState, qot_get_market_state);

#[derive(FromPrimitive, ToPrimitive)]
pub enum SubId {
	QotUpdateBasicQot = 3005,//	推送股票基本报价
	QotUpdateKL = 3007,//	推送 K 线
	QotUpdateRT = 3009,//	推送分时
	QotUpdateTicker = 3011,//	推送逐笔
	QotUpdateBroker = 3015,//	推送经纪队列
	QotUpdateOrderBook = 3013,//	推送买卖盘
}

pub trait SubscribeCommand
	where Self: prost::Message + Sized{
	const SUB_ID: SubId;
	const PROTO_ID: ProtoId;
	type SubResult: prost::Message + Default;
}

macro_rules! define_subscribe {
    ($id:ident ,$s:ident,$rid:ident, $r: ident) => {
        pub type $rid = crate::protos::$r::S2c;
        impl SubscribeCommand for crate::protos::$s::C2s {
            type SubResult = crate::protos::$r::Response;
            const PROTO_ID: ProtoId = ProtoId::$id;
			const SUB_ID: SubId = SubId::$rid;
        }
		define_command!($id, $s);
    }

}

define_subscribe!(QotGetBasicQot, qot_get_basic_qot, QotUpdateBasicQot, qot_update_basic_qot);
define_subscribe!(QotGetKL, qot_get_kl, QotUpdateKL, qot_update_kl);
define_subscribe!(QotGetRT, qot_get_rt, QotUpdateRT, qot_update_rt);
define_subscribe!(QotGetTicker, qot_get_ticker, QotUpdateTicker, qot_update_ticker);
define_subscribe!(QotGetOrderBook, qot_get_order_book, QotUpdateOrderBook, qot_update_order_book);
define_subscribe!(QotGetBroker, qot_get_broker, QotUpdateBroker, qot_update_broker);