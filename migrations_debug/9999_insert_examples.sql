-- Insert a board
INSERT INTO boards (title, lists_order) VALUES ('My Board', '[1,2,3]');
-- Get the ID of the inserted board

-- Insert "To Do" list with up to 4 cards
INSERT INTO lists (title, cards_order) VALUES ('To Do', '[3,2,1,4]');
INSERT INTO cards (title, list_id) VALUES
    ('Task 1', 1),
    ('Task 2', 1),
    ('Task 3', 1),
    ('Task 4', 1);

-- Insert "Doing" list with up to 4 cards
INSERT INTO lists (title, cards_order) VALUES ('Doing', '[6,5]');
INSERT INTO cards (title, list_id) VALUES
    ('Task 5', 2),
    ('Task 6', 2);

-- Insert "Done" list with up to 4 cards
INSERT INTO lists (title, cards_order) VALUES ('Done', '[12,11,7,8,9,10]');
INSERT INTO cards (title, list_id) VALUES
    ('Task 7', 3),
    ('Task 8', 3),
    ('Task 9', 3),
    ('Task 10', 3),
    ('Task 11', 3),
    ('Task 12', 3);
