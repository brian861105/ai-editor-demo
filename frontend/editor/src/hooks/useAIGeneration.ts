import { useCompletion } from '@ai-sdk/react'

import type { AIOption } from '@/types/ai'

export interface UseAIGenerationOptions {
  onFinish?: () => void
  onError?: (error: Error) => void
}

export interface UseAIGenerationReturn {
  completion: string
  isLoading: boolean
  generate: (prompt: string, options: { option: AIOption; command?: string }) => Promise<void>
}

export function useAIGeneration(options?: UseAIGenerationOptions): UseAIGenerationReturn {
  const { completion, complete, isLoading } = useCompletion({
    api: '/api/generate',
    onFinish: options?.onFinish,
    onError: options?.onError
  })

  const generate = async (
    prompt: string,
    generateOptions: { option: AIOption; command?: string }
  ): Promise<void> => {
    await complete(prompt, {
      body: {
        option: generateOptions.option,
        command: generateOptions.command
      }
    })
  }

  return {
    completion,
    isLoading,
    generate
  }
}
