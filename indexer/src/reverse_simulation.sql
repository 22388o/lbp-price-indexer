DROP TABLE IF EXISTS reverse_simulation;
CREATE TABLE reverse_simulation (
    height INT(11) NOT NULL ,
    offer_amount INT(11) NOT NULL ,
    spread_amount INT(11) NOT NULL ,
    commission_amount INT(11) NOT NULL ,
    ask_weight VARCHAR(255) NOT NULL ,
    offer_weight VARCHAR(255) NOT NULL ,
    block_time INT NOT NULL ,
    saved_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (height)
);