<script setup>
import { ref } from 'vue'
import {test1, test2} from "../api/web/hello.js";

defineProps({
  msg: String,
})

const count = ref(0)
const result = ref("")

const handleTest1 = () => {
  count.value++
  const result1 = test1();
  console.log(result1);
  result.value = result1;
}

const handleTest2 = () => {
  const result2 = test2();
}


</script>

<template>
  <h1>{{ msg }}</h1>

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
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
