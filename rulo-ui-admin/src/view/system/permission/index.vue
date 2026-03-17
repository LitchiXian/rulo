<script setup lang="ts" name="PermissionManage">
import { ref, onMounted } from 'vue'
import { ElMessageBox } from 'element-plus'
import { Search, Plus, Delete, Edit } from '@element-plus/icons-vue'
import permissionApi from '@/api/admin/permission'
import type {
  SysPermission,
  SysPermissionSaveDto,
  SysPermissionUpdateDto,
  SysPermissionListDto,
} from '@/type/permission'

const PERM_TYPE_MAP: Record<number, string> = { 1: '菜单', 2: '按钮', 3: 'API' }

// ---- 列表 ----
const tableData = ref<SysPermission[]>([])
const loading = ref(false)
const queryForm = ref<SysPermissionListDto>({})

const fetchList = async () => {
  loading.value = true
  try {
    tableData.value = await permissionApi.list(queryForm.value)
  } finally {
    loading.value = false
  }
}

const handleSearch = () => fetchList()

const handleReset = () => {
  queryForm.value = {}
  fetchList()
}

// ---- 新增/编辑弹窗 ----
const dialogVisible = ref(false)
const isEdit = ref(false)
const formData = ref<SysPermissionSaveDto & { id?: number }>({
  perm_code: '',
  perm_name: '',
  perm_type: 1,
})

const openAdd = () => {
  isEdit.value = false
  formData.value = { perm_code: '', perm_name: '', perm_type: 1 }
  dialogVisible.value = true
}

const openEdit = (row: SysPermission) => {
  isEdit.value = true
  formData.value = {
    id: row.id,
    perm_code: row.perm_code,
    perm_name: row.perm_name,
    perm_type: row.perm_type,
    remark: row.remark ?? undefined,
  }
  dialogVisible.value = true
}

const handleSave = async () => {
  if (isEdit.value) {
    const dto: SysPermissionUpdateDto = {
      id: formData.value.id!,
      perm_name: formData.value.perm_name || undefined,
      perm_type: formData.value.perm_type,
      remark: formData.value.remark,
    }
    await permissionApi.update(dto)
  } else {
    await permissionApi.save(formData.value as SysPermissionSaveDto)
  }
  dialogVisible.value = false
  fetchList()
}

// ---- 删除 ----
const handleDelete = async (row: SysPermission) => {
  await ElMessageBox.confirm(`确定删除权限「${row.perm_name}」吗？`, '提示', { type: 'warning' })
  await permissionApi.remove({ ids: [row.id] })
  fetchList()
}

onMounted(fetchList)
</script>

<template>
  <div class="page-container">
    <!-- 搜索栏 -->
    <el-card shadow="never" class="search-card">
      <el-form :model="queryForm" inline>
        <el-form-item label="权限编码">
          <el-input v-model="queryForm.perm_code" placeholder="如 sys:user:list" clearable />
        </el-form-item>
        <el-form-item label="权限名">
          <el-input v-model="queryForm.perm_name" placeholder="请输入权限名" clearable />
        </el-form-item>
        <el-form-item label="类型">
          <el-select v-model="queryForm.perm_type" placeholder="请选择" clearable style="width: 120px">
            <el-option label="菜单" :value="1" />
            <el-option label="按钮" :value="2" />
            <el-option label="API" :value="3" />
          </el-select>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" :icon="Search" @click="handleSearch">搜索</el-button>
          <el-button @click="handleReset">重置</el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- 表格 -->
    <el-card shadow="never" class="table-card">
      <div class="table-toolbar">
        <el-button type="primary" :icon="Plus" @click="openAdd">新增权限</el-button>
      </div>

      <el-table :data="tableData" v-loading="loading" stripe border style="width: 100%">
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="perm_code" label="权限编码" min-width="180" />
        <el-table-column prop="perm_name" label="权限名" width="150" />
        <el-table-column prop="perm_type" label="类型" width="80" align="center">
          <template #default="{ row }">
            <el-tag size="small">{{ PERM_TYPE_MAP[row.perm_type] || '未知' }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="create_time" label="创建时间" width="180">
          <template #default="{ row }">
            {{ new Date(row.create_time).toLocaleString('zh-CN') }}
          </template>
        </el-table-column>
        <el-table-column prop="remark" label="备注" min-width="150" show-overflow-tooltip />
        <el-table-column label="操作" width="160" fixed="right" align="center">
          <template #default="{ row }">
            <el-button link type="primary" :icon="Edit" @click="openEdit(row)">编辑</el-button>
            <el-button link type="danger" :icon="Delete" @click="handleDelete(row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>

      <!-- TODO: 后端 permission/list 暂不支持分页 -->
    </el-card>

    <!-- 新增/编辑弹窗 -->
    <el-dialog :title="isEdit ? '编辑权限' : '新增权限'" v-model="dialogVisible" width="500px">
      <el-form :model="formData" label-width="90px">
        <el-form-item label="权限编码" required>
          <el-input v-model="formData.perm_code" placeholder="如 sys:user:list" :disabled="isEdit" />
        </el-form-item>
        <el-form-item label="权限名" required>
          <el-input v-model="formData.perm_name" placeholder="请输入权限名" />
        </el-form-item>
        <el-form-item label="类型" required>
          <el-radio-group v-model="formData.perm_type">
            <el-radio :value="1">菜单</el-radio>
            <el-radio :value="2">按钮</el-radio>
            <el-radio :value="3">API</el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item label="备注">
          <el-input v-model="formData.remark" type="textarea" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSave">确定</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped>
.page-container { display: flex; flex-direction: column; gap: 16px; }
.search-card :deep(.el-card__body) { padding-bottom: 0; }
.table-toolbar { display: flex; justify-content: flex-start; margin-bottom: 16px; }
</style>
