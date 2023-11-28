import { defineStore } from 'pinia'
import type { Feed } from '@/models/Feed'
import http from '../api'
import type { Article, ArticleUpdateRequest } from '@/models/Article'

interface State {
  feeds: Feed[]
  articles: Article[]
}

export const useFeedStore = defineStore('feed', {
  state: (): State => ({
    feeds: [],
    articles: []
  }),
  getters: {
    unreadArticles(state): Article[] {
      return state.articles.filter((a) => !a.isRead)
    }
  },
  actions: {
    async fetchFeeds(): Promise<Feed[]> {
      try {
        const respone = await http.get('/feeds')
        this.feeds = respone.data
        return this.feeds
      } catch (error) {
        console.error(error)
        throw new Error(' ' + error)
      }
    },
    async fetchUnreadArticles(): Promise<Article[]> {
      try {
        const respone = await http.get('/articles')
        this.articles = respone.data
        return this.articles
      } catch (error) {
        console.error(error)
        throw new Error(' ' + error)
      }
    },
    updateArticleReadState(id: String, isRead: boolean) {
      const updatedArticles = this.articles.map((a) => {
        if (a.id === id) {
          return {
            ...a,
            isRead
          }
        }
        return a
      })
      this.articles = updatedArticles
      const req: ArticleUpdateRequest = {
        ids: [id],
        isRead
      }
      http
        .put('/articles', req)
        .then(() => {})
        .catch((e) => {
          console.error('update failed with ' + e)
        })
    },
    updateArticleFavouriteState(id: String, isFavourite: boolean) {
      const updatedArticles = this.articles.map((a) => {
        if (a.id === id) {
          return {
            ...a,
            isFavourite
          }
        }
        return a
      })
      this.articles = updatedArticles
      const req: ArticleUpdateRequest = {
        ids: [id],
        isFavourite
      }
      http
        .put('/articles', req)
        .then(() => {})
        .catch((e) => {
          console.error('update failed with ' + e)
        })
    }
  }
})
