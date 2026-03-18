export interface ServerInfo {
  cpu: CpuInfo
  mem: MemInfo
  sys: SysInfo
  disks: DiskInfo[]
  rust: RustInfo
}

export interface CpuInfo {
  cpu_num: number
  cpu_name: string
  used: number
  free: number
}

export interface MemInfo {
  total: number
  used: number
  free: number
  usage: number
}

export interface SysInfo {
  host_name: string
  os_name: string
  os_arch: string
  os_version: string
  uptime: number
}

export interface DiskInfo {
  mount_point: string
  fs_type: string
  total: number
  free: number
  used: number
  usage: number
}

export interface RustInfo {
  mem_used: number
  cpu_usage: number
  start_time: number
  pid: number
  run_time: string
}
