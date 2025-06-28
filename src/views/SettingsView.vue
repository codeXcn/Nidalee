<template>
  <div>
    <div>
      <h1 class="text-3xl font-bold text-foreground">设置</h1>
      <p class="text-muted-foreground">自定义您的应用程序外观和行为</p>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- 主题设置 -->
      <Card class="p-6">
        <div class="space-y-4">
          <div>
            <h2 class="text-lg font-semibold">主题定制</h2>
            <p class="text-sm text-muted-foreground">自定义应用程序的外观主题</p>
          </div>
          <ThemeCustomizer />
        </div>
      </Card>

      <!-- 其他设置 -->
      <Card class="p-6">
        <div class="space-y-4">
          <div>
            <h2 class="text-lg font-semibold">应用设置</h2>
            <p class="text-sm text-muted-foreground">配置应用程序的基本设置</p>
          </div>

          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <div>
                <Label class="text-sm font-medium">自动启动</Label>
                <p class="text-xs text-muted-foreground">随系统启动应用程序</p>
              </div>
              <Switch />
            </div>

            <div class="flex items-center justify-between">
              <div>
                <Label class="text-sm font-medium">最小化到托盘</Label>
                <p class="text-xs text-muted-foreground">关闭窗口时最小化到系统托盘</p>
              </div>
              <Switch />
            </div>

            <div class="flex items-center justify-between">
              <div>
                <Label class="text-sm font-medium">自动检查更新</Label>
                <p class="text-xs text-muted-foreground">定期检查并提示可用更新</p>
              </div>
              <Switch />
            </div>
          </div>
        </div>
      </Card>
    </div>
  </div>
</template>
