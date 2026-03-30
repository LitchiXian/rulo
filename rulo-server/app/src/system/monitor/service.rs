use common::result::{R, success};
use sysinfo::{Disks, System};

use crate::system::monitor::model::{CpuInfo, DiskInfo, MemInfo, RustInfo, ServerInfo, SysInfo};

pub fn get_server_info() -> R<ServerInfo> {
    let mut sys = System::new_all();
    // 需要刷新两次才能获得有效的 CPU 使用率
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_all();

    let cpu_info = {
        let cpus = sys.cpus();
        let cpu_num = cpus.len();
        let cpu_name = cpus
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_default();
        let used: f64 = if cpu_num > 0 {
            cpus.iter().map(|c| c.cpu_usage() as f64).sum::<f64>() / cpu_num as f64
        } else {
            0.0
        };
        CpuInfo {
            cpu_num,
            cpu_name,
            used: round2(used),
            free: round2(100.0 - used),
        }
    };

    let mem_info = {
        let total = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        let used = sys.used_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        let free = total - used;
        let usage = if total > 0.0 {
            used / total * 100.0
        } else {
            0.0
        };
        MemInfo {
            total: round2(total),
            used: round2(used),
            free: round2(free),
            usage: round2(usage),
        }
    };

    let sys_info = {
        let host_name = System::host_name().unwrap_or_else(|| "unknown".into());
        let os_name = System::name().unwrap_or_else(|| "unknown".into());
        let os_arch = System::cpu_arch();
        let os_version = System::long_os_version().unwrap_or_else(|| "unknown".into());
        let uptime = System::uptime();
        SysInfo {
            host_name,
            os_name,
            os_arch,
            os_version,
            uptime,
        }
    };

    let disks = {
        let disk_list = Disks::new_with_refreshed_list();
        disk_list
            .iter()
            .map(|d| {
                let total = d.total_space() as f64 / (1024.0 * 1024.0 * 1024.0);
                let free = d.available_space() as f64 / (1024.0 * 1024.0 * 1024.0);
                let used = total - free;
                let usage = if total > 0.0 {
                    used / total * 100.0
                } else {
                    0.0
                };
                DiskInfo {
                    mount_point: d.mount_point().to_string_lossy().to_string(),
                    fs_type: d.file_system().to_string_lossy().to_string(),
                    total: round2(total),
                    free: round2(free),
                    used: round2(used),
                    usage: round2(usage),
                }
            })
            .collect()
    };

    let rust_info = {
        let pid = sysinfo::get_current_pid().unwrap_or(sysinfo::Pid::from(0));
        let (mem_used, cpu_usage, start_time) = if let Some(proc) = sys.process(pid) {
            (
                proc.memory() as f64 / (1024.0 * 1024.0),
                proc.cpu_usage(),
                proc.start_time(),
            )
        } else {
            (0.0, 0.0, 0)
        };
        let run_time = if start_time > 0 {
            let elapsed = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                .saturating_sub(start_time);
            format_duration(elapsed)
        } else {
            "未知".into()
        };
        RustInfo {
            mem_used: round2(mem_used),
            cpu_usage,
            start_time,
            pid: pid.as_u32(),
            run_time,
        }
    };

    success(ServerInfo {
        cpu: (cpu_info),
        mem: (mem_info),
        sys: (sys_info),
        disks,
        rust: (rust_info),
    })
}

fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

fn format_duration(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let minutes = (secs % 3600) / 60;
    if days > 0 {
        format!("{}天{}小时{}分钟", days, hours, minutes)
    } else if hours > 0 {
        format!("{}小时{}分钟", hours, minutes)
    } else {
        format!("{}分钟", minutes)
    }
}
