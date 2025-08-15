<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { page } from '$app/state';
  import {
    quizStore,
    currentExam,
    currentQuestion,
    isLastQuestion,
    isLastExam,
    examProgress,
    type QuizData
  } from '$lib/stores/quiz';
  import { QuizApiService } from '$lib/api/quiz';

  // Get quiz ID from URL params or props
  export let quizId: string = '';

  // Reactive statements for store values
  $: quizState = $quizStore;
  $: currentExamData = $currentExam;
  $: currentQuestionData = $currentQuestion;
  $: isLastQuestionValue = $isLastQuestion;
  $: isLastExamValue = $isLastExam;
  $: getExamProgress = $examProgress;

  // Load quiz data on mount
  onMount(async () => {
    // Get quiz ID from URL params if not provided as prop
    const urlQuizId = page.params?.id || quizId;

    if (!urlQuizId) {
      quizStore.update(state => ({
        ...state,
        error: 'Quiz ID is required'
      }));
      return;
    }

    try {
      await quizStore.loadQuiz(urlQuizId);
    } catch (error) {
      console.error('Failed to load quiz:', error);
    }
  });

  // Cleanup on destroy
  onDestroy(() => {
    quizStore.destroy();
  });

  function selectAnswer(questionId: string, option: string) {
    quizStore.selectAnswer(questionId, option);
  }

  function goToNextQuestion() {
    quizStore.goToNextQuestion();
  }

  function goToPrevQuestion() {
    quizStore.goToPrevQuestion();
  }

  function selectExam(index: number) {
    quizStore.selectExam(index);
  }

  async function finishQuiz() {
    try {
      await quizStore.finishQuiz();

      // Optionally submit to API
      if (quizState.data) {
        await QuizApiService.submitQuiz(quizState.data);
      }
    } catch (error) {
      console.error('Failed to finish quiz:', error);
      // Still show results even if submission fails
      await quizStore.finishQuiz();
    }
  }

  // Handle loading and error states
  $: if (quizState.loading) {
    console.log('Loading quiz...');
  }

  $: if (quizState.error) {
    console.error('Quiz error:', quizState.error);
  }
</script>

<div class="quiz-container">
    {#if quizState.loading}
        <div class="loading-container">
            <div class="loading-spinner"></div>
            <p>Cargando evaluación...</p>
        </div>
    {:else if quizState.error}
        <div class="error-container">
            <h2>Error</h2>
            <p>{quizState.error}</p>
            <button class="btn btn-primary" on:click={() => window.location.reload()}>
                Reintentar
            </button>
        </div>
    {:else if !quizState.data}
        <div class="loading-container">
            <p>No se encontraron datos de la evaluación.</p>
        </div>
    {:else if !quizState.showResults}
        <div class="quiz-sidebar">
            <div class="exam-title">
                <div class="timer">{quizState.elapsedTime}</div>
            </div>

            <ul class="exam-tabs">
                {#each quizState.data.evaluacion.examenes as exam, index}
                    <li class:active={quizState.currentExamIndex === index}>
                        <button on:click={() => selectExam(index)}>
                            <div class="tab-content">
                                <span>{exam.titulo}</span>
                                <div class="progress-indicator">
                                    <div class="progress-bar" style="width: {getExamProgress(index)}%"></div>
                                </div>
                                <span class="progress-text">{getExamProgress(index)}%</span>
                            </div>
                        </button>
                    </li>
                {/each}
            </ul>

            <button class="finish-button" on:click={finishQuiz}>
                Finalizar evaluación
            </button>
        </div>

        <div class="quiz-content">
            {#if currentExamData && currentQuestionData}
                <div class="exam-header">
                    <h1>{quizState.data.evaluacion.nombre}</h1>
                    <h2>{currentExamData.titulo}</h2>
                    <p class="instructions">Instrucciones: {currentExamData.instrucciones}</p>
                    <div class="question-progress">
                        Pregunta {quizState.currentQuestionIndex + 1} de {currentExamData.preguntas.length}
                    </div>
                </div>

                <div class="question-container">
                    <h3>{currentQuestionData.contenido}</h3>

                    {#if currentQuestionData.tipo_de_pregunta === 'alternativa_unica' || currentQuestionData.tipo_de_pregunta === 'alternativa_peso'}
                        <div class="alternatives">
                            {#each Object.entries(currentQuestionData.alternativas) as [key, value]}
                                <label class="alternative-option"
                                       class:selected={quizState.userAnswers.get(currentQuestionData._id) === key}>
                                    <input
                                            type="radio"
                                            name="question-{currentQuestionData._id}"
                                            value={key}
                                            checked={quizState.userAnswers.get(currentQuestionData._id) === key}
                                            on:change={() => selectAnswer(currentQuestionData._id, key)}
                                    />
                                    <span class="option-text">{value}</span>
                                </label>
                            {/each}
                        </div>
                    {:else if currentQuestionData.tipo_de_pregunta === 'sola_respuesta'}
                        <div class="text-input">
                            <input
                                    type="text"
                                    placeholder="Ingrese su respuesta"
                                    value={quizState.userAnswers.get(currentQuestionData._id) || ''}
                                    on:input={(e) => selectAnswer(currentQuestionData._id, e.currentTarget.value)}
                            />
                        </div>
                    {/if}
                </div>

                <div class="navigation-buttons">
                    <button
                            class="btn btn-secondary"
                            on:click={goToPrevQuestion}
                            disabled={quizState.currentQuestionIndex === 0}
                    >
                        Anterior
                    </button>

                    <button class="btn btn-primary" on:click={goToNextQuestion}>
                        {isLastQuestionValue ? (isLastExamValue ? 'Finalizar' : 'Siguiente examen') : 'Siguiente'}
                    </button>
                </div>
            {/if}
        </div>
    {:else}
        <div class="results">
            <h1>Evaluación Completada</h1>
            <p>¡Gracias por completar la evaluación!</p>
            <p>Evaluación: {quizState.data.evaluacion.nombre}</p>
            <p>Tiempo total: {quizState.elapsedTime}</p>

            <div class="exams-summary">
                {#each quizState.data.evaluacion.examenes as exam, index}
                    <div class="exam-summary">
                        <h3>{exam.titulo}</h3>
                        <p>Preguntas respondidas: {exam.preguntas.filter(q => quizState.userAnswers.has(q._id)).length}
                            /{exam.preguntas.length}</p>
                    </div>
                {/each}
            </div>

            <button class="btn btn-primary" on:click={() => window.location.href = '/dashboard'}>
                Volver al Dashboard
            </button>
        </div>
    {/if}
</div>

<style>
    .quiz-container {
        display: grid;
        grid-template-columns: 350px 1fr;
        height: 100%;
        min-height: 600px;
        background: white;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    }

    .loading-container,
    .error-container {
        grid-column: 1 / span 2;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        padding: 2rem;
        text-align: center;
        min-height: 400px;
    }

    .loading-spinner {
        width: 40px;
        height: 40px;
        border: 4px solid #f3f3f3;
        border-top: 4px solid var(--color-primary, #3498db);
        border-radius: 50%;
        animation: spin 1s linear infinite;
        margin-bottom: 1rem;
    }

    @keyframes spin {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }

    .error-container h2 {
        color: #e74c3c;
        margin-bottom: 1rem;
    }

    .quiz-sidebar {
        background: #eee;
        display: flex;
        flex-direction: column;
        padding: 0;
    }

    .exam-title {
        padding: 1rem;
        background: var(--color-input-focus);
    }

    .timer {
        align-self: center;
        text-align: center;
        font-family: monospace;
        font-size: 2rem;
        font-weight: bold;
        color: white;
    }

    .exam-tabs {
        list-style: none;
        padding: 0;
        margin: 0;
        flex-grow: 1;
        overflow-y: auto;
    }

    .exam-tabs li {
        border-bottom: 1px solid #ddd;
    }

    .exam-tabs li.active {
        background-color: #e0e0e0;
    }

    .exam-tabs button {
        width: 100%;
        padding: 1rem;
        text-align: left;
        background: none;
        border: none;
        cursor: pointer;
        font-size: 0.9rem;
        font-weight: 500;
        transition: background-color 0.2s;
    }

    .exam-tabs button:hover {
        background-color: #e0e0e0;
    }

    .tab-content {
        display: flex;
        flex-direction: column;
    }

    .progress-indicator {
        margin-top: 0.5rem;
        height: 4px;
        background-color: var(--color-input-shadow-overlay);
        overflow: hidden;
    }

    .progress-bar {
        height: 100%;
        background-color: var(--color-primary);
        transition: width 0.3s ease;
    }

    .progress-text {
        font-size: 0.75rem;
        color: #666;
        margin-top: 0.25rem;
        align-self: flex-end;
    }

    .finish-button {
        margin: 1rem;
        padding: 0.75rem;
        background-color: var(--color-primary);
        color: white;
        border: 2px solid #000;
        font-weight: bold;
        font-size: 1.2rem;
        cursor: pointer;
        transition: background-color 0.3s;
    }

    .finish-button:hover {
        background-color: var(--color-primary-hover);
    }

    .quiz-content {
        padding: 1rem;
        overflow-y: auto;
    }

    .exam-header {
        margin-bottom: 0.5rem;
    }

    .exam-header h1 {
        font-size: 1.5rem;
        font-weight: bold;
        margin: 0 0 .2rem 0;
        color: var(--color-primary);
    }

    .exam-header h2 {
        font-size: 1.2rem;
        font-weight: bold;
        margin-top: 0;
        margin-bottom: 0.5rem;
        color: var(--color-secondary);
    }

    .instructions {
        font-style: italic;
        margin: 1rem 0 1.5rem 0;
    }

    .question-progress {
        font-size: 1.2rem;
        font-weight: bold;
        margin-bottom: 0;
        color: var(--color-accent-warning);
    }

    .question-container {
        margin-bottom: 2rem;
    }

    .question-container h3 {
        margin-bottom: 1.5rem;
    }

    .alternatives {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .alternative-option {
        display: flex;
        align-items: center;
        padding: 0.75rem;
        border: 2px solid #000;
        cursor: pointer;
        transition: all 0.3s;
    }

    .alternative-option:hover {
        background-color: var(--color-input-border);
        border-color: var(--color-secondary);
    }

    .alternative-option.selected {
        background-color: var(--color-input-shadow-overlay);
        border-color: var(--color-primary);
        font-weight: bold;
    }

    .option-text {
        margin-left: 0.5rem;
    }

    .text-input input {
        width: 100%;
        padding: 0.75rem;
        border: 1px solid #ddd;
        font-size: 1rem;
    }

    .navigation-buttons {
        display: flex;
        justify-content: space-between;
        margin-top: 2rem;
    }

    .btn {
        padding: 0.75rem 1.5rem;
        cursor: pointer;
        font-weight: bold;
        border: 2px solid #000;
        transition: background-color 0.2s;
    }

    .btn-primary {
        background-color: var(--color-accent-success);
        font-size: 0.9rem;
        color: black;
    }

    .btn-primary:hover {
        background-color: var(--color-accent-success-hover);
    }

    .btn-secondary {
        background-color: var(--color-secondary);
        font-size: 0.9rem;
        color: #fff;
    }

    .btn-secondary:hover {
        background-color: var(--color-secondary-hover);
        color: #000;
    }

    .btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .results {
        grid-column: 1 / span 2;
        padding: 2rem;
        text-align: center;
    }

    .exams-summary {
        display: flex;
        flex-wrap: wrap;
        gap: 1rem;
        justify-content: center;
        margin: 2rem 0;
    }

    .exam-summary {
        background-color: #f5f5f5;
        padding: 1rem;
        min-width: 200px;
    }
</style>