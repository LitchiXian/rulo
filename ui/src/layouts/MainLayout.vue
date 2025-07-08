<template>
  <div class="min-h-screen bg-gray-50">
    <header class="bg-white shadow-sm">
      <div class="container mx-auto py-4 px-4">
        <router-link to="/" class="text-xl font-bold">我的技术博客</router-link>
      </div>
    </header>

    <main class="container mx-auto py-8 px-4 max-w-3xl">
      <slot />
    </main>

    <footer class="mt-16 py-8 text-center text-gray-500 text-sm">
      © 2025 我的技术博客 · 用 ❤️ 搭建
    </footer>
  </div>
</template>