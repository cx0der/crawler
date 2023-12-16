<template>
  <v-layout class="rounded rounded-md">
    <v-app-bar title="Crawly"></v-app-bar>
    <FeedList
      :articles="feedStore.unreadArticles"
      :feeds="feedStore.feeds"
      @feed-select="onFeedSelected"
    />
    <v-main>
      <ArticleList
        :articles="articlesToDisplay"
        :feeds="feedStore.feeds"
        @read-toggle="onArticleReadToggle"
        @favorite-toggle="onArticleFavoriteToggle"
      />
    </v-main>
  </v-layout>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue'
import { useFeedStore } from '@/stores/feed'
import ArticleList from '@/components/ArticleList.vue'
import FeedList from '@/components/FeedList.vue'
import type { Article } from '@/models/Article'

const feedStore = useFeedStore()

onMounted(() => {
  feedStore.fetchFeeds()
  feedStore.fetchUnreadArticles()
})

const selectedFeed = ref('')
reactive({ selectedFeed })

const onArticleReadToggle = (isRead: boolean, id: string) => {
  feedStore.updateArticleReadState(id, isRead)
}
const onArticleFavoriteToggle = (isFavorite: boolean, id: string) => {
  feedStore.updateArticleFavouriteState(id, isFavorite)
}

const onFeedSelected = (id: string) => {
  selectedFeed.value = id
}

const articlesToDisplay = computed((): Article[] => {
  if (selectedFeed.value) {
    return feedStore.unreadArticles.filter((a) => a.feedId === selectedFeed.value)
  }
  return feedStore.unreadArticles
})
</script>

<style scoped></style>
