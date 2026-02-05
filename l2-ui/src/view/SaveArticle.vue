<template>
  <div class="create-article-container">
    <div v-if="loading" class="loading-container">
      <div class="loading">保存中...</div>
    </div>

    <div v-else class="article-form">
      <h2 class="form-title">创建新文章</h2>

      <!-- 标题输入 -->
      <div class="form-group">
        <label for="article-title">标题</label>
        <input
            id="article-title"
            type="text"
            v-model="form.title"
            placeholder="请输入文章标题"
            required
            class="form-input"
        />
      </div>

      <div class="form-group">
        是否公开: <el-switch
            v-model="form.isPublished"
            style="margin-left: 24px;--el-switch-on-color: #4abbb5;"
            inline-prompt
            :active-icon="Check"
            :inactive-icon="Close"
        />
      </div>

      <!-- 内容输入 -->
      <div class="form-group">
        <label for="article-content">内容 (Markdown格式)</label>
        <textarea
            id="article-content"
            v-model="form.content"
            placeholder="请输入文章内容（支持Markdown）"
            rows="15"
            required
            class="form-textarea"
        ></textarea>
      </div>

      <!-- 标签选择弹窗 -->
      <el-popover
        v-model:visible="showTagSelector"
        placement="bottom"
        trigger="click"
        class="tag-selector-popover"
        :teleported="true"
      >
        <div class="tag-selector-content">
          <div v-for="tag in tags" :key="tag.id" class="tag-option">
            <el-checkbox
              :label="tag.id"
              :checked="selectedTags.includes(tag.id)"
              @change="(checked:boolean) => handleTagCheck(tag.id, checked)"
            >
              {{ tag.name }}
            </el-checkbox>
          </div>
          <div class="tag-selector-footer">
            <el-button size="small" @click="showTagSelector = false">取消</el-button>
            <el-button size="small" type="primary" @click="confirmTagSelection">确认</el-button>
          </div>
        </div>
        <template #reference>
          <div class="form-group">
            <label for="article-tags">标签</label>
            <div
                class="tag-selector-container"
            >
              <!-- 选中的标签 -->
              <div class="selected-tags">
                <el-tag
                    v-for="selectedTag in selectedTagsObj"
                    :key="selectedTag.id"
                    closable
                    :disable-transitions="false"
                    @close.stop="removeTag(selectedTag.id)"
                    size="small"
                    class="selected-tag"
                >
                  {{ selectedTag.name }}
                </el-tag>
              </div>
              <!-- 占位文本 -->
              <div v-if="selectedTagsObj.length === 0" class="placeholder-text">
                点击选择标签
              </div>
              <!-- 下拉箭头 -->
              <el-icon class="dropdown-icon"><ArrowDown /></el-icon>
            </div>
          </div>
        </template>
      </el-popover>

      <!-- 按钮区域 -->
      <div class="form-actions">
        <button class="btn-submit" @click="submitForm">保存文章</button>
        <router-link to="/" class="btn-cancel">取消</router-link>
      </div>

      <!-- 错误提示 -->
      <div v-if="error" class="error-container">
        <p>⚠️ {{ error }}</p>
        <button @click="retrySubmit">重试</button>
      </div>
    </div>
  </div>
</template>

<script setup  lang="ts">
import {ref, reactive, onMounted, computed} from 'vue';
import { Check, Close, ArrowDown } from '@element-plus/icons-vue';
import {useRouter} from 'vue-router';
import blogArticleApi from '@/api/web/blogArticle.ts';
import {blogTagApi} from "@/api/web/blogTag.ts";
import type {Tag} from "@/type/article.ts";

const router = useRouter()

// 标签列表
const tags = ref<Tag[]>([])

// 选中的标签ID
const selectedTags = ref<string[]>([])

// 是否显示标签选择器
const showTagSelector = ref(false)

// 选中的标签对象
const selectedTagsObj = computed(() => {
  return tags.value.filter(tag => selectedTags.value.includes(tag.id))
})

// 处理标签勾选
const handleTagCheck = (tagId: string, checked: boolean) => {
  if (checked) {
    if (!selectedTags.value.includes(tagId)) {
      selectedTags.value.push(tagId)
    }
  } else {
    selectedTags.value = selectedTags.value.filter(id => id !== tagId)
  }
}

// 移除标签
const removeTag = (tagId: string) => {
  selectedTags.value = selectedTags.value.filter(id => id !== tagId)
}

// 确认标签选择
const confirmTagSelection = () => {
  showTagSelector.value = false
}

// 表单数据模型
const form = reactive({
  title: '',
  content: '',
  excerpt: '',
  isPublished: true,
  isFeatured: false,
})

// 状态数据
const loading = ref(false)
const error = ref<string>('')
const categories = ref([
  {id: 1, name: '前端开发'},
  {id: 2, name: '后端技术'},
  {id: 3, name: 'DevOps'},
  {id: 4, name: '技术心得'}
])

// 提交表单
const submitForm = async () => {
  // 简单的前端验证
  if (!form.title.trim()) {
    error.value = '标题不能为空'
    return
  }

  if (!form.content.trim()) {
    error.value = '内容不能为空'
    return
  }

  try {
    loading.value = true
    error.value = ''

    // 格式化数据
    const data = {
      title: form.title,
      content: form.content,
      tags: selectedTags.value.join(','),
      excerpt: form.excerpt || null,
      isPublished: form.isPublished ? 1 : 0,
      isFeatured: form.isFeatured
    }

    // 调用API保存
    await blogArticleApi.save(data)

    // 保存成功，返回首页
    router.push('/')
  } finally {
    loading.value = false
  }
}

const retrySubmit = () => {
  error.value = ''
  submitForm()
}

// 获取标签列表
const getTagList =() =>{
  blogTagApi.list().then(res => {
    tags.value = res
  })
}

// 初始化分类选项
onMounted(() => {
  getTagList();
})
</script>

<style scoped>
.create-article-container {
  padding: var(--spacing-xl) var(--spacing-lg);
  color: var(--text-primary);
}

.article-form {
  max-width: 800px;
  margin: 0 auto;
}

.form-title {
  font-size: 1.6rem;
  margin-bottom: var(--spacing-xl);
  padding-bottom: var(--spacing-md);
  border-bottom: 1px solid var(--border-color);
  color: var(--color-primary);
}

.form-group {
  margin-bottom: var(--spacing-lg);
}

.form-group label {
  display: block;
  margin-bottom: var(--spacing-sm);
  font-weight: 500;
  color: var(--text-title);
}

.form-input,
.form-textarea {
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background-color: var(--bg-body);
  color: var(--text-primary);
  font-size: 1rem;
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
}

.form-input:focus,
.form-textarea:focus {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px var(--color-primary-light);
  outline: none;
}

.form-textarea {
  min-height: 200px;
  resize: vertical;
  line-height: 1.6;
}

/* 标签选择器 */
.tag-selector-container {
  width: 100%;
  min-height: 42px;
  padding: var(--spacing-sm) var(--spacing-md);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  background-color: var(--bg-body);
  color: var(--text-primary);
  cursor: pointer;
  position: relative;
  display: flex;
  align-items: center;
  transition: all var(--transition-fast);
  margin-top: var(--spacing-sm);
}

.tag-selector-container:hover {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px var(--color-primary-light);
}

.selected-tags {
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-sm);
  width: 100%;
  padding-right: 24px;
}

.placeholder-text {
  color: var(--text-muted);
  font-style: italic;
}

.dropdown-icon {
  color: var(--text-primary);
  position: absolute;
  right: var(--spacing-md);
  top: 50%;
  transform: translateY(-50%);
  transition: color var(--transition-fast);
}

.tag-selector-container:hover .dropdown-icon {
  color: var(--color-primary);
}

.selected-tag {
  background-color: var(--color-primary);
  color: var(--bg-body);
  border: none;
}

.tag-selector-content {
  max-height: 200px;
  overflow-y: auto;
  padding-right: var(--spacing-xs);
}

.tag-option {
  padding: var(--spacing-xs) 0;
}

.tag-selector-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  margin-top: var(--spacing-sm);
  padding-top: var(--spacing-sm);
  border-top: 1px solid var(--border-color);
}

/* 按钮区域 */
.form-actions {
  display: flex;
  gap: var(--spacing-md);
  margin-top: var(--spacing-xl);
}

.btn-submit,
.btn-cancel {
  padding: var(--spacing-sm) var(--spacing-xl);
  border: none;
  border-radius: var(--radius-md);
  cursor: pointer;
  font-weight: 500;
  font-size: 1rem;
  transition: all var(--transition-fast);
  text-decoration: none;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.btn-submit {
  background-color: var(--color-primary);
  color: var(--bg-body);
}

.btn-submit:hover {
  background-color: var(--color-primary-hover);
  transform: translateY(-2px);
}

.btn-cancel {
  background-color: transparent;
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

.btn-cancel:hover {
  background-color: var(--bg-card);
  border-color: var(--text-muted);
  text-decoration: none;
}

/* 加载状态 */
.loading-container {
  text-align: center;
  padding: var(--spacing-2xl) 0;
}

.loading {
  display: inline-flex;
  align-items: center;
  font-size: 1.1rem;
  color: var(--text-secondary);
}

.loading::after {
  content: "";
  display: inline-block;
  margin-left: var(--spacing-sm);
  width: 20px;
  height: 20px;
  border: 2px solid var(--color-primary-light);
  border-radius: var(--radius-full);
  border-top-color: var(--color-primary);
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

/* 错误状态 */
.error-container {
  padding: var(--spacing-lg);
  margin-top: var(--spacing-xl);
  background-color: rgba(245, 108, 108, 0.15);
  border-radius: var(--radius-md);
  text-align: center;
  color: var(--color-danger);
}

.error-container button {
  margin-top: var(--spacing-md);
  padding: var(--spacing-sm) var(--spacing-lg);
  background-color: var(--color-danger);
  color: var(--text-white);
  border: none;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: opacity var(--transition-fast);
}

.error-container button:hover {
  opacity: 0.9;
}

/* 响应式 */
@media (max-width: 600px) {
  .form-title {
    font-size: 1.3rem;
  }
  
  .form-actions {
    flex-direction: column;
  }
  
  .btn-submit,
  .btn-cancel {
    width: 100%;
  }
}
</style>