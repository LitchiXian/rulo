package com.l2.exception;

import lombok.Getter;

/**
 * @Author: Linzx
 * @Date: 2025/7/23 15:09
 * @Desc: 错误码枚举
 * 参考《Java开发手册（黄山版）》错误码规范
 */
public enum ErrorCodeEnum {

    // ====================== 顶级错误码 ======================
    SUCCESS("00000", "一切 ok 正确执行后的返回"),
    CLIENT_ERROR("A0001", "用户端错误"),
    SYSTEM_ERROR("B0001", "系统执行出错"),
    THIRD_PARTY_ERROR("C0001", "调用第三方服务出错"),

    // ====================== 用户端错误 ======================
    USER_REGISTRATION_ERROR("A0100", "用户注册错误"),
    USER_LOGIN_ERROR("A0200", "用户登录异常"),
    ACCESS_PERMISSION_ERROR("A0300", "访问权限异常"),
    REQUEST_PARAM_ERROR("A0400", "用户请求参数错误"),
    USER_REQUEST_ERROR("A0500", "用户请求服务异常"),
    USER_RESOURCE_ERROR("A0600", "用户资源异常"),
    FILE_UPLOAD_ERROR("A0700", "用户上传文件异常"),
    VERSION_ERROR("A0800", "用户当前版本异常"),
    PRIVACY_ERROR("A0900", "用户隐私未授权"),
    DEVICE_ERROR("A1000", "用户设备异常"),

    PRIVACY_AGREEMENT_ERROR("A0101", "用户未同意隐私协议"),
    REGION_RESTRICTED("A0102", "注册国家或地区受限"),
    USERNAME_VALIDATION_FAILED("A0110", "用户名校验失败"),
    USERNAME_EXISTS("A0111", "用户名已存在"),
    USERNAME_SENSITIVE("A0112", "用户名包含敏感词"),
    USERNAME_SPECIAL_CHAR("A0113", "用户名包含特殊字符"),
    PASSWORD_VALIDATION_FAILED("A0120", "密码校验失败"),
    PASSWORD_TOO_SHORT("A0121", "密码长度不够"),
    PASSWORD_TOO_WEAK("A0122", "密码强度不够"),
    VERIFICATION_CODE_ERROR("A0130", "校验码输入错误"),
    SMS_CODE_ERROR("A0131", "短信校验码输入错误"),
    EMAIL_CODE_ERROR("A0132", "邮件校验码输入错误"),
    VOICE_CODE_ERROR("A0133", "语音校验码输入错误"),
    USER_ID_ERROR("A0140", "用户证件异常"),
    ID_TYPE_NOT_SELECTED("A0141", "用户证件类型未选择"),
    ID_CARD_INVALID("A0142", "大陆身份证编号校验非法"),
    PASSPORT_INVALID("A0143", "护照编号校验非法"),
    MILITARY_ID_INVALID("A0144", "军官证编号校验非法"),
    USER_INFO_VALIDATION_FAILED("A0150", "用户基本信息校验失败"),
    PHONE_FORMAT_ERROR("A0151", "手机格式校验失败"),
    ADDRESS_FORMAT_ERROR("A0152", "地址格式校验失败"),
    EMAIL_FORMAT_ERROR("A0153", "邮箱格式校验失败"),
    ACCOUNT_NOT_FOUND("A0201", "用户账户不存在"),
    ACCOUNT_FROZEN("A0202", "用户账户被冻结"),
    ACCOUNT_INVALID("A0203", "用户账户已作废"),
    WRONG_PASSWORD("A0210", "用户密码错误"),
    PASSWORD_ATTEMPTS_EXCEEDED("A0211", "用户输入密码错误次数超限"),
    IDENTITY_VERIFICATION_FAILED("A0220", "用户身份校验失败"),
    FINGERPRINT_VERIFICATION_FAILED("A0221", "用户指纹识别失败"),
    FACE_VERIFICATION_FAILED("A0222", "用户面容识别失败"),
    THIRD_PARTY_AUTH_MISSING("A0223", "用户未获得第三方登录授权"),
    LOGIN_EXPIRED("A0230", "用户登录已过期"),
    VERIFICATION_CODE_MISMATCH("A0240", "用户验证码错误"),
    VERIFICATION_ATTEMPTS_EXCEEDED("A0241", "用户验证码尝试次数超限"),
    UNAUTHORIZED_ACCESS("A0301", "访问未授权"),
    AUTHORIZATION_IN_PROGRESS("A0302", "正在授权中"),
    AUTHORIZATION_REJECTED("A0303", "用户授权申请被拒绝"),
    PRIVACY_BLOCKED("A0310", "因访问对象隐私设置被拦截"),
    AUTHORIZATION_EXPIRED("A0311", "授权已过期"),
    API_PERMISSION_DENIED("A0312", "无权限使用 API"),
    ACCESS_BLOCKED("A0320", "用户访问被拦截"),
    BLACKLISTED_USER("A0321", "黑名单用户"),
    ACCOUNT_SUSPENDED("A0322", "账号被冻结"),
    INVALID_IP_ADDRESS("A0323", "非法 IP 地址"),
    GATEWAY_ACCESS_RESTRICTED("A0324", "网关访问受限"),
    GEO_BLACKLIST("A0325", "地域黑名单"),
    SERVICE_ARREARS("A0330", "服务已欠费"),
    SIGNATURE_ERROR("A0340", "用户签名异常"),
    RSA_SIGNATURE_ERROR("A0341", "RSA 签名错误"),
    MALICIOUS_REDIRECT("A0401", "包含非法恶意跳转链接"),
    INVALID_USER_INPUT("A0402", "无效的用户输入"),
    REQUIRED_PARAM_MISSING("A0410", "请求必填参数为空"),
    ORDER_NUMBER_MISSING("A0411", "用户订单号为空"),
    QUANTITY_MISSING("A0412", "订购数量为空"),
    TIMESTAMP_MISSING("A0413", "缺少时间戳参数"),
    INVALID_TIMESTAMP("A0414", "非法的时间戳参数"),
    PARAM_OUT_OF_RANGE("A0420", "请求参数值超出允许的范围"),
    PARAM_FORMAT_MISMATCH("A0421", "参数格式不匹配"),
    ADDRESS_OUT_OF_SERVICE("A0422", "地址不在服务范围"),
    TIME_OUT_OF_SERVICE("A0423", "时间不在服务范围"),
    AMOUNT_EXCEEDED("A0424", "金额超出限制"),
    QUANTITY_EXCEEDED("A0425", "数量超出限制"),
    BATCH_LIMIT_EXCEEDED("A0426", "请求批量处理总个数超出限制"),
    JSON_PARSE_ERROR("A0427", "请求 JSON 解析失败"),
    ILLEGAL_CONTENT("A0430", "用户输入内容非法"),
    SENSITIVE_WORDS("A0431", "包含违禁敏感词"),
    PROHIBITED_IMAGE("A0432", "图片包含违禁信息"),
    COPYRIGHT_VIOLATION("A0433", "文件侵犯版权"),
    USER_OPERATION_ERROR("A0440", "用户操作异常"),
    PAYMENT_TIMEOUT("A0441", "用户支付超时"),
    ORDER_CONFIRMATION_TIMEOUT("A0442", "确认订单超时"),
    ORDER_CLOSED("A0443", "订单已关闭"),
    REQUEST_LIMIT_EXCEEDED("A0501", "请求次数超出限制"),
    CONCURRENT_REQUESTS_EXCEEDED("A0502", "请求并发数超出限制"),
    OPERATION_WAIT_REQUIRED("A0503", "用户操作请等待"),
    WEBSOCKET_CONNECTION_ERROR("A0504", "WebSocket 连接异常"),
    WEBSOCKET_DISCONNECTED("A0505", "WebSocket 连接断开"),
    DUPLICATE_REQUEST("A0506", "用户重复请求"),
    INSUFFICIENT_BALANCE("A0601", "账户余额不足"),
    INSUFFICIENT_DISK_SPACE("A0602", "用户磁盘空间不足"),
    INSUFFICIENT_MEMORY("A0603", "用户内存空间不足"),
    INSUFFICIENT_OSS_CAPACITY("A0604", "用户 OSS 容量不足"),
    QUOTA_EXHAUSTED("A0605", "用户配额已用光"),
    FILE_TYPE_MISMATCH("A0701", "用户上传文件类型不匹配"),
    FILE_TOO_LARGE("A0702", "用户上传文件太大"),
    IMAGE_TOO_LARGE("A0703", "用户上传图片太大"),
    VIDEO_TOO_LARGE("A0704", "用户上传视频太大"),
    COMPRESSED_FILE_TOO_LARGE("A0705", "用户上传压缩文件太大"),
    VERSION_MISMATCH("A0801", "用户安装版本与系统不匹配"),
    VERSION_TOO_LOW("A0802", "用户安装版本过低"),
    VERSION_TOO_HIGH("A0803", "用户安装版本过高"),
    VERSION_EXPIRED("A0804", "用户安装版本已过期"),
    API_VERSION_MISMATCH("A0805", "用户 API 请求版本不匹配"),
    API_VERSION_TOO_HIGH("A0806", "用户 API 请求版本过高"),
    API_VERSION_TOO_LOW("A0807", "用户 API 请求版本过低"),
    PRIVACY_NOT_SIGNED("A0901", "用户隐私未签署"),
    CAMERA_NOT_AUTHORIZED("A0902", "用户摄像头未授权"),
    CAMERA_ACCESS_DENIED("A0903", "用户相机未授权"),
    GALLERY_ACCESS_DENIED("A0904", "用户图片库未授权"),
    FILE_ACCESS_DENIED("A0905", "用户文件未授权"),
    LOCATION_ACCESS_DENIED("A0906", "用户位置信息未授权"),
    CONTACTS_ACCESS_DENIED("A0907", "用户通讯录未授权"),
    CAMERA_ERROR("A1001", "用户相机异常"),
    MICROPHONE_ERROR("A1002", "用户麦克风异常"),
    EARPIECE_ERROR("A1003", "用户听筒异常"),
    SPEAKER_ERROR("A1004", "用户扬声器异常"),
    GPS_ERROR("A1005", "用户 GPS 定位异常"),

    // ====================== 系统错误 ======================
    SYSTEM_TIMEOUT("B0100", "系统执行超时"),
    DISASTER_RECOVERY("B0200", "系统容灾功能被触发"),
    SYSTEM_RESOURCE_ERROR("B0300", "系统资源异常"),
    ORDER_PROCESSING_TIMEOUT("B0101", "系统订单处理超时"),
    RATE_LIMITING("B0210", "系统限流"),
    FUNCTION_DEGRADATION("B0220", "系统功能降级"),
    RESOURCE_EXHAUSTED("B0310", "系统资源耗尽"),
    DISK_SPACE_EXHAUSTED("B0311", "系统磁盘空间耗尽"),
    MEMORY_EXHAUSTED("B0312", "系统内存耗尽"),
    FILE_HANDLES_EXHAUSTED("B0313", "文件句柄耗尽"),
    CONNECTION_POOL_EXHAUSTED("B0314", "系统连接池耗尽"),
    THREAD_POOL_EXHAUSTED("B0315", "系统线程池耗尽"),
    RESOURCE_ACCESS_ERROR("B0320", "系统资源访问异常"),
    FILE_READ_ERROR("B0321", "系统读取磁盘文件失败"),

    // ====================== 第三方服务错误 ======================
    MIDDLEWARE_ERROR("C0100", "中间件服务出错"),
    THIRD_PARTY_TIMEOUT("C0200", "第三方系统执行超时"),
    DATABASE_ERROR("C0300", "数据库服务出错"),
    THIRD_PARTY_DISASTER("C0400", "第三方容灾系统被触发"),
    NOTIFICATION_ERROR("C0500", "通知服务出错"),
    RPC_ERROR("C0110", "RPC 服务出错"),
    RPC_SERVICE_NOT_FOUND("C0111", "RPC 服务未找到"),
    RPC_SERVICE_NOT_REGISTERED("C0112", "RPC 服务未注册"),
    INTERFACE_NOT_FOUND("C0113", "接口不存在"),
    MESSAGE_SERVICE_ERROR("C0120", "消息服务出错"),
    MESSAGE_DELIVERY_ERROR("C0121", "消息投递出错"),
    MESSAGE_CONSUMPTION_ERROR("C0122", "消息消费出错"),
    MESSAGE_SUBSCRIPTION_ERROR("C0123", "消息订阅出错"),
    MESSAGE_GROUP_NOT_FOUND("C0124", "消息分组未查到"),
    CACHE_SERVICE_ERROR("C0130", "缓存服务出错"),
    KEY_TOO_LONG("C0131", "key 长度超过限制"),
    VALUE_TOO_LONG("C0132", "value 长度超过限制"),
    STORAGE_FULL("C0133", "存储容量已满"),
    UNSUPPORTED_DATA_FORMAT("C0134", "不支持的数据格式"),
    CONFIG_SERVICE_ERROR("C0140", "配置服务出错"),
    NETWORK_RESOURCE_ERROR("C0150", "网络资源服务出错"),
    VPN_ERROR("C0151", "VPN 服务出错"),
    CDN_ERROR("C0152", "CDN 服务出错"),
    DNS_ERROR("C0153", "域名解析服务出错"),
    GATEWAY_ERROR("C0154", "网关服务出错"),
    RPC_TIMEOUT("C0210", "RPC 执行超时"),
    MESSAGE_DELIVERY_TIMEOUT("C0220", "消息投递超时"),
    CACHE_TIMEOUT("C0230", "缓存服务超时"),
    CONFIG_TIMEOUT("C0240", "配置服务超时"),
    DATABASE_TIMEOUT("C0250", "数据库服务超时"),
    TABLE_NOT_FOUND("C0311", "表不存在"),
    COLUMN_NOT_FOUND("C0312", "列不存在"),
    DUPLICATE_COLUMN_NAME("C0321", "多表关联中存在多个相同名称的列"),
    DATABASE_DEADLOCK("C0331", "数据库死锁"),
    PRIMARY_KEY_CONFLICT("C0341", "主键冲突"),
    THIRD_PARTY_RATE_LIMITING("C0401", "第三方系统限流"),
    THIRD_PARTY_DEGRADATION("C0402", "第三方功能降级"),
    SMS_NOTIFICATION_FAILED("C0501", "短信提醒服务失败"),
    VOICE_NOTIFICATION_FAILED("C0502", "语音提醒服务失败"),
    EMAIL_NOTIFICATION_FAILED("C0503", "邮件提醒服务失败");

    @Getter
    private final String code;

    @Getter
    private final String message;

    ErrorCodeEnum(String code, String message) {
        this.code = code;
        this.message = message;
    }

    /**
     * 根据错误码查找枚举
     */
    public static ErrorCodeEnum fromCode(String code) {
        for (ErrorCodeEnum errorCodeEnum : values()) {
            if (errorCodeEnum.code.equals(code)) {
                return errorCodeEnum;
            }
        }
        throw new IllegalArgumentException("未知错误码: " + code);
    }
}