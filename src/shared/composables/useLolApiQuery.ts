import { useQuery } from '@tanstack/vue-query'
import * as dataApi from '@/lib/dataApi'
import type {
  DDragonChampionsResponse,
  DDragonRuneTree,
  DDragonItemsResponse,
  CommunityDragonSkin,
  CommunityDragonChampionSummary,
  CommunityDragonChampion,
  QueueInfo,
  RiotMap,
  RiotGameMode,
  RiotGameType,
  ApiResponse,
  CommunityDragonPerk
} from '@/lib/dataApi'
import { invoke } from '@tauri-apps/api/core'

// 获取所有符文
export function useAllRunesQuery() {
  return useQuery({
    queryKey: ['all-runes'],
    queryFn: () => invoke<DataDragonRune[]>('get_all_runes')
  })
}

// 获取推荐出装和符文
export function useBuildsByAliasQuery(source: string, champion: string) {
  return useQuery({
    queryKey: ['builds-by-alias', source, champion],
    queryFn: () => invoke<BuildSection>('get_builds_by_alias', { source, champion }),
    enabled: false
  })
}

export function usePlayerListQuery(enabled: MaybeRefOrGetter<boolean>) {
  return useQuery({
    queryKey: ['liveclient-playerlist'],
    queryFn: async () => {
      const result = await invoke<string>('get_live_player_list')
      if (typeof result === 'string') {
        try {
          return JSON.parse(result) as LiveClientPlayer[]
        } catch (error) {
          console.error('Failed to parse player list JSON:', error)
          return []
        }
      }
      // 如果后端直接返回了数组，也做兼容
      return Array.isArray(result) ? result : []
    },
    enabled: () => toValue(enabled)
  })
}
// 具体API的类型安全封装
export function useChampionsQuery(version?: string) {
  return useQuery<ApiResponse<DDragonChampionsResponse>, unknown, DDragonChampionsResponse>({
    queryKey: ['champions', version],
    queryFn: () => dataApi.fetchChampions(version),
    select: (res) => res.data as DDragonChampionsResponse
  })
}

export function useRunesQuery(version?: string) {
  return useQuery<ApiResponse<DDragonRuneTree[]>, unknown, DDragonRuneTree[]>({
    queryKey: ['runes', version],
    queryFn: () => dataApi.fetchRunes(version),
    select: (res) => res.data as DDragonRuneTree[]
  })
}

export function useItemsQuery(version?: string) {
  return useQuery<ApiResponse<DDragonItemsResponse>, unknown, DDragonItemsResponse>({
    queryKey: ['items', version],
    queryFn: () => dataApi.fetchItems(version),
    select: (res) => res.data as DDragonItemsResponse
  })
}

export function useSkinsQuery() {
  return useQuery<ApiResponse<CommunityDragonSkin[]>, unknown, CommunityDragonSkin[]>({
    queryKey: ['skins'],
    queryFn: () => dataApi.fetchSkins(),
    select: (res) => res.data as CommunityDragonSkin[]
  })
}

export function useChampionSummaryQuery() {
  return useQuery<ApiResponse<CommunityDragonChampionSummary[]>, unknown, CommunityDragonChampionSummary[]>({
    queryKey: ['championSummary'],
    queryFn: () => dataApi.fetchChampionSummary(),
    staleTime: 2 * 60 * 60 * 1000, // 2小时缓存
    gcTime: 4 * 60 * 60 * 1000, // 4小时垃圾回收
    select: (res) => res.data as CommunityDragonChampionSummary[]
  })
}

export function useChampionDetailsQuery(championId: MaybeRefOrGetter<number>) {
  return useQuery<ApiResponse<CommunityDragonChampion>, unknown, CommunityDragonChampion>({
    queryKey: ['championDetails', championId],
    queryFn: () => dataApi.fetchChampionDetails(toValue(championId)),
    select: (res) => res.data as CommunityDragonChampion,
    enabled: () => !!toValue(championId)
  })
}

export function useMapsQuery() {
  return useQuery<ApiResponse<RiotMap[]>, unknown, RiotMap[]>({
    queryKey: ['maps'],
    queryFn: () => dataApi.fetchMaps(),
    select: (res) => res.data as RiotMap[]
  })
}

export function useGameModesQuery() {
  return useQuery<ApiResponse<RiotGameMode[]>, unknown, RiotGameMode[]>({
    queryKey: ['gameModes'],
    queryFn: () => dataApi.fetchGameModes(),
    select: (res) => res.data as RiotGameMode[]
  })
}

export function useGameTypesQuery() {
  return useQuery<ApiResponse<RiotGameType[]>, unknown, RiotGameType[]>({
    queryKey: ['gameTypes'],
    queryFn: () => dataApi.fetchGameTypes(),
    select: (res) => res.data as RiotGameType[]
  })
}

export function useQueuesQuery() {
  return useQuery({
    queryKey: ['queues'],
    queryFn: () => dataApi.fetchQueues(),
    select: (res) => res.data as QueueInfo[]
  })
}

export function useCommunityDragonPerksQuery() {
  return useQuery({
    queryKey: ['communityDragonPerks'],
    queryFn: () => dataApi.fetchCommunityDragonPerks(),
    select: (res) => res.data as CommunityDragonPerk[],
    staleTime: 2 * 60 * 60 * 1000, // 2小时缓存
    gcTime: 24 * 60 * 60 * 1000 // 24小时垃圾回收
  })
}
