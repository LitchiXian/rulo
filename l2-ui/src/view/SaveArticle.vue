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

      <!-- 标签选择 -->
      <div class="form-group">
        <label for="article-tags">标签</label>
        <div 
          class="tag-selector-container"
          @click="showTagSelector = true"
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

      <!-- 标签选择弹窗 -->
      <el-popover
        v-model:visible="showTagSelector"
        placement="bottom"
        trigger="manual"
        class="tag-selector-popover"
        :teleported="true"
      >
        <div class="tag-selector-content">
          <div v-for="tag in tags" :key="tag.id" class="tag-option">
            <el-checkbox
              :label="tag.id"
              :checked="selectedTags.includes(tag.id)"
              @change="(checked) => handleTagCheck(tag.id, checked)"
            >
              {{ tag.name }}
            </el-checkbox>
          </div>
          <div class="tag-selector-footer">
            <el-button size="small" @click="showTagSelector = false">取消</el-button>
            <el-button size="small" type="primary" @click="confirmTagSelection">确认</el-button>
          </div>
        </div>
      </el-popover>

      <!-- 摘要和状态 -->
      <!--      <div class="form-group">-->
      <!--        <label for="article-excerpt">文章摘要</label>-->
      <!--        <textarea-->
      <!--            id="article-excerpt"-->
      <!--            v-model="form.excerpt"-->
      <!--            placeholder="输入简要说明，可选"-->
      <!--            rows="3"-->
      <!--            class="form-textarea"-->
      <!--        ></textarea>-->
      <!--      </div>-->

      <!-- 状态控制 -->
      <!--      <div class="form-group">-->
      <!--        <div class="status-row">-->
      <!--          <div>-->
      <!--            <input-->
      <!--                type="checkbox"-->
      <!--                id="publish-now"-->
      <!--                v-model="form.isPublished"-->
      <!--                class="status-checkbox"-->
      <!--            />-->
      <!--            <label for="publish-now">立即发布</label>-->
      <!--          </div>-->

      <!--          <div>-->
      <!--            <input-->
      <!--                type="checkbox"-->
      <!--                id="featured-article"-->
      <!--                v-model="form.isFeatured"-->
      <!--                class="status-checkbox"-->
      <!--            />-->
      <!--            <label for="featured-article">设为特色文章</label>-->
      <!--          </div>-->
      <!--        </div>-->
      <!--      </div>-->

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
      tags: selectedTags.value,
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
  width: 90%;
  padding: 40px 20px;
  color: #ece2c0;

}

.tag-selector-container {
  width: 100%;
  min-height: 42px;
  padding: 10px 15px;
  border: 1px solid #555;
  border-radius: 6px;
  background-color: #303030;
  color: #ece2c0;
  cursor: pointer;
  position: relative;
  display: flex;
  align-items: center;
  transition: all 0.3s;
  margin-top: 8px;
}

.tag-selector-container:hover {
  border-color: #4abbb5;
  box-shadow: 0 0 0 3px rgba(74, 187, 181, 0.2);
}

.selected-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  width: 100%;
  padding-right: 24px;
}

.placeholder-text {
  color: #888;
  font-style: italic;
  margin-left: 10px;
}

.dropdown-icon {
  color: #ece2c0;
  position: absolute;
  right: 15px;
  top: 50%;
  transform: translateY(-50%);
  transition: color 0.2s;
}

.tag-selector-container:hover .dropdown-icon {
  color: #4abbb5;
}

.selected-tag {
  background-color: #4abbb5;
  color: #000;
  border: none;
  transition: all 0.2s;
}

.selected-tag:hover {
  background-color: #5cd6cf;
}

.tag-selector-popover {
  width: 300px;
  background-color: #303030;
  border: 1px solid #555;
  border-radius: 6px;
  padding: 10px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.tag-selector-content {
  max-height: 200px;
  overflow-y: auto;
  padding-right: 5px;
}

.tag-selector-content::-webkit-scrollbar {
  width: 8px;
}

.tag-selector-content::-webkit-scrollbar-track {
  background: #444;
  border-radius: 4px;
}

.tag-selector-content::-webkit-scrollbar-thumb {
  background: #666;
  border-radius: 4px;
}

.tag-selector-content::-webkit-scrollbar-thumb:hover {
  background: #4abbb5;
}

.tag-option {
  padding: 5px 0;
}

.tag-option .el-checkbox__label {
  color: #ece2c0;
}

.tag-option .el-checkbox__input.is-checked .el-checkbox__inner {
  background-color: #4abbb5;
  border-color: #4abbb5;
}

.tag-selector-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 10px;
  padding-top: 10px;
  border-top: 1px solid #555;
}

.tag-selector-footer .el-button {
  background-color: #444;
  border-color: #555;
  color: #ece2c0;
}

.tag-selector-footer .el-button:hover {
  background-color: #555;
  border-color: #666;
}

.tag-selector-footer .el-button--primary {
  background-color: #4abbb5;
  border-color: #4abbb5;
  color: #000;
}

.tag-selector-footer .el-button--primary:hover {
  background-color: #5cd6cf;
  border-color: #5cd6cf;
}

.form-title {
  font-size: 1.8rem;
  margin-bottom: 30px;
  padding-bottom: 15px;
  border-bottom: 1px solid #555;
  color: #4abbb5;
}

.form-group {
  margin-bottom: 25px;
}

/* 保留原有的其他样式 */
.form-half {
  flex: 1;
}

label {
  display: block;
  margin-bottom: 8px;
  font-weight: 500;
  color: #e2e2ec;
}

.form-input, .form-select, .form-textarea {
  width: 100%;
  padding: 12px 15px;
  border: 1px solid #555;
  border-radius: 6px;
  background-color: #303030;
  color: #ece2c0;
  font-size: 1rem;
  transition: all 0.3s;
}

.form-input:focus, .form-textarea:focus {
  border-color: #4abbb5;
  box-shadow: 0 0 0 3px rgba(74, 187, 181, 0.2);
  outline: none;
}

.form-textarea {
  min-height: 150px;
  resize: vertical;
}

.form-select {
  appearance: none;
  background-image: url("data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='%23ece2c0' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3e%3cpolyline points='6 9 12 15 18 9'%3e%3c/polyline%3e%3c/svg%3e");
  background-repeat: no-repeat;
  background-position: right 12px center;
  background-size: 16px;
  cursor: pointer;
}

.form-row {
  display: flex;
  gap: 20px;
}

.status-row {
  display: flex;
  gap: 25px;
  margin-top: 15px;
}

.status-row div {
  display: flex;
  align-items: center;
}

.status-checkbox {
  margin-right: 8px;
  width: 18px;
  height: 18px;
  accent-color: #4abbb5;
}

.form-actions {
  display: flex;
  gap: 15px;
  margin-top: 30px;
}

.btn-submit, .btn-cancel {
  padding: 12px 25px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 500;
  transition: all 0.3s;
  font-size: 1rem;
}

.btn-submit {
  background-color: #4abbb5;
  color: #303030;
}

.btn-submit:hover {
  background-color: #3ca9a4;
  transform: translateY(-2px);
}

.btn-cancel {
  background-color: transparent;
  color: #ece2c0;
  border: 1px solid #555;
}

.btn-cancel:hover {
  background-color: rgba(255, 255, 255, 0.1);
  border-color: #777;
}

/* 加载状态样式 */
.loading-container {
  text-align: center;
  padding: 50px 0;
}

.loading {
  display: inline-flex;
  align-items: center;
  font-size: 1.2rem;
}

.loading::after {
  content: "";
  display: inline-block;
  margin-left: 10px;
  width: 24px;
  height: 24px;
  border: 3px solid rgba(74, 187, 181, 0.3);
  border-radius: 50%;
  border-top-color: #4abbb5;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

/* 错误容器样式 */
.error-container {
  padding: 20px;
  margin-top: 30px;
  background-color: rgba(255, 107, 107, 0.15);
  border-radius: 6px;
  text-align: center;
}

.error-container button {
  margin-top: 15px;
  padding: 8px 20px;
  background-color: rgba(255, 107, 107, 0.5);
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.3s;
}

.error-container button:hover {
  background-color: rgba(255, 107, 107, 0.7);
}
</style>