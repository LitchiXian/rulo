<script setup lang="ts" name="MonitorServer">
import { ref, onMounted } from 'vue'
import { Monitor, Cpu, Coin, Box, Discount } from '@element-plus/icons-vue'
import monitorApi from '@/api/admin/monitor'
import type { ServerInfo } from '@/type/monitor'

const loading = ref(true)
const info = ref<ServerInfo | null>(null)

const fetchInfo = async () => {
  loading.value = true
  try {
    info.value = await monitorApi.getServerInfo()
  } finally {
    loading.value = false
  }
}

const formatUptime = (secs: number): string => {
  const days = Math.floor(secs / 86400)
  const hours = Math.floor((secs % 86400) / 3600)
  const minutes = Math.floor((secs % 3600) / 60)
  if (days > 0) return `${days}天${hours}小时${minutes}分钟`
  if (hours > 0) return `${hours}小时${minutes}分钟`
  return `${minutes}分钟`
}

onMounted(fetchInfo)
</script>

<template>
  <div v-loading="loading" class="monitor-page">
    <template v-if="info">
      <!-- CPU + 内存 -->
      <el-row :gutter="16">
        <el-col :span="12">
          <el-card shadow="never">
            <template #header>
              <div class="card-header">
                <el-icon :size="18" color="#409eff"><Cpu /></el-icon>
                <span>CPU</span>
              </div>
            </template>
            <table class="info-table">
              <thead>
                <tr><th class="label">属性</th><th>值</th></tr>
              </thead>
              <tbody>
                <tr><td class="label">核心数</td><td>{{ info.cpu.cpu_num }}</td></tr>
                <tr><td class="label">型号</td><td>{{ info.cpu.cpu_name }}</td></tr>
                <tr><td class="label">使用率</td><td :class="{ danger: info.cpu.used > 80 }">{{ info.cpu.used }}%</td></tr>
                <tr><td class="label">空闲率</td><td>{{ info.cpu.free }}%</td></tr>
              </tbody>
            </table>
          </el-card>
        </el-col>
        <el-col :span="12">
          <el-card shadow="never">
            <template #header>
              <div class="card-header">
                <el-icon :size="18" color="#67c23a"><Coin /></el-icon>
                <span>内存</span>
              </div>
            </template>
            <table class="info-table">
              <thead>
                <tr><th class="label">属性</th><th>物理内存</th></tr>
              </thead>
              <tbody>
                <tr><td class="label">总内存</td><td>{{ info.mem.total }} GB</td></tr>
                <tr><td class="label">已用内存</td><td>{{ info.mem.used }} GB</td></tr>
                <tr><td class="label">剩余内存</td><td>{{ info.mem.free }} GB</td></tr>
                <tr><td class="label">使用率</td><td :class="{ danger: info.mem.usage > 80 }">{{ info.mem.usage }}%</td></tr>
              </tbody>
            </table>
          </el-card>
        </el-col>
      </el-row>

      <!-- 服务器信息 -->
      <el-card shadow="never" style="margin-top: 16px;">
        <template #header>
          <div class="card-header">
            <el-icon :size="18" color="#e6a23c"><Monitor /></el-icon>
            <span>服务器信息</span>
          </div>
        </template>
        <table class="info-table">
          <tbody>
            <tr>
              <td class="label">主机名</td><td>{{ info.sys.host_name }}</td>
              <td class="label">操作系统</td><td>{{ info.sys.os_name }}</td>
            </tr>
            <tr>
              <td class="label">系统架构</td><td>{{ info.sys.os_arch }}</td>
              <td class="label">系统版本</td><td>{{ info.sys.os_version }}</td>
            </tr>
            <tr>
              <td class="label">系统运行时长</td><td colspan="3">{{ formatUptime(info.sys.uptime) }}</td>
            </tr>
          </tbody>
        </table>
      </el-card>

      <!-- Rust 进程信息 -->
      <el-card shadow="never" style="margin-top: 16px;">
        <template #header>
          <div class="card-header">
            <el-icon :size="18" color="#f56c6c"><Discount /></el-icon>
            <span>Rust 进程信息</span>
          </div>
        </template>
        <table class="info-table">
          <tbody>
            <tr>
              <td class="label">进程 PID</td><td>{{ info.rust.pid }}</td>
              <td class="label">CPU 使用率</td><td>{{ info.rust.cpu_usage.toFixed(2) }}%</td>
            </tr>
            <tr>
              <td class="label">占用内存</td><td>{{ info.rust.mem_used }} MB</td>
              <td class="label">运行时长</td><td>{{ info.rust.run_time }}</td>
            </tr>
          </tbody>
        </table>
      </el-card>

      <!-- 磁盘状态 -->
      <el-card shadow="never" style="margin-top: 16px;">
        <template #header>
          <div class="card-header">
            <el-icon :size="18" color="#909399"><Box /></el-icon>
            <span>磁盘状态</span>
            <el-button type="primary" link style="margin-left: auto;" @click="fetchInfo">
              <el-icon style="margin-right: 4px;"><Monitor /></el-icon>刷新
            </el-button>
          </div>
        </template>
        <el-table :data="info.disks" stripe style="width: 100%;">
          <el-table-column prop="mount_point" label="挂载点" min-width="120" />
          <el-table-column prop="fs_type" label="文件系统" min-width="100" />
          <el-table-column prop="total" label="总大小" min-width="100">
            <template #default="{ row }">{{ row.total }} GB</template>
          </el-table-column>
          <el-table-column prop="free" label="可用" min-width="100">
            <template #default="{ row }">{{ row.free }} GB</template>
          </el-table-column>
          <el-table-column prop="used" label="已用" min-width="100">
            <template #default="{ row }">{{ row.used }} GB</template>
          </el-table-column>
          <el-table-column prop="usage" label="使用率" min-width="140">
            <template #default="{ row }">
              <el-progress
                :percentage="row.usage"
                :color="row.usage > 80 ? '#f56c6c' : row.usage > 60 ? '#e6a23c' : '#67c23a'"
                :stroke-width="16"
                :text-inside="true"
              >
                <span style="font-size: 12px;">{{ row.usage }}%</span>
              </el-progress>
            </template>
          </el-table-column>
        </el-table>
      </el-card>
    </template>
  </div>
</template>

<style scoped>
.monitor-page {
  min-height: 200px;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 600;
}

.info-table {
  width: 100%;
  border-collapse: collapse;
}

.info-table th,
.info-table td {
  padding: 12px 16px;
  text-align: left;
  border-bottom: 1px solid var(--el-border-color-lighter);
  font-size: 14px;
  color: var(--el-text-color-regular);
}

.info-table th {
  color: var(--el-text-color-secondary);
  font-weight: 500;
}

.info-table .label {
  color: var(--el-text-color-secondary);
  width: 130px;
  white-space: nowrap;
}

.danger {
  color: #f56c6c;
  font-weight: 600;
}
</style>
