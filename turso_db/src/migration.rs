pub fn get_migration() -> String {
    r#"
-- 1) user_rol
CREATE TABLE user_rol (
    user_rol       TEXT PRIMARY KEY,
    deleted        INTEGER NOT NULL DEFAULT 0  -- 0 = false, 1 = true
);

-- 2) identification_type
CREATE TABLE identification_type (
    identification_type  TEXT PRIMARY KEY,
    deleted             INTEGER NOT NULL DEFAULT 0
);

-- 3) category
CREATE TABLE category (
    id_category   TEXT PRIMARY KEY,
    name          TEXT NOT NULL,
    min_age       INTEGER NOT NULL,
    max_age       INTEGER NOT NULL,
    deleted       INTEGER NOT NULL DEFAULT 0
);

-- 4) level
CREATE TABLE level (
    level_name    TEXT PRIMARY KEY,
    deleted       INTEGER NOT NULL DEFAULT 0
);

-- 5) person
CREATE TABLE person (
    id_user                TEXT PRIMARY KEY,
    first_name             TEXT NOT NULL,
    last_name              TEXT NOT NULL,
    birth_date             TEXT NOT NULL,             -- Example: 'YYYY-MM-DD'
    registration_date      TEXT NOT NULL,             -- Example: 'YYYY-MM-DD HH:MM:SS'
    email                  TEXT NOT NULL UNIQUE,
    email_verified         INTEGER NOT NULL DEFAULT 0,  -- 0 = false, 1 = true
    phone_number           TEXT NOT NULL,
    country_code           TEXT NOT NULL,
    password               TEXT NOT NULL,
    identification_number  TEXT NOT NULL,
    identification_type    TEXT NOT NULL,
    user_rol               TEXT NOT NULL,
    deleted                INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (identification_type) REFERENCES identification_type(identification_type),
    FOREIGN KEY (user_rol) REFERENCES user_rol(user_rol)
);

-- 6) category_requirement
CREATE TABLE category_requirement(
    id_category               TEXT NOT NULL,
    id_category_requirement   TEXT NOT NULL,
    requirement_description   TEXT,
    required_level            TEXT NOT NULL,
    deleted                   INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (id_category, id_category_requirement),
    FOREIGN KEY (id_category)    REFERENCES category(id_category),
    FOREIGN KEY (required_level) REFERENCES level(level_name)
);

-- 7) user_category
CREATE TABLE user_category (
    id_user      TEXT NOT NULL,
    id_category  TEXT NOT NULL,
    user_level   TEXT NOT NULL,
    deleted      INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (id_user, id_category),
    FOREIGN KEY (id_user)     REFERENCES person(id_user),
    FOREIGN KEY (id_category) REFERENCES category(id_category),
    FOREIGN KEY (user_level)  REFERENCES level(level_name)
);

-- 8) tournament
CREATE TABLE tournament (
    id_tournament  TEXT PRIMARY KEY,
    name           TEXT NOT NULL,
    id_category    TEXT NOT NULL,
    start_datetime TEXT NOT NULL,     -- Example: 'YYYY-MM-DD HH:MM:SS'
    end_datetime   TEXT NOT NULL,     -- Example: 'YYYY-MM-DD HH:MM:SS'
    deleted        INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (id_category) REFERENCES category(id_category)
);

-- 9) tournament_registration
CREATE TABLE tournament_registration (
    id_tournament        TEXT NOT NULL,
    id_user              TEXT NOT NULL,
    registration_datetime  TEXT NOT NULL,  -- Example: 'YYYY-MM-DD HH:MM:SS'
    deleted              INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (id_tournament, id_user),
    FOREIGN KEY (id_tournament) REFERENCES tournament(id_tournament),
    FOREIGN KEY (id_user)       REFERENCES person(id_user)
);

-- 10) tournament_attendance
CREATE TABLE tournament_attendance (
    id_tournament       TEXT NOT NULL,
    id_user             TEXT NOT NULL,
    attendance_datetime TEXT NOT NULL,   -- Example: 'YYYY-MM-DD HH:MM:SS'
    position            INTEGER NOT NULL,
    deleted             INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (id_tournament, id_user),
    FOREIGN KEY (id_tournament) REFERENCES tournament(id_tournament),
    FOREIGN KEY (id_user)       REFERENCES person(id_user)
);

-- 11) training
CREATE TABLE training (
    id_training    TEXT PRIMARY KEY,
    name           TEXT NOT NULL,
    id_category    TEXT NOT NULL,
    start_datetime TEXT NOT NULL,     -- Example: 'YYYY-MM-DD HH:MM:SS'
    end_datetime   TEXT NOT NULL,     -- Example: 'YYYY-MM-DD HH:MM:SS'
    minimum_payment REAL,
    deleted        INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (id_category) REFERENCES category(id_category)
);

-- 12) training_registration (updated: attendance_datetime is now nullable)
CREATE TABLE training_registration (
    id_training          TEXT NOT NULL,
    id_user              TEXT NOT NULL,
    registration_datetime TEXT NOT NULL,   -- Example: 'YYYY-MM-DD HH:MM:SS'
    attended             INTEGER NOT NULL DEFAULT 0,  -- 0 = false, 1 = true
    attendance_datetime  TEXT,              -- Changed to nullable
    deleted              INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (id_training, id_user),
    FOREIGN KEY (id_training) REFERENCES training(id_training),
    FOREIGN KEY (id_user)     REFERENCES person(id_user)
);

-- 13) tuition
CREATE TABLE tuition (
    id_tuition   TEXT PRIMARY KEY,
    id_user      TEXT NOT NULL,
    amount       REAL NOT NULL,
    payment_date TEXT NOT NULL,  -- Example: 'YYYY-MM-DD HH:MM:SS'
    deleted      INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (id_user) REFERENCES person(id_user)
);

-- 14) request_for_approval (new table)
CREATE TABLE request (
    request_id         TEXT PRIMARY KEY,
    requester_id       TEXT NOT NULL,
    requested_command  TEXT NOT NULL,
    justification      TEXT NOT NULL,
    approved           INTEGER,  -- Bool stored as INTEGER (0 = false, 1 = true)
    approver_id        TEXT,             -- Nullable, as approval may be pending
    deleted            INTEGER NOT NULL DEFAULT 0,  -- Added for consistency
    FOREIGN KEY (requester_id) REFERENCES person(id_user),
    FOREIGN KEY (approver_id)  REFERENCES person(id_user)
);
"#
    .to_string()
}
