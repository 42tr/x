CREATE TABLE `news` (
    `id` int unsigned NOT NULL,
    `content` text NOT NULL COMMENT '内容',
    `timestamp` bigint NOT NULL COMMENT '时间戳',
    `target` varchar(128) NOT NULL COMMENT '链接',
    PRIMARY KEY (`id`),
    KEY `time` (`timestamp`) USING BTREE
) ENGINE = InnoDB;

create table gold_info (
    date varchar(32) not null comment '日期',
    price float not null comment '价格',
    primary key (date)
);
