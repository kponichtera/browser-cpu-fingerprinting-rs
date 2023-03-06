create table upload_benchmarkresult
(
    id                bigserial primary key,
    model             varchar(255) not null,
    user_agent        varchar(255) not null,
    benchmark_results jsonb        not null,
    b64_charts        jsonb        not null,
    times             jsonb        not null
);
