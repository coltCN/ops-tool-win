const children = {
  DumpUtil: () => import('~/views/mysql/DumpUtil/index.vue'),
}

export default {
  children,
  meta: {
    title: 'Mysql Dump 文件提取',
    icon: 'database-center',
  },
}
