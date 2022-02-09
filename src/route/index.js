import { createRouter, createWebHistory } from 'vue-router'
import Layout from '~/layouts/index.vue'
const constantRoutes = [
  {
    path: '/',
    name: 'Home',
    component: () => import('~/views/home/index.vue'),
    hidden: true,
  },
  {
    path: '/setting',
    name: 'Setting',
    component: () => import('~/views/Setting.vue'),
    hidden: true,
  },
  {
    path: '/404',
    name: '404',
    component: () => import('~/views/error-page/404.vue'),
    hidden: true,
  },
  { path: '/:pathMatch(.*)*', redirect: '/404' },
]

const routeFiles = import.meta.globEager('./*/*.js')

const routes = Object.keys(routeFiles).reduce((routes, path) => {
  const routeName = path.replace(/^\.\/(.*)\/.+\.\w+$/, '$1')
  const module = routeFiles[path].default
  routes.push({
    path: `/${routeName}`,
    name: routeName,
    component: Layout,
    meta: module.meta,
    children: Object.keys(module.children).map((child) => {
      return {
        path: child,
        name: child,
        component: module.children[child],
      }
    }),
  })
  return routes
}, [])

export default createRouter({
  history: createWebHistory(),
  routes: [...routes, ...constantRoutes],
})
