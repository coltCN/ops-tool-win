import { useRouter } from 'vue-router'
const router = useRouter()
export function goTo(name) {
  router.push({ name })
}
