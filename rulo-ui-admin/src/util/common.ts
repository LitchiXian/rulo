/**
 * 通用工具函数
 * 业务开发高频使用，按需扩展
 */

/**
 * 日期格式化
 * @param time 时间戳/Date/字符串
 * @param pattern 格式，默认 yyyy-MM-dd HH:mm:ss
 */
export function parseTime(time: Date | string | number | null | undefined, pattern = 'yyyy-MM-dd HH:mm:ss'): string {
  if (!time) return ''
  const date = time instanceof Date ? time : new Date(typeof time === 'string' ? time.replace(/-/g, '/') : time)
  if (isNaN(date.getTime())) return ''

  const map: Record<string, number> = {
    yyyy: date.getFullYear(),
    MM: date.getMonth() + 1,
    dd: date.getDate(),
    HH: date.getHours(),
    mm: date.getMinutes(),
    ss: date.getSeconds(),
  }
  return pattern.replace(/yyyy|MM|dd|HH|mm|ss/g, (match) => String(map[match]).padStart(2, '0'))
}

/** 判断值是否为空（null/undefined/空串/空数组/空对象） */
export function isEmpty(value: unknown): boolean {
  if (value === null || value === undefined || value === '') return true
  if (Array.isArray(value) && value.length === 0) return true
  if (typeof value === 'object' && Object.keys(value as object).length === 0) return true
  return false
}

/** 判断是否为外部链接（http/https/mailto/tel） */
export function isExternal(path: string): boolean {
  return /^(https?:|mailto:|tel:)/.test(path)
}

/**
 * 表单重置：将 reactive 对象的每个属性设为初始空值
 * 用法：resetForm(formRef) 先 resetFields，再清理
 */
export function resetForm(formRef: { resetFields: () => void } | undefined) {
  formRef?.resetFields()
}

/**
 * 给查询参数追加日期范围
 * @param params 查询参数对象
 * @param dateRange 日期范围数组 [开始, 结束]
 * @param propName 默认字段前缀
 */
export function addDateRange(params: Record<string, unknown>, dateRange: string[] | undefined, propName = '') {
  const search = { ...params }
  if (dateRange && dateRange.length === 2) {
    const beginKey = propName ? `begin${propName}` : 'beginTime'
    const endKey = propName ? `end${propName}` : 'endTime'
    search[beginKey] = dateRange[0]
    search[endKey] = dateRange[1]
  }
  return search
}

/**
 * 将树形数据扁平化，或将扁平数据构造为树
 * @param data 原始数组
 * @param config id/parentId/children 字段名
 */
export function handleTree<T extends Record<string, any>>(
  data: T[],
  config?: { id?: string; parentId?: string; children?: string },
): T[] {
  const { id = 'id', parentId = 'parentId', children = 'children' } = config || {}
  const map = new Map<unknown, T>()
  const roots: T[] = []

  for (const item of data) {
    map.set(item[id], { ...item, [children]: [] })
  }
  for (const item of data) {
    const node = map.get(item[id])!
    const parent = map.get(item[parentId])
    if (parent) {
      ;(parent as any)[children].push(node)
    } else {
      roots.push(node)
    }
  }
  return roots
}
