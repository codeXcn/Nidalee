<style scoped>
.z {
  position: absolute;
  font-size: 32px;
  opacity: 0;
}
.z-1 {
  animation: swayUpToRight 2s ease-out infinite;
}
.z-2 {
  animation: swayUpToRight 2s ease-out 0.5s infinite;
}
.z-3 {
  animation: swayUpToRight 2s ease-out 1s infinite;
}
.z-4 {
  animation: swayUpToRight 2s ease-out 1.5s infinite;
}
@keyframes swayUpToRight {
  0% {
    transform: translate(0, 0) rotate(0deg);
    opacity: 1;
  }
  100% {
    transform: translate(80px, -100px) rotate(30deg);
    opacity: 0;
  }
}
</style>

<template>
  <div class="flex items-center justify-center h-full">
    <div class="z z-1">Z</div>
    <div class="z z-2">Z</div>
    <div class="z z-3">Z</div>
    <div class="z z-4">Z</div>
  </div>
</template>
