-- Create profiles table
CREATE TABLE profiles (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    full_name TEXT NOT NULL,
    headline TEXT NOT NULL,
    summary TEXT NOT NULL,
    role TEXT NOT NULL,
    location TEXT NOT NULL,
    profile_image TEXT NOT NULL,
    availability TEXT NOT NULL,
    years_of_experience INTEGER NOT NULL,
    resume_url TEXT NOT NULL,
    email TEXT NOT NULL,
    work_philosophy TEXT NOT NULL,
    timezone TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create profile_specializations table
CREATE TABLE profile_specializations (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    profile_id INTEGER NOT NULL,
    specialization TEXT NOT NULL,
    FOREIGN KEY(profile_id) REFERENCES profiles(id) ON DELETE CASCADE
);

-- Create profile_tech_focus table
CREATE TABLE profile_tech_focus (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    profile_id INTEGER NOT NULL,
    tech_focus TEXT NOT NULL,
    FOREIGN KEY(profile_id) REFERENCES profiles(id) ON DELETE CASCADE
);

-- Create profile_languages table
CREATE TABLE profile_languages (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    profile_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    level TEXT NOT NULL,
    FOREIGN KEY(profile_id) REFERENCES profiles(id) ON DELETE CASCADE
);
