<template>
  <div class="min-h-screen bg-gradient-to-br from-background via-muted/60 to-background/80 py-10 px-4">
    <div class="max-w-4xl mx-auto space-y-8">
      <div class="flex items-center gap-4 mb-4">
        <span class="inline-flex h-10 w-10 items-center justify-center rounded-full bg-primary/10 text-primary shadow">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
            <path
              d="M12 3v2m0 14v2m9-9h-2M5 12H3m15.364-6.364l-1.414 1.414M6.05 17.95l-1.414 1.414m12.728 0l-1.414-1.414M6.05 6.05L4.636 4.636"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
          </svg>
        </span>
        <div>
          <h1 class="text-3xl font-extrabold text-foreground tracking-tight">设置</h1>
          <p class="text-muted-foreground text-base mt-1">自定义您的应用程序外观和行为</p>
        </div>
      </div>
      <Card
        class="p-8 rounded-2xl shadow-xl bg-gradient-to-br from-white/80 to-muted/60 dark:from-background/80 dark:to-muted/40 border border-border"
      >
        <div class="space-y-6">
          <div>
            <h2 class="text-xl font-bold text-primary">主题定制</h2>
            <p class="text-sm text-muted-foreground">自定义应用程序的外观主题</p>
          </div>
          <div class="border-t border-dashed border-border pt-6">
            <ThemeCustomizer />
          </div>
        </div>
      </Card>
      <Card
        class="p-8 rounded-2xl shadow-xl bg-gradient-to-br from-white/80 to-muted/60 dark:from-background/80 dark:to-muted/40 border border-border"
      >
        <div class="space-y-6">
          <div>
            <h2 class="text-xl font-bold text-primary">快捷键设置</h2>
            <p class="text-sm text-muted-foreground">自定义您的快捷键</p>
          </div>
          <div class="border-t border-dashed border-border pt-6">
            <!-- 这里后续可以放快捷键设置组件 -->
            <div class="text-muted-foreground">敬请期待...</div>
          </div>
        </div>
      </Card>
    </div>
  </div>
</template>
