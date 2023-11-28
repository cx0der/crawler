export interface Article {
  id: string
  feedId: string
  title: string
  body: string
  url: string
  publishedAt: Date
  isRead: boolean
  isFavorite: boolean
}

export interface ArticleUpdateRequest {
  ids: String[]
  isRead?: boolean
  isFavourite?: boolean
}
