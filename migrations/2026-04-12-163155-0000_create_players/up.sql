CREATE TABLE `players`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL,
	`variant` INTEGER NOT NULL,
	`hashed_password` TEXT NOT NULL
);
