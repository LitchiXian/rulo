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

const PERM_TYPE_MAP: Record<number, string> = { 1: 'API权限', 2: '菜单入口' }

// ---- 列表 ----
const tableData = ref<SysPermission[]>([])
const total = ref(0)
const loading = ref(false)
const queryForm = ref<SysPermissionListDto>({ page_num: 1, page_size: 10 })

const fetchList = async () => {
  loading.value = true
  try {
    const res = await permissionApi.list(queryForm.value)
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
            <el-option label="API权限" :value="1" />
            <el-option label="菜单入口" :value="2" />
          </el-select>
        </el-form-item>
        <el-form-item>
          <el-button v-auth="'sys:permission:list'" type="primary" :icon="Search" @click="handleSearch">搜索</el-button>
          <el-button @click="handleReset">重置</el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- 表格 -->
    <el-card shadow="never" class="table-card">
      <div class="table-toolbar">
        <el-button v-auth="'sys:permission:save'" type="primary" :icon="Plus" @click="openAdd">新增权限</el-button>
      </div>

      <el-table :data="tableData" v-loading="loading" stripe border style="width: 100%">
        <el-table-column prop="perm_code" label="权限编码" min-width="150" />
        <el-table-column prop="perm_name" label="权限名" width="150" />
        <el-table-column prop="perm_type" label="类型" width="90" align="center">
          <template #default="{ row }">
            <el-tag size="small">{{ PERM_TYPE_MAP[row.perm_type] || '未知' }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="remark" label="备注" min-width="150" show-overflow-tooltip />
        <el-table-column label="操作" width="160" fixed="right" align="center">
          <template #default="{ row }">
            <el-button v-auth="'sys:permission:update'" link type="primary" :icon="Edit" @click="openEdit(row)">编辑</el-button>
            <el-button v-auth="'sys:permission:remove'" link type="danger" :icon="Delete" @click="handleDelete(row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>

      <div class="table-pagination">
        <el-pagination
          v-model:current-page="queryForm.page_num"
          v-model:page-size="queryForm.page_size"
          background
          layout="total, sizes, prev, pager, next, jumper"
          :page-sizes="[10, 20, 50, 100]"
          :total="total"
          @current-change="handlePageChange"
          @size-change="handleSizeChange"
        />
      </div>
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
            <el-radio :value="1">API权限</el-radio>
            <el-radio :value="2">菜单入口</el-radio>
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
.table-pagination { display: flex; justify-content: flex-end; margin-top: 16px; }
</style>
