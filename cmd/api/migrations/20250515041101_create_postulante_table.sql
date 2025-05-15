CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE postulante
(
    id                UUID PRIMARY KEY         DEFAULT gen_random_uuid(),
    documento         VARCHAR(255) NOT NULL,
    nombre            VARCHAR(255) NOT NULL,
    apellido_paterno  VARCHAR(255) NOT NULL,
    apellido_materno  VARCHAR(255) NOT NULL,
    fecha_nacimiento  VARCHAR(255) NOT NULL,
    grado_instruccion VARCHAR(255) NOT NULL,
    genero            VARCHAR(50)  NOT NULL,
    created_at        TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at        TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
