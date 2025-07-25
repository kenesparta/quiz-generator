<script lang="ts">
  import { onMount } from 'svelte';

  // Types
  type Alternative = {
    [key: string]: string;
  };

  type Question = {
    _id: string;
    contenido: string;
    tipo_de_pregunta: string;
    etiqueta: string;
    alternativas: Alternative;
    respuestas: string[];
  };

  type Exam = {
    _id: string;
    titulo: string;
    descripcion: string;
    instrucciones: string;
    preguntas: Question[];
  };

  type Evaluation = {
    _id: string;
    nombre: string;
    descripcion: string;
    examenes: Exam[];
  };

  type QuizData = {
    _id: string;
    fecha_tiempo_inicio: string;
    fecha_tiempo_fin: string;
    postulante_id: string;
    evaluacion: Evaluation;
  };

  // Mock data - in a real app, you'd fetch this from an API
  const quizData: QuizData = {
    "_id": "fd470a84-11cd-40ce-94d4-3661fadf100f",
    "fecha_tiempo_inicio": "2025-07-24T04:31:54.830135+00:00",
    "fecha_tiempo_fin": "",
    "postulante_id": "",
    "evaluacion": {
      "_id": "2cf52b7a-0ee3-43a9-9b89-4a8baaa22250",
      "nombre": "Proceso de evaluacion A-I",
      "descripcion": "Aqui el proceso de evaluacion para la licencia A-I",
      "examenes": [
        {
          "_id": "19573e4f-321d-41ad-a8a9-3807c6fd3d65",
          "titulo": "Test de Audit",
          "descripcion": "Protocolo de Calificación - Evaluación Primaria",
          "instrucciones": "Marcar las opciones",
          "preguntas": [
            {
              "_id": "847d6869-4e61-41fb-9806-c14a74bb0c36",
              "contenido": "¿Con qué frecuencia consume alguna bebida alcohólica?",
              "tipo_de_pregunta": "alternativa_peso",
              "etiqueta": "",
              "alternativas": {
                "C": "De 2 a 4 veces al mes",
                "B": "Una o menos veces al mes",
                "A": "Nunca",
                "D": "De 2 a 3 veces a la semana",
                "E": "4 o más veces a la semana"
              },
              "respuestas": []
            },
            {
              "_id": "ef5ffb5e-da45-4da7-9c5d-39d309528748",
              "contenido": "¿Cuántos consumos de bebidas alcohólicas suele realizar en un día de consumo normal?",
              "tipo_de_pregunta": "alternativa_peso",
              "etiqueta": "",
              "alternativas": {
                "A": "1 ó 2",
                "D": "7, 8 ó 9",
                "E": "10 o más",
                "B": "3 ó 4",
                "C": "5 ó 6"
              },
              "respuestas": []
            },
            {
              "_id": "4f30b90d-5a1f-4504-b5ba-804719941034",
              "contenido": "¿Con qué frecuencia toma 6 o más bebidas alcohólicas en un solo día?",
              "tipo_de_pregunta": "alternativa_peso",
              "etiqueta": "",
              "alternativas": {
                "B": "Menos de una vez al mes",
                "A": "Nunca",
                "C": "Mensualmente",
                "E": "A diario o casi a diario",
                "D": "Semanalmente"
              },
              "respuestas": []
            },
            {
              "_id": "64e4d384-de17-406a-89bd-1ee271092ff3",
              "contenido": "¿Con qué frecuencia en el curso del último año ha sido incapaz de parar de beber una vez había empezado?",
              "tipo_de_pregunta": "alternativa_peso",
              "etiqueta": "",
              "alternativas": {
                "A": "Nunca",
                "C": "Mensualmente",
                "B": "Menos de una vez al mes",
                "D": "Semanalmente",
                "E": "A diario o casi a diario"
              },
              "respuestas": []
            },
            {
              "_id": "b0f0fecb-485d-444d-97cf-e92ed3325b2d",
              "contenido": "¿Con qué frecuencia en el curso del último año no pudo hacer lo que se esperaba de usted porque había bebido?",
              "tipo_de_pregunta": "alternativa_peso",
              "etiqueta": "",
              "alternativas": {
                "B": "Menos de una vez al mes",
                "E": "A diario o casi a diario",
                "A": "Nunca",
                "C": "Mensualmente",
                "D": "Semanalmente"
              },
              "respuestas": []
            },
            {
              "_id": "3ecbcb6f-ec1f-4f20-b44e-f93536b241d0",
              "contenido": "¿Con qué frecuencia en el curso del último año ha necesitado beber en ayunas para recuperarse después de haber bebido mucho el día anterior?",
              "tipo_de_pregunta": "alternativa_peso",
              "etiqueta": "",
              "alternativas": {
                "C": "Mensualmente",
                "D": "Semanalmente",
                "E": "A diario o casi a diario",
                "A": "Nunca",
                "B": "Menos de una vez al mes"
              },
              "respuestas": []
            },
            {
              "_id": "34c0e37a-a886-427c-b222-7e56033f9a03",
              "contenido": "¿Con qué frecuencia en el curso del último año ha tenido remordimientos o sentimientos de culpa después de haber bebido?",
              "tipo_de_pregunta": "alternativa_peso",
              "etiqueta": "",
              "alternativas": {
                "E": "A diario o casi a diario",
                "B": "Menos de una vez al mes",
                "D": "Semanalmente",
                "A": "Nunca",
                "C": "Mensualmente"
              },
              "respuestas": []
            },
            {
              "_id": "b930f6bc-312a-4475-8950-845656c4a362",
              "contenido": "¿Con qué frecuencia en el curso del último año no ha podido recordar lo que sucedió la noche anterior porque había estado bebiendo?",
              "tipo_de_pregunta": "alternativa_peso",
              "etiqueta": "",
              "alternativas": {
                "D": "Semanalmente",
                "E": "A diario o casi a diario",
                "B": "Menos de una vez al mes",
                "A": "Nunca",
                "C": "Mensualmente"
              },
              "respuestas": []
            },
            {
              "_id": "593bc5e3-e860-4169-ba3d-ee96cb37d23b",
              "contenido": "¿Usted o alguna otra persona ha resultado herido porque usted había bebido?",
              "tipo_de_pregunta": "alternativa_peso",
              "etiqueta": "",
              "alternativas": {
                "C": "Sí, en el último año",
                "A": "No",
                "B": "Sí, pero no en curso del último año"
              },
              "respuestas": []
            },
            {
              "_id": "6981ae59-0d1a-49b0-b901-2b9b56eb8f2c",
              "contenido": "¿Algún familiar, amigo, médico o profesional sanitario ha mostrado preocupación por su consumo de bebidas alcohólicas o le han sugerido que deje de beber?",
              "tipo_de_pregunta": "alternativa_peso",
              "etiqueta": "",
              "alternativas": {
                "A": "No",
                "B": "Sí, pero no en curso del último año",
                "C": "Sí, en el último año"
              },
              "respuestas": []
            }
          ]
        },
        {
          "_id": "2fcb7b0d-30e2-4853-afbf-9df79dd83ecb",
          "titulo": "Test de OTIS abreviado",
          "descripcion": "Protocolo de Calificación - Evaluación Primaria",
          "instrucciones": "Marcar las opciones",
          "preguntas": [
            {
              "_id": "cffa88a1-c4c5-490f-a6f7-31df77f7ddf5",
              "contenido": "Lo opuesto al ODIO es:",
              "tipo_de_pregunta": "alternativa_unica",
              "etiqueta": "",
              "alternativas": {
                "E": "Alegría",
                "A": "Enemigo",
                "C": "Amor",
                "B": "Temor",
                "D": "Amigo"
              },
              "respuestas": []
            },
            {
              "_id": "1a170d70-1ea8-41a0-a884-68cf567a2e37",
              "contenido": "Lo opuesto al HONOR es:",
              "tipo_de_pregunta": "alternativa_unica",
              "etiqueta": "",
              "alternativas": {
                "B": "Villanía",
                "D": "Cobardía",
                "C": "Humillación",
                "A": "Derrota",
                "E": "Miedo"
              },
              "respuestas": []
            },
            {
              "_id": "7eb5205d-3127-4aee-9a70-ebeb2d734fd6",
              "contenido": "El ZORRO se parece más a:",
              "tipo_de_pregunta": "alternativa_unica",
              "etiqueta": "",
              "alternativas": {
                "C": "El cerdo",
                "B": "La cabra",
                "D": "El tigre",
                "E": "El gato",
                "A": "El lobo"
              },
              "respuestas": []
            },
            {
              "_id": "4aac702c-ef1b-4379-9f74-552609f55536",
              "contenido": "El SILENCIO tiene la misma relación con el SONIDO, que la OSCURIDAD con:",
              "tipo_de_pregunta": "alternativa_unica",
              "etiqueta": "",
              "alternativas": {
                "D": "Quietud",
                "E": "Noche",
                "A": "Sótano",
                "C": "Ruido",
                "B": "Luz"
              },
              "respuestas": []
            },
            {
              "_id": "a0654ff1-d185-482a-be01-99f2a4086348",
              "contenido": "Un grupo consiste en dos matrimonios, dos hermanos, dos hermanas. ¿Cuál es número MÍNIMO de personas que podrían componer el grupo?",
              "tipo_de_pregunta": "alternativa_unica",
              "etiqueta": "",
              "alternativas": {
                "B": "6",
                "A": "4",
                "C": "8",
                "D": "10",
                "E": "12"
              },
              "respuestas": []
            },
            {
              "_id": "2bba8e13-2c4b-4f62-8edd-5ff55ec93f3d",
              "contenido": "Un ÁRBOL siempre tiene:",
              "tipo_de_pregunta": "alternativa_unica",
              "etiqueta": "",
              "alternativas": {
                "B": "Fruto",
                "C": "Yemas",
                "D": "Raíces",
                "E": "Sombra",
                "A": "Hojas"
              },
              "respuestas": []
            },
            {
              "_id": "72232d9f-de81-471c-a932-ca42bd1b98c2",
              "contenido": "Lo opuesto a ECONÓMICO es:",
              "tipo_de_pregunta": "alternativa_unica",
              "etiqueta": "",
              "alternativas": {
                "C": "Gastador",
                "A": "Barato",
                "D": "Valor",
                "E": "Rico",
                "B": "Avaro"
              },
              "respuestas": []
            },
            {
              "_id": "0d96f227-cf75-4743-8273-5825c4e01ae5",
              "contenido": "Una COMIDA siempre supone:",
              "tipo_de_pregunta": "alternativa_unica",
              "etiqueta": "",
              "alternativas": {
                "C": "Hambre",
                "A": "Mesa",
                "E": "Agua",
                "D": "Alimento",
                "B": "Plato"
              },
              "respuestas": []
            },
            {
              "_id": "e3ff2826-cbd9-4231-957e-d21e9a6544ce",
              "contenido": "De las 5 palabras siguientes 4 son parecidas. ¿Cuál es la NO PARECIDA a esas 4?",
              "tipo_de_pregunta": "alternativa_unica",
              "etiqueta": "",
              "alternativas": {
                "D": "Papel",
                "C": "Pato",
                "A": "Postre",
                "B": "Lima",
                "E": "Claro"
              },
              "respuestas": []
            },
            {
              "_id": "ec1f48e5-0a3b-468d-a05c-61ffa4c0b9e2",
              "contenido": "Lo opuesto a NUNCA es:",
              "tipo_de_pregunta": "alternativa_unica",
              "etiqueta": "",
              "alternativas": {
                "E": "De vez en cuando",
                "B": "A veces",
                "A": "A menudo",
                "D": "Siempre",
                "C": "Frecuentemente"
              },
              "respuestas": []
            },
            {
              "_id": "c052e558-f5e1-447f-a677-476637c143d9",
              "contenido": "Un RELOJ tiene con el TIEMPO, la misma relación que un TERMÓMETRO con:",
              "tipo_de_pregunta": "alternativa_unica",
              "etiqueta": "",
              "alternativas": {
                "E": "Temperatura",
                "C": "Tubo",
                "A": "Un reloj",
                "D": "Mercurio",
                "B": "Caliente"
              },
              "respuestas": []
            },
            {
              "_id": "71de10cc-24eb-4a5d-8229-9724e0555739",
              "contenido": "Una LUCHA siempre tiene:",
              "tipo_de_pregunta": "alternativa_unica",
              "etiqueta": "",
              "alternativas": {
                "B": "Contendientes",
                "C": "Espectadores",
                "E": "Victoria",
                "A": "Reñir",
                "D": "Aplausos"
              },
              "respuestas": []
            },
            {
              "_id": "86e9e704-b243-40bb-9e9c-21cc1f3c53a5",
              "contenido": "La LUNA se relaciona con la TIERRA, como la TIERRA con:",
              "tipo_de_pregunta": "alternativa_unica",
              "etiqueta": "",
              "alternativas": {
                "D": "Las estrellas",
                "B": "Sol",
                "A": "Marte",
                "E": "El Universo",
                "C": "Nubes"
              },
              "respuestas": []
            },
            {
              "_id": "80d09498-14b7-428a-ac06-fa02d6fb5e52",
              "contenido": "Lo opuesto a TORPE es:",
              "tipo_de_pregunta": "alternativa_unica",
              "etiqueta": "",
              "alternativas": {
                "D": "Hábil",
                "B": "Bonito",
                "C": "Corto",
                "A": "Fuerte",
                "E": "Rápido"
              },
              "respuestas": []
            },
            {
              "_id": "e8945335-d47d-4f2b-8236-c017540b7ce0",
              "contenido": "Una madre es _______ que su hija:",
              "tipo_de_pregunta": "alternativa_unica",
              "etiqueta": "",
              "alternativas": {
                "C": "Más gruesa",
                "D": "Más Vieja",
                "A": "Más sabia",
                "E": "Más arrugada",
                "B": "Más alta"
              },
              "respuestas": []
            },
            {
              "_id": "418b3b49-ee5d-4586-879e-7fe91943f06e",
              "contenido": "¿Qué se relaciona con ENFERMEDAD, como CUIDADO se relaciona con ACCIDENTE?",
              "tipo_de_pregunta": "alternativa_unica",
              "etiqueta": "",
              "alternativas": {
                "A": "Doctor",
                "B": "Cirugía",
                "C": "Medicina",
                "E": "Salubridad",
                "D": "Hospital"
              },
              "respuestas": []
            },
            {
              "_id": "c338e5b9-2bc2-4053-b450-2ed0a29d783b",
              "contenido": "Lo opuesto a ESPERANZA es:",
              "tipo_de_pregunta": "alternativa_unica",
              "etiqueta": "",
              "alternativas": {
                "C": "Tristeza",
                "A": "Fe",
                "E": "Odio",
                "B": "Desaliento",
                "D": "Desgracia"
              },
              "respuestas": []
            },
            {
              "_id": "d660a4d2-9209-4074-bc89-5931eb03547e",
              "contenido": "Lo que la gente dice una persona constituye su:",
              "tipo_de_pregunta": "alternativa_unica",
              "etiqueta": "",
              "alternativas": {
                "B": "Nombradía",
                "A": "Carácter",
                "C": "Reputación",
                "D": "Disposición",
                "E": "Personalidad"
              },
              "respuestas": []
            },
            {
              "_id": "46af27dd-069b-42b5-8c78-c6662685d505",
              "contenido": "Si las dos proposiciones son verdaderas, la tercera es:\n\n- **Jorge es mayor que Paco**\n- **Jaime es mayor que Jorge**\n- **Paco es menor que Jaime**",
              "tipo_de_pregunta": "alternativa_unica",
              "etiqueta": "",
              "alternativas": {
                "A": "Verdadera",
                "C": "Dudosa",
                "B": "Falsa"
              },
              "respuestas": []
            },
            {
              "_id": "f13069d5-8cd9-435d-a156-3580d89eb950",
              "contenido": "Lo opuesto a TRAIDOR es:",
              "tipo_de_pregunta": "alternativa_unica",
              "etiqueta": "",
              "alternativas": {
                "E": "Leal",
                "D": "Cobarde",
                "B": "Valiente",
                "C": "Sabio",
                "A": "Amistoso"
              },
              "respuestas": []
            },
            {
              "_id": "70057616-15b2-4fd7-93bf-0fa0037ca43d",
              "contenido": "¿Qué número está en el espacio que pertenece al rectángulo y al triángulo pero no al círculo?",
              "tipo_de_pregunta": "sola_respuesta",
              "etiqueta": "",
              "alternativas": {},
              "respuestas": []
            },
            {
              "_id": "e0cc56cf-3663-4c49-b85d-9f6c1dcf2a56",
              "contenido": "¿Qué número está en las mismas figuras geométricas que el número 8?",
              "tipo_de_pregunta": "sola_respuesta",
              "etiqueta": "",
              "alternativas": {},
              "respuestas": []
            },
            {
              "_id": "f98ad323-88f5-4de9-ade3-1d27ca7eaa72",
              "contenido": "Una SUPERFICIE se relaciona con LÍNEA, como una LÍNEA se relaciona con:",
              "tipo_de_pregunta": "alternativa_unica",
              "etiqueta": "",
              "alternativas": {
                "C": "Curva",
                "B": "Plano",
                "A": "Sólido",
                "E": "Hilo",
                "D": "Punto"
              },
              "respuestas": []
            },
            {
              "_id": "165ae139-7677-4068-9c31-1d4c9810ee0d",
              "contenido": "Un JUICIO cuyo significado no es definido, se dice que es:",
              "tipo_de_pregunta": "alternativa_unica",
              "etiqueta": "",
              "alternativas": {
                "D": "Desfigurado",
                "E": "Hipotético",
                "B": "Dudoso",
                "C": "Ambiguo",
                "A": "Erróneo"
              },
              "respuestas": []
            },
            {
              "_id": "69f96f38-b571-4f23-9b45-a1071ac330a0",
              "contenido": "Si se corta un alambre de 20 cm de largo de modo que un pedazo sea 2/3 del otro ¿Cuántos centímetros más corto será el menor?",
              "tipo_de_pregunta": "sola_respuesta",
              "etiqueta": "",
              "alternativas": {},
              "respuestas": []
            }
          ]
        }
      ]
    }
  };

  let currentExamIndex = 0;
  let currentQuestionIndex = 0;
  let userAnswers: Map<string, string> = new Map();
  let showResults = false;
  let elapsedTime = '00:00:00';
  let startTime = new Date();
  let timer: ReturnType<typeof setInterval>;

  $: currentExam = quizData.evaluacion.examenes[currentExamIndex];
  $: currentQuestion = currentExam?.preguntas[currentQuestionIndex];
  $: isLastQuestion = currentQuestionIndex === currentExam?.preguntas.length - 1;
  $: isLastExam = currentExamIndex === quizData.evaluacion.examenes.length - 1;

  function selectAnswer(questionId: string, option: string) {
    userAnswers.set(questionId, option);
    userAnswers = userAnswers; // Trigger reactivity
  }

  function goToNextQuestion() {
    if (isLastQuestion) {
      if (isLastExam) {
        finishQuiz();
      } else {
        currentExamIndex++;
        currentQuestionIndex = 0;
      }
    } else {
      currentQuestionIndex++;
    }
  }

  function goToPrevQuestion() {
    if (currentQuestionIndex > 0) {
      currentQuestionIndex--;
    }
  }

  function selectExam(index: number) {
    currentExamIndex = index;
    currentQuestionIndex = 0;
  }

  function finishQuiz() {
    clearInterval(timer);
    quizData.fecha_tiempo_fin = new Date().toISOString();

    // Store answers in the quizData structure
    quizData.evaluacion.examenes.forEach(exam => {
      exam.preguntas.forEach(question => {
        const answer = userAnswers.get(question._id);
        if (answer) {
          question.respuestas = [answer];
        }
      });
    });

    showResults = true;
    console.log('Quiz completed:', quizData);
  }

  function updateTimer() {
    const now = new Date();
    const diff = Math.floor((now.getTime() - startTime.getTime()) / 1000);

    const hours = Math.floor(diff / 3600).toString().padStart(2, '0');
    const minutes = Math.floor((diff % 3600) / 60).toString().padStart(2, '0');
    const seconds = (diff % 60).toString().padStart(2, '0');

    elapsedTime = `${hours}:${minutes}:${seconds}`;
  }

  // Check if all questions in the current exam are answered
  function examProgress(examIndex: number) {
    const exam = quizData.evaluacion.examenes[examIndex];
    if (!exam) return 0;

    const answeredQuestions = exam.preguntas.filter(q => userAnswers.has(q._id)).length;
    return Math.round((answeredQuestions / exam.preguntas.length) * 100);
  }

  onMount(() => {
    startTime = new Date();
    quizData.fecha_tiempo_inicio = startTime.toISOString();
    timer = setInterval(updateTimer, 1000);

    return () => {
      clearInterval(timer);
    };
  });
</script>

<div class="quiz-container">
    {#if !showResults}
        <div class="quiz-sidebar">
            <div class="exam-title">
                <div class="timer">{elapsedTime}</div>
            </div>

            <ul class="exam-tabs">
                {#each quizData.evaluacion.examenes as exam, index}
                    <li class:active={currentExamIndex === index}>
                        <button on:click={() => selectExam(index)}>
                            <div class="tab-content">
                                <span>{exam.titulo}</span>
                                <div class="progress-indicator">
                                    <div class="progress-bar" style="width: {examProgress(index)}%"></div>
                                </div>
                                <span class="progress-text">{examProgress(index)}%</span>
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
            <div class="exam-header">
                <h1>{quizData.evaluacion.nombre}</h1>
                <h2>{currentExam.titulo}</h2>
                <p class="instructions">Instrucciones: {currentExam.instrucciones}</p>
                <div class="question-progress">
                    Pregunta {currentQuestionIndex + 1} de {currentExam.preguntas.length}
                </div>
            </div>

            <div class="question-container">
                <h3>{currentQuestion.contenido}</h3>

                {#if currentQuestion.tipo_de_pregunta === 'alternativa_unica' || currentQuestion.tipo_de_pregunta === 'alternativa_peso'}
                    <div class="alternatives">
                        {#each Object.entries(currentQuestion.alternativas) as [key, value]}
                            <label class="alternative-option" class:selected={userAnswers.get(currentQuestion._id) === key}>
                                <input
                                        type="radio"
                                        name="question-{currentQuestion._id}"
                                        value={key}
                                        checked={userAnswers.get(currentQuestion._id) === key}
                                        on:change={() => selectAnswer(currentQuestion._id, key)}
                                />
                                <span class="option-text">{value}</span>
                            </label>
                        {/each}
                    </div>
                {:else if currentQuestion.tipo_de_pregunta === 'sola_respuesta'}
                    <div class="text-input">
                        <input
                                type="text"
                                placeholder="Ingrese su respuesta"
                                value={userAnswers.get(currentQuestion._id) || ''}
                                on:input={(e) => selectAnswer(currentQuestion._id, e.target.value)}
                        />
                    </div>
                {/if}
            </div>

            <div class="navigation-buttons">
                <button
                        class="btn btn-secondary"
                        on:click={goToPrevQuestion}
                        disabled={currentQuestionIndex === 0}
                >
                    Anterior
                </button>

                <button class="btn btn-primary" on:click={goToNextQuestion}>
                    {isLastQuestion ? (isLastExam ? 'Finalizar' : 'Siguiente examen') : 'Siguiente'}
                </button>
            </div>
        </div>
    {:else}
        <div class="results">
            <h1>Evaluación Completada</h1>
            <p>¡Gracias por completar la evaluación!</p>
            <p>Evaluación: {quizData.evaluacion.nombre}</p>
            <p>Tiempo total: {elapsedTime}</p>

            <div class="exams-summary">
                {#each quizData.evaluacion.examenes as exam, index}
                    <div class="exam-summary">
                        <h3>{exam.titulo}</h3>
                        <p>Preguntas respondidas: {exam.preguntas.filter(q => userAnswers.has(q._id)).length}/{exam.preguntas.length}</p>
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

    .quiz-sidebar {
        background: #eee;
        display: flex;
        flex-direction: column;
        padding: 0;
    }

    .exam-title {
        padding: 1rem;
        background: var(--color-accent-error);
        color: white;
    }

    .timer {
        align-self: center;
        text-align: center;
        font-family: monospace;
        font-size: 2rem;
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
        background-color: #ddd;
        overflow: hidden;
    }

    .progress-bar {
        height: 100%;
        background-color: var(--color-accent-success-hover, #28a745);
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
        /*display: flex;*/
        /*justify-content: space-between;*/
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