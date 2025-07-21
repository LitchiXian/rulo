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

      <!-- 分类和标签 -->
      <!--      <div class="form-row">-->
      <!--        <div class="form-group form-half">-->
      <!--          <label for="article-category">分类</label>-->
      <!--          <select-->
      <!--              id="article-category"-->
      <!--              v-model="form.category"-->
      <!--              class="form-select"-->
      <!--          >-->
      <!--            <option v-for="cat in categories" :key="cat.id" :value="cat.id">-->
      <!--              {{ cat.name }}-->
      <!--            </option>-->
      <!--          </select>-->
      <!--        </div>-->

      <!--        <div class="form-group form-half">-->
      <!--          <label for="article-tags">标签（多个用逗号分隔）</label>-->
      <!--          <input-->
      <!--              id="article-tags"-->
      <!--              type="text"-->
      <!--              v-model="form.tags"-->
      <!--              placeholder="例如: Vue,前端,JavaScript"-->
      <!--              class="form-input"-->
      <!--          />-->
      <!--        </div>-->
      <!--      </div>-->

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
import {ref, reactive, onMounted} from 'vue'
import {useRouter} from 'vue-router'
import {save} from '@/api/web/blogArticle.ts'

const router = useRouter()

// 表单数据模型
const form = reactive({
  title: '',
  content: '',
  category: 1,
  tags: '',
  excerpt: '',
  isPublished: true,
  isFeatured: false
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
    const tagsArray = form.tags
        ? form.tags.split(',').map(tag => tag.trim())
        : []

    const data = {
      title: form.title,
      content: form.content,
      categoryId: form.category,
      tags: tagsArray,
      excerpt: form.excerpt || null,
      isPublished: form.isPublished ? 1 : 0,
      isFeatured: form.isFeatured
    }

    // 调用API保存
    await save(data)

    // 保存成功，返回首页
    router.push('/')
  } catch (err) {
    console.error('保存文章失败:', err)
    error.value = '保存失败: ' + (err.response?.data?.message || '服务器错误')
  } finally {
    loading.value = false
  }
}

const retrySubmit = () => {
  error.value = ''
  submitForm()
}

// 初始化分类选项
onMounted(() => {
  form.category = categories.value[0].id
})
</script>

<style scoped>
.create-article-container {
  width: 90%;
  padding: 40px 20px;
  color: #ece2c0;

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