import { RouteRecordRaw, RouteComponent } from 'vue-router'

import NotFound from '@/views/NotFound.vue'
import MainWrap from '@/views/MainWrap.vue'
import SimpleWrap from '@/views/SimpleWrap.vue'

const LoginPage = (): Promise<RouteComponent> => import('./views/LoginPage.vue')
const BlogPostList = (): Promise<RouteComponent> => import('./views/BlogPostList.vue')
const BlogPostEdit = (): Promise<RouteComponent> => import('./views/BlogPostEdit.vue')
const BlogCategoryList = (): Promise<RouteComponent> => import('./views/BlogCategoryList.vue')

export const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: MainWrap,
    children: [
      { path: 'posts', component: SimpleWrap, name: 'post', redirect: () => ({ name: 'post.list' }),
      children: [
        { path: '', component: BlogPostList, name: 'post.list' },
        { path: 'new', component: BlogPostEdit, name: 'post.new' },
        { path: ':id', component: BlogPostEdit, name: 'post.edit' },
      ]
      },
      { path: 'categories', component: BlogCategoryList, name: 'category.list' },
    ],
  },
  { path: '/login', component: LoginPage, name: 'login' },
  { path: '/:path(.*)', component: NotFound },
]
