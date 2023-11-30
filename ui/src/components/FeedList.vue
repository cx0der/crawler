<template>
  <aside>
    <nav>
      <ul class="feed">
        <li @click="onFeedSelect('')" class="feed__item">
          <span class="title-medium"> All </span>
          <span class="title-small">({{ articles.length }})</span>
        </li>
        <li v-for="feed in feeds" :key="feed.id" @click="onFeedSelect(feed.id)" class="feed__item">
          <span class="title-medium">
            {{ feed.name }}
          </span>
          <span class="title-small">({{ getUnreadCount(feed.id) }})</span>
        </li>
      </ul>
    </nav>
  </aside>
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
<style scoped>
.feed {
  list-style: none;
  padding: 0;
}
.feed__item {
  cursor: pointer;
  margin-bottom: 8px;
  margin-left: 8px;
  margin-right: 8px;
}
.feed__item:hover {
  text-decoration: underline;
}
</style>
