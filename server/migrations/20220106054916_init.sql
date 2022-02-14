create table project
(
    project_id uuid primary key,
    project    text not null
);

insert into project (project_id, project)
values ('00000000-0000-0000-0000-000000000001', 'project 1'),
       ('00000000-0000-0000-0000-000000000002', 'project 2');

-- todo: remove on delete cascade and change to soft delete
create table estimate
(
    estimate_id uuid primary key,
    project_id  uuid not null references project (project_id) on update cascade on delete cascade,
    estimate    text not null
);

insert into estimate (estimate_id, project_id, estimate)
values ('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'estimate 1'),
       ('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000001', 'estimate 2'),
       ('00000000-0000-0000-0000-000000000003', '00000000-0000-0000-0000-000000000002', 'estimate 3');

create table estimate_groups
(
    group_id    uuid primary key,
    estimate_id uuid not null references estimate (estimate_id) on update cascade on delete cascade,
    name        text not null -- TODO: rename this
);

insert into estimate_groups (group_id, estimate_id, name)
values ('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'Default Group'),
       ('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000001', 'Another Custom Group');


create table assembly
(
    assembly_id uuid primary key,
    assembly    text not null
);

insert into assembly (assembly_id, assembly)
values ('00000000-0000-0000-0000-000000000001', 'assembly 1'),
       ('00000000-0000-0000-0000-000000000002', 'assembly 2'),
       ('00000000-0000-0000-0000-000000000003', 'assembly 3');

create table estimate_group_items
(
    id          uuid primary key,
    group_id    uuid not null references estimate_groups (group_id) on update cascade,
    assembly_id uuid not null references assembly ( assembly_id) on update cascade,
    -- TODO: Add product ID for union, use null
    quantity    int not null,
    unique (group_id, assembly_id)
);

-- TODO: Not sure indexes are required? does foreignkey automatically get indexed?
create index estimate_line_item_estimate_index on estimate_group_items (group_id);
create index estimate_line_item_assembly_index on estimate_group_items (assembly_id);

insert into estimate_group_items (id, group_id, assembly_id, quantity)
values ('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001',
        '00000000-0000-0000-0000-000000000001', 10),
       ('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000001',
        '00000000-0000-0000-0000-000000000002', 20),
       ('00000000-0000-0000-0000-000000000003', '00000000-0000-0000-0000-000000000001',
        '00000000-0000-0000-0000-000000000003', 30),
       ('00000000-0000-0000-0000-000000000004', '00000000-0000-0000-0000-000000000002',
        '00000000-0000-0000-0000-000000000001', 40);


create table product
(
    product_id uuid primary key,
    product    text not null,
    cost       int  not null, -- Cost in cents
    labour     int  not null  -- Labour in minutes
);

insert into product (product_id, product, cost, labour)
values ('00000000-0000-0000-0000-000000000001', 'product 1', 10, 60),
       ('00000000-0000-0000-0000-000000000002', 'product 2', 20, 30),
       ('00000000-0000-0000-0000-000000000003', 'product 3', 30, 15);

create table assembly_component
(
    id          uuid primary key,
    assembly_id uuid not null references assembly (assembly_id) on update cascade,
    product_id  uuid not null references product (product_id) on update cascade,
    quantity    int  not null,
    unique (assembly_id, product_id)
);

create index assembly_component_assembly_index on assembly_component (assembly_id);
create index assembly_component_product_index on assembly_component (product_id);


insert into assembly_component (id, assembly_id, product_id, quantity)
values ('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001',
        '00000000-0000-0000-0000-000000000001', 100),
       ('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000001',
        '00000000-0000-0000-0000-000000000003', 300),
       ('00000000-0000-0000-0000-000000000003', '00000000-0000-0000-0000-000000000002',
        '00000000-0000-0000-0000-000000000002', 200),
       ('00000000-0000-0000-0000-000000000004', '00000000-0000-0000-0000-000000000002',
        '00000000-0000-0000-0000-000000000003', 300);