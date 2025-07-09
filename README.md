# Generador de preguntas para pruebas de manejo

## 1. Estructura del proyecto
- Puede encontrar más documentacion en este [🔗enlace](https://github.com/kenesparta/quiz-generator/wiki).

## 2. Bibliografia
- https://martinfowler.com/bliki/AnemicDomainModel.html

## Como comenzar


# Domain-driven Design (DDD)
Conceptos basicos

## Modelo
Cuando se hace el modelaje, se tiene que crear primero el objeto de valor, luego, la entidad y finalmente
el servicio de dominio
L1 -> L2 -> L3

### L1 Objeto de Valor

### L2 Entidad

### L3 Servicios de dominio
- No confundir Servicio de Aplicacion (cas0 de uso)
- Tiene como funcion de implementar una regla de negocio de dominio
- No trabaja con I/O, DB, coin otras APIs

#### Los servicios de dominio:
Es una operacion sin estado (stateless) que realiza una tarea específica de dominio, frecuentemente a mejor indicacion
que se debe crear un servicio en el modelo de dominio es cuando la operacion a ser ejecutada parece sin proposito en 
un agregado o un objeto de valor.

Para aliviar esa sensacion desconfortable, nuestra tendencia natural puede ser crear un metodo estatico en la clase de 
la Raiz de un Agregado. Pero, al utilizar DDD, esa tactica es un codigo que probablemente indica que se necesita, 
en vez de eso, de un Servicio.

- Se usan cuando existen reglas con objetos no relacionados
- Cuando una regla manipula una lista de objetos
- Extraer una regla grande, esa regla puede ser un servicio de dominio

## Casos de uso
Son los flujos de la aplicacion.
- Describe los flujos de los eventos de como el sistema se debe comportar
- Son usados para describir las interacciones entre usuarios y sistemas
- Ofrecen una vision general del sistema y sus funcionalidades.
> La logica queda dentro del caso de la entidad y no del caso de uso.

### Flujos
- Frontend -> Backend -> Casos de Uso (Usa los casos de uso/) -> Modelo de dominio
- Frontend -> Casos de uso -> Modelo de dominio

## Agregados (raiz)
- Union de varias entidades y objetos de valor que son persistidos de forma transaccional (persistidos juntos).

## Eventos de dominio
- Use un evento de dominio para capturar una ocurrencia de algo que acontecio en el Dominio. Esta es una herramienta 
  estremadamente poderosa de modelaje. Despues de dominar el uso de eventos de domnio, nos quedaremoos viciados de 
  usarlo y nos preguntaremos com hemos podido vivir sin el.
