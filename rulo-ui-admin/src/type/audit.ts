export interface SysAuditLog {
  id: number
  user_id: number | null
  user_name: string | null
  method: string
  path: string
  params: string | null
  status: number
  duration_ms: number
  ip: string | null
  is_sensitive: boolean
  created_time: string
}

export interface AuditLogListDto {
  user_name?: string
  method?: string
  path?: string
  is_sensitive?: boolean
  start_time?: string
  end_time?: string
  page_num?: number
  page_size?: number
}
