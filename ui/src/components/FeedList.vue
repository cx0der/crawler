<template>
  <aside>
    <nav>
      <ul class="feed">
        <li v-for="feed in feeds" :key="feed.id" class="feed__item">
          <a class="feed__item--link title-medium" href="#">{{ feed.name }}</a>
        </li>
      </ul>
    </nav>
  </aside>
</template>
<script setup lang="ts">
import type { Feed } from '@/models/Feed'

interface Props {
  feeds: Feed[]
}

defineProps<Props>()
</script>
<style scoped>
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
</style>
