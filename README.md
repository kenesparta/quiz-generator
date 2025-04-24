# Generador de preguntas para pruebas de manejo

## 1. Estructura del proyecto
- Puede encontrar más documentacion en este [🔗enlace](https://github.com/kenesparta/quiz-generator/wiki).

## 2. Bibliografia
- https://martinfowler.com/bliki/AnemicDomainModel.html

## Como comenzar


# Domain-driven Design (DDD)
Conceptos basicos
## L1 Objeto de Valor

## L2 Entidad

## L3 Servicios de dominio
- No confundir Servicio de Aplicacion (cas0 de uso)
- Tiene como funcion de implementar una regla de negocio de dominio
- No trabaja con I/O, DB, coin otras APIs
### Reglas
- Se usa cuando existen reglas con objetos no relacionados
- Cuando una regla manipula una lista de objetos
- Extraer una regla grande, esa regla puede ser un servicio de dominio


Cuando se hace el modelaje, se tiene que crear primero el objeto de valor, luego, la entidad y finalmente
el servicio de dominio
L1 -> L2 -> L3

## Agregados (raiz)
