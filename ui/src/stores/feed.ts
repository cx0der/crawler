import { defineStore } from 'pinia'
import type { Feed } from '@/models/Feed'
import http from '../api'
import type { Article } from '@/models/Article'

interface MappedFeed {
  [id: string]: Feed,
}

interface State {
  rawFeeds: Feed[],
  mappedFeed: MappedFeed,
  rawUnreadArticles: Article[],
}

export const useFeedStore = defineStore('feed', {
  state: (): State => ({
    rawFeeds: [],
    mappedFeed: {},
    rawUnreadArticles: [],
  }),
  getters: {
    getFeeds(state) {
      return state.rawFeeds
    }
  },
  actions: {
    async fetchFeeds() {
      try {
        const respone = await http.get("/feeds")
        this.rawFeeds = respone.data
        const mappedFeed: MappedFeed = {};
        this.rawFeeds.forEach((f: Feed) => {
          mappedFeed[f.id] = f;
        })
        this.mappedFeed = mappedFeed;
      } catch (error) {
        console.error(error)
      }
    },
    async fetchUnreadArticles() {
      try {
        const respone = await http.get("/articles")
        this.rawUnreadArticles = respone.data
      } catch (error) {
        console.error(error)
      }
    }
  }
})
