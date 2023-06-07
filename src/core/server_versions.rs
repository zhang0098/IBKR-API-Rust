//! The known server versions.

//MIN_SERVER_VER_REAL_TIME_BARS       = 34
//MIN_SERVER_VER_SCALE_ORDERS         = 35
//MIN_SERVER_VER_SNAPSHOT_MKT_DATA    = 35
//MIN_SERVER_VER_SSHORT_COMBO_LEGS    = 35
//MIN_SERVER_VER_WHAT_IF_ORDERS       = 36
//MIN_SERVER_VER_CONTRACT_CONID       = 37
pub const MIN_SERVER_VER_PTA_ORDERS: i32 = 39;
pub const MIN_SERVER_VER_FUNDAMENTAL_DATA: i32 = 40;
pub const MIN_SERVER_VER_DELTA_NEUTRAL: i32 = 40;
pub const MIN_SERVER_VER_CONTRACT_DATA_CHAIN: i32 = 40;
pub const MIN_SERVER_VER_SCALE_ORDERS2: i32 = 40;
pub const MIN_SERVER_VER_ALGO_ORDERS: i32 = 41;
pub const MIN_SERVER_VER_EXECUTION_DATA_CHAIN: i32 = 42;
pub const MIN_SERVER_VER_NOT_HELD: i32 = 44;
pub const MIN_SERVER_VER_SEC_ID_TYPE: i32 = 45;
pub const MIN_SERVER_VER_PLACE_ORDER_CONID: i32 = 46;
pub const MIN_SERVER_VER_REQ_MKT_DATA_CONID: i32 = 47;
pub const MIN_SERVER_VER_REQ_CALC_IMPLIED_VOLAT: i32 = 49;
pub const MIN_SERVER_VER_REQ_CALC_OPTION_PRICE: i32 = 50;
pub const MIN_SERVER_VER_SSHORTX_OLD: i32 = 51;
pub const MIN_SERVER_VER_SSHORTX: i32 = 52;
pub const MIN_SERVER_VER_REQ_GLOBAL_CANCEL: i32 = 53;
pub const MIN_SERVER_VER_HEDGE_ORDERS: i32 = 54;
pub const MIN_SERVER_VER_REQ_MARKET_DATA_TYPE: i32 = 55;
pub const MIN_SERVER_VER_OPT_OUT_SMART_ROUTING: i32 = 56;
pub const MIN_SERVER_VER_SMART_COMBO_ROUTING_PARAMS: i32 = 57;
pub const MIN_SERVER_VER_DELTA_NEUTRAL_CONID: i32 = 58;
pub const MIN_SERVER_VER_SCALE_ORDERS3: i32 = 60;
pub const MIN_SERVER_VER_ORDER_COMBO_LEGS_PRICE: i32 = 61;
pub const MIN_SERVER_VER_TRAILING_PERCENT: i32 = 62;
pub const MIN_SERVER_VER_DELTA_NEUTRAL_OPEN_CLOSE: i32 = 66;
pub const MIN_SERVER_VER_POSITIONS: i32 = 67;
pub const MIN_SERVER_VER_ACCOUNT_SUMMARY: i32 = 67;
pub const MIN_SERVER_VER_TRADING_CLASS: i32 = 68;
pub const MIN_SERVER_VER_SCALE_TABLE: i32 = 69;
pub const MIN_SERVER_VER_LINKING: i32 = 70;
pub const MIN_SERVER_VER_ALGO_ID: i32 = 71;
pub const MIN_SERVER_VER_OPTIONAL_CAPABILITIES: i32 = 72;
pub const MIN_SERVER_VER_ORDER_SOLICITED: i32 = 73;
pub const MIN_SERVER_VER_LINKING_AUTH: i32 = 74;
pub const MIN_SERVER_VER_PRIMARYEXCH: i32 = 75;
pub const MIN_SERVER_VER_RANDOMIZE_SIZE_AND_PRICE: i32 = 76;
pub const MIN_SERVER_VER_FRACTIONAL_POSITIONS: i32 = 101;
pub const MIN_SERVER_VER_PEGGED_TO_BENCHMARK: i32 = 102;
pub const MIN_SERVER_VER_MODELS_SUPPORT: i32 = 103;
pub const MIN_SERVER_VER_SEC_DEF_OPT_PARAMS_REQ: i32 = 104;
pub const MIN_SERVER_VER_EXT_OPERATOR: i32 = 105;
pub const MIN_SERVER_VER_SOFT_DOLLAR_TIER: i32 = 106;
pub const MIN_SERVER_VER_REQ_FAMILY_CODES: i32 = 107;
pub const MIN_SERVER_VER_REQ_MATCHING_SYMBOLS: i32 = 108;
pub const MIN_SERVER_VER_PAST_LIMIT: i32 = 109;
pub const MIN_SERVER_VER_MD_SIZE_MULTIPLIER: i32 = 110;
pub const MIN_SERVER_VER_CASH_QTY: i32 = 111;
pub const MIN_SERVER_VER_REQ_MKT_DEPTH_EXCHANGES: i32 = 112;
pub const MIN_SERVER_VER_TICK_NEWS: i32 = 113;
pub const MIN_SERVER_VER_REQ_SMART_COMPONENTS: i32 = 114;
pub const MIN_SERVER_VER_REQ_NEWS_PROVIDERS: i32 = 115;
pub const MIN_SERVER_VER_REQ_NEWS_ARTICLE: i32 = 116;
pub const MIN_SERVER_VER_REQ_HISTORICAL_NEWS: i32 = 117;
pub const MIN_SERVER_VER_REQ_HEAD_TIMESTAMP: i32 = 118;
pub const MIN_SERVER_VER_REQ_HISTOGRAM: i32 = 119;
pub const MIN_SERVER_VER_SERVICE_DATA_TYPE: i32 = 120;
pub const MIN_SERVER_VER_AGG_GROUP: i32 = 121;
pub const MIN_SERVER_VER_UNDERLYING_INFO: i32 = 122;
pub const MIN_SERVER_VER_CANCEL_HEADTIMESTAMP: i32 = 123;
pub const MIN_SERVER_VER_SYNT_REALTIME_BARS: i32 = 124;
pub const MIN_SERVER_VER_CFD_REROUTE: i32 = 125;
pub const MIN_SERVER_VER_MARKET_RULES: i32 = 126;
pub const MIN_SERVER_VER_PNL: i32 = 127;
pub const MIN_SERVER_VER_NEWS_QUERY_ORIGINS: i32 = 128;
pub const MIN_SERVER_VER_UNREALIZED_PNL: i32 = 129;
pub const MIN_SERVER_VER_HISTORICAL_TICKS: i32 = 130;
pub const MIN_SERVER_VER_MARKET_CAP_PRICE: i32 = 131;
pub const MIN_SERVER_VER_PRE_OPEN_BID_ASK: i32 = 132;
pub const MIN_SERVER_VER_REAL_EXPIRATION_DATE: i32 = 134;
pub const MIN_SERVER_VER_REALIZED_PNL: i32 = 135;
pub const MIN_SERVER_VER_LAST_LIQUIDITY: i32 = 136;
pub const MIN_SERVER_VER_TICK_BY_TICK: i32 = 137;
pub const MIN_SERVER_VER_DECISION_MAKER: i32 = 138;
pub const MIN_SERVER_VER_MIFID_EXECUTION: i32 = 139;
pub const MIN_SERVER_VER_TICK_BY_TICK_IGNORE_SIZE: i32 = 140;
pub const MIN_SERVER_VER_AUTO_PRICE_FOR_HEDGE: i32 = 141;
pub const MIN_SERVER_VER_WHAT_IF_EXT_FIELDS: i32 = 142;
pub const MIN_SERVER_VER_SCANNER_GENERIC_OPTS: i32 = 143;
pub const MIN_SERVER_VER_API_BIND_ORDER: i32 = 144;
pub const MIN_SERVER_VER_ORDER_CONTAINER: i32 = 145;
pub const MIN_SERVER_VER_SMART_DEPTH: i32 = 146;
pub const MIN_SERVER_VER_REMOVE_NULL_ALL_CASTING: i32 = 147;
pub const MIN_SERVER_VER_D_PEG_ORDERS: i32 = 148;
pub const MIN_SERVER_VER_MKT_DEPTH_PRIM_EXCHANGE: i32 = 149;
pub const MIN_SERVER_VER_COMPLETED_ORDERS: i32 = 150;
pub const MIN_SERVER_VER_PRICE_MGMT_ALGO: i32 = 151;

pub const MIN_SERVER_VER_STOCK_TYPE               : i32 = 152;
pub const MIN_SERVER_VER_ENCODE_MSG_ASCII7        : i32 = 153;
pub const MIN_SERVER_VER_SEND_ALL_FAMILY_CODES    : i32 = 154;
pub const MIN_SERVER_VER_NO_DEFAULT_OPEN_CLOSE    : i32 = 155;
pub const MIN_SERVER_VER_PRICE_BASED_VOLATILITY   : i32 = 156;
pub const MIN_SERVER_VER_REPLACE_FA_END           : i32 = 157
pub const MIN_SERVER_VER_DURATION                 : i32 = 158;
pub const MIN_SERVER_VER_MARKET_DATA_IN_SHARES    : i32 = 159;
pub const MIN_SERVER_VER_POST_TO_ATS              : i32 = 160;
pub const MIN_SERVER_VER_WSHE_CALENDAR           : i32 = 161;
pub const MIN_SERVER_VER_AUTO_CANCEL_PARENT       : i32 = 162;
pub const MIN_SERVER_VER_FRACTIONAL_SIZE_SUPPORT  : i32 = 163;
pub const MIN_SERVER_VER_SIZE_RULES               : i32 = 164;
pub const MIN_SERVER_VER_HISTORICAL_SCHEDULE      : i32 = 165;
pub const MIN_SERVER_VER_ADVANCED_ORDER_REJECT    : i32 = 166;
pub const MIN_SERVER_VER_USER_INFO                : i32 = 167;
pub const MIN_SERVER_VER_CRYPTO_AGGREGATED_TRADES : i32 = 168;
pub const MIN_SERVER_VER_MANUAL_ORDER_TIME        : i32 = 169;
pub const MIN_SERVER_VER_PEGBEST_PEGMID_OFFSETS   : i32 = 170;
pub const MIN_SERVER_VER_WSH_EVENT_DATA_FILTERS   : i32 = 171;
pub const MIN_SERVER_VER_IPO_PRICES               : i32 = 172;
pub const MIN_SERVER_VER_WSH_EVENT_DATA_FILTERS_DATE : i32 = 173;
pub const MIN_SERVER_VER_INSTRUMENT_TIMEZONE         : i32 = 174;
pub const MIN_SERVER_VER_HMDS_MARKET_DATA_IN_SHARES  : i32 = 175;
pub const MIN_SERVER_VER_BOND_ISSUERID               : i32 = 176;
pub const MIN_SERVER_VER_FA_PROFILE_DESUPPORT        : i32 = 177;
// 100+ messaging */
// 100 = enhanced handshake, msg length prefixes

pub const MIN_CLIENT_VER: i32 = 100;
pub const MAX_CLIENT_VER: i32 = MIN_SERVER_VER_FA_PROFILE_DESUPPORT;
