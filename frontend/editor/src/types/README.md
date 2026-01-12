```typescript
// editor.ts
export interface Document {
  id: string
  title: string
  content: string
  createdAt: Date
  updatedAt: Date
}

export interface EditorSettings {
  theme: 'light' | 'dark'
  fontSize: number
  autoSave: boolean
}
```

## 參考 tradetioh 的 types

- `api.ts`
- `form.ts` 
