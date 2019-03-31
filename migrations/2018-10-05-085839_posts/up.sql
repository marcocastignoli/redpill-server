CREATE TABLE posts (
    post_id int(11) NOT NULL AUTO_INCREMENT,
    content varchar(50) NOT NULL,
    author_id int(11) NOT NULL,
    parent_id int(11) NULL,
    created DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (post_id),
    foreign key (author_id) references users(user_id)
);