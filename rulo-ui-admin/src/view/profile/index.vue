<script setup lang="ts" name="ProfileCenter">
import { computed, onMounted, reactive, ref } from 'vue'
import { EditPen, Lock, Picture, RefreshLeft, UploadFilled, UserFilled } from '@element-plus/icons-vue'
import userApi from '@/api/admin/user'
import fileApi from '@/api/admin/file'
import { useUserStore } from '@/store/user'
import { showMessage } from '@/util/message'
import { clearProfileDecor, loadProfileDecor, saveProfileDecor } from '@/util/profileDecor'

const userStore = useUserStore()

const basicLoading = ref(false)
const passwordLoading = ref(false)
const avatarLoading = ref(false)
const basicForm = reactive({
  nick_name: '',
  email: '',
  remark: '',
})
const passwordForm = reactive({
  password: '',
  confirmPassword: '',
})
const decor = ref(loadProfileDecor())
const avatarInputRef = ref<HTMLInputElement | null>(null)
const coverInputRef = ref<HTMLInputElement | null>(null)

const currentUser = computed(() => userStore.userInfo)
const displayAvatar = computed(() => currentUser.value?.avatar_url || decor.value.avatar)
const displayCover = computed(() => decor.value.cover)

const syncFromUser = () => {
  basicForm.nick_name = currentUser.value?.nick_name || ''
  basicForm.email = currentUser.value?.email || ''
  basicForm.remark = currentUser.value?.remark || ''
}

const readFileAsDataUrl = (file: File) => new Promise<string>((resolve, reject) => {
  const reader = new FileReader()
  reader.onload = () => resolve(String(reader.result || ''))
  reader.onerror = () => reject(new Error('文件读取失败'))
  reader.readAsDataURL(file)
})

const handleBasicSave = async () => {
  if (!currentUser.value) return
  basicLoading.value = true
  try {
    await userApi.update({
      id: currentUser.value.id,
      nick_name: basicForm.nick_name || undefined,
      email: basicForm.email || undefined,
      remark: basicForm.remark || undefined,
    })
    await userStore.initUser()
    syncFromUser()
    showMessage('基础信息已更新', 'success')
  } finally {
    basicLoading.value = false
  }
}

const handlePasswordSave = async () => {
  if (!currentUser.value) return
  if (!passwordForm.password) {
    showMessage('请输入新密码', 'warning')
    return
  }
  if (passwordForm.password !== passwordForm.confirmPassword) {
    showMessage('两次输入的密码不一致', 'warning')
    return
  }
  passwordLoading.value = true
  try {
    await userApi.update({
      id: currentUser.value.id,
      password: passwordForm.password,
    })
    passwordForm.password = ''
    passwordForm.confirmPassword = ''
    showMessage('密码已更新', 'success')
  } finally {
    passwordLoading.value = false
  }
}

const triggerAvatarPick = () => avatarInputRef.value?.click()
const triggerCoverPick = () => coverInputRef.value?.click()

const handleAvatarChange = async (event: Event) => {
  const input = event.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return
  if (!currentUser.value) return
  avatarLoading.value = true
  try {
    const url = await fileApi.upload(file)
    await userApi.update({ id: currentUser.value.id, avatar_url: url })
    await userStore.initUser()
    showMessage('头像已更新', 'success')
  } finally {
    avatarLoading.value = false
    input.value = ''
  }
}

const handleCoverChange = async (event: Event) => {
  const input = event.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return
  const cover = await readFileAsDataUrl(file)
  decor.value = saveProfileDecor({ cover })
  input.value = ''
  showMessage('背景图已本地保存', 'success')
}

const resetAvatar = async () => {
  if (!currentUser.value) return
  await userApi.update({ id: currentUser.value.id, avatar_url: '' })
  await userStore.initUser()
  decor.value = clearProfileDecor('avatar')
  showMessage('头像已清空', 'success')
}

const resetCover = () => {
  decor.value = clearProfileDecor('cover')
  showMessage('背景图已清空', 'success')
}

onMounted(async () => {
  if (!userStore.userInfo && userStore.isLoggedIn) {
    await userStore.initUser()
  }
  syncFromUser()
})
</script>

<template>
  <div class="profile-page">
    <div class="profile-hero">
      <div class="profile-cover" :style="displayCover ? { backgroundImage: `url(${displayCover})` } : undefined">
        <div class="profile-cover-mask" />
      </div>
      <div class="profile-summary">
        <el-avatar v-if="displayAvatar" :size="92" class="profile-avatar">
          <img :src="displayAvatar" alt="avatar" />
        </el-avatar>
        <el-avatar v-else :size="92" class="profile-avatar profile-avatar-fallback">
          <el-icon :size="40"><UserFilled /></el-icon>
        </el-avatar>
        <div class="profile-meta">
          <div class="profile-name">{{ currentUser?.nick_name || currentUser?.user_name || '管理员' }}</div>
          <div class="profile-subtitle">@{{ currentUser?.user_name || 'admin' }}</div>
          <div class="profile-desc">{{ currentUser?.remark || '这里展示的是基础资料、密码修改以及本地头像背景设置。' }}</div>
        </div>
      </div>
    </div>

    <el-row :gutter="16" class="profile-content">
      <el-col :xs="24" :lg="15">
        <el-card shadow="never" class="panel-card">
          <template #header>
            <div class="panel-title">
              <el-icon><EditPen /></el-icon>
              <span>基础信息</span>
            </div>
          </template>
          <el-form label-width="88px" class="profile-form">
            <el-form-item label="用户名">
              <el-input :model-value="currentUser?.user_name || ''" disabled />
            </el-form-item>
            <el-form-item label="昵称">
              <el-input v-model="basicForm.nick_name" placeholder="请输入昵称" />
            </el-form-item>
            <el-form-item label="邮箱">
              <el-input v-model="basicForm.email" placeholder="请输入邮箱" />
            </el-form-item>
            <el-form-item label="备注">
              <el-input v-model="basicForm.remark" type="textarea" :rows="4" placeholder="请输入备注" />
            </el-form-item>
            <el-form-item>
              <el-button type="primary" :loading="basicLoading" @click="handleBasicSave">保存基础信息</el-button>
              <el-button @click="syncFromUser">重置</el-button>
            </el-form-item>
          </el-form>
        </el-card>

        <el-card shadow="never" class="panel-card">
          <template #header>
            <div class="panel-title">
              <el-icon><Lock /></el-icon>
              <span>修改密码</span>
            </div>
          </template>
          <el-form label-width="88px" class="profile-form">
            <el-form-item label="新密码">
              <el-input v-model="passwordForm.password" type="password" show-password placeholder="请输入新密码" />
            </el-form-item>
            <el-form-item label="确认密码">
              <el-input v-model="passwordForm.confirmPassword" type="password" show-password placeholder="请再次输入新密码" />
            </el-form-item>
            <el-form-item>
              <el-button type="primary" :loading="passwordLoading" @click="handlePasswordSave">更新密码</el-button>
            </el-form-item>
          </el-form>
        </el-card>
      </el-col>

      <el-col :xs="24" :lg="9">
        <el-card shadow="never" class="panel-card">
          <template #header>
            <div class="panel-title">
              <el-icon><Picture /></el-icon>
              <span>头像与背景</span>
            </div>
          </template>

          <div class="appearance-panel">
            <div class="mini-cover" :style="displayCover ? { backgroundImage: `url(${displayCover})` } : undefined">
              <div class="mini-cover-mask" />
              <el-avatar v-if="displayAvatar" :size="72" class="mini-avatar">
                <img :src="displayAvatar" alt="avatar" />
              </el-avatar>
              <el-avatar v-else :size="72" class="mini-avatar profile-avatar-fallback">
                <el-icon :size="30"><UserFilled /></el-icon>
              </el-avatar>
            </div>

            <div class="appearance-actions">
              <input ref="avatarInputRef" type="file" accept="image/*" class="hidden-input" @change="handleAvatarChange" />
              <input ref="coverInputRef" type="file" accept="image/*" class="hidden-input" @change="handleCoverChange" />

              <el-button type="primary" plain :icon="UploadFilled" :loading="avatarLoading" @click="triggerAvatarPick">上传头像</el-button>
              <el-button plain :icon="Picture" @click="triggerCoverPick">设置背景图</el-button>
              <el-button plain :icon="RefreshLeft" @click="resetAvatar">清空头像</el-button>
              <el-button plain :icon="RefreshLeft" @click="resetCover">清空背景图</el-button>
            </div>

            <el-alert
              title="头像通过后端对象存储持久化保存；背景图仅保存在本地浏览器。"
              type="info"
              :closable="false"
              show-icon
            />
          </div>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<style scoped>
.profile-page {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.profile-hero {
  position: relative;
  overflow: hidden;
  border-radius: 16px;
  border: 1px solid var(--el-border-color-lighter);
  background: var(--el-bg-color);
}

.profile-cover {
  height: 180px;
  background:
    linear-gradient(135deg, color-mix(in srgb, var(--rulo-primary) 70%, #ffffff 30%), color-mix(in srgb, var(--rulo-primary) 52%, #0f172a 48%));
  background-size: cover;
  background-position: center;
}

.profile-cover-mask,
.mini-cover-mask {
  position: absolute;
  inset: 0;
  background: linear-gradient(180deg, rgba(0, 0, 0, 0.04), rgba(0, 0, 0, 0.18));
}

.profile-summary {
  position: relative;
  display: flex;
  align-items: flex-end;
  gap: 16px;
  padding: 0 24px 22px;
  margin-top: -46px;
}

.profile-avatar {
  border: 4px solid rgba(255, 255, 255, 0.95);
  box-shadow: 0 10px 26px rgba(0, 0, 0, 0.18);
  background: var(--el-fill-color-light);
  flex-shrink: 0;
}

.profile-avatar-fallback {
  color: var(--rulo-primary);
}

.profile-meta {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding-bottom: 4px;
}

.profile-name {
  font-size: 24px;
  font-weight: 700;
  color: var(--el-text-color-primary);
}

.profile-subtitle {
  font-size: 13px;
  color: var(--el-text-color-secondary);
}

.profile-desc {
  font-size: 14px;
  color: var(--el-text-color-regular);
}

.profile-content {
  margin: 0 !important;
}

.panel-card {
  border-radius: 16px;
}

.panel-card + .panel-card {
  margin-top: 16px;
}

.panel-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 600;
}

.profile-form {
  max-width: 640px;
}

.appearance-panel {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.mini-cover {
  position: relative;
  height: 150px;
  border-radius: 14px;
  overflow: hidden;
  background:
    linear-gradient(135deg, color-mix(in srgb, var(--rulo-primary) 76%, #ffffff 24%), color-mix(in srgb, var(--rulo-primary) 46%, #0f172a 54%));
  background-size: cover;
  background-position: center;
}

.mini-avatar {
  position: absolute;
  left: 18px;
  bottom: 16px;
  z-index: 1;
  border: 3px solid rgba(255, 255, 255, 0.94);
  box-shadow: 0 8px 20px rgba(0, 0, 0, 0.18);
}

.appearance-actions {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 10px;
}

.hidden-input {
  display: none;
}

@media (max-width: 992px) {
  .profile-summary {
    flex-direction: column;
    align-items: flex-start;
  }

  .appearance-actions {
    grid-template-columns: 1fr;
  }
}
</style>