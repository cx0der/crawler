<template>
  <ul class="articles">
    <li v-for="article in articles" :key="article.id">
      <ArticleListItem
        :article="article"
        :feed-name="getFeedName(article.feedId)"
        @read-toggle="onArticleReadToggle"
        @favorite-toggle="onArticleFavoriteToggle"
      />
    </li>
  </ul>
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
  'read-toggle': [isRead: boolean, id: String]
  'favorite-toggle': [isFavorite: boolean, id: String]
}>()

const props = defineProps<Props>()

const getFeedName = (feedId: String): String => {
  const feed = props.feeds.find((f) => f.id === feedId)
  return feed ? feed.name : ''
}

const onArticleReadToggle = (isRead: boolean, id: String) => {
  emit('read-toggle', isRead, id)
}
const onArticleFavoriteToggle = (isFavorite: boolean, id: String) => {
  emit('favorite-toggle', isFavorite, id)
}
</script>
<style scoped></style>
