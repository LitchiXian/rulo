<script setup lang="ts" name="AuditLog">
import { ref, onMounted } from 'vue'
import { Search, Refresh } from '@element-plus/icons-vue'
import auditApi from '@/api/admin/audit'
import type { SysAuditLog, AuditLogListDto } from '@/type/audit'

const tableData = ref<SysAuditLog[]>([])
const total = ref(0)
const loading = ref(false)
const queryForm = ref<AuditLogListDto>({ page_num: 1, page_size: 10 })
const dateRange = ref<[string, string] | null>(null)

const fetchList = async () => {
  loading.value = true
  try {
    if (dateRange.value) {
      queryForm.value.start_time = dateRange.value[0]
      queryForm.value.end_time = dateRange.value[1]
    } else {
      queryForm.value.start_time = undefined
      queryForm.value.end_time = undefined
    }
    const res = await auditApi.list(queryForm.value)
    tableData.value = res.list
    total.value = res.total
  } finally {
    loading.value = false
  }
}

const handleSearch = () => {
  queryForm.value.page_num = 1
  fetchList()
}

const handleReset = () => {
  queryForm.value = { page_num: 1, page_size: 10 }
  dateRange.value = null
  fetchList()
}

const handlePageChange = (page: number) => {
  queryForm.value.page_num = page
  fetchList()
}

const handleSizeChange = (size: number) => {
  queryForm.value.page_size = size
  queryForm.value.page_num = 1
  fetchList()
}

const methodTagType = (method: string) => {
  const map: Record<string, string> = {
    GET: 'success',
    POST: 'primary',
    PUT: 'warning',
    DELETE: 'danger',
  }
  return (map[method] || 'info') as 'success' | 'primary' | 'warning' | 'danger' | 'info'
}

const statusTagType = (status: number) => {
  if (status >= 200 && status < 300) return 'success'
  if (status >= 400 && status < 500) return 'warning'
  return 'danger'
}

const formatTime = (val: string) => {
  if (!val) return ''
  return new Date(val).toLocaleString('zh-CN', { hour12: false })
}

onMounted(fetchList)
</script>

<template>
  <div class="audit-page">
    <!-- 搜索栏 -->
    <el-card shadow="never" class="search-card">
      <el-form :inline="true" :model="queryForm" @submit.prevent="handleSearch">
        <el-form-item label="用户名">
          <el-input v-model="queryForm.user_name" placeholder="请输入用户名" clearable style="width: 150px" />
        </el-form-item>
        <el-form-item label="请求方法">
          <el-select v-model="queryForm.method" placeholder="全部" clearable style="width: 120px">
            <el-option label="GET" value="GET" />
            <el-option label="POST" value="POST" />
            <el-option label="PUT" value="PUT" />
            <el-option label="DELETE" value="DELETE" />
          </el-select>
        </el-form-item>
        <el-form-item label="请求路径">
          <el-input v-model="queryForm.path" placeholder="请输入路径" clearable style="width: 180px" />
        </el-form-item>
        <el-form-item label="敏感操作">
          <el-select v-model="queryForm.is_sensitive" placeholder="全部" clearable style="width: 100px">
            <el-option label="是" :value="true" />
            <el-option label="否" :value="false" />
          </el-select>
        </el-form-item>
        <el-form-item label="时间范围">
          <el-date-picker
            v-model="dateRange"
            type="datetimerange"
            range-separator="至"
            start-placeholder="开始时间"
            end-placeholder="结束时间"
            value-format="YYYY-MM-DDTHH:mm:ssZ"
            style="width: 340px"
          />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" :icon="Search" @click="handleSearch">搜索</el-button>
          <el-button :icon="Refresh" @click="handleReset">重置</el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- 数据表格 -->
    <el-card shadow="never" class="table-card">
      <el-table v-loading="loading" :data="tableData" stripe border style="width: 100%">
        <el-table-column prop="user_name" label="用户" width="100" show-overflow-tooltip>
          <template #default="{ row }">
            {{ row.user_name || '-' }}
          </template>
        </el-table-column>
        <el-table-column prop="method" label="方法" width="80" align="center">
          <template #default="{ row }">
            <el-tag :type="methodTagType(row.method)" size="small">{{ row.method }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="path" label="请求路径" min-width="220" show-overflow-tooltip />
        <el-table-column prop="status" label="状态码" width="80" align="center">
          <template #default="{ row }">
            <el-tag :type="statusTagType(row.status)" size="small">{{ row.status }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="duration_ms" label="耗时" width="90" align="center">
          <template #default="{ row }">
            <span :class="{ 'text-danger': row.duration_ms > 1000 }">{{ row.duration_ms }}ms</span>
          </template>
        </el-table-column>
        <el-table-column prop="ip" label="IP" width="130" show-overflow-tooltip>
          <template #default="{ row }">
            {{ row.ip || '-' }}
          </template>
        </el-table-column>
        <el-table-column prop="is_sensitive" label="敏感" width="70" align="center">
          <template #default="{ row }">
            <el-tag v-if="row.is_sensitive" type="danger" size="small">是</el-tag>
            <el-tag v-else type="info" size="small">否</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="params" label="请求参数" min-width="200" show-overflow-tooltip>
          <template #default="{ row }">
            {{ row.params || '-' }}
          </template>
        </el-table-column>
        <el-table-column prop="created_time" label="时间" width="170">
          <template #default="{ row }">
            {{ formatTime(row.created_time) }}
          </template>
        </el-table-column>
      </el-table>

      <!-- 分页 -->
      <div class="pagination-wrap">
        <el-pagination
          v-model:current-page="queryForm.page_num"
          v-model:page-size="queryForm.page_size"
          :page-sizes="[10, 20, 50, 100]"
          :total="total"
          layout="total, sizes, prev, pager, next, jumper"
          @current-change="handlePageChange"
          @size-change="handleSizeChange"
        />
      </div>
    </el-card>
  </div>
</template>

<style scoped>
.audit-page {
  padding: 16px;
}

.search-card {
  margin-bottom: 16px;
}

.table-card {
  margin-bottom: 16px;
}

.pagination-wrap {
  display: flex;
  justify-content: flex-end;
  margin-top: 16px;
}

.text-danger {
  color: var(--el-color-danger);
  font-weight: 600;
}
</style>
