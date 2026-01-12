/**
 * AI Service Layer
 * 
 * 這個服務層提供了 AI 功能的抽象接口。
 * 
 * 目前實作：調用 Next.js API Routes (/api/generate)
 * 未來實作：改成調用獨立後端 API (${API_ENDPOINT}/generate)
 * 
 * 優點：組件不需要知道 API 的實際位置，只需要調用這個服務
 */

import type { GenerateRequest } from '@/types/ai'

/**
 * 調用 AI 生成文本
 * 
 * @param request - 生成請求參數
 * @returns Response stream
 */
export async function generateText(request: GenerateRequest): Promise<Response> {
  // 目前：調用 Next.js API Route
  // 未來：改成 `${env.API_ENDPOINT}/generate`
  const endpoint = '/api/generate'
  
  const response = await fetch(endpoint, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(request)
  })

  if (!response.ok) {
    throw new Error(`AI generation failed: ${response.statusText}`)
  }

  return response
}

/**
 * 未來切換到獨立後端時，只需要改這裡：
 * 
 * export async function generateText(request: GenerateRequest): Promise<Response> {
 *   const endpoint = `${env.API_ENDPOINT}/generate`
 *   
 *   const response = await fetch(endpoint, {
 *     method: 'POST',
 *     headers: {
 *       'Content-Type': 'application/json',
 *       'Authorization': `Bearer ${getAuthToken()}` // 可能需要認證
 *     },
 *     body: JSON.stringify(request)
 *   })
 * 
 *   return response
 * }
 */
