<script setup lang="ts" name="DeptManage">
import { ref, computed, onMounted, nextTick } from 'vue'
import { ElMessageBox } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import { Search, Plus, Delete, Edit } from '@element-plus/icons-vue'
import deptApi from '@/api/admin/dept'
import type { SysDept, SysDeptSaveDto, SysDeptUpdateDto, SysDeptListDto } from '@/type/dept'

// ---- 列表 ----
const tableData = ref<SysDept[]>([])
const loading = ref(false)
const queryForm = ref<SysDeptListDto>({})

const fetchList = async () => {
  loading.value = true
  try {
    tableData.value = await deptApi.listAll(queryForm.value)
  } finally {
    loading.value = false
  }
}

interface DeptTreeNode extends SysDept {
  children?: DeptTreeNode[]
}

const treeData = computed<DeptTreeNode[]>(() => {
  const map = new Map<string, DeptTreeNode>()
  const roots: DeptTreeNode[] = []
  for (const item of tableData.value) {
    map.set(item.id, { ...item, children: [] })
  }
  for (const node of map.values()) {
    if (node.parent_id && node.parent_id !== '0' && map.has(node.parent_id)) {
      map.get(node.parent_id)!.children!.push(node)
    } else {
      roots.push(node)
    }
  }
  return roots
})

// 展开/折叠
const isExpandAll = ref(true)
const tableKey = ref(0)
const toggleExpandAll = () => {
  isExpandAll.value = !isExpandAll.value
  tableKey.value++
}

const handleSearch = () => fetchList()
const handleReset = () => {
  queryForm.value = {}
  fetchList()
}

// ---- 新增/编辑弹窗 ----
const dialogVisible = ref(false)
const isEdit = ref(false)
const formData = ref<SysDeptSaveDto & { id?: string; is_active?: boolean }>({
  name: '',
  sort_order: 10,
})

const formRef = ref<FormInstance>()
const formRules: FormRules = {
  name: [{ required: true, message: '请输入部门名称', trigger: 'blur' }],
  email: [{
    validator: (_rule: any, value: string, callback: any) => {
      if (value && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value)) {
        callback(new Error('邮箱格式不正确'))
      } else {
        callback()
      }
    },
    trigger: 'blur',
  }],
}

// 父级部门树（编辑时排除自身及其后代）
const parentTreeOptions = computed(() => {
  const excludeIds = new Set<string>()
  if (isEdit.value && formData.value.id) {
    const collectIds = (id: string) => {
      excludeIds.add(id)
      for (const item of tableData.value) {
        if (item.parent_id === id) collectIds(item.id)
      }
    }
    collectIds(formData.value.id)
  }
  interface TreeOption { value: string; label: string; children?: TreeOption[] }
  const map = new Map<string, TreeOption>()
  const roots: TreeOption[] = []
  for (const item of tableData.value) {
    if (excludeIds.has(item.id)) continue
    map.set(item.id, { value: item.id, label: item.name, children: [] })
  }
  for (const item of tableData.value) {
    if (excludeIds.has(item.id)) continue
    const node = map.get(item.id)!
    if (item.parent_id && item.parent_id !== '0' && map.has(item.parent_id)) {
      map.get(item.parent_id)!.children!.push(node)
    } else {
      roots.push(node)
    }
  }
  return roots
})

const openAdd = (parentRow?: SysDept) => {
  isEdit.value = false
  formData.value = {
    name: '',
    sort_order: 10,
    parent_id: parentRow?.id,
  }
  dialogVisible.value = true
  nextTick(() => formRef.value?.clearValidate())
}

const openEdit = (row: SysDept) => {
  isEdit.value = true
  formData.value = {
    id: row.id,
    parent_id: row.parent_id === '0' ? undefined : row.parent_id,
    name: row.name,
    sort_order: row.sort_order,
    is_active: row.is_active,
    leader: row.leader ?? undefined,
    phone: row.phone ?? undefined,
    email: row.email ?? undefined,
    remark: row.remark ?? undefined,
  }
  dialogVisible.value = true
  nextTick(() => formRef.value?.clearValidate())
}

const formSaving = ref(false)

const handleSave = async () => {
  await formRef.value?.validate()
  formSaving.value = true
  try {
    if (isEdit.value) {
      const dto: SysDeptUpdateDto = {
        id: formData.value.id!,
        parent_id: formData.value.parent_id,
        name: formData.value.name || undefined,
        sort_order: formData.value.sort_order,
        is_active: formData.value.is_active,
        leader: formData.value.leader,
        phone: formData.value.phone,
        email: formData.value.email,
        remark: formData.value.remark,
      }
      await deptApi.update(dto)
    } else {
      const saveDto: SysDeptSaveDto = {
        parent_id: formData.value.parent_id,
        name: formData.value.name,
        sort_order: formData.value.sort_order,
        leader: formData.value.leader,
        phone: formData.value.phone,
        email: formData.value.email,
        remark: formData.value.remark,
      }
      await deptApi.save(saveDto)
    }
    dialogVisible.value = false
    fetchList()
  } finally {
    formSaving.value = false
  }
}

// ---- 删除 ----
const handleDelete = async (row: SysDept) => {
  await ElMessageBox.confirm(`确定删除部门「${row.name}」吗？`, '提示', { type: 'warning' })
  await deptApi.remove({ ids: [row.id] })
  fetchList()
}

onMounted(fetchList)
</script>

<template>
  <div class="page-container">
    <!-- 搜索栏 -->
    <el-card shadow="never" class="search-card">
      <el-form :model="queryForm" inline>
        <el-form-item label="部门名称">
          <el-input v-model="queryForm.name" placeholder="请输入部门名称" clearable @keyup.enter="handleSearch" />
        </el-form-item>
        <el-form-item label="状态">
          <el-select v-model="queryForm.is_active" placeholder="请选择" clearable style="width: 120px">
            <el-option label="启用" :value="true" />
            <el-option label="禁用" :value="false" />
          </el-select>
        </el-form-item>
        <el-form-item>
          <el-button v-auth="'sys:dept:list'" type="primary" :icon="Search" @click="handleSearch">搜索</el-button>
          <el-button @click="handleReset">重置</el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- 表格 -->
    <el-card shadow="never" class="table-card">
      <div class="table-toolbar">
        <el-button v-auth="'sys:dept:save'" type="primary" :icon="Plus" @click="openAdd()">新增部门</el-button>
        <el-button @click="toggleExpandAll">{{ isExpandAll ? '全部折叠' : '全部展开' }}</el-button>
      </div>

      <el-skeleton v-if="loading && !tableData.length" :rows="8" animated style="padding: 10px 0" />
      <el-table
        v-else
        :key="tableKey"
        :data="treeData"
        v-loading="loading"
        stripe
        border
        style="width: 100%"
        row-key="id"
        :tree-props="{ children: 'children' }"
        :default-expand-all="isExpandAll"
      >
        <el-table-column prop="name" label="部门名称" min-width="200" />
        <el-table-column prop="leader" label="负责人" width="120" />
        <el-table-column prop="phone" label="联系电话" width="140" />
        <el-table-column prop="email" label="邮箱" min-width="180" show-overflow-tooltip />
        <el-table-column prop="sort_order" label="排序" width="80" align="center" />
        <el-table-column prop="is_active" label="状态" width="80" align="center">
          <template #default="{ row }">
            <el-tag :type="row.is_active ? 'success' : 'info'" size="small">
              {{ row.is_active ? '启用' : '禁用' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="create_time" label="创建时间" width="170" />
        <el-table-column label="操作" width="220" fixed="right" align="center">
          <template #default="{ row }">
            <el-button v-auth="'sys:dept:save'" link type="primary" :icon="Plus" @click="openAdd(row)">新增</el-button>
            <el-button v-auth="'sys:dept:update'" link type="primary" :icon="Edit" @click="openEdit(row)">编辑</el-button>
            <el-button v-auth="'sys:dept:remove'" link type="danger" :icon="Delete" @click="handleDelete(row)">删除</el-button>
          </template>
        </el-table-column>
        <template #empty><el-empty description="暂无数据" /></template>
      </el-table>
    </el-card>

    <!-- 新增/编辑弹窗 -->
    <el-dialog :title="isEdit ? '编辑部门' : '新增部门'" v-model="dialogVisible" width="560px">
      <el-form ref="formRef" :model="formData" :rules="formRules" label-width="90px">
        <el-form-item label="父级部门">
          <el-tree-select
            v-model="formData.parent_id"
            :data="parentTreeOptions"
            :render-after-expand="false"
            placeholder="不选则为顶级部门"
            clearable
            check-strictly
          />
        </el-form-item>
        <el-form-item label="部门名称" prop="name">
          <el-input v-model="formData.name" placeholder="请输入部门名称" maxlength="50" />
        </el-form-item>
        <el-form-item label="负责人">
          <el-input v-model="formData.leader" placeholder="请输入负责人" maxlength="50" />
        </el-form-item>
        <el-form-item label="联系电话">
          <el-input v-model="formData.phone" placeholder="请输入联系电话" maxlength="30" />
        </el-form-item>
        <el-form-item label="邮箱" prop="email">
          <el-input v-model="formData.email" placeholder="请输入邮箱" maxlength="50" />
        </el-form-item>
        <el-form-item label="排序">
          <el-input-number v-model="formData.sort_order" :min="0" />
        </el-form-item>
        <el-form-item v-if="isEdit" label="状态">
          <el-switch v-model="formData.is_active" />
        </el-form-item>
        <el-form-item label="备注">
          <el-input v-model="formData.remark" type="textarea" maxlength="500" show-word-limit />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="formSaving" @click="handleSave">确定</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped>
.page-container { display: flex; flex-direction: column; gap: 16px; }
.search-card :deep(.el-card__body) { padding-bottom: 0; }
.table-toolbar { display: flex; justify-content: flex-start; margin-bottom: 16px; }
</style>
