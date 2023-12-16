<template>
  <v-navigation-drawer>
    <v-list nav>
      <v-list-item @click="onFeedSelect('')">
        <span> All </span>
        <span>({{ articles.length }})</span>
      </v-list-item>
      <v-list-item v-for="feed in feeds" :key="feed.id" @click="onFeedSelect(feed.id)">
        <span>
          {{ feed.name }}
        </span>
        <span>({{ getUnreadCount(feed.id) }})</span>
      </v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>
<script setup lang="ts">
import type { Article } from '@/models/Article'
import type { Feed } from '@/models/Feed'
interface Props {
  feeds: Feed[]
  articles: Article[]
}

const emit = defineEmits<{
  'feed-select': [id: string]
}>()
const props = defineProps<Props>()

const getUnreadCount = (id: string): number => {
  const articles = props.articles.filter((a) => a.feedId === id)
  return articles.length
}

const onFeedSelect = (id: string): void => {
  emit('feed-select', id)
}
</script>
<style scoped></style>
