<template>
  <v-list>
    <v-list-item v-for="article in articles" :key="article.id">
      <ArticleListItem
        :article="article"
        :feed-name="getFeedName(article.feedId)"
        @read-toggle="onArticleReadToggle"
        @favorite-toggle="onArticleFavoriteToggle"
      />
    </v-list-item>
  </v-list>
</template>
<script setup lang="ts">
import type { Article } from '@/models/Article'
import type { Feed } from '@/models/Feed'
import ArticleListItem from './ArticleListItem.vue'

interface Props {
  feeds: Feed[]
  articles: Article[]
}

const emit = defineEmits<{
  'read-toggle': [isRead: boolean, id: string]
  'favorite-toggle': [isFavorite: boolean, id: string]
}>()

const props = defineProps<Props>()

const getFeedName = (feedId: string): string => {
  const feed = props.feeds.find((f) => f.id === feedId)
  return feed ? feed.name : ''
}

const onArticleReadToggle = (isRead: boolean, id: string) => {
  emit('read-toggle', isRead, id)
}
const onArticleFavoriteToggle = (isFavorite: boolean, id: string) => {
  emit('favorite-toggle', isFavorite, id)
}
</script>
<style scoped></style>
