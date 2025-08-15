import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';

// Types
export type Alternative = {
  [key: string]: string;
};

export type Question = {
  _id: string;
  contenido: string;
  tipo_de_pregunta: string;
  etiqueta: string;
  alternativas: Alternative;
  respuestas: string[];
};

export type Exam = {
  _id: string;
  titulo: string;
  descripcion: string;
  instrucciones: string;
  preguntas: Question[];
};

export type Evaluation = {
  _id: string;
  nombre: string;
  descripcion: string;
  examenes: Exam[];
};

export type QuizData = {
  _id: string;
  fecha_tiempo_inicio: string;
  fecha_tiempo_fin: string;
  postulante_id: string;
  evaluacion: Evaluation;
};

export type QuizState = {
  data: QuizData | null;
  loading: boolean;
  error: string | null;
  currentExamIndex: number;
  currentQuestionIndex: number;
  userAnswers: Map<string, string>;
  showResults: boolean;
  startTime: Date | null;
  elapsedTime: string;
};

// Create the quiz store
function createQuizStore() {
  const initialState: QuizState = {
    data: null,
    loading: false,
    error: null,
    currentExamIndex: 0,
    currentQuestionIndex: 0,
    userAnswers: new Map(),
    showResults: false,
    startTime: null,
    elapsedTime: '00:00:00'
  };

  const { subscribe, set, update } = writable(initialState);

  let timer: ReturnType<typeof setInterval> | null = null;

  const updateTimer = () => {
    if (!browser) return;

    update(state => {
      if (!state.startTime) return state;

      const now = new Date();
      const diff = Math.floor((now.getTime() - state.startTime.getTime()) / 1000);

      const hours = Math.floor(diff / 3600).toString().padStart(2, '0');
      const minutes = Math.floor((diff % 3600) / 60).toString().padStart(2, '0');
      const seconds = (diff % 60).toString().padStart(2, '0');

      return {
        ...state,
        elapsedTime: `${hours}:${minutes}:${seconds}`
      };
    });
  };

  const startTimer = () => {
    if (!browser) return;

    if (timer) {
      clearInterval(timer);
    }
    timer = setInterval(updateTimer, 1000);
  };

  const stopTimer = () => {
    if (timer) {
      clearInterval(timer);
      timer = null;
    }
  };

  return {
    subscribe,
    update,
    set,
    // Load quiz data
    loadQuiz: async (quizId: string) => {
      update(state => ({ ...state, loading: true, error: null }));

      try {
        const apiUrl = import.meta.env.VITE_API_URL;
        if (!apiUrl) {
          throw new Error('API URL not configured. Please set VITE_API_URL environment variable.');
        }

        const response = await fetch(`${apiUrl}/quiz/${quizId}`);

        if (!response.ok) {
          throw new Error(`Failed to fetch quiz: ${response.status} ${response.statusText}`);
        }

        const quizData: QuizData = await response.json();
        const startTime = new Date();

        // Update quiz data with start time
        quizData.fecha_tiempo_inicio = startTime.toISOString();

        update(state => ({
          ...state,
          data: quizData,
          loading: false,
          startTime,
          elapsedTime: '00:00:00'
        }));

        startTimer();
      } catch (error) {
        update(state => ({
          ...state,
          loading: false,
          error: error instanceof Error ? error.message : 'Failed to load quiz'
        }));
      }
    },

    // Navigation methods
    selectAnswer: (questionId: string, answer: string) => {
      update(state => {
        const newAnswers = new Map(state.userAnswers);
        newAnswers.set(questionId, answer);
        return { ...state, userAnswers: newAnswers };
      });
    },

    goToNextQuestion: () => {
      update(state => {
        if (!state.data) return state;

        const currentExam = state.data.evaluacion.examenes[state.currentExamIndex];
        const isLastQuestion = state.currentQuestionIndex === currentExam.preguntas.length - 1;
        const isLastExam = state.currentExamIndex === state.data.evaluacion.examenes.length - 1;

        if (isLastQuestion) {
          if (isLastExam) {
            return finishQuizInternal(state);
          } else {
            return {
              ...state,
              currentExamIndex: state.currentExamIndex + 1,
              currentQuestionIndex: 0
            };
          }
        } else {
          return {
            ...state,
            currentQuestionIndex: state.currentQuestionIndex + 1
          };
        }
      });
    },

    goToPrevQuestion: () => {
      update(state => {
        if (state.currentQuestionIndex > 0) {
          return {
            ...state,
            currentQuestionIndex: state.currentQuestionIndex - 1
          };
        }
        return state;
      });
    },

    selectExam: (examIndex: number) => {
      update(state => ({
        ...state,
        currentExamIndex: examIndex,
        currentQuestionIndex: 0
      }));
    },

    finishQuiz: async () => {
      update(state => finishQuizInternal(state));
    },

    reset: () => {
      stopTimer();
      set(initialState);
    },

    // Cleanup method
    destroy: () => {
      stopTimer();
    }
  };

  function finishQuizInternal(state: QuizState): QuizState {
    if (!state.data) return state;

    stopTimer();

    const finishTime = new Date().toISOString();
    const updatedData = { ...state.data };
    updatedData.fecha_tiempo_fin = finishTime;

    // Store answers in the quiz data structure
    updatedData.evaluacion.examenes.forEach(exam => {
      exam.preguntas.forEach(question => {
        const answer = state.userAnswers.get(question._id);
        if (answer) {
          question.respuestas = [answer];
        }
      });
    });

    // In a real app, you would submit this data to the API
    console.log('Quiz completed:', updatedData);

    return {
      ...state,
      data: updatedData,
      showResults: true
    };
  }
}

// Create and export the quiz store
export const quizStore = createQuizStore();

// Derived stores for computed values
export const currentExam = derived(
  quizStore,
  ($quizStore) => {
    if (!$quizStore.data) return null;
    return $quizStore.data.evaluacion.examenes[$quizStore.currentExamIndex];
  }
);

export const currentQuestion = derived(
  [quizStore, currentExam],
  ([$quizStore, $currentExam]) => {
    if (!$currentExam) return null;
    return $currentExam.preguntas[$quizStore.currentQuestionIndex];
  }
);

export const isLastQuestion = derived(
  [quizStore, currentExam],
  ([$quizStore, $currentExam]) => {
    if (!$currentExam) return false;
    return $quizStore.currentQuestionIndex === $currentExam.preguntas.length - 1;
  }
);

export const isLastExam = derived(
  quizStore,
  ($quizStore) => {
    if (!$quizStore.data) return false;
    return $quizStore.currentExamIndex === $quizStore.data.evaluacion.examenes.length - 1;
  }
);

export const examProgress = derived(
  quizStore,
  ($quizStore) => {
    return (examIndex: number) => {
      if (!$quizStore.data) return 0;

      const exam = $quizStore.data.evaluacion.examenes[examIndex];
      if (!exam) return 0;

      const answeredQuestions = exam.preguntas.filter(q =>
        $quizStore.userAnswers.has(q._id)
      ).length;

      return Math.round((answeredQuestions / exam.preguntas.length) * 100);
    };
  }
);