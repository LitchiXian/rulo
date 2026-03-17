<script setup lang="ts" name="MenuManage">
import { ref, onMounted } from 'vue'
import { ElMessageBox } from 'element-plus'
import { Search, Plus, Delete, Edit } from '@element-plus/icons-vue'
import menuApi from '@/api/admin/menu'
import type { SysMenu, SysMenuSaveDto, SysMenuUpdateDto, SysMenuListDto } from '@/type/menu'

const MENU_TYPE_MAP: Record<number, string> = { 1: '目录', 2: '菜单', 3: '按钮' }

// ---- 列表 ----
const tableData = ref<SysMenu[]>([])
const loading = ref(false)
const queryForm = ref<SysMenuListDto>({})

const fetchList = async () => {
  loading.value = true
  try {
    tableData.value = await menuApi.list(queryForm.value)
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
const formData = ref<SysMenuSaveDto & { id?: number; is_hidden?: boolean }>({
  name: '',
  menu_type: 2,
})

const openAdd = () => {
  isEdit.value = false
  formData.value = { name: '', menu_type: 2 }
  dialogVisible.value = true
}

const openEdit = (row: SysMenu) => {
  isEdit.value = true
  formData.value = {
    id: row.id,
    parent_id: row.parent_id || undefined,
    perm_id: row.perm_id ?? undefined,
    name: row.name,
    menu_type: row.menu_type,
    path: row.path ?? undefined,
    component: row.component ?? undefined,
    icon: row.icon ?? undefined,
    sort_order: row.sort_order,
    is_hidden: row.is_hidden,
    remark: row.remark ?? undefined,
  }
  dialogVisible.value = true
}

const handleSave = async () => {
  if (isEdit.value) {
    const dto: SysMenuUpdateDto = {
      id: formData.value.id!,
      name: formData.value.name || undefined,
      path: formData.value.path,
      component: formData.value.component,
      icon: formData.value.icon,
      sort_order: formData.value.sort_order,
      is_hidden: formData.value.is_hidden,
      remark: formData.value.remark,
    }
    await menuApi.update(dto)
  } else {
    await menuApi.save(formData.value as SysMenuSaveDto)
  }
  dialogVisible.value = false
  fetchList()
}

// ---- 删除 ----
const handleDelete = async (row: SysMenu) => {
  await ElMessageBox.confirm(`确定删除菜单「${row.name}」吗？`, '提示', { type: 'warning' })
  await menuApi.remove({ ids: [row.id] })
  fetchList()
}

// TODO: 后续改为树形表格展示（根据 parent_id 构建树结构）
// TODO: 后端 menu/list 暂不支持分页

onMounted(fetchList)
</script>

<template>
  <div class="page-container">
    <!-- 搜索栏 -->
    <el-card shadow="never" class="search-card">
      <el-form :model="queryForm" inline>
        <el-form-item label="菜单名">
          <el-input v-model="queryForm.name" placeholder="请输入菜单名" clearable />
        </el-form-item>
        <el-form-item label="类型">
          <el-select v-model="queryForm.menu_type" placeholder="请选择" clearable style="width: 120px">
            <el-option label="目录" :value="1" />
            <el-option label="菜单" :value="2" />
            <el-option label="按钮" :value="3" />
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
        <el-button type="primary" :icon="Plus" @click="openAdd">新增菜单</el-button>
      </div>

      <el-table :data="tableData" v-loading="loading" stripe border style="width: 100%">
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="name" label="菜单名" width="150" />
        <el-table-column prop="menu_type" label="类型" width="80" align="center">
          <template #default="{ row }">
            <el-tag size="small">{{ MENU_TYPE_MAP[row.menu_type] || '未知' }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="parent_id" label="父级ID" width="80" />
        <el-table-column prop="path" label="路由路径" min-width="150" show-overflow-tooltip />
        <el-table-column prop="component" label="组件路径" min-width="180" show-overflow-tooltip />
        <el-table-column prop="icon" label="图标" width="100" />
        <el-table-column prop="sort_order" label="排序" width="70" align="center" />
        <el-table-column prop="is_hidden" label="是否隐藏" width="90" align="center">
          <template #default="{ row }">
            <el-tag :type="row.is_hidden ? 'info' : 'success'" size="small">
              {{ row.is_hidden ? '隐藏' : '显示' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="create_time" label="创建时间" width="180">
          <template #default="{ row }">
            {{ new Date(row.create_time).toLocaleString('zh-CN') }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="160" fixed="right" align="center">
          <template #default="{ row }">
            <el-button link type="primary" :icon="Edit" @click="openEdit(row)">编辑</el-button>
            <el-button link type="danger" :icon="Delete" @click="handleDelete(row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <!-- 新增/编辑弹窗 -->
    <el-dialog :title="isEdit ? '编辑菜单' : '新增菜单'" v-model="dialogVisible" width="560px">
      <el-form :model="formData" label-width="90px">
        <el-form-item label="菜单名" required>
          <el-input v-model="formData.name" placeholder="请输入菜单名" />
        </el-form-item>
        <el-form-item label="类型" required>
          <el-radio-group v-model="formData.menu_type">
            <el-radio :value="1">目录</el-radio>
            <el-radio :value="2">菜单</el-radio>
            <el-radio :value="3">按钮</el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item label="父级ID">
          <!-- TODO: 改为树形选择器，从已有菜单中选 -->
          <el-input-number v-model="formData.parent_id" :min="0" placeholder="0 表示顶级" />
        </el-form-item>
        <el-form-item label="路由路径" v-if="formData.menu_type !== 3">
          <el-input v-model="formData.path" placeholder="如 /system/user" />
        </el-form-item>
        <el-form-item label="组件路径" v-if="formData.menu_type === 2">
          <el-input v-model="formData.component" placeholder="如 system/UserManage" />
        </el-form-item>
        <el-form-item label="图标">
          <el-input v-model="formData.icon" placeholder="Element Plus 图标名" />
        </el-form-item>
        <el-form-item label="排序">
          <el-input-number v-model="formData.sort_order" :min="0" />
        </el-form-item>
        <el-form-item label="是否隐藏" v-if="isEdit">
          <el-switch v-model="formData.is_hidden" />
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
