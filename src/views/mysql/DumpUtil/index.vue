<template>
  <div>导出工具</div>
  <div>
    <el-button type="primary" @click="openFile">打开</el-button>
    <el-button type="primary" @click="extract">提取</el-button>
  </div>
  <el-table :data="dbData" @selection-change="handleSelectionChange">
    <el-table-column type="selection" width="50"></el-table-column>
    <el-table-column label="数据库" property="name"></el-table-column>
  </el-table>
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
