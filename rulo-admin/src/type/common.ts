export interface PageResult<T> {
  list: T[]
  total: number
  page_num: number
  page_size: number
}
