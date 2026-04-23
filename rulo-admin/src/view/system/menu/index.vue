<script setup lang="ts" name="MenuManage">
import { ref, computed, onMounted, nextTick } from 'vue'
import { ElMessageBox } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import { Search, Plus, Delete, Edit } from '@element-plus/icons-vue'
import menuApi from '@/api/admin/menu'
import type { SysMenu, SysMenuSaveDto, SysMenuUpdateDto, SysMenuListDto } from '@/type/menu'

const MENU_TYPE_MAP: Record<number, string> = { 1: '目录', 2: '菜单' }

// ---- 列表 ----
const tableData = ref<SysMenu[]>([])
const total = ref(0)
const loading = ref(false)
const queryForm = ref<SysMenuListDto>({ page_num: 1, page_size: 10 })

const fetchList = async () => {
  loading.value = true
  try {
    const res = await menuApi.list(queryForm.value)
    tableData.value = res.list
    total.value = res.total
  } finally {
    loading.value = false
  }
}

// 构建树形结构
interface MenuTreeNode extends SysMenu {
  children?: MenuTreeNode[]
}

const treeData = computed<MenuTreeNode[]>(() => {
  const map = new Map<string, MenuTreeNode>()
  const roots: MenuTreeNode[] = []
  // 先把所有节点放入 map
  for (const item of tableData.value) {
    map.set(item.id, { ...item, children: [] })
  }
  // 构建父子关系
  for (const node of map.values()) {
    if (node.parent_id && map.has(node.parent_id)) {
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
const formData = ref<SysMenuSaveDto & { id?: string; is_hidden?: boolean }>({
  name: '',
  menu_type: 2,
  sort_order: 100,
})
const editingOriginal = ref<SysMenu | null>(null)

const formRef = ref<FormInstance>()
const formRules: FormRules = {
  name: [{ required: true, message: '请输入菜单名', trigger: 'blur' }],
  path: [{
    validator: (_rule: any, value: string, callback: any) => {
      if (formData.value.menu_type === 2 && !value?.trim()) {
        callback(new Error('路由路径不能为空'))
      } else {
        callback()
      }
    },
    trigger: 'blur',
  }],
  component: [{
    validator: (_rule: any, value: string, callback: any) => {
      if (formData.value.menu_type === 2 && !value?.trim()) {
        callback(new Error('组件路径不能为空'))
      } else {
        callback()
      }
    },
    trigger: 'blur',
  }],
  auto_perm_code: [{
    validator: (_rule: any, value: string, callback: any) => {
      if (!isEdit.value && formData.value.menu_type === 2 && !value) {
        callback(new Error('菜单权限码不能为空'))
      } else {
        callback()
      }
    },
    trigger: 'blur',
  }],
}

// 父级菜单树选项（编辑时排除自身及其后代）
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
    if (item.menu_type !== 1) continue  // 只有目录可作为父级
    map.set(item.id, { value: item.id, label: item.name, children: [] })
  }
  for (const item of tableData.value) {
    if (excludeIds.has(item.id)) continue
    if (item.menu_type !== 1) continue
    const node = map.get(item.id)!
    if (item.parent_id && map.has(item.parent_id)) {
      map.get(item.parent_id)!.children!.push(node)
    } else {
      roots.push(node)
    }
  }
  return roots
})

const openAdd = () => {
  isEdit.value = false
  formData.value = { name: '', menu_type: 2, sort_order: 100, auto_perm_code: '' }
  dialogVisible.value = true
  nextTick(() => formRef.value?.clearValidate())
}

const openEdit = (row: SysMenu) => {
  isEdit.value = true
  editingOriginal.value = { ...row }
  formData.value = {
    id: row.id,
    parent_id: row.parent_id || undefined,
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
  nextTick(() => formRef.value?.clearValidate())
}

const formSaving = ref(false)

const handleSave = async () => {
  await formRef.value?.validate()
  formSaving.value = true
  try {
    if (isEdit.value) {
      const orig = editingOriginal.value
      const dto: SysMenuUpdateDto = {
        id: formData.value.id!,
        name: formData.value.name || undefined,
        // 只有值实际发生变化时才发送，避免非超管因携带未改动的结构字段被后端拒绝
        path: formData.value.path !== (orig?.path ?? undefined) ? formData.value.path : undefined,
        component: formData.value.component !== (orig?.component ?? undefined) ? formData.value.component : undefined,
        icon: formData.value.icon,
        sort_order: formData.value.sort_order,
        is_hidden: formData.value.is_hidden,
        remark: formData.value.remark,
      }
      await menuApi.update(dto)
    } else {
      const saveDto = { ...formData.value } as SysMenuSaveDto
      if (!saveDto.auto_perm_code) delete saveDto.auto_perm_code
      await menuApi.save(saveDto)
    }
    dialogVisible.value = false
    fetchList()
  } finally {
    formSaving.value = false
  }
}

// ---- 删除 ----
const handleDelete = async (row: SysMenu) => {
  await ElMessageBox.confirm(`确定删除菜单「${row.name}」吗？`, '提示', { type: 'warning' })
  await menuApi.remove({ ids: [row.id] })
  fetchList()
}

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
          </el-select>
        </el-form-item>
        <el-form-item>
          <el-button v-auth="'sys:menu:list'" type="primary" :icon="Search" @click="handleSearch">搜索</el-button>
          <el-button @click="handleReset">重置</el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- 表格 -->
    <el-card shadow="never" class="table-card">
      <div class="table-toolbar">
        <el-button v-auth="'sys:menu:save'" type="primary" :icon="Plus" @click="openAdd">新增菜单</el-button>
        <el-button @click="toggleExpandAll">{{ isExpandAll ? '全部折叠' : '全部展开' }}</el-button>
      </div>

      <el-skeleton v-if="loading && !tableData.length" :rows="8" animated style="padding: 10px 0" />
      <el-table v-else :key="tableKey" :data="treeData" v-loading="loading" stripe border style="width: 100%" row-key="id" :tree-props="{ children: 'children' }" :default-expand-all="isExpandAll">
        <el-table-column prop="name" label="菜单名" width="200" />
        <el-table-column prop="menu_type" label="类型" width="80" align="center">
          <template #default="{ row }">
            <el-tag size="small">{{ MENU_TYPE_MAP[row.menu_type] || '未知' }}</el-tag>
          </template>
        </el-table-column>
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
        <el-table-column label="操作" width="160" fixed="right" align="center">
          <template #default="{ row }">
            <el-button v-auth="'sys:menu:update'" link type="primary" :icon="Edit" @click="openEdit(row)">编辑</el-button>
            <el-button v-auth="'sys:menu:remove'" link type="danger" :icon="Delete" @click="handleDelete(row)">删除</el-button>
          </template>
        </el-table-column>
        <template #empty><el-empty description="暂无数据" /></template>
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
    <el-dialog :title="isEdit ? '编辑菜单' : '新增菜单'" v-model="dialogVisible" width="560px">
      <el-form ref="formRef" :model="formData" :rules="formRules" label-width="90px">
        <el-form-item label="菜单名" prop="name">
          <el-input v-model="formData.name" placeholder="请输入菜单名" maxlength="50" />
        </el-form-item>
        <el-form-item label="类型">
          <el-radio-group v-model="formData.menu_type">
            <el-radio :value="1">目录</el-radio>
            <el-radio :value="2">菜单</el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item v-if="!isEdit && formData.menu_type === 2" label="菜单权限码" prop="auto_perm_code">
          <el-input v-model="formData.auto_perm_code" placeholder="如 sys:user:menu，留空则不自动关联菜单权限" clearable maxlength="100" />
        </el-form-item>
        <el-form-item label="父级菜单">
          <el-tree-select
            v-model="formData.parent_id"
            :data="parentTreeOptions"
            :render-after-expand="false"
            placeholder="不选则为顶级菜单"
            clearable
          />
        </el-form-item>
        <el-form-item label="路由路径" prop="path">
          <el-input v-model="formData.path" placeholder="如 /system/user" maxlength="200" />
        </el-form-item>
        <el-form-item label="组件路径" v-if="formData.menu_type === 2" prop="component">
          <el-input v-model="formData.component" placeholder="如 view/system/user/index.vue" maxlength="200" />
        </el-form-item>
        <el-form-item label="图标">
          <el-input v-model="formData.icon" placeholder="Element Plus 图标名" maxlength="50" />
        </el-form-item>
        <el-form-item label="排序">
          <el-input-number v-model="formData.sort_order" :min="0" />
        </el-form-item>
        <el-form-item label="是否隐藏" v-if="isEdit">
          <el-switch v-model="formData.is_hidden" />
        </el-form-item>
        <el-form-item label="备注">
          <el-input v-model="formData.remark" type="textarea" maxlength="200" show-word-limit />
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
.table-pagination { display: flex; justify-content: flex-end; margin-top: 16px; }
</style>
