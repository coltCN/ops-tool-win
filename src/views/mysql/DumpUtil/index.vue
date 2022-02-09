<template>
  <div class="dump-container">
    <div class="icon-btn-group">
      <span class="icon-btn icon-primary" @click="openFile">
        <svg-icon icon-class="folder-open" />
      </span>
      <span class="icon-btn icon-danger" @click="extract">
        <svg-icon icon-class="save" />
      </span>
    </div>
    <el-table
      class="table"
      :data="dbData"
      @selection-change="handleSelectionChange"
    >
      <el-table-column type="selection" width="50"></el-table-column>
      <el-table-column label="数据库" property="name"></el-table-column>
    </el-table>
  </div>
</template>
<script setup>
import { ElButton, ElTable, ElTableColumn } from 'element-plus'
import { ref } from 'vue'
import { dialog, invoke } from '@tauri-apps/api'
let file = ref('')
const dbData = ref([])
const selection = ref([])
const openFile = () => {
  let prop = {
    default: '.',
    directory: false,
  }
  dialog.open(prop).then((path) => {
    if (!path) {
      return
    }
    file.value = path
    invoke('list_db', { path }).then((dbList) => {
      dbData.value = dbList.map((v) => {
        return { name: v }
      })
      console.log(dbData.value)
    })
  })
}
const handleSelectionChange = (val) => {
  selection.value = val.map((d) => d.name)
}
const extract = () => {
  if (selection.value.length > 0) {
    dialog.open({ directory: true }).then((path) => {
      console.log(path)
      if (path) {
        invoke('extract_dumpfile', {
          path: file.value,
          saveDir: path,
          dbList: selection.value,
        })
      }
    })
  }
}
</script>
<style lang="scss" scoped>
.dump-container {
  height: 100%;
  display: flex;
  flex-direction: column;
  .icon-btn-group {
    height: 20px;
    .icon-btn {
      display: inline-block;
      margin: 0 10px;
      font-size: 20px;
      cursor: pointer;
    }
    .icon-primary {
      color: var(--el-color-primary);
    }
    .icon-danger {
      color: var(--el-color-danger);
    }
  }
  .table {
    flex: 1;
  }
}
</style>
