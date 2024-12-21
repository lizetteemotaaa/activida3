-- up.sql

CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR NOT NULL
);

CREATE TABLE permisos (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR NOT NULL
);

CREATE TABLE usuarios (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    contraseña VARCHAR NOT NULL,
    rol_id INT REFERENCES roles(id),
    creado_en TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE usuario_permisos (
    usuario_id INT REFERENCES usuarios(id),
    permiso_id INT REFERENCES permisos(id),
    PRIMARY KEY (usuario_id, permiso_id)
);
-- down.sql

DROP TABLE usuario_permisos;
DROP TABLE usuarios;
DROP TABLE permisos;
DROP TABLE roles;
