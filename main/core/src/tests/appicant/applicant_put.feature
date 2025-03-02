Feature: Crear un candidato
  Para poder hacer la evaluacion para obtener la licencia de conducir
  Todo candidato debera registrarse con numero de documento legal unico
  Se requiere este paso para poder crear la evalacion

  Scenario: Registrar un nuevo candidato
    Given Envio una peticion PUT a "/applicant/6a09f79a-3a12-41d6-8316-4f2d0635737d" con el body:
    """
    {
      "nombre": "",
      "primer_apellido": "",
      "segundo_apellido": "",
      "document_number": ""
    }
    """
    Then el estado de la respuesta deberia ser 201
    And la respuesta debe ser un pin provisional de 4 digitos
    """
    {
      "pin": "1234"
    }
    """