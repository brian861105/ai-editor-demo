export type AIOption = 'continue' | 'improve' | 'shorter' | 'longer' | 'fix' | 'zap'

export interface GenerateRequest {
  prompt: string
  option: AIOption
  command?: string
}

export interface GenerateResponse {
  text: string
  error?: string
}

export interface AIError {
  message: string
  code?: string
  details?: unknown
}
