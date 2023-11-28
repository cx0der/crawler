<template>
  <main>
    <h1 class="headline-large">Crawly</h1>
    <div class="main__container">
      <div class="home__nav">
        <FeedList :feeds="feedStore.feeds" />
      </div>
      <div class="home__articles">
        <ArticleList
          :articles="feedStore.unreadArticles"
          :feeds="feedStore.feeds"
          @read-toggle="onArticleReadToggle"
          @favorite-toggle="onArticleFavoriteToggle"
        />
      </div>
    </div>
  </main>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useFeedStore } from '@/stores/feed'
import ArticleList from '@/components/ArticleList.vue'
import FeedList from '@/components/FeedList.vue'

const feedStore = useFeedStore()

onMounted(() => {
  feedStore.fetchFeeds()
  feedStore.fetchUnreadArticles()
})

const onArticleReadToggle = (isRead: boolean, id: String) => {
  feedStore.updateArticleReadState(id, isRead)
}
const onArticleFavoriteToggle = (isFavorite: boolean, id: String) => {
  feedStore.updateArticleFavouriteState(id, isFavorite)
}
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
