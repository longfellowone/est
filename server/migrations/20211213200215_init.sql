-- https://dba.stackexchange.com/questions/146906/insert-into-three-tables-with-many-to-many-from-one-table?
-- http://sqlfiddle.com/#!17/390a7/77
-- http://sqlfiddle.com/#!17/390a7/18

-- explain analyze

-- SELECT a.id, a.title, a.score
--      , ARRAY (
--         SELECT t.name
--         FROM   article_tag a_t
--                    JOIN   tag t ON t.id = a_t.tag_id
--         WHERE  a_t.article_id = a.id
--     -- ORDER  BY t.id  -- optionally sort array elements
--     ) AS names
-- FROM   article a
-- ORDER  BY a.score DESC
-- LIMIT  10;

CREATE TABLE estimate
(
    id       int primary key GENERATED BY DEFAULT AS IDENTITY,
    estimate text NOT NULL
);

CREATE TABLE assembly
(
    id       int primary key GENERATED BY DEFAULT AS IDENTITY,
    assembly text NOT NULL
);

CREATE TABLE estimate_assembly
(
    estimate_id int REFERENCES estimates (id) ON UPDATE CASCADE ON DELETE CASCADE,
    assembly_id int REFERENCES assemblies (id) ON UPDATE CASCADE,
    quantity    int NOT NULL,
    PRIMARY KEY (estimate_id, assembly_id)
);

INSERT INTO estimate (id, estimate)
VALUES (1, 'Estimate 1'),
       (2, 'Estimate 2'),
       (3, 'Estimate 3'),
       (4, 'Estimate 4'),
       (5, 'Estimate 5');

INSERT INTO assembly (id, assembly)
VALUES (1, 'Assembly 1'),
       (2, 'Assembly 2'),
       (3, 'Assembly 3'),
       (4, 'Assembly 4');

INSERT INTO estimate_assembly (estimate_id, assembly_id, quantity)
VALUES (1, 1, 100),
       (1, 2, 200),
       (1, 3, 300),
       (2, 4, 400);

-- CREATE TABLE projects
-- (
--     id   int primary key GENERATED BY DEFAULT AS IDENTITY,
--     name text NOT NULL
-- );
--
-- INSERT INTO projects
-- VALUES (1, 'Project 1'),
--        (2, 'Project 2'),
--        (3, 'Project 3');
--
-- CREATE TABLE project_estimates
-- (
--     project_id  int REFERENCES projects (id) ON UPDATE CASCADE,
--     estimate_id int REFERENCES estimates (id) ON UPDATE CASCADE ON DELETE CASCADE
-- );
--
-- INSERT INTO project_estimates (project_id, estimate_id)
-- VALUES (1, 1),
--        (1, 2),
--        (2, 3),
--        (2, 4),
--        (2, 5);
