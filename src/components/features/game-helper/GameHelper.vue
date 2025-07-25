<template>
  <div class="space-y-8">
    <!-- 美化后的 Tabs -->
    <Tabs v-model="activeTab" class="w-full">
      <TabsList class="w-full flex gap-2 bg-transparent p-0 border-0 shadow-none">
        <TabsTrigger
          v-for="tab in tabs"
          :key="tab.id"
          :value="tab.id"
          class="flex-1 min-w-fit flex items-center justify-center gap-2 rounded-lg py-2 px-4 text-base font-medium transition-all duration-200 border border-transparent hover:border-primary/40 hover:bg-primary/10 hover:text-primary focus-visible:ring-2 focus-visible:ring-primary/40 data-[state=active]:bg-primary data-[state=active]:text-primary-foreground data-[state=active]:shadow-md data-[state=active]:border-primary"
        >
          <component :is="tab.icon" class="h-4 w-4" />
          {{ tab.label }}
        </TabsTrigger>
      </TabsList>
      <TabsContent value="profile-background">
        <div class="space-y-6">
          <SummonerNoteEditor />
          <SummonerRankEditor />
          <ProfileBackgroundManager />
        </div>
      </TabsContent>
      <TabsContent value="champion-builds">
        <!-- <ChampionBuildManager /> -->
        <Card class="p-8 text-center">
          <div class="space-y-4">
            <div class="w-16 h-16 mx-auto bg-muted rounded-full flex items-center justify-center">
              <Sparkles class="h-8 w-8 text-muted-foreground" />
            </div>
            <div class="space-y-2">
              <h3 class="text-lg font-semibold">更多功能开发中</h3>
              <p class="text-muted-foreground">我们正在开发更多实用的游戏助手功能，敬请期待！</p>
            </div>
          </div>
        </Card>
      </TabsContent>
      <TabsContent value="more">
        <Card class="p-8 text-center">
          <div class="space-y-4">
            <div class="w-16 h-16 mx-auto bg-muted rounded-full flex items-center justify-center">
              <Sparkles class="h-8 w-8 text-muted-foreground" />
            </div>
            <div class="space-y-2">
              <h3 class="text-lg font-semibold">更多功能开发中</h3>
              <p class="text-muted-foreground">我们正在开发更多实用的游戏助手功能，敬请期待！</p>
            </div>
          </div>
        </Card>
      </TabsContent>
    </Tabs>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Settings, Sparkles, Users } from 'lucide-vue-next'
import ProfileBackgroundManager from './ProfileBackgroundManager.vue'
// import ChampionBuildManager from './ChampionBuildManager.vue'

const tabs = [
  {
    id: 'profile-background',
    label: '生涯名片',
    icon: Users
  },
  // {
  //   id: 'champion-builds',
  //   label: '符文出装',
  //   icon: Settings
  // },
  {
    id: 'more',
    label: '更多功能',
    icon: Sparkles
  }
]
const activeTab = ref('profile-background')
</script>
