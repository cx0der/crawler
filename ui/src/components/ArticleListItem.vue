<template>
  <article class="article-card on-surface">
    <div
        class="article-card__title-container">
      <span
        :class="[isRead ? 'article-card__read-indicator--read' : 'article-card__read-indicator--unread']"
        @click="onReadToggle"
        class="article-card__read-indicator"
      />
      <span
        :class="[isFavorite ? 'article-card__favorite--favorite': 'article-card__favorite--not-favorite']"
        @click="onFavoriteToggle"
        class="article-card__favorite-indicator"
      />
      <span
        @click="onDetailsToggle"
        class="title-medium article-card__title">
        {{ article.title }}
      </span>
      <div
        @click="onDetailsToggle"
        :class="[showDetails ? 'article-card__chevron--open': 'article-card__chevron--closed']"
        class="article-card__chevron"
      />
    </div>
    <div
      v-if="showDetails"
      class="article-card__contents-holder">
      <h5 class="headline-small">
        <a
          :href="article.url"
          target="_blank">
          {{ article.title }}
        </a>
      </h5>
      <div
        v-html="article.body"
        class="body-large"
      />
    </div>
    <span class="article-card__published-at body-medium">{{ formattedDate }}</span>
  </article>
</template>
<script setup lang="ts">
import type { Article } from '@/models/Article';
import { computed, reactive, ref } from 'vue';

const emit = defineEmits<{
  'read-toggle': [isRead: boolean, id: String]
  'favorite-toggle': [isFavorite: boolean, id: String]
}>()
interface Props {
  article: Article,
  feedName: String
}
const props = defineProps<Props>();

const showDetails = ref(false)
const isRead = ref(props.article.isRead)
const isFavorite = ref(props.article.isFavorite)
reactive({ showDetails, isRead, isFavorite })

const formattedDate = computed(() => {
  const date = new Date(props.article.publishedAt)
  return `Published at: ${date.toLocaleString()} by: ${props.feedName}`
})

const onDetailsToggle = () => {
  showDetails.value = !showDetails.value
}

const onReadToggle = () => {
  isRead.value = !isRead.value
  emit('read-toggle', isRead.value, props.article.id)
}

const onFavoriteToggle = () => {
  isFavorite.value = !isFavorite.value
  emit('favorite-toggle', isFavorite.value, props.article.id)
}
</script>
<style scoped>
.article-card {
  border-radius: 12px;
  box-shadow: 0 1px 2px 0 rgba(0,0,0,.15);
  margin: 0px 8px 8px 8px;
  padding-left: 16px;
  padding-right: 16px;
}
.article-card__title-container {
  display: flex;
}
.article-card__title {
  cursor: pointer;
  margin-top: 8px;
}
.article-card__read-indicator {
  cursor: pointer;
  height: 1.5em;
  margin-top: 8px;
  width: 1.5em;
}
.article-card__read-indicator--unread {
  background-image: url("../assets/icons/unread.svg");
}
.article-card__read-indicator--read {
  background-image: url("../assets/icons/read.svg");
}
.article-card__favorite-indicator {
  cursor: pointer;
  height: 1.5em;
  margin-left: 4px;
  margin-top: 8px;
  margin-right: 8px;
  width: 1.5em;
}
.article-card__favorite--not-favorite {
  background-image: url("../assets/icons/not_favorite.svg");
}
.article-card__favorite--favorite {
  background-image: url("../assets/icons/favorite.svg");
}
.article-card__chevron {
  cursor: pointer;
  height: 1.5em;
  margin-left: auto; /* push this to the end */
  margin-top: 8px;
  width: 1.5em;
}
.article-card__chevron--closed {
  background-image: url("../assets/icons/expand_more.svg");
}
.article-card__chevron--open {
  background-image: url("../assets/icons/expand_less.svg");
}
.article-card__contents-holder {
  margin: 16px auto;
}
.article-card__published-at {
  display: block;
  margin-top: 8px;
}
</style>
