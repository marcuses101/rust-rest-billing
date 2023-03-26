CREATE TABLE lesson_students (
    lesson_id INTEGER NOT NULL REFERENCES lessons (id),
    student_id INTEGER NOT NULL REFERENCES students (id),
    PRIMARY KEY (lesson_id, student_id),
    CONSTRAINT unique_lesson_student UNIQUE (lesson_id, student_id)
);
