<script setup>
import { ref } from 'vue'
import {test1, test2} from "../api/web/hello.js";
import {Check, Delete, Edit, Message, Search, Star} from "@element-plus/icons-vue";

defineProps({
  msg: String,
})

const count = ref(0)
const result = ref("")
const openReportDialog = ref(false) // 控制举报弹窗显示
const reportResult = ref("")

const handleTest1 = async () => {
  // count.value++
  // const result1 = test1();
  // console.log(result1);
  // result.value = result1;

  try {
    const response = await test1();
    console.log(response);
    result.value = response;
  } catch (error) {
    result.value = "请求失败";
  }
}

const handleTest2 = () => {
  const result2 = test2();
}

// 打开举报弹窗
const handleReportDialogOpen = () => {
  openReportDialog.value = true;
}

// 提交举报
const submitReport = () => {
  // 这里模拟举报提交，实际应该调用后端接口
  reportResult.value = "举报失败：您举报的虚假宣传已被他人举报";

  // 3秒后清除消息
  setTimeout(() => {
    reportResult.value = "";
  }, 3000);

  // 关闭弹窗
  openReportDialog.value = false;
}

</script>

<template>
  <h1>{{ msg }}</h1>

  <!-- 举报弹窗 -->
  <div v-if="openReportDialog" class="report-overlay">
    <div class="report-dialog">
      <h3>举报虚假宣传</h3>
      <p>您确定要举报此内容存在虚假宣传吗？</p>

      <div class="dialog-actions">
        <button class="cancel-btn" @click="openReportDialog = false">取消</button>
        <button class="confirm-btn" @click="submitReport">确认举报</button>
      </div>
    </div>
  </div>

  <div class="card">
    <button type="button" @click="handleTest1">count is {{ count }}</button>
    <p>
      Edit
      <code>components/HelloWorld.vue</code> to test HMR
    </p>
  </div>

  <p>
    Check out
    <a href="https://vuejs.org/guide/quick-start.html#local" target="_blank"
      >create-vue</a
    >, the official Vue + Vite starter
  </p>
  <p>
    Learn more about IDE Support for Vue in the
    <a
      href="https://vuejs.org/guide/scaling-up/tooling.html#ide-support"
      target="_blank"
      >Vue Docs Scaling up Guide</a
    >.
  </p>
  <p class="read-the-docs">Click on the Vite and Vue logos to learn more</p>
  <div>
    <p>"""""{{result}}"""""</p>
  </div>

  <!-- 添加举报按钮 -->
  <div class="report-section">
    <button class="report-btn" @click="handleReportDialogOpen">举报</button>
    <div v-if="reportResult" class="report-message">{{ reportResult }}</div>
  </div>

  <div class="mb-4">
    <el-button>Default</el-button>
    <el-button type="primary">Primary</el-button>
    <el-button type="success">Success</el-button>
    <el-button type="info">Info</el-button>
    <el-button type="warning">Warning</el-button>
    <el-button type="danger">Danger</el-button>
  </div>

  <div class="mb-4">
    <el-button plain>Plain</el-button>
    <el-button type="primary" plain>Primary</el-button>
    <el-button type="success" plain>Success</el-button>
    <el-button type="info" plain>Info</el-button>
    <el-button type="warning" plain>Warning</el-button>
    <el-button type="danger" plain>Danger</el-button>
  </div>

  <div class="mb-4">
    <el-button round>Round</el-button>
    <el-button type="primary" round>Primary</el-button>
    <el-button type="success" round>Success</el-button>
    <el-button type="info" round>Info</el-button>
    <el-button type="warning" round>Warning</el-button>
    <el-button type="danger" round>Danger</el-button>
  </div>

  <div>
    <el-button :icon="Search" circle />
    <el-button type="primary" :icon="Edit" circle />
    <el-button type="success" :icon="Check" circle />
    <el-button type="info" :icon="Message" circle />
    <el-button type="warning" :icon="Star" circle />
    <el-button type="danger" :icon="Delete" circle />
  </div>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}

/* 举报按钮样式 */
.report-btn {
  background-color: #ff4d4f;
  color: white;
  border: none;
  border-radius: 4px;
  padding: 8px 16px;
  margin-top: 20px;
  cursor: pointer;
  font-weight: bold;
  transition: background-color 0.3s;
}

.report-btn:hover {
  background-color: #d9363e;
}

/* 举报弹窗样式 */
.report-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.report-dialog {
  background-color: white;
  border-radius: 8px;
  padding: 24px;
  width: 320px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.report-dialog h3 {
  margin-top: 0;
  color: #333;
}

.report-dialog p {
  color: #666;
  margin-bottom: 24px;
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

.dialog-actions button {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.cancel-btn {
  background-color: #f0f0f0;
  color: #333;
}

.cancel-btn:hover {
  background-color: #e6e6e6;
}

.confirm-btn {
  background-color: #ff4d4f;
  color: white;
}

.confirm-btn:hover {
  background-color: #d9363e;
}

/* 举报结果消息 */
.report-message {
  margin-top: 15px;
  padding: 10px;
  background-color: #fff2f0;
  border: 1px solid #ffccc7;
  border-radius: 4px;
  color: #d9363e;
  font-weight: 500;
}

/* 举报区域 */
.report-section {
  margin-top: 30px;
  padding: 20px;
  border-radius: 8px;
  background-color: #fafafa;
  border: 1px solid #f0f0f0;
  text-align: center;
}
</style>
