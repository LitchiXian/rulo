<script setup lang="ts" name="RoleManage">
import { ref, onMounted } from 'vue'
import { ElMessageBox } from 'element-plus'
import { Search, Plus, Delete, Edit } from '@element-plus/icons-vue'
import roleApi from '@/api/admin/role'
import type { SysRole, SysRoleSaveDto, SysRoleUpdateDto, SysRoleListDto } from '@/type/role'

// ---- 列表 ----
const tableData = ref<SysRole[]>([])
const loading = ref(false)
const queryForm = ref<SysRoleListDto>({})

const fetchList = async () => {
  loading.value = true
  try {
    tableData.value = await roleApi.list(queryForm.value)
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
const formData = ref<SysRoleSaveDto & { id?: number; is_active?: boolean }>({
  role_name: '',
  role_key: '',
})

const openAdd = () => {
  isEdit.value = false
  formData.value = { role_name: '', role_key: '' }
  dialogVisible.value = true
}

const openEdit = (row: SysRole) => {
  isEdit.value = true
  formData.value = {
    id: row.id,
    role_name: row.role_name,
    role_key: row.role_key,
    is_active: row.is_active,
    remark: row.remark ?? undefined,
  }
  dialogVisible.value = true
}

const handleSave = async () => {
  if (isEdit.value) {
    const dto: SysRoleUpdateDto = {
      id: formData.value.id!,
      role_name: formData.value.role_name || undefined,
      role_key: formData.value.role_key || undefined,
      is_active: formData.value.is_active,
      remark: formData.value.remark,
    }
    await roleApi.update(dto)
  } else {
    await roleApi.save({ role_name: formData.value.role_name, role_key: formData.value.role_key, remark: formData.value.remark })
  }
  dialogVisible.value = false
  fetchList()
}

// ---- 删除 ----
const handleDelete = async (row: SysRole) => {
  await ElMessageBox.confirm(`确定删除角色「${row.role_name}」吗？`, '提示', { type: 'warning' })
  await roleApi.remove({ ids: [row.id] })
  fetchList()
}

// TODO: 后续增加"分配权限"功能，调用 sys_role_permission 关联表接口（后端暂未提供）

onMounted(fetchList)
</script>

<template>
  <div class="page-container">
    <!-- 搜索栏 -->
    <el-card shadow="never" class="search-card">
      <el-form :model="queryForm" inline>
        <el-form-item label="角色名">
          <el-input v-model="queryForm.role_name" placeholder="请输入角色名" clearable />
        </el-form-item>
        <el-form-item label="角色标识">
          <el-input v-model="queryForm.role_key" placeholder="请输入角色标识" clearable />
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
        <el-button type="primary" :icon="Plus" @click="openAdd">新增角色</el-button>
      </div>

      <el-table :data="tableData" v-loading="loading" stripe border style="width: 100%">
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="role_name" label="角色名" width="150" />
        <el-table-column prop="role_key" label="角色标识" width="150" />
        <el-table-column prop="is_super" label="超级管理" width="100" align="center">
          <template #default="{ row }">
            <el-tag v-if="row.is_super" type="warning" size="small">是</el-tag>
            <el-tag v-else type="info" size="small">否</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="is_active" label="状态" width="80" align="center">
          <template #default="{ row }">
            <el-tag :type="row.is_active ? 'success' : 'danger'" size="small">
              {{ row.is_active ? '启用' : '停用' }}
            </el-tag>
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

      <!-- TODO: 后端 role/list 暂不支持分页，后续增加 PageDto 分页参数 -->
    </el-card>

    <!-- 新增/编辑弹窗 -->
    <el-dialog :title="isEdit ? '编辑角色' : '新增角色'" v-model="dialogVisible" width="500px">
      <el-form :model="formData" label-width="90px">
        <el-form-item label="角色名" required>
          <el-input v-model="formData.role_name" placeholder="请输入角色名" />
        </el-form-item>
        <el-form-item label="角色标识" required>
          <el-input v-model="formData.role_key" placeholder="如 admin / editor" :disabled="isEdit" />
        </el-form-item>
        <el-form-item label="状态" v-if="isEdit">
          <el-switch v-model="formData.is_active" />
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
