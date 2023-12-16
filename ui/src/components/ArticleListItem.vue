<template>
  <v-card variant="elevated" @click="onDetailsToggle" class="mb-2">
    <template v-slot:prepend>
      <v-icon :icon="readIcon" @click="onReadToggle"></v-icon>
      <v-icon :icon="favoriteIcon" @click="onFavoriteToggle"></v-icon>
    </template>
    <template v-slot:append>
      <span class="text-body-2">{{ formattedDate }}</span>
      <v-icon :icon="chevronIcon"></v-icon>
    </template>
    <template v-slot:title>
      <span>{{ article.title }}</span>
    </template>
    <v-card-text v-if="showDetails">
      <div class="text-h4">
        <a :href="article.url" target="_blank">{{ article.title }}</a>
      </div>
      <div v-html="article.body"></div>
    </v-card-text>
  </v-card>
</template>
<script setup lang="ts">
import type { Article } from '@/models/Article'
import { computed, reactive, ref } from 'vue'

const emit = defineEmits<{
  'read-toggle': [isRead: boolean, id: string]
  'favorite-toggle': [isFavorite: boolean, id: string]
}>()
interface Props {
  article: Article
  feedName: string
}
const props = defineProps<Props>()

const showDetails = ref(false)
const isRead = ref(props.article.isRead)
const isFavorite = ref(props.article.isFavorite)
reactive({ showDetails, isRead, isFavorite })

const formattedDate = computed((): string => {
  const date = new Date(props.article.publishedAt)
  return `Published at: ${date.toLocaleString(navigator.language, {
    dateStyle: 'short',
    timeStyle: 'short',
    hour12: true
  })}`
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

const readIcon = computed((): string => {
  return isRead.value ? 'mdi-email-open-outline' : 'mdi-email-outline'
})

const favoriteIcon = computed((): string => {
  return isFavorite.value ? 'mdi-heart' : 'mdi-heart-outline'
})

const chevronIcon = computed((): string => {
  return showDetails.value ? 'mdi-chevron-up' : 'mdi-chevron-down'
})
</script>
<style scoped></style>
