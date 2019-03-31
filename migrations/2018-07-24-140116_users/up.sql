CREATE TABLE users (
  user_id INT(11) PRIMARY KEY AUTO_INCREMENT,
  email VARCHAR(60) NOT NULL UNIQUE,
  password VARCHAR(60) NOT NULL,
  created DATETIME DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO users (email, password) VALUES ('marco.castignoli@gmail.com', '12345');