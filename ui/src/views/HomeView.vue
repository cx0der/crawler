<template>
  <main>
    <h1 class="headline-large">Crawly</h1>
    <div class="main__container">
      <div class="home__nav">
        <FeedList
          :articles="feedStore.unreadArticles"
          :feeds="feedStore.feeds"
          @feed-select="onFeedSelected"
        />
      </div>
      <div class="home__articles">
        <ArticleList
          :articles="articlesToDisplay"
          :feeds="feedStore.feeds"
          @read-toggle="onArticleReadToggle"
          @favorite-toggle="onArticleFavoriteToggle"
        />
      </div>
    </div>
  </main>
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

<style scoped>
.main__container {
  display: flex;
}
.home__nav {
  width: 15vw;
}

.home__articles {
  width: 85vw;
}
</style>
