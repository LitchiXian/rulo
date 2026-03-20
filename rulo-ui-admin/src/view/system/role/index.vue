<script setup lang="ts" name="RoleManage">
import { ref, onMounted, nextTick } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Search, Plus, Delete, Edit, Menu, Key } from '@element-plus/icons-vue'
import roleApi from '@/api/admin/role'
import menuApi from '@/api/admin/menu'
import permissionApi from '@/api/admin/permission'
import type { SysRole, SysRoleSaveDto, SysRoleUpdateDto, SysRoleListDto } from '@/type/role'
import type { SysMenu } from '@/type/menu'
import type { SysPermission } from '@/type/permission'
import type { ElTree } from 'element-plus'

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

// ---- 分配菜单弹窗 ----
const menuDialogVisible = ref(false)
const menuTreeRef = ref<InstanceType<typeof ElTree>>()
const allMenus = ref<SysMenu[]>([])
const currentRole = ref<SysRole | null>(null)
const menuSaving = ref(false)

interface MenuTreeNode {
  id: number
  label: string
  children?: MenuTreeNode[]
}

const buildMenuTree = (menus: SysMenu[], parentId = 0): MenuTreeNode[] => {
  return menus
    .filter(m => m.parent_id === parentId)
    .sort((a, b) => a.sort_order - b.sort_order)
    .map(m => ({
      id: m.id,
      label: m.name,
      children: buildMenuTree(menus, m.id),
    }))
}

const menuTreeData = ref<MenuTreeNode[]>([])

const openMenuDialog = async (row: SysRole) => {
  currentRole.value = row
  try {
    const [menus, checkedIds] = await Promise.all([
      menuApi.list(),
      roleApi.listMenus(row.id),
    ])
    allMenus.value = menus
    menuTreeData.value = buildMenuTree(menus)
    menuDialogVisible.value = true
    await nextTick()
    // 只勾选叶子节点，避免父节点被自动全选
    const leafIds = checkedIds.filter(id => {
      return !menus.some(m => m.parent_id === id)
    })
    menuTreeRef.value?.setCheckedKeys(leafIds)
  } catch { /* 错误已由拦截器提示 */ }
}

const handleMenuSave = async () => {
  if (!currentRole.value) return
  menuSaving.value = true
  try {
    const checked = menuTreeRef.value?.getCheckedKeys() as number[]
    const halfChecked = menuTreeRef.value?.getHalfCheckedKeys() as number[]
    const menu_ids = [...checked, ...halfChecked]
    await roleApi.updateBindMenus({ role_id: currentRole.value.id, menu_ids })
    ElMessage.success('分配菜单成功')
    menuDialogVisible.value = false
  } finally {
    menuSaving.value = false
  }
}

// ---- 分配权限弹窗 ----
const permDialogVisible = ref(false)
const allPerms = ref<SysPermission[]>([])
const checkedPermIds = ref<number[]>([])
const permSaving = ref(false)
const permSearch = ref('')

import { computed } from 'vue'

const filteredPerms = computed(() => {
  if (!permSearch.value) return allPerms.value
  const kw = permSearch.value.toLowerCase()
  return allPerms.value.filter(
    p => p.perm_code.toLowerCase().includes(kw) || p.perm_name.toLowerCase().includes(kw)
  )
})

// 按模块分组（取 perm_code 中间段，如 sys:user:list → user）
interface PermGroup {
  module: string
  perms: typeof allPerms.value
}
const groupedPerms = computed<PermGroup[]>(() => {
  const map = new Map<string, typeof allPerms.value>()
  for (const p of filteredPerms.value) {
    const parts = p.perm_code.split(':')
    const mod = parts.length >= 2 ? parts[1] : 'other'
    if (!map.has(mod)) map.set(mod, [])
    map.get(mod)!.push(p)
  }
  return Array.from(map, ([module, perms]) => ({ module, perms }))
})

// 模块全选/取消
const isGroupAllChecked = (group: PermGroup) => group.perms.every(p => checkedPermIds.value.includes(p.id))
const isGroupIndeterminate = (group: PermGroup) => {
  const count = group.perms.filter(p => checkedPermIds.value.includes(p.id)).length
  return count > 0 && count < group.perms.length
}
const handleGroupCheckAll = (group: PermGroup, checked: boolean) => {
  const ids = group.perms.map(p => p.id)
  if (checked) {
    const set = new Set(checkedPermIds.value)
    ids.forEach(id => set.add(id))
    checkedPermIds.value = Array.from(set)
  } else {
    const remove = new Set(ids)
    checkedPermIds.value = checkedPermIds.value.filter(id => !remove.has(id))
  }
}

const openPermDialog = async (row: SysRole) => {
  currentRole.value = row
  permSearch.value = ''
  try {
    const [perms, checkedIds] = await Promise.all([
      permissionApi.list({ perm_type: 1 }),
      roleApi.listPerms(row.id),
    ])
    allPerms.value = perms
    checkedPermIds.value = checkedIds
    permDialogVisible.value = true
  } catch { /* 错误已由拦截器提示 */ }
}

const handlePermSave = async () => {
  if (!currentRole.value) return
  permSaving.value = true
  try {
    await roleApi.updateBindPerms({ role_id: currentRole.value.id, perm_ids: checkedPermIds.value })
    ElMessage.success('分配权限成功')
    permDialogVisible.value = false
  } finally {
    permSaving.value = false
  }
}

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
        <el-table-column label="操作" width="320" fixed="right" align="center">
          <template #default="{ row }">
            <el-button link type="primary" :icon="Edit" @click="openEdit(row)">编辑</el-button>
            <el-button link type="warning" :icon="Menu" @click="openMenuDialog(row)">分配菜单</el-button>
            <el-button link type="success" :icon="Key" @click="openPermDialog(row)">分配权限</el-button>
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

    <!-- 分配菜单弹窗 -->
    <el-dialog
      :title="`分配菜单 - ${currentRole?.role_name ?? ''}`"
      v-model="menuDialogVisible"
      width="800px"
    >
      <el-tree
        ref="menuTreeRef"
        :data="menuTreeData"
        show-checkbox
        node-key="id"
        default-expand-all
        :props="{ label: 'label', children: 'children' }"
        class="menu-tree"
      />
      <template #footer>
        <el-button @click="menuDialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="menuSaving" @click="handleMenuSave">确定</el-button>
      </template>
    </el-dialog>

    <!-- 分配权限弹窗 -->
    <el-dialog
      :title="`分配权限 - ${currentRole?.role_name ?? ''}`"
      v-model="permDialogVisible"
      width="720px"
    >
      <el-input v-model="permSearch" placeholder="搜索权限编码或名称" clearable style="margin-bottom: 12px" />
      <div style="max-height: 450px; overflow-y: auto">
        <div v-for="group in groupedPerms" :key="group.module" class="perm-group">
          <div class="perm-group-header">
            <el-checkbox
              :model-value="isGroupAllChecked(group)"
              :indeterminate="isGroupIndeterminate(group)"
              @change="(val: any) => handleGroupCheckAll(group, !!val)"
            >
              <span class="perm-group-title">{{ group.module }}</span>
            </el-checkbox>
          </div>
          <el-checkbox-group v-model="checkedPermIds" class="perm-group-body">
            <div v-for="perm in group.perms" :key="perm.id" class="perm-item">
              <el-checkbox :value="perm.id">
                <span class="perm-code">{{ perm.perm_code.split(':').pop() }}</span>
                <span class="perm-name">{{ perm.perm_name }}</span>
              </el-checkbox>
            </div>
          </el-checkbox-group>
        </div>
      </div>
      <template #footer>
        <el-button @click="permDialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="permSaving" @click="handlePermSave">确定</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped>
.page-container { display: flex; flex-direction: column; gap: 16px; }
.search-card :deep(.el-card__body) { padding-bottom: 0; }
.table-toolbar { display: flex; justify-content: flex-start; margin-bottom: 16px; }

/* 菜单树：子节点横排 4 个一行 */
.menu-tree :deep(.el-tree-node__children) {
  display: flex;
  flex-wrap: wrap;
}
.menu-tree :deep(.el-tree-node__children > .el-tree-node) {
  width: 25%;
}
/* 一级节点：分组卡片感 */
.menu-tree :deep(> .el-tree-node) {
  background: var(--el-fill-color-lighter);
  border-radius: 6px;
  padding: 8px 4px 4px;
  margin-bottom: 10px;
}
/* 一级节点文字加粗 */
.menu-tree :deep(> .el-tree-node > .el-tree-node__content) {
  font-weight: 600;
  font-size: 14px;
  margin-bottom: 4px;
}
/* 子节点稍小 */
.menu-tree :deep(.el-tree-node__children > .el-tree-node > .el-tree-node__content) {
  font-size: 13px;
  padding: 2px 0;
}

/* 权限分组卡片 */
.perm-group {
  background: var(--el-fill-color-lighter);
  border-radius: 6px;
  padding: 10px 12px 6px;
  margin-bottom: 10px;
}
.perm-group-header {
  margin-bottom: 6px;
  border-bottom: 1px solid var(--el-border-color-lighter);
  padding-bottom: 6px;
}
.perm-group-title {
  font-weight: 600;
  font-size: 14px;
  text-transform: capitalize;
}
.perm-group-body {
  display: flex;
  flex-wrap: wrap;
}
.perm-item {
  width: 33.33%;
  padding: 3px 0;
  font-size: 13px;
}
.perm-code {
  font-family: monospace;
  margin-right: 4px;
}
.perm-name {
  color: #999;
  font-size: 12px;
}
</style>
