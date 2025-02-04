import { z } from 'zod'

export const CategorySchema = z.object({
  id: z.string().uuid().nullable().default(null),
  title: z.string().default(''),
  slug: z.string().default(''),
})

export const WithCategoriesSchema = z.object({
  id: z.string().nullable(),
  categories: z.array(CategorySchema),
})

export const PostSchema = z.object({
  id: z.string().uuid().nullable().default(null),
  title: z.string().default(''),
  slug: z.string().default(''),
  body: z.string().nullable().default(null),
  locale: z.string().nullable().default(null),
  is_published: z.boolean().default(false),
  categories: z.array(CategorySchema).default([]),
  og_image: z.string().nullable().default(null),
})

export type Post = z.infer<typeof PostSchema>
export type Category = z.infer<typeof CategorySchema>
export type WithCategories = z.infer<typeof WithCategoriesSchema>
