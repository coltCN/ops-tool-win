<template>
  <div class="header-panel">
    <div class="title">工具箱</div>
    <div class="button-group">
      <el-icon class="icon-button" :size="20" @click="goTo('Home')"
        ><home-filled
      /></el-icon>
    </div>
  </div>
</template>
<script setup>
import { HomeFilled } from '@element-plus/icons-vue'
import { ElIcon } from 'element-plus'
import { useRouter } from 'vue-router'

const router = useRouter()
const goTo = (name) => {
  router.push({ name })
}
</script>
<style lang="scss" scoped>
.header-panel {
  display: flex;
  justify-content: space-between;
  margin: 10px;

  .button-group {
    .icon-button {
      cursor: pointer;
      :hover {
        color: var(--el-color-primary);
      }
    }
  }
}
</style>
