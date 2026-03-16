/**
 * 日期格式化工具函数
 */

/**
 * 智能格式化时间戳为易读格式
 * @param timestamp - 时间戳（毫秒）
 * @returns 格式化后的时间字符串
 */
export function smartFormatDate(timestamp: number): string {
    if (!timestamp) return '';

    const now = new Date();
    const date = new Date(timestamp);

    // 计算今天的开始时间（0点0分0秒）
    const todayStart = new Date(now).setHours(0, 0, 0, 0);

    // 计算目标日期的开始时间
    const targetDateStart = new Date(date).setHours(0, 0, 0, 0);

    // 计算日历天数差
    const dayDiff = Math.round((todayStart - targetDateStart) / (1000 * 60 * 60 * 24));

    // 获取时间组件
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    const hours = String(date.getHours()).padStart(2, '0');
    const minutes = String(date.getMinutes()).padStart(2, '0');

    // 判断时间范围并返回对应格式
    if (dayDiff === 0) {
        // 今天：显示"今天 HH:mm"
        return `今天 ${hours}:${minutes}`;
    } else if (dayDiff === 1) {
        // 昨天：显示"昨天 HH:mm"
        return `昨天 ${hours}:${minutes}`;
    } else if (dayDiff <= 7) {
        // 7天内：显示"X天前"
        return `${dayDiff}天前`;
    } else if (date.getFullYear() === now.getFullYear()) {
        // 今年内：显示"MM-DD HH:mm"
        return `${month}-${day} ${hours}:${minutes}`;
    } else {
        // 往年：显示完整时间
        return `${year}-${month}-${day} ${hours}:${minutes}`;
    }
}

/**
 * 格式化日期为标准格式
 * @param timestamp - 时间戳（毫秒）
 * @param format - 格式化模板，支持 YYYY, MM, DD, HH, mm, ss
 * @returns 格式化后的日期字符串
 */
export function formatDate(timestamp: number, format: string = 'YYYY-MM-DD HH:mm:ss'): string {
    if (!timestamp) return '';

    const date = new Date(timestamp);

    const tokens: Record<string, string> = {
        'YYYY': String(date.getFullYear()),
        'MM': String(date.getMonth() + 1).padStart(2, '0'),
        'DD': String(date.getDate()).padStart(2, '0'),
        'HH': String(date.getHours()).padStart(2, '0'),
        'mm': String(date.getMinutes()).padStart(2, '0'),
        'ss': String(date.getSeconds()).padStart(2, '0'),
    };

    let result = format;
    for (const [token, value] of Object.entries(tokens)) {
        result = result.replace(token, value);
    }

    return result;
}

/**
 * 获取相对时间描述
 * @param timestamp - 时间戳（毫秒）
 * @returns 相对时间描述
 */
export function getRelativeTime(timestamp: number): string {
    if (!timestamp) return '';

    const now = Date.now();
    const diff = now - timestamp;

    const seconds = Math.floor(diff / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);
    const days = Math.floor(hours / 24);
    const months = Math.floor(days / 30);
    const years = Math.floor(days / 365);

    if (seconds < 60) return '刚刚';
    if (minutes < 60) return `${minutes}分钟前`;
    if (hours < 24) return `${hours}小时前`;
    if (days < 30) return `${days}天前`;
    if (months < 12) return `${months}个月前`;
    return `${years}年前`;
}
