CREATE TABLE reactions (
    reaction_id int(11) NOT NULL AUTO_INCREMENT,
    reaction_type int(3) NOT NULL,
    author_id int(11) NOT NULL,
    post_id int(11) NOT NULL,
    created DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (reaction_id),
    foreign key (post_id) references posts(post_id),
    foreign key (author_id) references users(user_id)
);

ALTER TABLE reactions ADD UNIQUE (author_id,post_id);