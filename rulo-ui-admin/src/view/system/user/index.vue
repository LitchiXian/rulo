<script setup lang="ts" name="UserManage">
import { ref, onMounted } from 'vue'
import { ElMessageBox } from 'element-plus'
import { Search, Plus, Delete, Edit } from '@element-plus/icons-vue'
import userApi from '@/api/admin/user'
import type { UserInfo, SysUserSaveDto, SysUserUpdateDto, SysUserListDto } from '@/type/user'

// ---- 列表 ----
const tableData = ref<UserInfo[]>([])
const loading = ref(false)
const queryForm = ref<SysUserListDto>({})

const fetchList = async () => {
  loading.value = true
  try {
    tableData.value = await userApi.list(queryForm.value)
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
const formData = ref<SysUserSaveDto & { id?: number }>({
  nick_name: '',
  password: '',
})

const openAdd = () => {
  isEdit.value = false
  formData.value = { nick_name: '', password: '' }
  dialogVisible.value = true
}

const openEdit = (row: UserInfo) => {
  isEdit.value = true
  formData.value = {
    id: row.id,
    nick_name: row.nick_name,
    password: '',
    email: row.email ?? undefined,
    remark: row.remark ?? undefined,
  }
  dialogVisible.value = true
}

const handleSave = async () => {
  if (isEdit.value) {
    const dto: SysUserUpdateDto = {
      id: formData.value.id!,
      nick_name: formData.value.nick_name || undefined,
      password: formData.value.password || undefined,
      email: formData.value.email,
      remark: formData.value.remark,
    }
    await userApi.update(dto)
  } else {
    await userApi.save(formData.value as SysUserSaveDto)
  }
  dialogVisible.value = false
  fetchList()
}

// ---- 删除 ----
const handleDelete = async (row: UserInfo) => {
  await ElMessageBox.confirm(`确定删除用户「${row.nick_name}」吗？`, '提示', { type: 'warning' })
  await userApi.remove({ ids: [row.id] })
  fetchList()
}

onMounted(fetchList)
</script>

<template>
  <div class="page-container">
    <!-- 搜索栏 -->
    <el-card shadow="never" class="search-card">
      <el-form :model="queryForm" inline>
        <el-form-item label="昵称">
          <el-input v-model="queryForm.nick_name" placeholder="请输入昵称" clearable />
        </el-form-item>
        <el-form-item label="邮箱">
          <el-input v-model="queryForm.email" placeholder="请输入邮箱" clearable />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" :icon="Search" @click="handleSearch">搜索</el-button>
          <el-button @click="handleReset">重置</el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- 操作栏 + 表格 -->
    <el-card shadow="never" class="table-card">
      <div class="table-toolbar">
        <el-button type="primary" :icon="Plus" @click="openAdd">新增用户</el-button>
      </div>

      <el-table :data="tableData" v-loading="loading" stripe border style="width: 100%">
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="user_name" label="用户名" width="130" />
        <el-table-column prop="nick_name" label="昵称" width="130" />
        <el-table-column prop="email" label="邮箱" min-width="180" />
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

      <!-- TODO: 后端 user/list 暂不支持分页，后续增加 PageDto 分页参数 -->
    </el-card>

    <!-- 新增/编辑弹窗 -->
    <el-dialog :title="isEdit ? '编辑用户' : '新增用户'" v-model="dialogVisible" width="500px">
      <el-form :model="formData" label-width="80px">
        <el-form-item label="昵称" required>
          <el-input v-model="formData.nick_name" placeholder="请输入昵称" />
        </el-form-item>
        <el-form-item label="密码" :required="!isEdit">
          <el-input v-model="formData.password" type="password" show-password
            :placeholder="isEdit ? '不修改请留空' : '请输入密码'" />
        </el-form-item>
        <el-form-item label="邮箱">
          <el-input v-model="formData.email" placeholder="请输入邮箱" />
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
