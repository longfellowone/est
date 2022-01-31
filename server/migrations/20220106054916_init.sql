CREATE TABLE project
(
    id      uuid PRIMARY KEY,
    project text NOT NULL
);

INSERT INTO project (id, project)
VALUES ('00000000-0000-0000-0000-000000000001', 'Project 1'),
       ('00000000-0000-0000-0000-000000000002', 'Project 2');

-- TODO: Remove ON DELETE CASCADE and change to soft delete
CREATE TABLE estimate
(
    id         uuid PRIMARY KEY,
    project_id uuid NOT NULL REFERENCES project (id) ON UPDATE CASCADE ON DELETE CASCADE,
    estimate   text NOT NULL,
    cost       int  NOT NULL
);

INSERT INTO estimate (id, project_id, estimate, cost)
VALUES ('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'Estimate 1', 100),
       ('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000001', 'Estimate 2', 200),
       ('00000000-0000-0000-0000-000000000003', '00000000-0000-0000-0000-000000000002', 'Estimate 3', 300);

CREATE TABLE assembly
(
    id       uuid PRIMARY KEY,
    assembly text    NOT NULL,
    cost     numeric NOT NULL
);

INSERT INTO assembly (id, assembly, cost)
VALUES ('00000000-0000-0000-0000-000000000001', 'Assembly 1', 99.995),
       ('00000000-0000-0000-0000-000000000002', 'Assembly 2', 200.00),
       ('00000000-0000-0000-0000-000000000003', 'Assembly 3', 300.00);

CREATE TABLE estimate_assemblies
(
    estimate_id uuid REFERENCES estimate (id) ON UPDATE CASCADE ON DELETE CASCADE,
    assembly_id uuid REFERENCES assembly (id) ON UPDATE CASCADE,
    quantity    int NOT NULL,
    PRIMARY KEY (estimate_id, assembly_id)
);

-- CREATE INDEX estimate_assemblies_estimate_index ON estimate_assemblies (estimate_id);
-- CREATE INDEX estimate_assemblies_assembly_index ON estimate_assemblies (assembly_id);

INSERT INTO estimate_assemblies (estimate_id, assembly_id, quantity)
VALUES ('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 10),
       ('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000002', 20),
       ('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000003', 30),
       ('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000001', 40);

CREATE TABLE item
(
    id   uuid PRIMARY KEY,
    item text NOT NULL,
    cost numeric NOT NULL
);

INSERT INTO item (id, item, cost)
VALUES ('00000000-0000-0000-0000-000000000001', 'Item 1', 9.995),
       ('00000000-0000-0000-0000-000000000002', 'Item 2', 20.00),
       ('00000000-0000-0000-0000-000000000003', 'Item 3', 30.00);

CREATE TABLE assembly_items
(
    assembly_id uuid REFERENCES assembly (id) ON UPDATE CASCADE,
    item_id     uuid REFERENCES item (id) ON UPDATE CASCADE,
    quantity    int NOT NULL,
    PRIMARY KEY (assembly_id, item_id)
);

-- CREATE INDEX assembly_items_assembly_index ON assembly_items (assembly_id);
-- CREATE INDEX assembly_items_item_index ON assembly_items (item_id);


INSERT INTO assembly_items (assembly_id, item_id, quantity)
VALUES ('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 100),
       ('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000003', 300),
       ('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000002', 200),
       ('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000003', 300);

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
