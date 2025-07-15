// Live Client Data API - 玩家基础信息类型
export interface LiveClientPlayer {
  summonerName: string
  championName: string
  team: string
  level: number
  isBot: boolean
  rawChampionName: string
  skinID: number
  summonerSpells: {
    summonerSpellOne: string
    summonerSpellTwo: string
  }
  runes: {
    keystone: string
    primaryRuneTree: string
    secondaryRuneTree: string
  }
  items: Array<{
    itemID: number
    count: number
  }>
  scores: Record<string, number>
  position: string
  currentGold: number
  totalGold: number
  respawnTimer: number
  isDead: boolean
  rawSkinName: string
}

// 获取所有玩家基础信息（队友和对手）

import { useQuery } from '@tanstack/vue-query'
import { toValue, type MaybeRefOrGetter } from 'vue'
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
  ApiResponse
} from '@/lib/dataApi'
import { invoke } from '@tauri-apps/api/core'
export function usePlayerListQuery(enabled: MaybeRefOrGetter<boolean>) {
  return useQuery<LiveClientPlayer[], unknown, LiveClientPlayer[]>({
    queryKey: ['liveclient-playerlist'],
    queryFn: async () => {
      return await invoke('get_live_player_list')
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
  return useQuery<ApiResponse<QueueInfo[]>, unknown, QueueInfo[]>({
    queryKey: ['queues'],
    queryFn: () => dataApi.fetchQueues(),
    select: (res) => res.data as QueueInfo[]
  })
}
