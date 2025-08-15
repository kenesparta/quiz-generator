import type { QuizData } from '$lib/stores/quiz';

const API_BASE_URL = import.meta.env.VITE_API_URL;

if (!API_BASE_URL) {
  console.warn('VITE_API_URL is not set. API calls will fail.');
}

export class QuizApiError extends Error {
  constructor(
    message: string,
    public status?: number,
    public statusText?: string
  ) {
    super(message);
    this.name = 'QuizApiError';
  }
}

export class QuizApiService {
  private static async handleResponse<T>(response: Response): Promise<T> {
    if (!response.ok) {
      const errorMessage = `API request failed: ${response.status} ${response.statusText}`;
      throw new QuizApiError(errorMessage, response.status, response.statusText);
    }

    try {
      return await response.json();
    } catch (error) {
      throw new QuizApiError('Failed to parse response as JSON');
    }
  }

  static async fetchQuiz(quizId: string): Promise<QuizData> {
    if (!API_BASE_URL) {
      throw new QuizApiError('API URL not configured. Please set VITE_API_URL environment variable.');
    }

    try {
      const response = await fetch(`${API_BASE_URL}/quiz/${quizId}`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      return await this.handleResponse<QuizData>(response);
    } catch (error) {
      if (error instanceof QuizApiError) {
        throw error;
      }
      throw new QuizApiError(
        error instanceof Error ? error.message : 'Unknown error occurred while fetching quiz'
      );
    }
  }

  static async submitQuiz(quizData: QuizData): Promise<void> {
    if (!API_BASE_URL) {
      throw new QuizApiError('API URL not configured. Please set VITE_API_URL environment variable.');
    }

    try {
      const response = await fetch(`${API_BASE_URL}/quiz/${quizData._id}/submit`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(quizData),
      });

      await this.handleResponse<void>(response);
    } catch (error) {
      if (error instanceof QuizApiError) {
        throw error;
      }
      throw new QuizApiError(
        error instanceof Error ? error.message : 'Unknown error occurred while submitting quiz'
      );
    }
  }

  static async fetchQuizList(): Promise<Array<{ _id: string; title: string; description: string }>> {
    if (!API_BASE_URL) {
      throw new QuizApiError('API URL not configured. Please set VITE_API_URL environment variable.');
    }

    try {
      const response = await fetch(`${API_BASE_URL}/quizzes`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      return await this.handleResponse<Array<{ _id: string; title: string; description: string }>>(response);
    } catch (error) {
      if (error instanceof QuizApiError) {
        throw error;
      }
      throw new QuizApiError(
        error instanceof Error ? error.message : 'Unknown error occurred while fetching quiz list'
      );
    }
  }
}