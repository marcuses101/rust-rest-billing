CREATE TABLE lessons (
    id SERIAL PRIMARY KEY,
    coach_id integer not null references coaches (id),
    student_count INTEGER NOT NULL,
    lesson_duration_minutes INTEGER NOT NULL,
    lesson_date DATE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    total_cost_cents INTEGER NOT NULL
    CONSTRAINT positive_duration CHECK (lesson_duration_minutes > 0)
);

