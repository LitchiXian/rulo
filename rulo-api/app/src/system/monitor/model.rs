use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ServerInfo {
    pub cpu: CpuInfo,
    pub mem: MemInfo,
    pub sys: SysInfo,
    pub disks: Vec<DiskInfo>,
    pub rust: RustInfo,
}

#[derive(Serialize, ToSchema)]
pub struct CpuInfo {
    // CPU 核心数
    pub cpu_num: usize,
    // CPU 品牌/型号
    pub cpu_name: String,
    // 总使用率(%)
    pub used: f64,
    // 空闲率(%)
    pub free: f64,
}

#[derive(Serialize, ToSchema)]
pub struct MemInfo {
    // 总内存(GB)
    pub total: f64,
    // 已用内存(GB)
    pub used: f64,
    // 剩余内存(GB)
    pub free: f64,
    // 使用率(%)
    pub usage: f64,
}

#[derive(Serialize, ToSchema)]
pub struct SysInfo {
    // 主机名
    pub host_name: String,
    // 操作系统
    pub os_name: String,
    // 系统架构
    pub os_arch: String,
    // 系统版本
    pub os_version: String,
    // 系统启动时间(秒)
    pub uptime: u64,
}

#[derive(Serialize, ToSchema)]
pub struct DiskInfo {
    // 挂载点
    pub mount_point: String,
    // 文件系统类型
    pub fs_type: String,
    // 总大小(GB)
    pub total: f64,
    // 可用大小(GB)
    pub free: f64,
    // 已用大小(GB)
    pub used: f64,
    // 已用百分比(%)
    pub usage: f64,
}

#[derive(Serialize, ToSchema)]
pub struct RustInfo {
    // 进程内存(MB)
    pub mem_used: f64,
    // 进程CPU使用率(%)
    pub cpu_usage: f32,
    // 进程启动时间(unix timestamp)
    pub start_time: u64,
    // 进程PID
    pub pid: u32,
    // 进程运行时长描述
    pub run_time: String,
}
