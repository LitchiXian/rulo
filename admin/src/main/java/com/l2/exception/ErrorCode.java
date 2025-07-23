package com.l2.exception;

/**
 * @Author: Linzx
 * @Date: 2025/7/23 15:09
 * @Desc: 错误码常量
 * 参考《Java开发手册（黄山版）》错误码规范
 */
public interface ErrorCode {

    /**
     * 00000: 一切 ok 正确执行后的返回
     */
    String SUCCESS = "00000";

    /**
     * A0001: 用户端错误
     */
    String CLIENT_ERROR = "A0001";

    /**
     * B0001: 系统执行出错
     */
    String SYSTEM_ERROR = "B0001";

    /**
     * C0001: 调用第三方服务出错
     */
    String THIRD_PARTY_ERROR = "C0001";

    // ====================== 用户端错误 ======================
    interface User {
        /**
         * A0100: 用户注册错误
         */
        String USER_REGISTRATION_ERROR = "A0100";

        /**
         * A0200: 用户登录异常
         */
        String USER_LOGIN_ERROR = "A0200";

        /**
         * A0300: 访问权限异常
         */
        String ACCESS_PERMISSION_ERROR = "A0300";

        /**
         * A0400: 用户请求参数错误
         */
        String REQUEST_PARAM_ERROR = "A0400";

        /**
         * A0500: 用户请求服务异常
         */
        String USER_REQUEST_ERROR = "A0500";

        /**
         * A0600: 用户资源异常
         */
        String USER_RESOURCE_ERROR = "A0600";

        /**
         * A0700: 用户上传文件异常
         */
        String FILE_UPLOAD_ERROR = "A0700";

        /**
         * A0800: 用户当前版本异常
         */
        String VERSION_ERROR = "A0800";

        /**
         * A0900: 用户隐私未授权
         */
        String PRIVACY_ERROR = "A0900";

        /**
         * A1000: 用户设备异常
         */
        String DEVICE_ERROR = "A1000";

        /**
         * A0101: 用户未同意隐私协议
         */
        String PRIVACY_AGREEMENT_ERROR = "A0101";

        /**
         * A0102: 注册国家或地区受限
         */
        String REGION_RESTRICTED = "A0102";

        /**
         * A0110: 用户名校验失败
         */
        String USERNAME_VALIDATION_FAILED = "A0110";

        /**
         * A0111: 用户名已存在
         */
        String USERNAME_EXISTS = "A0111";

        /**
         * A0112: 用户名包含敏感词
         */
        String USERNAME_SENSITIVE = "A0112";

        /**
         * A0113: 用户名包含特殊字符
         */
        String USERNAME_SPECIAL_CHAR = "A0113";

        /**
         * A0120: 密码校验失败
         */
        String PASSWORD_VALIDATION_FAILED = "A0120";

        /**
         * A0121: 密码长度不够
         */
        String PASSWORD_TOO_SHORT = "A0121";

        /**
         * A0122: 密码强度不够
         */
        String PASSWORD_TOO_WEAK = "A0122";

        /**
         * A0130: 校验码输入错误
         */
        String VERIFICATION_CODE_ERROR = "A0130";

        /**
         * A0131: 短信校验码输入错误
         */
        String SMS_CODE_ERROR = "A0131";

        /**
         * A0132: 邮件校验码输入错误
         */
        String EMAIL_CODE_ERROR = "A0132";

        /**
         * A0133: 语音校验码输入错误
         */
        String VOICE_CODE_ERROR = "A0133";

        /**
         * A0140: 用户证件异常
         */
        String USER_ID_ERROR = "A0140";

        /**
         * A0141: 用户证件类型未选择
         */
        String ID_TYPE_NOT_SELECTED = "A0141";

        /**
         * A0142: 大陆身份证编号校验非法
         */
        String ID_CARD_INVALID = "A0142";

        /**
         * A0143: 护照编号校验非法
         */
        String PASSPORT_INVALID = "A0143";

        /**
         * A0144: 军官证编号校验非法
         */
        String MILITARY_ID_INVALID = "A0144";

        /**
         * A0150: 用户基本信息校验失败
         */
        String USER_INFO_VALIDATION_FAILED = "A0150";

        /**
         * A0151: 手机格式校验失败
         */
        String PHONE_FORMAT_ERROR = "A0151";

        /**
         * A0152: 地址格式校验失败
         */
        String ADDRESS_FORMAT_ERROR = "A0152";

        /**
         * A0153: 邮箱格式校验失败
         */
        String EMAIL_FORMAT_ERROR = "A0153";

        /**
         * A0201: 用户账户不存在
         */
        String ACCOUNT_NOT_FOUND = "A0201";

        /**
         * A0202: 用户账户被冻结
         */
        String ACCOUNT_FROZEN = "A0202";

        /**
         * A0203: 用户账户已作废
         */
        String ACCOUNT_INVALID = "A0203";

        /**
         * A0210: 用户密码错误
         */
        String WRONG_PASSWORD = "A0210";

        /**
         * A0211: 用户输入密码错误次数超限
         */
        String PASSWORD_ATTEMPTS_EXCEEDED = "A0211";

        /**
         * A0220: 用户身份校验失败
         */
        String IDENTITY_VERIFICATION_FAILED = "A0220";

        /**
         * A0221: 用户指纹识别失败
         */
        String FINGERPRINT_VERIFICATION_FAILED = "A0221";

        /**
         * A0222: 用户面容识别失败
         */
        String FACE_VERIFICATION_FAILED = "A0222";

        /**
         * A0223: 用户未获得第三方登录授权
         */
        String THIRD_PARTY_AUTH_MISSING = "A0223";

        /**
         * A0230: 用户登录已过期
         */
        String LOGIN_EXPIRED = "A0230";

        /**
         * A0240: 用户验证码错误
         */
        String VERIFICATION_CODE_MISMATCH = "A0240";

        /**
         * A0241: 用户验证码尝试次数超限
         */
        String VERIFICATION_ATTEMPTS_EXCEEDED = "A0241";

        /**
         * A0301: 访问未授权
         */
        String UNAUTHORIZED_ACCESS = "A0301";

        /**
         * A0302: 正在授权中
         */
        String AUTHORIZATION_IN_PROGRESS = "A0302";

        /**
         * A0303: 用户授权申请被拒绝
         */
        String AUTHORIZATION_REJECTED = "A0303";

        /**
         * A0310: 因访问对象隐私设置被拦截
         */
        String PRIVACY_BLOCKED = "A0310";

        /**
         * A0311: 授权已过期
         */
        String AUTHORIZATION_EXPIRED = "A0311";

        /**
         * A0312: 无权限使用 API
         */
        String API_PERMISSION_DENIED = "A0312";

        /**
         * A0320: 用户访问被拦截
         */
        String ACCESS_BLOCKED = "A0320";

        /**
         * A0321: 黑名单用户
         */
        String BLACKLISTED_USER = "A0321";

        /**
         * A0322: 账号被冻结
         */
        String ACCOUNT_SUSPENDED = "A0322";

        /**
         * A0323: 非法 IP 地址
         */
        String INVALID_IP_ADDRESS = "A0323";

        /**
         * A0324: 网关访问受限
         */
        String GATEWAY_ACCESS_RESTRICTED = "A0324";

        /**
         * A0325: 地域黑名单
         */
        String GEO_BLACKLIST = "A0325";

        /**
         * A0330: 服务已欠费
         */
        String SERVICE_ARREARS = "A0330";

        /**
         * A0340: 用户签名异常
         */
        String SIGNATURE_ERROR = "A0340";

        /**
         * A0341: RSA 签名错误
         */
        String RSA_SIGNATURE_ERROR = "A0341";

        /**
         * A0401: 包含非法恶意跳转链接
         */
        String MALICIOUS_REDIRECT = "A0401";

        /**
         * A0402: 无效的用户输入
         */
        String INVALID_USER_INPUT = "A0402";

        /**
         * A0410: 请求必填参数为空
         */
        String REQUIRED_PARAM_MISSING = "A0410";

        /**
         * A0411: 用户订单号为空
         */
        String ORDER_NUMBER_MISSING = "A0411";

        /**
         * A0412: 订购数量为空
         */
        String QUANTITY_MISSING = "A0412";

        /**
         * A0413: 缺少时间戳参数
         */
        String TIMESTAMP_MISSING = "A0413";

        /**
         * A0414: 非法的时间戳参数
         */
        String INVALID_TIMESTAMP = "A0414";

        /**
         * A0420: 请求参数值超出允许的范围
         */
        String PARAM_OUT_OF_RANGE = "A0420";

        /**
         * A0421: 参数格式不匹配
         */
        String PARAM_FORMAT_MISMATCH = "A0421";

        /**
         * A0422: 地址不在服务范围
         */
        String ADDRESS_OUT_OF_SERVICE = "A0422";

        /**
         * A0423: 时间不在服务范围
         */
        String TIME_OUT_OF_SERVICE = "A0423";

        /**
         * A0424: 金额超出限制
         */
        String AMOUNT_EXCEEDED = "A0424";

        /**
         * A0425: 数量超出限制
         */
        String QUANTITY_EXCEEDED = "A0425";

        /**
         * A0426: 请求批量处理总个数超出限制
         */
        String BATCH_LIMIT_EXCEEDED = "A0426";

        /**
         * A0427: 请求 JSON 解析失败
         */
        String JSON_PARSE_ERROR = "A0427";

        /**
         * A0430: 用户输入内容非法
         */
        String ILLEGAL_CONTENT = "A0430";

        /**
         * A0431: 包含违禁敏感词
         */
        String SENSITIVE_WORDS = "A0431";

        /**
         * A0432: 图片包含违禁信息
         */
        String PROHIBITED_IMAGE = "A0432";

        /**
         * A0433: 文件侵犯版权
         */
        String COPYRIGHT_VIOLATION = "A0433";

        /**
         * A0440: 用户操作异常
         */
        String USER_OPERATION_ERROR = "A0440";

        /**
         * A0441: 用户支付超时
         */
        String PAYMENT_TIMEOUT = "A0441";

        /**
         * A0442: 确认订单超时
         */
        String ORDER_CONFIRMATION_TIMEOUT = "A0442";

        /**
         * A0443: 订单已关闭
         */
        String ORDER_CLOSED = "A0443";

        /**
         * A0501: 请求次数超出限制
         */
        String REQUEST_LIMIT_EXCEEDED = "A0501";

        /**
         * A0502: 请求并发数超出限制
         */
        String CONCURRENT_REQUESTS_EXCEEDED = "A0502";

        /**
         * A0503: 用户操作请等待
         */
        String OPERATION_WAIT_REQUIRED = "A0503";

        /**
         * A0504: WebSocket 连接异常
         */
        String WEBSOCKET_CONNECTION_ERROR = "A0504";

        /**
         * A0505: WebSocket 连接断开
         */
        String WEBSOCKET_DISCONNECTED = "A0505";

        /**
         * A0506: 用户重复请求
         */
        String DUPLICATE_REQUEST = "A0506";

        /**
         * A0601: 账户余额不足
         */
        String INSUFFICIENT_BALANCE = "A0601";

        /**
         * A0602: 用户磁盘空间不足
         */
        String INSUFFICIENT_DISK_SPACE = "A0602";

        /**
         * A0603: 用户内存空间不足
         */
        String INSUFFICIENT_MEMORY = "A0603";

        /**
         * A0604: 用户 OSS 容量不足
         */
        String INSUFFICIENT_OSS_CAPACITY = "A0604";

        /**
         * A0605: 用户配额已用光
         */
        String QUOTA_EXHAUSTED = "A0605";

        /**
         * A0701: 用户上传文件类型不匹配
         */
        String FILE_TYPE_MISMATCH = "A0701";

        /**
         * A0702: 用户上传文件太大
         */
        String FILE_TOO_LARGE = "A0702";

        /**
         * A0703: 用户上传图片太大
         */
        String IMAGE_TOO_LARGE = "A0703";

        /**
         * A0704: 用户上传视频太大
         */
        String VIDEO_TOO_LARGE = "A0704";

        /**
         * A0705: 用户上传压缩文件太大
         */
        String COMPRESSED_FILE_TOO_LARGE = "A0705";

        /**
         * A0801: 用户安装版本与系统不匹配
         */
        String VERSION_MISMATCH = "A0801";

        /**
         * A0802: 用户安装版本过低
         */
        String VERSION_TOO_LOW = "A0802";

        /**
         * A0803: 用户安装版本过高
         */
        String VERSION_TOO_HIGH = "A0803";

        /**
         * A0804: 用户安装版本已过期
         */
        String VERSION_EXPIRED = "A0804";

        /**
         * A0805: 用户 API 请求版本不匹配
         */
        String API_VERSION_MISMATCH = "A0805";

        /**
         * A0806: 用户 API 请求版本过高
         */
        String API_VERSION_TOO_HIGH = "A0806";

        /**
         * A0807: 用户 API 请求版本过低
         */
        String API_VERSION_TOO_LOW = "A0807";

        /**
         * A0901: 用户隐私未签署
         */
        String PRIVACY_NOT_SIGNED = "A0901";

        /**
         * A0902: 用户摄像头未授权
         */
        String CAMERA_NOT_AUTHORIZED = "A0902";

        /**
         * A0903: 用户相机未授权
         */
        String CAMERA_ACCESS_DENIED = "A0903";

        /**
         * A0904: 用户图片库未授权
         */
        String GALLERY_ACCESS_DENIED = "A0904";

        /**
         * A0905: 用户文件未授权
         */
        String FILE_ACCESS_DENIED = "A0905";

        /**
         * A0906: 用户位置信息未授权
         */
        String LOCATION_ACCESS_DENIED = "A0906";

        /**
         * A0907: 用户通讯录未授权
         */
        String CONTACTS_ACCESS_DENIED = "A0907";

        /**
         * A1001: 用户相机异常
         */
        String CAMERA_ERROR = "A1001";

        /**
         * A1002: 用户麦克风异常
         */
        String MICROPHONE_ERROR = "A1002";

        /**
         * A1003: 用户听筒异常
         */
        String EARPIECE_ERROR = "A1003";

        /**
         * A1004: 用户扬声器异常
         */
        String SPEAKER_ERROR = "A1004";

        /**
         * A1005: 用户 GPS 定位异常
         */
        String GPS_ERROR = "A1005";

    }

    interface System {
        /**
         * B0100: 系统执行超时
         */
        String SYSTEM_TIMEOUT = "B0100";

        /**
         * B0200: 系统容灾功能被触发
         */
        String DISASTER_RECOVERY = "B0200";

        /**
         * B0300: 系统资源异常
         */
        String SYSTEM_RESOURCE_ERROR = "B0300";

        /**
         * B0101: 系统订单处理超时
         */
        String ORDER_PROCESSING_TIMEOUT = "B0101";

        /**
         * B0210: 系统限流
         */
        String RATE_LIMITING = "B0210";

        /**
         * B0220: 系统功能降级
         */
        String FUNCTION_DEGRADATION = "B0220";

        /**
         * B0310: 系统资源耗尽
         */
        String RESOURCE_EXHAUSTED = "B0310";

        /**
         * B0311: 系统磁盘空间耗尽
         */
        String DISK_SPACE_EXHAUSTED = "B0311";

        /**
         * B0312: 系统内存耗尽
         */
        String MEMORY_EXHAUSTED = "B0312";

        /**
         * B0313: 文件句柄耗尽
         */
        String FILE_HANDLES_EXHAUSTED = "B0313";

        /**
         * B0314: 系统连接池耗尽
         */
        String CONNECTION_POOL_EXHAUSTED = "B0314";

        /**
         * B0315: 系统线程池耗尽
         */
        String THREAD_POOL_EXHAUSTED = "B0315";

        /**
         * B0320: 系统资源访问异常
         */
        String RESOURCE_ACCESS_ERROR = "B0320";

        /**
         * B0321: 系统读取磁盘文件失败
         */
        String FILE_READ_ERROR = "B0321";

    }

    interface ThirdParty {


        /**
         * C0100: 中间件服务出错
         */
        String MIDDLEWARE_ERROR = "C0100";

        /**
         * C0200: 第三方系统执行超时
         */
        String THIRD_PARTY_TIMEOUT = "C0200";

        /**
         * C0300: 数据库服务出错
         */
        String DATABASE_ERROR = "C0300";

        /**
         * C0400: 第三方容灾系统被触发
         */
        String THIRD_PARTY_DISASTER = "C0400";

        /**
         * C0500: 通知服务出错
         */
        String NOTIFICATION_ERROR = "C0500";


        /**
         * C0110: RPC 服务出错
         */
        String RPC_ERROR = "C0110";

        /**
         * C0111: RPC 服务未找到
         */
        String RPC_SERVICE_NOT_FOUND = "C0111";

        /**
         * C0112: RPC 服务未注册
         */
        String RPC_SERVICE_NOT_REGISTERED = "C0112";

        /**
         * C0113: 接口不存在
         */
        String INTERFACE_NOT_FOUND = "C0113";

        /**
         * C0120: 消息服务出错
         */
        String MESSAGE_SERVICE_ERROR = "C0120";

        /**
         * C0121: 消息投递出错
         */
        String MESSAGE_DELIVERY_ERROR = "C0121";

        /**
         * C0122: 消息消费出错
         */
        String MESSAGE_CONSUMPTION_ERROR = "C0122";

        /**
         * C0123: 消息订阅出错
         */
        String MESSAGE_SUBSCRIPTION_ERROR = "C0123";

        /**
         * C0124: 消息分组未查到
         */
        String MESSAGE_GROUP_NOT_FOUND = "C0124";

        /**
         * C0130: 缓存服务出错
         */
        String CACHE_SERVICE_ERROR = "C0130";

        /**
         * C0131: key 长度超过限制
         */
        String KEY_TOO_LONG = "C0131";

        /**
         * C0132: value 长度超过限制
         */
        String VALUE_TOO_LONG = "C0132";

        /**
         * C0133: 存储容量已满
         */
        String STORAGE_FULL = "C0133";

        /**
         * C0134: 不支持的数据格式
         */
        String UNSUPPORTED_DATA_FORMAT = "C0134";

        /**
         * C0140: 配置服务出错
         */
        String CONFIG_SERVICE_ERROR = "C0140";

        /**
         * C0150: 网络资源服务出错
         */
        String NETWORK_RESOURCE_ERROR = "C0150";

        /**
         * C0151: VPN 服务出错
         */
        String VPN_ERROR = "C0151";

        /**
         * C0152: CDN 服务出错
         */
        String CDN_ERROR = "C0152";

        /**
         * C0153: 域名解析服务出错
         */
        String DNS_ERROR = "C0153";

        /**
         * C0154: 网关服务出错
         */
        String GATEWAY_ERROR = "C0154";

        /**
         * C0210: RPC 执行超时
         */
        String RPC_TIMEOUT = "C0210";

        /**
         * C0220: 消息投递超时
         */
        String MESSAGE_DELIVERY_TIMEOUT = "C0220";

        /**
         * C0230: 缓存服务超时
         */
        String CACHE_TIMEOUT = "C0230";

        /**
         * C0240: 配置服务超时
         */
        String CONFIG_TIMEOUT = "C0240";

        /**
         * C0250: 数据库服务超时
         */
        String DATABASE_TIMEOUT = "C0250";

        /**
         * C0311: 表不存在
         */
        String TABLE_NOT_FOUND = "C0311";

        /**
         * C0312: 列不存在
         */
        String COLUMN_NOT_FOUND = "C0312";

        /**
         * C0321: 多表关联中存在多个相同名称的列
         */
        String DUPLICATE_COLUMN_NAME = "C0321";

        /**
         * C0331: 数据库死锁
         */
        String DATABASE_DEADLOCK = "C0331";

        /**
         * C0341: 主键冲突
         */
        String PRIMARY_KEY_CONFLICT = "C0341";

        /**
         * C0401: 第三方系统限流
         */
        String THIRD_PARTY_RATE_LIMITING = "C0401";

        /**
         * C0402: 第三方功能降级
         */
        String THIRD_PARTY_DEGRADATION = "C0402";

        /**
         * C0501: 短信提醒服务失败
         */
        String SMS_NOTIFICATION_FAILED = "C0501";

        /**
         * C0502: 语音提醒服务失败
         */
        String VOICE_NOTIFICATION_FAILED = "C0502";

        /**
         * C0503: 邮件提醒服务失败
         */
        String EMAIL_NOTIFICATION_FAILED = "C0503";
    }

}
