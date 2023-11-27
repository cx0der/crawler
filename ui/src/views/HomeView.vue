<template>
  <main>
    <h1 class="headline-large">Crawly</h1>
    <div class="main__container">
      <div class="home__nav">
        <ul class="feed">
          <li v-for="feed in feedStore.rawFeeds" :key="feed.id" class="feed__item">
            <a class="feed__item--link title-medium" href="#">{{ feed.name }}</a>
          </li>
        </ul>
      </div>
      <div class="home__articles">
        <ul class="articles">
          <li v-for="article in feedStore.rawUnreadArticles" :key="article.id">
            <ArticleListItem
              :article="article"
              :feed-name="getFeedName(article.feedId)"
              @read-toggle="onArticleReadToggle"
              @favorite-toggle="onArticleFavoriteToggle"
            />
          </li>
        </ul>
      </div>
    </div>
  </main>
</template>

<script setup lang="ts">
import { onMounted } from "vue";
import { useFeedStore } from "@/stores/feed";
import ArticleListItem from "@/components/ArticleListItem.vue";

const feedStore = useFeedStore()

onMounted(() => {
  feedStore.fetchFeeds()
  feedStore.fetchUnreadArticles()
})

const getFeedName = (feedId: String): String => {
  const feed = feedStore.rawFeeds.find(f => f.id === feedId)
  return feed ? feed.name : ""
}

const onArticleReadToggle = (isRead: boolean, id: String) => {
  console.log(`isRead: ${isRead}, id: ${id}`)
}
const onArticleFavoriteToggle = (isFavorite: boolean, id: String) => {
  console.log(`isFavorite: ${isFavorite}, id: ${id}`)
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
.feed {
  list-style: none;
  padding: 0;
}
.feed__item {
  margin-bottom: 8px;
  margin-left: 8px;
  margin-right: 8px;
}
.feed__item--link {
  text-decoration: none;
}
.feed__link:hover {
  text-decoration: underline;
}
.articles {
  list-style: none;
}
</style>
