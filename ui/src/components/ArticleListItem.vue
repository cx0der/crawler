<template>
  <article class="article-card on-surface">
    <div class="article-card__title-container">
      <span
        @click="onTitleClick"
        class="headline-small article-card__title">
        {{ article.title }}
      </span>
      <div
        :class="[showDetails ? 'article-card__chevron--open': 'article-card__chevron--closed']"
        @click="onDetailsToggle"
        class="article-card__chevron" />
    </div>
    <div v-if="showDetails" class="article-card__contents-holder">
      {{ article.body }}
    </div>
    <span class="body-medium">{{ formattedDate }}</span>
  </article>
</template>
<script setup lang="ts">
import type { Article } from '@/models/Article';
import { computed, reactive, ref } from 'vue';
interface Props {
  article: Article,
  feedName: String
}
const props = defineProps<Props>();

const showDetails = ref(false)
reactive({ showDetails })

const formattedDate = computed(() => {
  const date = new Date(props.article.publishedAt)
  return `Published at: ${date.toLocaleString()} by: ${props.feedName}`
})
const onTitleClick = () => {
  window.open(props.article.url, '_blank')
}

const onDetailsToggle = () => {
  showDetails.value = !showDetails.value
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
  justify-content: space-between;
}
.article-card__title:hover {
  cursor: pointer;
  text-decoration: underline;
}
.article-card__chevron {
  height: 1.5em;
  margin-top: 4px;
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
</style>
