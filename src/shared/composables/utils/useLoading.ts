export function useLoading() {
  const loading = ref(false)
  const toggle = () => {
    loading.value = !loading.value
  }
  return {
    loading,
    toggle
  }
}
